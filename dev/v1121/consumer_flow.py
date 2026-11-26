#!/usr/bin/env python3
"""Offline replay of the coverage action against a clean clone of a fixture consumer.

Runs the `run:` blocks of .github/actions/coverage/action.yml in their order, with
the environment GitHub would give them, and records every workspace path each
step creates (`git status --porcelain --ignored -uall` before/after). Uses only a
local bare origin: no network. Usage: consumer_flow.py KIT_ROOT SCRATCH_DIR
"""
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import textwrap

KIT, SCRATCH = (Path(a).resolve() for a in sys.argv[1:3])
ACTION = KIT / '.github/actions/coverage/action.yml'
IDENT = ['-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid']
EXCEPTIONS = ('\n## Exceptions\n\n| Commit | Approving maintainer | Reason | Restoration obligation |\n'
              '| --- | --- | --- | --- |\n')


def git(work, *args):
    return subprocess.check_output(['git', '-C', str(work), *IDENT, *args], text=True).strip()


def action_steps():
    """(name, script) for each step with a run block, in file order."""
    steps = []
    for block in ACTION.read_text().split('\n    - ')[1:]:
        name = re.search(r'(?:name|id): (.+)', block).group(1)
        if '      run: |\n' in block:
            script = textwrap.dedent(block.split('      run: |\n', 1)[1])
        elif (match := re.search(r'\n      run: (.+)', block)):
            script = match.group(1)
        else:
            continue  # `uses:` steps (cache/upload) write only runner.temp or read
        condition = re.search(r'\n      if: (.+)', block)
        steps.append((name, script, condition.group(1) if condition else None))
    return steps


def seed_origin(mode, floor_kind):
    seed, origin = SCRATCH / mode / 'seed', SCRATCH / mode / 'origin.git'
    shutil.copytree(KIT / 'fixtures/inventory', seed)
    (seed / '.gitignore').write_text('/target/\n')
    (seed / 'COVERAGE.md').write_text('Intro\n<!-- measured coverage: begin -->\n<!-- measured coverage: end -->\n'
                                     + EXCEPTIONS)
    (seed / 'coverage').mkdir()
    report = SCRATCH / mode / 'seed-report.json'
    subprocess.run([str(KIT / 'target/debug/formal-coverage'), 'report', '.', '--json', str(report)],
                   cwd=seed, check=True, capture_output=True)
    subprocess.run([sys.executable, str(KIT / 'ci/write-coverage.py'), str(report)],
                   cwd=seed, check=True, capture_output=True)
    config = {'schema': 1} if floor_kind == 'committed' else {'schema': 1, 'floor': 'recorded'}
    (seed / 'coverage/ci.json').write_text(json.dumps(config, indent=2) + '\n')
    git(seed, 'init', '-q', '-b', 'main')
    git(seed, 'add', '.')
    git(seed, 'commit', '-qm', 'initial reviewed floor')
    base = git(seed, 'rev-parse', 'HEAD')
    floor = (seed / 'coverage/baseline.json').read_text()
    if floor_kind == 'recorded':
        (seed / 'COVERAGE.md').write_text('Prose only\n' + EXCEPTIONS)
        git(seed, 'rm', '-q', 'coverage/baseline.json')
    (seed / 'README.md').write_text('Docs only\n')
    git(seed, 'add', '.')
    git(seed, 'commit', '-qm', 'docs')
    subprocess.run(['git', 'clone', '-q', '--bare', str(seed), str(origin)], check=True)
    if floor_kind == 'recorded':
        record_floor(origin, base, floor)
    return origin, base


def record_floor(origin, base, floor):
    """coverage-data holds baselines/<base>.json, as publish-baseline would write it."""
    data = origin.parent / 'data'
    git(origin.parent, 'init', '-q', '-b', 'coverage-data', str(data))
    (data / 'baselines').mkdir()
    (data / 'baselines' / f'{base}.json').write_text(floor)
    git(data, 'add', '.')
    git(data, 'commit', '-qm', f'record {base}')
    git(data, 'push', '-q', str(origin), 'coverage-data')


def status(work):
    out = subprocess.check_output(['git', '-C', str(work), 'status', '--porcelain', '--ignored', '-uall'], text=True)
    return set(out.splitlines())


