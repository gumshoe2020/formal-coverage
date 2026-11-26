"""run.py refuses to compute a ratchet verdict while the tree differs from HEAD."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import gitenv
from gitenv import git
import run
from test_recorded import RecordedRepo
import test_write_coverage

ROOT = Path(__file__).resolve().parents[2]
PARSER = ''.join(f'fn line_{n}() {{}}\n' for n in range(1, 26))  # security span is lines 8-20
# run.py's entry point with the dirty-tree guard removed: reproduces 1.12.0 behaviour.
UNGUARDED = (f'import sys; sys.path.insert(0, {str(ROOT / "ci")!r}); import run; '
             'run.require_committed = lambda *args: None; run.main()')
EXCEPTIONS = ('\n## Exceptions\n\n| Commit | Approving maintainer | Reason | Restoration obligation |\n'
              '| --- | --- | --- | --- |\n')


class CommittedRepo:
    """Committed-floor project at its reviewed baseline; the tool replays a fixed report."""

    def __init__(self, directory):
        self.work = Path(directory)
        self.work.mkdir()
        git(self.work, 'init', '-q', '-b', 'main')
        (self.work / '.gitignore').write_text('/target/\n*.tmp\n')
        (self.work / 'src').mkdir()
        (self.work / 'src/parser.rs').write_text(PARSER)
        report = self.work.parent / 'report.json'
        report.write_text(json.dumps(test_write_coverage.WriteCoverageTests().report()))
        (self.work / 'COVERAGE.md').write_text('Intro\n<!-- measured coverage: begin -->\n'
                                               '<!-- measured coverage: end -->\n' + EXCEPTIONS)
        subprocess.run([sys.executable, str(ROOT / 'ci/write-coverage.py'), str(report)],
                       cwd=self.work, capture_output=True, check=True)
        self.base = self.commit('initial reviewed floor')
        self.tool = self.work.parent / 'tool'
        self.tool.write_text(f'#!{sys.executable}\nimport sys,shutil\nfrom pathlib import Path\na=sys.argv\n'
                             f'shutil.copyfile({str(report)!r},a[a.index("--json")+1])\n'
                             'Path(a[a.index("--markdown")+1]).write_text("fixture")\n')
        self.tool.chmod(0o755)

    def commit(self, message):
        git(self.work, 'add', '-A')
        git(self.work, 'commit', '-q', '-m', message)
        return git(self.work, 'rev-parse', 'HEAD')

    def run(self, enforce='true', guarded=True):
        out = self.work / 'target/coverage-ci'
        runner = [str(ROOT / 'ci/run.py')] if guarded else ['-c', UNGUARDED]
        command = [sys.executable, *runner, '--tool', str(self.tool), '--workspace', str(self.work),
                   '--base', self.base, '--enforce-ratchet', enforce, '--output', str(out)]
        return gitenv.run(command, capture_output=True, text=True), out

    def edit_security_line(self):
        parser = self.work / 'src/parser.rs'
        parser.write_text(parser.read_text().replace('fn line_10() {}', 'fn line_10() { let _ = 1; }'))


class DirtyTreeTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.repo = CommittedRepo(Path(directory.name) / 'project')

    def assertProceeds(self):
        result, out = self.repo.run()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('ratchet PASS', (out / 'summary.md').read_text())

    def assertRefuses(self, *paths, enforce='true'):
        shutil.rmtree(self.repo.work / 'target/coverage-ci', ignore_errors=True)
        result, out = self.repo.run(enforce)
        self.assertEqual(result.returncode, 1, result.stdout)
        first, *listed = result.stderr.strip().splitlines()
        self.assertEqual(first, f'coverage FAIL: uncommitted changes in {self.repo.work.resolve()}; '
                                f'commit them first: the ratchet compares {self.repo.base} to HEAD')
        self.assertEqual([line.strip() for line in listed], list(paths))
        self.assertNotIn('ratchet', result.stdout)
        self.assertFalse(out.exists(), 'refused before measuring')

    def test_clean_tree_proceeds(self):
        self.assertProceeds()

    def test_tracked_edit_refuses(self):
        (self.repo.work / 'COVERAGE.md').write_text('edited\n')
        self.assertRefuses('COVERAGE.md')

    def test_staged_only_edit_refuses(self):
        self.repo.edit_security_line()
        git(self.repo.work, 'add', 'src/parser.rs')
        self.assertEqual(git(self.repo.work, 'diff', '--name-only'), '', 'worktree matches index')
        self.assertRefuses('src/parser.rs')

    def test_deleted_tracked_file_refuses(self):
        (self.repo.work / 'src/parser.rs').unlink()
        self.assertRefuses('src/parser.rs')

    def test_untracked_file_refuses(self):
        (self.repo.work / 'src/new_parser.rs').write_text('pub fn parse() {}\n')
        self.assertRefuses('src/new_parser.rs')

    def test_untracked_file_in_new_directory_is_named_individually(self):
        (self.repo.work / 'src/proto').mkdir()
        (self.repo.work / 'src/proto/frame.rs').write_text('pub fn frame() {}\n')
        self.assertRefuses('src/proto/frame.rs')

    def test_ignored_files_proceed(self):
        (self.repo.work / 'target/fstar').mkdir(parents=True)
        (self.repo.work / 'target/fstar/fstar.log').write_text('receipts\n')
        (self.repo.work / 'scratch.tmp').write_text('ignored\n')
        self.assertProceeds()

    def test_uncommitted_security_edit_no_longer_passes(self):
        """The 1.12.0 false PASS: HEAD == base, so base..HEAD was empty and nothing looked touched."""
        self.repo.edit_security_line()
        result, out = self.repo.run(guarded=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip().splitlines()[-1], 'ratchet PASS', 'the bug, without the guard')
        self.assertRefuses('src/parser.rs')
        self.repo.commit('touch security code without improving coverage')
        result, _ = self.repo.run()
        self.assertEqual(result.returncode, 1)
        self.assertNotIn('uncommitted', result.stderr)
        self.assertEqual(result.stderr, 'coverage FAIL: security code changed without increasing proven share\n')

    def test_action_log_files_are_exempt_by_exact_path_only(self):
        logs = self.repo.work / 'logs/ci-kit'
        logs.mkdir(parents=True)
        (logs / 'build.log').write_text('build\n')
        (logs / 'coverage.log').write_text('')
        self.assertProceeds()
        (logs / 'other.log').write_text('not the action log\n')
        self.assertRefuses('logs/ci-kit/other.log')

    def test_exempt_paths_are_the_coverage_action_logs(self):
        action = (ROOT / '.github/actions/coverage/action.yml').read_text()
        self.assertIn('> logs/ci-kit/build.log 2>&1', action)
        self.assertIn('> logs/ci-kit/coverage.log 2>&1', action)
        self.assertEqual(run.ACTION_LOGS, ('logs/ci-kit/build.log', 'logs/ci-kit/coverage.log'))

    def test_tracked_action_log_path_is_not_exempt(self):
        logs = self.repo.work / 'logs/ci-kit'
        logs.mkdir(parents=True)
        (logs / 'build.log').write_text('committed\n')
        self.repo.base = self.repo.commit('track a file at the log path')
        (logs / 'build.log').write_text('modified\n')
        self.assertRefuses('logs/ci-kit/build.log')

    def test_lists_at_most_ten_paths(self):
        for n in range(12):
            (self.repo.work / f'new_{n:02}.rs').write_text('\n')
        self.assertRefuses(*[f'new_{n:02}.rs' for n in range(10)], '... and 2 more')

    def test_advisory_mode_refuses_too(self):
        self.repo.edit_security_line()
        self.assertRefuses('src/parser.rs', enforce='false')


class RecordedDirtyTreeTests(unittest.TestCase):
    def test_recorded_floor_refuses_uncommitted_edit(self):
        with tempfile.TemporaryDirectory() as directory:
            repo = RecordedRepo(directory)
            report = test_write_coverage.WriteCoverageTests().report()
            repo.record(repo.base, report)
            result, _ = repo.run(report)
            self.assertEqual(result.returncode, 0, result.stderr)
            (repo.work / 'src').mkdir()
            (repo.work / 'src/parser.rs').write_text(PARSER)
            result, _ = repo.run(report)
            self.assertEqual(result.returncode, 1)
            self.assertIn('uncommitted changes in', result.stderr)
            self.assertIn('src/parser.rs', result.stderr)


if __name__ == '__main__':
    unittest.main()
