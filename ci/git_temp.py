"""Git environment for a publisher's temporary repository."""
import os
import subprocess


def temp_repo_env(env=None):
    env = dict(os.environ if env is None else env)
    local = subprocess.check_output(['git', 'rev-parse', '--local-env-vars'], text=True)
    for key in local.split():
        # The count activates the numbered config entries, including auth headers.
        if key != 'GIT_CONFIG_COUNT':
            env.pop(key, None)
    return env
