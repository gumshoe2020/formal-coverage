#!/usr/bin/env python3
"""Append main's prepared module receipts to the coverage data branch only."""
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
from receipts import full_sha, module_name


def load_record(source):
    commit = full_sha((source / 'sha').read_text().strip())
    branch = (source / 'branch').read_text().strip()
    if not valid_branch(branch):
        raise ValueError('invalid data branch')
    records = {}
    for path in sorted(source.iterdir()):
        if path.name in ('sha', 'branch'):
            continue
        text = path.read_text()
        r = json.loads(text)
        expected = {'schema', 'module', 'main_commit', 'file', 'toolchain', 'command', 'dependencies', 'log'}
        if (set(r) != expected or type(r['schema']) is not int or r['schema'] != 1
                or r['main_commit'] != commit or r['module'] != module_name(r['file'])
                or path.name != r['module'] + '.json'):
            raise ValueError(f'{path.name}: invalid receipt identity/schema')
        if (set(r['toolchain']) != {'fstar', 'z3', 'hax'} or
                any(not isinstance(v, str) or not v.strip() for v in r['toolchain'].values())):
            raise ValueError(f'{path.name}: invalid toolchain')
        deps = r['dependencies']
        if (not isinstance(deps, dict) or r['file'] not in deps or any(
                Path(p).is_absolute() or '..' in Path(p).parts or not p.endswith(('.fst', '.fsti'))
                or not isinstance(h, str) or not re.fullmatch('[0-9a-f]{64}', h) for p, h in deps.items())):
            raise ValueError(f'{path.name}: invalid dependencies')
        args = r['command']
        if not isinstance(args, list) or not args or args[0] != 'fstar.exe':
            raise ValueError(f'{path.name}: invalid options')
        rest, inputs, includes = args[1:], 0, set()
        while rest:
            if rest[0] == '--include' and len(rest) >= 2 and isinstance(rest[1], str):
                inc = rest[1]
                if not inc or Path(inc).is_absolute() or '..' in Path(inc).parts or inc in includes:
                    raise ValueError(f'{path.name}: invalid include')
                includes.add(inc)
                rest = rest[2:]
            elif rest[0] == r['file'] and inputs == 0:
                inputs += 1
                rest = rest[1:]
            else:
                raise ValueError(f'{path.name}: unsupported strict options')
        if inputs != 1 or not any(l.strip() == f'Verified module: {r["module"]}' for l in r['log'].splitlines()):
            raise ValueError(f'{path.name}: missing strict success log')
        records[path.name] = text
    if not records:
        raise ValueError('no module receipts')
    return commit, branch, records


def record(source, remote, env=None, attempts=3):
    commit, branch, records = load_record(Path(source))
    for attempt in range(attempts):
        try:
            return record_once(commit, branch, records, remote, env)
        except subprocess.CalledProcessError:
            if attempt + 1 == attempts:
                raise
            print(f'push of {branch} raced; retrying', file=sys.stderr)


def record_once(commit, branch, records, remote, env):
    with tempfile.TemporaryDirectory(prefix='coverage-receipts-', dir=os.environ.get('RUNNER_TEMP')) as directory:
        def git(*args, capture=False):
            return subprocess.run(['git', '-C', directory, *args], env=env, check=True,
                                  stdout=subprocess.PIPE if capture else None, text=True)
        git('init', '--quiet')
        git('remote', 'add', 'origin', remote)
        ref = f'refs/heads/{branch}'
        if git('ls-remote', '--heads', 'origin', ref, capture=True).stdout.split():
            git('fetch', '--quiet', '--depth=1', 'origin', ref)
            git('checkout', '--quiet', '-b', branch, 'FETCH_HEAD')
        else:
            git('checkout', '--quiet', '--orphan', branch)
        changed = []
        for name, text in records.items():
            relative = f'receipts/{commit}/{name}'
            path = Path(directory, relative)
            if path.is_symlink() or any(p.is_symlink() for p in path.parents):
                raise ValueError(f'{relative}: symlink on data branch')
            if path.exists():
                if path.read_text() != text:
                    raise ValueError(f'{relative}: different content; receipts are append-only')
                continue
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text)
            changed.append(relative)
        if not changed:
            print(f'receipts/{commit}: identical, nothing to do')
            return False
        git('add', '--', *changed)
        git('-c', 'user.name=github-actions[bot]', '-c', 'user.email=41898282+github-actions[bot]@users.noreply.github.com',
            'commit', '--quiet', '-m', f'Record F* receipts for {commit}')
        git('push', '--quiet', 'origin', f'HEAD:{ref}')
        return True


def publish(source):
    if os.environ.get('GITHUB_EVENT_NAME') != 'push' or os.environ.get('GITHUB_REF') != 'refs/heads/main':
        raise ValueError('receipt publication restricted to push on main')
    commit, _, _ = load_record(source)
    if commit != os.environ['GITHUB_SHA']:
        raise ValueError('receipts are not for pushed commit')
    repository = os.environ['GITHUB_REPOSITORY']
    if not re.fullmatch(r'[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+', repository):
        raise ValueError('invalid GitHub repository')
    record(source, f'https://github.com/{repository}.git', github_env(os.environ['GITHUB_TOKEN']))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    args = parser.parse_args()
    try:
        publish(args.source)
    except (ValueError, KeyError, TypeError, OSError, subprocess.CalledProcessError) as exc:
        parser.exit(1, f'publish-receipts FAIL: {exc}\n')
