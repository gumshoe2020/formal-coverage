"""Git for tests, isolated from any repository the caller's environment points at.

A git hook exports GIT_DIR (and possibly GIT_INDEX_FILE, GIT_WORK_TREE, ...), and those
override `git -C <dir>`. Test git calls and kit subprocesses take their environment from
clean_env(), which removes git's own list of repository-locating variables
(`git rev-parse --local-env-vars`), ignores global and system config and commits as a
fixed identity. Importing this module also cleans os.environ once, so kit code the tests
call in-process (record, publish_receipts, fetch_data_branch, run) inherits the same.
"""
import contextlib
import os
import subprocess
from unittest.mock import patch

ISOLATION = {
    'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_NOSYSTEM': '1',
    'GIT_AUTHOR_NAME': 'Test',
    'GIT_AUTHOR_EMAIL': 'test@example.invalid',
    'GIT_COMMITTER_NAME': 'Test',
    'GIT_COMMITTER_EMAIL': 'test@example.invalid',
}
LOCAL_ENV_VARS = tuple(subprocess.check_output(['git', 'rev-parse', '--local-env-vars'],
                                               text=True).split())
assert 'GIT_DIR' in LOCAL_ENV_VARS, LOCAL_ENV_VARS


def clean_env(base=None):
    """base (default os.environ) without repository-locating variables, config isolated."""
    base = os.environ if base is None else base
    return {**{k: v for k, v in base.items() if k not in LOCAL_ENV_VARS}, **ISOLATION}


def git(path, *args):
    """`git -C path args` in clean_env(); returns stripped stdout, raises with stderr."""
    return subprocess.run(['git', '-C', str(path), *args], env=clean_env(), check=True,
                          capture_output=True, text=True).stdout.strip()


def run(command, **kwargs):
    """subprocess.run for kit scripts that spawn git, with the child environment cleaned."""
    return subprocess.run(command, env=clean_env(kwargs.pop('env', None)), **kwargs)


@contextlib.contextmanager
def scrubbed_environ():
    """os.environ replaced by clean_env() for the duration, for in-process kit code."""
    with patch.dict(os.environ, clean_env(), clear=True):
        yield


def _clean_process_environ():
    for key in LOCAL_ENV_VARS:
        os.environ.pop(key, None)
    os.environ.update(ISOLATION)


_clean_process_environ()
