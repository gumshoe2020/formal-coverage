"""Recording job: a local bare repository stands in for the remote (no network)."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import fetch_data_branch
import record
from test_recorded import RecordedRepo, encoded, git
from test_write_coverage import WriteCoverageTests
from test_ratchet import fixture, with_depth

SHA1, SHA2 = '1' * 40, '2' * 40


class RecordTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        self.remote = self.root / 'remote.git'
        subprocess.run(['git', 'init', '--bare', '-q', '-b', 'main', str(self.remote)], check=True)
        self.url = 'file://' + str(self.remote)
        self.report = WriteCoverageTests().report()

    def source(self, sha, report=None, branch='coverage-data'):
        path = Path(tempfile.mkdtemp(dir=self.root))
        (path / 'baseline.json').write_text(encoded(report or self.report))
        (path / 'COVERAGE-MEASURED.md').write_text(f'measured for {sha}\n')
        (path / 'sha').write_text(sha + '\n')
        (path / 'branch').write_text(branch + '\n')
        return path

    def remote_git(self, *args):
        return subprocess.check_output(['git', '--git-dir', str(self.remote), *args], text=True).strip()

    def test_first_record_creates_orphan_data_branch(self):
        self.assertTrue(record.record(self.source(SHA1), self.url))
        self.assertEqual(self.remote_git('show', f'coverage-data:baselines/{SHA1}.json'), encoded(self.report).strip())
        self.assertIn(SHA1, self.remote_git('show', 'coverage-data:COVERAGE-MEASURED.md'))
        self.assertEqual(self.remote_git('rev-list', '--count', 'coverage-data'), '1')

    def test_second_record_for_new_sha_appends_and_refreshes_page(self):
        record.record(self.source(SHA1), self.url)
        other = fixture(proven=40, tested=30, unverified=30) | {'proof_ratio': self.report['proof_ratio']}
        record.record(self.source(SHA2, other), self.url)
        files = self.remote_git('ls-tree', '--name-only', '-r', 'coverage-data').split()
        self.assertEqual(sorted(files), sorted([f'baselines/{SHA1}.json', f'baselines/{SHA2}.json', 'COVERAGE-MEASURED.md']))
        self.assertEqual(self.remote_git('rev-list', '--count', 'coverage-data'), '2')
        self.assertIn(SHA2, self.remote_git('show', 'coverage-data:COVERAGE-MEASURED.md'))
        self.assertEqual(self.remote_git('show', f'coverage-data:baselines/{SHA1}.json'), encoded(self.report).strip())

    def test_existing_sha_is_never_overwritten(self):
        record.record(self.source(SHA1), self.url)
        before = self.remote_git('rev-parse', 'coverage-data')
        different = fixture(proven=90, tested=5, unverified=5) | {'proof_ratio': self.report['proof_ratio']}
        with self.assertRaisesRegex(ValueError, 'append-only'):
            record.record(self.source(SHA1, different), self.url)
        self.assertEqual(self.remote_git('rev-parse', 'coverage-data'), before)

    def test_identical_rerecord_is_a_noop(self):
        record.record(self.source(SHA1), self.url)
        before = self.remote_git('rev-parse', 'coverage-data')
        self.assertFalse(record.record(self.source(SHA1), self.url))
        self.assertEqual(self.remote_git('rev-parse', 'coverage-data'), before)

    def test_only_the_data_branch_is_touched(self):
        record.record(self.source(SHA1), self.url)
        self.assertEqual(self.remote_git('for-each-ref', '--format=%(refname)').split(), ['refs/heads/coverage-data'])

    def test_invalid_records_are_rejected(self):
        for mutate in (lambda p: (p / 'sha').write_text('abc\n'),
                       lambda p: (p / 'branch').write_text('main\n'),
                       lambda p: (p / 'branch').write_text('../x\n'),
                       lambda p: (p / 'baseline.json').write_text('{}'),
                       lambda p: (p / 'baseline.json').write_text('nope')):
            source = self.source(SHA1)
            mutate(source)
            with self.subTest(), self.assertRaises(ValueError):
                record.record(source, self.url)
        self.assertEqual(self.remote_git('for-each-ref').split(), [])

    def test_publish_restricted_to_push_on_main_for_pushed_sha(self):
        source = self.source(SHA1)
        env = {'GITHUB_EVENT_NAME': 'pull_request', 'GITHUB_REF': 'refs/heads/main',
               'GITHUB_REPOSITORY': 'o/r', 'GITHUB_SHA': SHA1, 'GH_TOKEN': 't'}
        with patch.dict(os.environ, env):
            with self.assertRaisesRegex(ValueError, 'restricted'):
                record.publish(source)
        env.update(GITHUB_EVENT_NAME='push', GITHUB_SHA=SHA2)
        with patch.dict(os.environ, env):
            with self.assertRaisesRegex(ValueError, 'pushed commit'):
                record.publish(source)

    def test_end_to_end_main_records_then_pr_reads_merge_base_floor(self):
        repo = RecordedRepo(tempfile.mkdtemp(dir=self.root), {})
        git(repo.work, 'remote', 'add', 'origin', self.url)
        # main's push run: no floor yet (first adoption, no bootstrap) -> fails closed but records.
        result, out = repo.run(with_depth(self.report, 2, 3))
        self.assertNotEqual(result.returncode, 0)
        sha = git(repo.work, 'rev-parse', 'HEAD')
        record.record(out / 'record', self.url)
        # a PR branched from that main commit: fetch the data branch, compare to the recorded sha.
        (repo.work / 'src').mkdir()
        (repo.work / 'src/parser.rs').write_text('code\n' * 10)
        repo.commit('pr change')
        self.assertTrue(fetch_data_branch.fetch(repo.work, repo.work / 'coverage/ci.json'))
        result, out = repo.run(with_depth(self.report, 2, 4), base=sha)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('ratchet PASS', (out / 'summary.md').read_text())

    def test_fetch_tolerates_missing_branch_and_skips_committed_mode(self):
        repo = RecordedRepo(tempfile.mkdtemp(dir=self.root), {})
        git(repo.work, 'remote', 'add', 'origin', self.url)
        self.assertFalse(fetch_data_branch.fetch(repo.work, repo.work / 'coverage/ci.json'))
        (repo.work / 'coverage/ci.json').write_text('{}')
        self.assertFalse(fetch_data_branch.fetch(repo.work, repo.work / 'coverage/ci.json'))


if __name__ == '__main__':
    unittest.main()