def replay(mode, origin, base, scenario, kit=KIT, before=None):
    root = SCRATCH / mode / scenario
    work, temp = root / 'workspace', root / 'runner-temp'
    subprocess.run(['git', 'clone', '-q', str(origin), str(work)], check=True)  # checkout, fetch-depth 0
    temp.mkdir()
    dirty(work, scenario)
    shared = SCRATCH / 'runner-temp-build'  # stands in for actions/cache of the pinned build
    env = dict(os.environ, GITHUB_WORKSPACE=str(work), RUNNER_TEMP=str(temp),
               GITHUB_OUTPUT=str(temp / 'output'), GITHUB_STEP_SUMMARY=str(temp / 'summary'),
               ACTION_PATH=str(kit / '.github/actions/coverage'), ACTION_REF='0' * 40,
               KIT_ROOT=str(kit), PROJECT='.', ENFORCE_RATCHET='true', REPORT_ARGS='[]',
               BASELINE='coverage/baseline.json', COVERAGE='COVERAGE.md', CONFIG='coverage/ci.json',
               EVENT_NAME='push', BASE_REF='', BEFORE=before or base, GH_TOKEN='')
    (temp / 'formal-coverage-build').symlink_to(shared)
    lines, outcome = [f'### {mode} / {scenario}'], {}
    for name, script, condition in action_steps():
        if not runs(condition, outcome):
            lines.append(f'- step `{name}`: skipped ({condition or "earlier step failed"})')
            continue
        env['RESULT'] = outcome.get('measure', 'success')
        if name == 'Stage badges including failed ratchets':
            env['RESULT'] = ratchet_result(temp)
        seen = status(work)
        result = subprocess.run(['bash', '-ec', script], cwd=work, env=env, capture_output=True, text=True)
        created = sorted(p for p in status(work) - seen)
        if name == 'measure':
            outcome['measure'] = 'success' if result.returncode == 0 else 'failure'
        if result.returncode:
            outcome['failed'] = True
        lines.append(f'- step `{name}`: exit {result.returncode}; new status entries: {created or "none"}')
        if name == 'measure':
            log = (work / 'logs/ci-kit/coverage.log').read_text().strip().splitlines()
            lines += ['  ```', *('  ' + line for line in log[-12:]), '  ```']
    return '\n'.join(lines).replace(str(SCRATCH), '$SCRATCH')


def runs(condition, outcome):
    if condition is None or condition == "inputs.config != ''":
        return not outcome.get('failed')
    if condition == 'failure()':
        return outcome.get('measure') == 'failure'
    return True  # always() / badge staging condition (enforced mode)


def ratchet_result(temp):
    text = (temp / 'output').read_text()
    return re.findall(r'result=(\S+)', text)[-1]


def dirty(work, scenario):
    lib = work / 'src/lib.rs'
    if scenario in ('tracked-edit', 'false-pass-v1.12.0', 'false-pass-v1.12.1'):
        lib.write_text(lib.read_text().replace('let _x = 1;', 'let _x = 2;'))
    elif scenario == 'untracked-rs':
        (work / 'src/new_parser.rs').write_text('pub fn parse(input: &[u8]) -> usize {\n    input.len()\n}\n')


def old_kit():
    kit = SCRATCH / 'kit-v1.12.0'
    kit.mkdir()
    archive = subprocess.run(['git', '-C', str(KIT), 'archive', 'v1.12.0', 'Cargo.toml', 'Cargo.lock', 'src', 'registry', 'ci', '.github'],
                             check=True, capture_output=True).stdout
    subprocess.run(['tar', '-x', '-C', str(kit)], input=archive, check=True)
    return kit


def main():
    if SCRATCH.exists():
        shutil.rmtree(SCRATCH)
    (SCRATCH / 'runner-temp-build').mkdir(parents=True)
    report = []
    for mode in ('committed', 'recorded'):
        origin, base = seed_origin(mode, mode)
        for scenario in ('clean', 'tracked-edit', 'untracked-rs'):
            report.append(replay(mode, origin, base, scenario))
    origin, _ = seed_origin('false-pass', 'committed')
    head = git(origin, 'rev-parse', 'main')
    report.append(replay('false-pass', origin, head, 'false-pass-v1.12.0', kit=old_kit(), before=head))
    report.append(replay('false-pass', origin, head, 'false-pass-v1.12.1', before=head))
    print('\n\n'.join(report))


if __name__ == '__main__':
    main()
