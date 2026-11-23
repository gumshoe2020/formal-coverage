#!/usr/bin/env python3
"""Exact paired control report and Markdown equality; no normalization."""
import json
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/v182'
results = []
for name in ['scratch-spec-lemmas', 'vest', *['fixture-' + n for n in ['classification', 'doc_comments', 'evidence', 'ghost', 'hax', 'inventory']]]:
    for mode in ['separate', 'runtime']:
        before = OUT / f'{name}-{mode}-before'
        after = OUT / f'{name}-{mode}-after'
        a = json.loads((before / 'report.json').read_text())
        b = json.loads((after / 'report.json').read_text())
        if a != b:
            keys = [k for k in a.keys() | b.keys() if a.get(k) != b.get(k)]
            raise AssertionError(f'HONEST-FINDING: control {name} {mode} changed: {keys}')
        assert (before / 'report.md').read_bytes() == (after / 'report.md').read_bytes(), f'HONEST-FINDING: control Markdown {name} {mode}'
        results.append(f'{name} {mode}: exact JSON and Markdown equality PASS')
(OUT / 'controls.txt').write_text('\n'.join(results) + '\n')
print('\n'.join(results))
