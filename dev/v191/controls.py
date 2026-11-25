#!/usr/bin/env python3
"""Exact bytes: no JSON normalization, fields removed, or Markdown rewriting."""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/v191'


def check():
    results = []
    names = ['scratch-spec-lemmas', 'vest', *['fixture-' + name for name in
             ('classification', 'doc_comments', 'evidence', 'ghost', 'hax', 'inventory')]]
    for name in names:
        for mode in ('separate', 'runtime'):
            for extension in ('json', 'md'):
                before = OUT / f'{name}-{mode}-before' / f'report.{extension}'
                after = OUT / f'{name}-{mode}-after' / f'report.{extension}'
                assert before.read_bytes() == after.read_bytes(), (
                    f'HONEST-FINDING: {name} {mode} {extension} differs')
            results.append(f'{name} {mode}: exact JSON + Markdown equality PASS')
    return results


if __name__ == '__main__':
    print('\n'.join(check()))
