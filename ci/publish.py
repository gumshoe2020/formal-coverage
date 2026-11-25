#!/usr/bin/env python3
"""Publish endpoint JSON in an isolated git directory; only the badges ref changes."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import tempfile

NAMES = {'proven', 'tested', 'unverified', 'proof-ratio', 'ratchet'}


def validate(source, result):
    files = sorted(source.glob('*'))
    if not files:
        raise ValueError('empty badge artifact')
    badges = {}
    for path in files:
        if path.is_symlink() or path.name not in {n + '.json' for n in NAMES} or not path.is_file():
            raise ValueError('unexpected badge artifact member')
        data = json.loads(path.read_text())
        if set(data) != {'schemaVersion', 'label', 'message', 'color'} or data['schemaVersion'] != 1 or any(
                not isinstance(data[k], str) for k in ('label', 'message', 'color')):
            raise ValueError('invalid badge endpoint')
        if path.stem == 'ratchet' and result != 'success':
            data.update(message='fail', color='red')
        badges[path.name] = json.dumps(data, sort_keys=True) + '\n'
    return badges


def publish(source, result):
    if os.environ.get('GITHUB_EVENT_NAME') != 'push' or os.environ.get('GITHUB_REF') != 'refs/heads/main':
        raise ValueError('badge publication is restricted to push on main')
    repository = os.environ['GITHUB_REPOSITORY']
    if not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repository):
        raise ValueError('invalid GitHub repository')
    badges = validate(source, result)
    # Token stays in an environment-provided HTTP header, never in the remote URL
    # or printed command. This is the job's GITHUB_TOKEN, no other credential.
    import base64
    header = base64.b64encode(('x-access-token:' + os.environ['GH_TOKEN']).encode()).decode()
    env = {**os.environ, 'GIT_CONFIG_COUNT': '1',
           'GIT_CONFIG_KEY_0': 'http.https://github.com/.extraheader',
           'GIT_CONFIG_VALUE_0': 'AUTHORIZATION: basic ' + header}
    with tempfile.TemporaryDirectory(prefix='coverage-badges-', dir=os.environ.get('RUNNER_TEMP')) as directory:
        def git(*args, capture=False):
            return subprocess.run(['git', '-C', directory, *args], env=env, check=True,
                                  stdout=subprocess.PIPE if capture else None, text=True)
        git('init', '--quiet')
        git('remote', 'add', 'origin', f'https://github.com/{repository}.git')
        refs = git('ls-remote', '--heads', 'origin', 'refs/heads/badges', capture=True).stdout.split()
        previous = refs[0] if refs else ''
        if previous:
            git('fetch', '--depth=1', 'origin', 'refs/heads/badges')
            git('checkout', '--quiet', '-b', 'badges', 'FETCH_HEAD')
            git('rm', '-rf', '--ignore-unmatch', '.')
        else:
            git('checkout', '--orphan', 'badges')
        for name, content in badges.items():
            Path(directory, name).write_text(content)
        git('add', '--', '*.json')
        git('-c', 'user.name=github-actions[bot]',
            '-c', 'user.email=41898282+github-actions[bot]@users.noreply.github.com',
            'commit', '--allow-empty', '-m', 'Update coverage badges for ' + os.environ['GITHUB_SHA'])
        git('push', f'--force-with-lease=refs/heads/badges:{previous}', 'origin', 'HEAD:refs/heads/badges')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    parser.add_argument('result', choices=['success', 'failure', 'cancelled', 'skipped'])
    args = parser.parse_args()
    publish(args.source, args.result)
