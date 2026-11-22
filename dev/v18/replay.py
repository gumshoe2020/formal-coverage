#!/usr/bin/env python3
"""Reproduce v1.8 paired retained-receipt reports; all writes stay in this repo.

Build target/v18/baseline from pre-v1.8 main, target/v18/candidate from final
source first. Bertie source and its existing receipt must be copied as in NOTES.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / 'scripts'))
from v15_replay import cases, command_line  # noqa: E402

OUT = REPO / 'target/v18'


def run(args, out, env=None, cwd=REPO):
    out.parent.mkdir(parents=True, exist_ok=True)
    with out.open('w') as log:
        log.write('$ ' + ' '.join(map(str, args)) + '\n')
        log.flush()
        result = subprocess.run(list(map(str, args)), cwd=cwd, env={**os.environ, **(env or {})}, stdout=log, stderr=subprocess.STDOUT)
    assert result.returncode == 0, f'replay failed ({result.returncode}): {out}'


def bertie():
    root = OUT / 'bertie'
    config = OUT / 'bertie-proof.toml'
    original = root / 'formal-coverage.toml'
    config.write_text(original.read_text() + '\n[fstar.proof]\npatches=["proofs/fstar/extraction-panic-free.patch"]\nfiles=[]\nmodels=["proofs/fstar/libcrux-models/*.fsti"]\n')
    for label in ['before', 'after', 'after-runtime']:
        tool = OUT / ('baseline' if label == 'before' else 'candidate')
        # The old tool does not accept new optional config keys. The baseline
        # uses the identical original config, including its 5–15 ratio range.
        cfg = original if label == 'before' else config
        args = [tool, 'report', '.', '--config', cfg, '--fstar-log', 'target/ci-logs/fstar/fstar.log', '--json', OUT / f'bertie-{label}.json', '--markdown', OUT / f'bertie-{label}.md']
        if label == 'after-runtime':
            args += ['--ghost-lines', 'runtime']
        run(args, REPO / f'logs/v1.8/m4-bertie-{label}.log', cwd=root)
        print(f'Bertie {label}: PASS', flush=True)


def verus():
    for name, case in cases(['scratch-spec-lemmas', 'vest'], REPO / 'fixtures/v1.5/v1.4.2-scratch-server.toml'):
        for mode in ['separate', 'runtime']:
            for label, tool in [('before', 'baseline'), ('after', 'candidate')]:
                output = OUT / f'{name}-{mode}-{label}'
                output.mkdir(parents=True, exist_ok=True)
                args, env = command_line(OUT / tool, case, output, ['--size-mode', 'whole-crate', '--ghost-lines', mode])
                env['TMPDIR'] = str(OUT / 'tmp')
                Path(env['TMPDIR']).mkdir(exist_ok=True)
                run(args, output / 'report.log', env)
                print(f'{name} {mode} {label}: PASS', flush=True)


def fixtures():
    for name in ['classification', 'doc_comments', 'evidence', 'ghost', 'hax', 'inventory']:
        root = OUT / 'fixtures' / name
        if root.exists():
            shutil.rmtree(root)
        shutil.copytree(REPO / 'fixtures' / name, root, ignore=shutil.ignore_patterns('target'))
        for label, tool in [('before', 'baseline'), ('after', 'candidate')]:
            output = OUT / f'fixture-{name}-{label}'
            output.mkdir(parents=True, exist_ok=True)
            args = [OUT / tool, 'report', root, '--size-mode', 'whole-crate', '--json', output / 'report.json', '--markdown', output / 'report.md']
            run(args, output / 'report.log')
            print(f'fixture {name} {label}: PASS', flush=True)


def main():
    p = argparse.ArgumentParser()
    p.add_argument('group', choices=['bertie', 'verus', 'fixtures'])
    globals()[p.parse_args().group]()


if __name__ == '__main__':
    main()
