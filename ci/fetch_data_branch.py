#!/usr/bin/env python3
"""Fetch the recorded-floor data branch into refs/remotes/origin/<branch>.

No-op for committed-floor projects. A missing branch is tolerated (first
adoption; the ratchet then fails closed or uses bootstrap_baseline); any other
fetch failure is an error.
"""
import argparse
import os
from pathlib import Path
import subprocess
import sys

from config import load
from gitauth import github_env


def fetch(workspace, config_path, env=None):
    config = load(config_path)
    if config['floor'] != 'recorded':
        return False
    ref = 'refs/heads/' + config['data_branch']
    def git(*args):
        return subprocess.run(['git', '-C', str(workspace), *args], env=env, check=True,
                              stdout=subprocess.PIPE, text=True).stdout
    if not git('ls-remote', '--heads', 'origin', ref).split():
        print(f'data branch {config["data_branch"]} does not exist yet')
        return False
    git('fetch', '--quiet', '--depth=1', 'origin', f'+{ref}:refs/remotes/origin/{config["data_branch"]}')
    return True


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--workspace', type=Path, default=Path('.'))
    parser.add_argument('--config', required=True, type=Path)
    args = parser.parse_args()
    token = os.environ.get('GH_TOKEN')
    try:
        fetch(args.workspace, args.config, github_env(token) if token else None)
    except (ValueError, OSError, subprocess.CalledProcessError) as exc:
        sys.exit(f'fetch FAIL: {exc}')
