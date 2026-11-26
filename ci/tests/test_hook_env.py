"""Test git fixtures stay out of the repository a git hook points at.

A hook exports GIT_DIR (and may export GIT_WORK_TREE and GIT_INDEX_FILE); they override
`git -C`. In v1.12.1 a pre-push test run committed a fixture onto the real branch.
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import gitenv
from gitenv import git
from test_dirty_tree import CommittedRepo
from test_recorded import RecordedRepo
import test_write_coverage

HERE = Path(__file__).resolve().parent
SCENARIOS = (('GIT_DIR',), ('GIT_DIR', 'GIT_WORK_TREE', 'GIT_INDEX_FILE'))


def snapshot(directory):
    """Every file under directory: worktree and .git (HEAD, index, refs, objects, logs)."""
    return {path.relative_to(directory): path.read_bytes()
            for path in sorted(Path(directory).rglob('*')) if path.is_file()}


def changed(before, after):
    return sorted(str(k) for k in before.keys() | after.keys() if before.get(k) != after.get(k))


def hook_env(decoy, names):
    values = {'GIT_DIR': str(decoy / '.git'), 'GIT_WORK_TREE': str(decoy),
              'GIT_INDEX_FILE': str(decoy / '.git/index')}
    return {name: values[name] for name in names}


def make_decoy(directory):
    decoy = Path(directory) / 'decoy'
    decoy.mkdir()
    (decoy / 'keep.txt').write_text('tracked\n')
    git(decoy, 'init', '-q', '-b', 'main')
    git(decoy, 'add', '.')
    git(decoy, 'commit', '-q', '-m', 'decoy')
    return decoy


def run_fixtures(directory):
    """The shared git-using fixtures, including run.py subprocesses."""
    report = test_write_coverage.WriteCoverageTests().report()
    recorded = RecordedRepo(Path(tempfile.mkdtemp(dir=directory)))
    recorded.record(recorded.base, report)
    result, _ = recorded.run(report)
    assert result.returncode == 0, result.stderr
    committed = CommittedRepo(Path(directory) / 'committed')
    result, _ = committed.run()
    assert result.returncode == 0, result.stderr
    remote = Path(directory) / 'remote.git'
    git(directory, 'init', '--bare', '-q', '-b', 'main', str(remote))
    git(directory, 'clone', '-q', str(remote), str(Path(directory) / 'clone'))


def unscrubbed_fixture(directory):
    """v1.12.1 fixture code: plain `git -C`, inheriting the process environment."""
    work = Path(directory) / 'unscrubbed'
    (work / 'src').mkdir(parents=True)
    (work / 'src/lib.rs').write_text('fn fixture() {}\n')
    for args in (['init', '-q', '-b', 'main'], ['add', '.'],
                 ['-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-qm', 'fixture']):
        subprocess.run(['git', '-C', str(work), *args], capture_output=True)


class HookEnvironmentTests(unittest.TestCase):
    def test_fixtures_leave_hook_repository_byte_unchanged(self):
        for names in SCENARIOS:
            with self.subTest(env=names), tempfile.TemporaryDirectory() as directory:
                decoy = make_decoy(directory)
                before = snapshot(decoy)
                with patch.dict(os.environ, hook_env(decoy, names)):
                    try:
                        run_fixtures(directory)
                    finally:
                        # The decoy is the point, even when a redirected fixture also fails.
                        self.assertEqual(changed(before, snapshot(decoy)), [],
                                         'a test fixture wrote to the hook repository')
                    unscrubbed_fixture(directory)
                damage = changed(before, snapshot(decoy))
                self.assertTrue(damage, 'control did not reach the decoy; the test proves nothing')
                if names == ('GIT_DIR',):
                    self.assertIn('.git/refs/heads/main', damage)

    def test_importing_gitenv_cleans_the_process_environment(self):
        with tempfile.TemporaryDirectory() as directory:
            env = dict(os.environ, **hook_env(Path(directory), SCENARIOS[1]), GIT_CONFIG_GLOBAL='/x')
            code = (f'import sys, os, json; sys.path.insert(0, {str(HERE)!r}); import gitenv; '
                    'print(json.dumps({k: os.environ.get(k) for k in '
                    '("GIT_DIR", "GIT_WORK_TREE", "GIT_INDEX_FILE", "GIT_CONFIG_GLOBAL", "GIT_CONFIG_NOSYSTEM")}))')
            seen = json.loads(subprocess.check_output([sys.executable, '-c', code], env=env, text=True))
        self.assertEqual(seen, {'GIT_DIR': None, 'GIT_WORK_TREE': None, 'GIT_INDEX_FILE': None,
                                'GIT_CONFIG_GLOBAL': '/dev/null', 'GIT_CONFIG_NOSYSTEM': '1'})

    def test_clean_env_removes_gits_own_list(self):
        listed = subprocess.check_output(['git', 'rev-parse', '--local-env-vars'], text=True).split()
        self.assertEqual(gitenv.LOCAL_ENV_VARS, tuple(listed))
        env = gitenv.clean_env({name: 'x' for name in listed} | {'PATH': os.environ['PATH']})
        self.assertFalse(set(listed) & set(env))
        self.assertEqual(env['PATH'], os.environ['PATH'])


if __name__ == '__main__':
    unittest.main()
