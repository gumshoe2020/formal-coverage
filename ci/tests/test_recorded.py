"""Recorded-floor mode: floor read from the data branch, nothing numeric committed in PRs."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import config
import run
from coverage_metrics import snapshot
from test_ratchet import fixture, EXCEPTION
from test_write_coverage import WriteCoverageTests

ROOT = Path(__file__).resolve().parents[2]
IDENT = ['-c', 'user.name=Test', '-c', 'user.email=test@example.invalid']


def git(work, *args):
    return subprocess.check_output(['git', '-C', str(work), *IDENT, *args], text=True,
                                   stderr=subprocess.DEVNULL).strip()


def encoded(report):
    return json.dumps(snapshot(report), indent=2, sort_keys=True) + '\n'


class RecordedRepo:
    """A project whose main has one commit; measurement tool replays tool_report.json."""

    def __init__(self, directory, ci_config=None):
        self.work = Path(directory)
        git(self.work, 'init', '-q', '-b', 'main')
        self.config = {'floor': 'recorded', **(ci_config or {})}
        (self.work / 'coverage').mkdir()
        (self.work / 'coverage/ci.json').write_text(json.dumps(self.config))
        (self.work / 'COVERAGE.md').write_text('Prose only\n\n## Exceptions\n\n| Commit | Approving maintainer | Reason | Restoration obligation |\n| --- | --- | --- | --- |\n')
        self.commit('adopt recorded floor')
        self.base = git(self.work, 'rev-parse', 'HEAD')
        self.tool = self.work.parent / (self.work.name + '-tool')
        self.tool.write_text('#!' + sys.executable + '\nimport sys,shutil\nfrom pathlib import Path\na=sys.argv\n'
                             'shutil.copyfile(Path(__file__).with_name(Path(__file__).name+"-report.json"),a[a.index("--json")+1])\n'
                             'Path(a[a.index("--markdown")+1]).write_text("fixture")\n')
        self.tool.chmod(0o755)

    def commit(self, message):
        git(self.work, 'add', '.')
        git(self.work, 'commit', '-q', '-m', message)
        return git(self.work, 'rev-parse', 'HEAD')

    def record(self, sha, report, branch='coverage-data'):
        """Put baselines/<sha>.json on the data branch, as the recording job would."""
        exists = subprocess.run(['git', '-C', str(self.work), 'rev-parse', '--verify', '-q', 'refs/heads/' + branch],
                                capture_output=True).returncode == 0
        here = git(self.work, 'rev-parse', '--abbrev-ref', 'HEAD')
        if exists:
            git(self.work, 'checkout', '-q', branch)
        else:
            git(self.work, 'checkout', '-q', '--orphan', branch)
            git(self.work, 'rm', '-rfq', '.')
        (self.work / 'baselines').mkdir(exist_ok=True)
        (self.work / 'baselines' / f'{sha}.json').write_text(encoded(report))
        git(self.work, 'add', 'baselines')
        git(self.work, 'commit', '-q', '-m', f'record {sha}')
        git(self.work, 'checkout', '-q', here)

    def run(self, report, base=None, enforce='true'):
        Path(str(self.tool) + '-report.json').write_text(json.dumps(report))
        out = self.work / 'target/out'
        command = [sys.executable, str(ROOT / 'ci/run.py'), '--tool', str(self.tool), '--workspace', str(self.work),
                   '--enforce-ratchet', enforce, '--config', 'coverage/ci.json', '--base', base or self.base, '--output', str(out)]
        return subprocess.run(command, capture_output=True, text=True), out


class RecordedConfigTests(unittest.TestCase):
    def load(self, data):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory, 'ci.json')
            path.write_text(json.dumps(data))
            return config.load(path)

    def test_defaults_keep_committed_mode(self):
        data = config.load()
        self.assertEqual((data['floor'], data['data_branch'], data['bootstrap_baseline']), ('committed', 'coverage-data', ''))

    def test_recorded_mode_accepted(self):
        data = self.load({'floor': 'recorded', 'data_branch': 'cov/data', 'bootstrap_baseline': 'coverage/baseline.json'})
        self.assertEqual(data['floor'], 'recorded')

    def test_invalid_floor_settings_fail(self):
        for data in ({'floor': 'other'}, {'floor': 'recorded', 'data_branch': ''},
                     {'floor': 'recorded', 'data_branch': '../x'}, {'floor': 'recorded', 'data_branch': 'main'}, {'floor': 'recorded', 'data_branch': 'a b'},
                     {'floor': 'recorded', 'bootstrap_baseline': '/abs.json'},
                     {'floor': 'recorded', 'bootstrap_baseline': '../up.json'},
                     {'bootstrap_baseline': 'coverage/baseline.json'}):
            with self.subTest(data=data), self.assertRaises(ValueError):
                self.load(data)


class RecordedRatchetTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.report = WriteCoverageTests().report()

    def repo(self, **ci_config):
        return RecordedRepo(tempfile.mkdtemp(dir=self.directory.name), ci_config)

    def test_normal_compare_passes_and_writes_record_files(self):
        repo = self.repo()
        repo.record(repo.base, self.report)
        (repo.work / 'README.md').write_text('docs\n')
        repo.commit('docs')
        result, out = repo.run(self.report)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('ratchet PASS', (out / 'summary.md').read_text())
        self.assertEqual((out / 'record/baseline.json').read_text(), encoded(self.report))
        self.assertIn('Security runtime', (out / 'record/COVERAGE-MEASURED.md').read_text())
        self.assertEqual((out / 'record/sha').read_text().strip(), git(repo.work, 'rev-parse', 'HEAD'))
        self.assertEqual(json.loads((out / 'badges/ratchet.json').read_text())['message'], 'pass')

    def test_no_committed_numbers_are_required_or_checked(self):
        repo = self.repo()
        repo.record(repo.base, self.report)
        self.assertFalse((repo.work / 'coverage/baseline.json').exists())
        self.assertNotIn('measured coverage', (repo.work / 'COVERAGE.md').read_text())
        result, _ = repo.run(self.report)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_decrease_against_recorded_floor_fails(self):
        repo = self.repo()
        repo.record(repo.base, fixture(proven=60, tested=20, unverified=20) | {'proof_ratio': self.report['proof_ratio']})
        result, _ = repo.run(self.report)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('proven share decreased', result.stderr)
        reason = result.stderr.strip().removeprefix('coverage FAIL: ')
        result, out = repo.run(self.report, enforce='false')
        self.assertEqual(result.returncode, 0, result.stderr)
        notice = f'ratchet: not enforced (would report ratchet FAIL: {reason})'
        self.assertIn(notice, result.stdout)
        self.assertIn(notice, (out / 'summary.md').read_text())
        self.assertEqual((out / 'record/baseline.json').read_text(), encoded(self.report))
        badge = json.loads((out / 'badges/ratchet.json').read_text())
        self.assertEqual((badge['message'], badge['color']), ('off', 'lightgrey'))

    def test_exception_in_coverage_md_still_waives(self):
        repo = self.repo()
        worse = fixture(proven=60, tested=20, unverified=20) | {'proof_ratio': self.report['proof_ratio']}
        repo.record(repo.base, worse)
        (repo.work / 'COVERAGE.md').write_text('Prose\n\n' + EXCEPTION.replace('a' * 40, 'PLACEHOLDER'))
        sha = repo.commit('urgent fix')
        text = (repo.work / 'COVERAGE.md').read_text().replace('PLACEHOLDER', sha)
        (repo.work / 'COVERAGE.md').write_text(text)
        repo.commit('record exception')
        # the exception names a commit in base..HEAD: the first fix commit
        result, out = repo.run(self.report)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('maintainer exception', (out / 'summary.md').read_text())
        result, _ = repo.run(self.report, enforce='false')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('would report ratchet PASS (maintainer exception:', result.stdout)

    def test_missing_sha_fails_closed_naming_the_commit(self):
        repo = self.repo()
        result, out = repo.run(self.report)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn(repo.base, result.stderr)
        self.assertIn('no recorded baseline', result.stderr)
        self.assertTrue((out / 'record/baseline.json').exists(), 'main still records its own measurement')

    def test_missing_sha_with_other_baselines_recorded_fails_even_with_bootstrap(self):
        repo = self.repo(bootstrap_baseline='coverage/bootstrap.json')
        (repo.work / 'coverage/bootstrap.json').write_text(encoded(self.report))
        base = repo.commit('bootstrap file')
        repo.record('f' * 40, self.report)
        result, _ = repo.run(self.report, base=base)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('only before the first recording', result.stderr)

    def test_bootstrap_used_before_first_recording_and_read_from_merge_base(self):
        repo = self.repo(bootstrap_baseline='coverage/bootstrap.json')
        (repo.work / 'coverage/bootstrap.json').write_text(encoded(self.report))
        base = repo.commit('bootstrap file')
        # A PR cannot lower its own floor by editing the bootstrap file.
        (repo.work / 'coverage/bootstrap.json').write_text(encoded(fixture(proven=1, tested=1, unverified=98) | {'proof_ratio': self.report['proof_ratio']}))
        repo.commit('tamper')
        result, _ = repo.run(self.report, base=base)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_bootstrap_path_missing_in_merge_base_fails(self):
        repo = self.repo(bootstrap_baseline='coverage/absent.json')
        result, _ = repo.run(self.report)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('absent.json', result.stderr)

    def test_unreadable_floor_fails_closed(self):
        for bad in ('not json', '[]', '{"total": 1}'):
            with self.subTest(bad=bad):
                repo = self.repo()
                repo.record(repo.base, self.report)
                git(repo.work, 'checkout', '-q', 'coverage-data')
                (repo.work / f'baselines/{repo.base}.json').write_text(bad)
                git(repo.work, 'commit', '-qam', 'corrupt')
                git(repo.work, 'checkout', '-q', 'main')
                result, _ = repo.run(self.report)
                self.assertNotEqual(result.returncode, 0)

    def test_remote_tracking_data_branch_is_read(self):
        repo = self.repo()
        repo.record(repo.base, self.report)
        git(repo.work, 'update-ref', 'refs/remotes/origin/coverage-data', 'coverage-data')
        git(repo.work, 'branch', '-qD', 'coverage-data')
        result, _ = repo.run(self.report)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_custom_data_branch(self):
        repo = self.repo(data_branch='cov-data')
        repo.record(repo.base, self.report, branch='cov-data')
        result, _ = repo.run(self.report)
        self.assertEqual(result.returncode, 0, result.stderr)


if __name__ == '__main__':
    unittest.main()
