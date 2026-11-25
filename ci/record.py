#!/usr/bin/env python3
"""Record main's measurement on the data branch: baselines/<sha>.json, append-only.

Also refreshes COVERAGE-MEASURED.md. Git work happens in an isolated temp
directory and only the data branch is pushed (never forced, never main).
"""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

from config import valid_branch
from gitauth import github_env

KEYS = {'schema', 'scope', 'total', 'proven', 'tested', 'unverified', 'statuses',
        'proof_sloc', 'proof_ratio', 'inventory'}
ATTEMPTS = 3


def load_record(source):
    sha = (source / 'sha').read_text().strip()
    branch = (source / 'branch').read_text().strip()
    if not re.fullmatch(r'[0-9a-f]{40}', sha):
        raise ValueError('record sha must be a full 40-character commit id')
    if not valid_branch(branch):
        raise ValueError('invalid data branch in record')
    baseline = (source / 'baseline.json').read_text()
    data = json.loads(baseline)
    if not isinstance(data, dict) or set(data) != KEYS or any(
            type(data[k]) is not int for k in ('total', 'proven', 'tested', 'unverified')):
        raise ValueError('record baseline is not a schema-1 baseline')
    return sha, branch, baseline, (source / 'COVERAGE-MEASURED.md').read_text()


def record(source, remote, env=None, attempts=ATTEMPTS):
    sha, branch, baseline, page = load_record(Path(source))
    for attempt in range(1, attempts + 1):
        try:
            return record_once(sha, branch, baseline, page, remote, env)
        except subprocess.CalledProcessError:
            if attempt == attempts:
                raise
            print(f'push of {branch} raced with another recording; retrying', file=sys.stderr)


def record_once(sha, branch, baseline, page, remote, env):
    with tempfile.TemporaryDirectory(prefix='coverage-record-', dir=os.environ.get('RUNNER_TEMP')) as directory:
        def git(*args, capture=False):
            return subprocess.run(['git', '-C', directory, *args], env=env, check=True,
                                  stdout=subprocess.PIPE if capture else None, text=True)
        git('init', '--quiet')
        git('remote', 'add', 'origin', remote)
        ref = f'refs/heads/{branch}'
        previous = git('ls-remote', '--heads', 'origin', ref, capture=True).stdout.split()
        if previous:
            git('fetch', '--quiet', '--depth=1', 'origin', ref)
            git('checkout', '--quiet', '-b', branch, 'FETCH_HEAD')
        else:
            git('checkout', '--quiet', '--orphan', branch)
        target = Path(directory, 'baselines', f'{sha}.json')
        if target.exists():
            if target.read_text() == baseline:
                print(f'baselines/{sha}.json already recorded; nothing to do')
                return False
            raise ValueError(f'baselines/{sha}.json already exists with different content; recordings are append-only')
        target.parent.mkdir(exist_ok=True)
        target.write_text(baseline)
        Path(directory, 'COVERAGE-MEASURED.md').write_text(page)
        git('add', '--', f'baselines/{sha}.json', 'COVERAGE-MEASURED.md')
        git('-c', 'user.name=github-actions[bot]',
            '-c', 'user.email=41898282+github-actions[bot]@users.noreply.github.com',
            'commit', '--quiet', '-m', f'Record coverage for {sha}')
        git('push', '--quiet', 'origin', f'HEAD:{ref}')
        print(f'recorded baselines/{sha}.json on {branch}')
        return True


def publish(source):
    if os.environ.get('GITHUB_EVENT_NAME') != 'push' or os.environ.get('GITHUB_REF') != 'refs/heads/main':
        raise ValueError('baseline recording is restricted to push on main')
    repository = os.environ['GITHUB_REPOSITORY']
    if not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repository):
        raise ValueError('invalid GitHub repository')
    if (Path(source) / 'sha').read_text().strip() != os.environ['GITHUB_SHA']:
        raise ValueError('recorded measurement is not for the pushed commit')
    record(source, f'https://github.com/{repository}.git', github_env(os.environ['GH_TOKEN']))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    args = parser.parse_args()
    try:
        publish(args.source)
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as exc:
        sys.exit(f'record FAIL: {exc}')
