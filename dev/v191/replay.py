#!/usr/bin/env python3
"""Replay staged v191 evidence and existing controls; never run hax or F*."""
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
from datetime import datetime, timezone
from concurrent.futures import ThreadPoolExecutor, as_completed

REPO = Path(__file__).resolve().parents[2]
OUT = REPO / 'target/v191'
sys.path.insert(0, str(REPO / 'scripts'))
from v15_replay import command_line, prepare_bundle_case

MODES = ('separate', 'runtime')
FIXTURES = ('classification', 'doc_comments', 'evidence', 'ghost', 'hax', 'inventory')


def run(args, log, cwd=REPO, env=None):
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open('w') as stream:
        stream.write('$ ' + ' '.join(map(str, args)) + '\n')
        stream.flush()
        result = subprocess.run(list(map(str, args)), cwd=cwd,
                                env={**os.environ, **(env or {})},
                                stdout=stream, stderr=subprocess.STDOUT)
    if result.returncode:
        detail = '\n' + log.read_text()[-8000:] if log.name == 'compare.log' else ''
        raise RuntimeError(f'exit {result.returncode}: {log.relative_to(REPO)}{detail}')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def prepare():
    if 'RUSTUP_TOOLCHAIN' in os.environ:
        raise RuntimeError('Run with RUSTUP_TOOLCHAIN unset')
    version = subprocess.check_output(['rustc', '--version'], text=True).strip()
    active = subprocess.check_output(['rustup', 'show', 'active-toolchain'], text=True).strip()
    assert active.startswith('stable-'), f'Expected current stable: {active}'
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True)
    (OUT / 'toolchain.txt').write_text(version + '\n')
    archive = subprocess.check_output(['git', 'archive', 'v1.9.0'])
    source = OUT / 'baseline-src'
    source.mkdir()
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        tar.extractall(source, filter='data')
    run(['cargo', 'build', '--release', '--locked', '--manifest-path', source / 'Cargo.toml',
         '--target-dir', OUT / 'baseline-build'], OUT / 'baseline-build.log')
    run(['cargo', 'build', '--release', '--locked', '--target-dir', OUT / 'candidate-build'],
        OUT / 'candidate-build.log')
    for label in ('baseline', 'candidate'):
        shutil.copy2(OUT / f'{label}-build/release/formal-coverage', OUT / label)
    inputs = REPO / 'target/v191-inputs'
    shutil.copytree(inputs / 'bertie', OUT / 'bertie',
                    ignore=shutil.ignore_patterns('target', '.git'))
    evidence = OUT / 'bertie/target/ci-logs/fstar'
    shutil.copytree(inputs / 'bertie/target/ci-logs/fstar', evidence)
    shutil.copy2(inputs / 'ci-bab3c13/report.json', OUT / 'ci-report.json')
    receipt_path = evidence / 'fstar.log.evidence.json'
    assert receipt_path.read_bytes() == (inputs / 'ci-evidence/fstar.log.evidence.json').read_bytes()
    assert (evidence / 'fstar.log').read_bytes() == (inputs / 'ci-evidence/fstar.log').read_bytes()
    receipt = json.loads(receipt_path.read_text())
    assert digest(evidence / 'fstar.log') == receipt['log_sha256']
    for name, expected in receipt['files'].items():
        assert digest(OUT / 'bertie' / name) == expected, name
    for field in ('fresh', 'checked', 'patches'):
        for name, expected in receipt[field].items():
            assert digest(evidence / name) == expected, name
    assert '[ship]' in (OUT / 'bertie/formal-coverage.toml').read_text()


def controls():
    index = json.loads((REPO / 'logs/release-evidence.json').read_text())
    known = {case['name']: case for case in index['scratch'] + index['survey']}
    for name in ('scratch-spec-lemmas', 'vest'):
        case = {**known[name], 'source': str((OUT / 'controls' / name).relative_to(REPO))}
        prepare_bundle_case(case)
        # Isolate Cargo's writable cache; retain the original offline receipt inputs.
        home = OUT / 'cargo-homes' / name
        shutil.copytree(REPO / case['cargo_home'], home,
                        ignore=shutil.ignore_patterns('target'))
        case['cargo_home'] = str(home.relative_to(REPO))
        for mode in MODES:
            for label, tool in (('before', 'baseline'), ('after', 'candidate')):
                output = OUT / f'{name}-{mode}-{label}'
                output.mkdir()
                args, env = command_line(OUT / tool, case, output,
                                         ['--size-mode', 'whole-crate', '--ghost-lines', mode])
                env['TMPDIR'] = str(OUT / 'tmp')
                Path(env['TMPDIR']).mkdir(exist_ok=True)
                run(args, output / 'report.log', env=env)
            print(f'{name} {mode}: generated', flush=True)
    for name in FIXTURES:
        root = OUT / 'fixtures' / name
        shutil.copytree(REPO / 'fixtures' / name, root,
                        ignore=shutil.ignore_patterns('target'))
        for mode in MODES:
            for label, tool in (('before', 'baseline'), ('after', 'candidate')):
                output = OUT / f'fixture-{name}-{mode}-{label}'
                output.mkdir()
                run([OUT / tool, 'report', root, '--size-mode', 'whole-crate',
                     '--ghost-lines', mode, '--json', output / 'report.json',
                     '--markdown', output / 'report.md'], output / 'report.log')
            print(f'fixture-{name} {mode}: generated', flush=True)


def bertie():
    host = next(line[6:] for line in subprocess.check_output(
        ['rustc', '-vV'], text=True).splitlines() if line.startswith('host: '))
    for mode in MODES:
        for label, tool in (('before', 'baseline'), ('after', 'candidate')):
            name = f'bertie-{mode}-{label}'
            run([OUT / tool, 'report', '.', '--target', host,
                 '--fstar-log', 'target/ci-logs/fstar/fstar.log', '--ghost-lines', mode,
                 '--json', OUT / f'{name}.json', '--markdown', OUT / f'{name}.md'],
                OUT / f'{name}.log', cwd=OUT / 'bertie')
            print(name + ': generated', flush=True)


def main():
    try:
        prepare()
        # Distinct archived workspaces, outputs and build directories; no shared config writes.
        with ThreadPoolExecutor(max_workers=2) as pool:
            futures = [pool.submit(controls), pool.submit(bertie)]
            for future in as_completed(futures):
                future.result()
        run(['python3', REPO / 'dev/v191/compare.py'], OUT / 'compare.log')
        print((OUT / 'compare.log').read_text(), end='')
    except Exception as error:
        stamp = datetime.now(timezone.utc).isoformat()
        with (REPO / 'dev/v191/NOTES.md').open('a') as notes:
            notes.write(f'\n## {stamp} — HONEST-FINDING / replay stopped\n\n{error}\n')
        with (REPO / 'dev/v191/compare.md').open('a') as comparison:
            comparison.write(f'\n## {stamp} — HONEST-FINDING / STOPPED\n\n{error}\n')
        raise


if __name__ == '__main__':
    main()
