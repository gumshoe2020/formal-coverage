#!/usr/bin/env python3
"""Require v1.6 report equality, allowing only the new empty extracted column."""
import argparse
import json
from pathlib import Path
from v16_regression import CASES


def legacy(value):
    if isinstance(value, dict):
        if 'proven-extracted' in value:
            assert value['proven-extracted'] == 0, 'unexpected hax credit in retained Verus/Kani replay'
        return {k: legacy(v) for k, v in value.items() if k != 'proven-extracted'}
    if isinstance(value, list):
        return [legacy(v) for v in value]
    return value


def compare(before, after):
    failures = 0
    for case in CASES:
        a, b = before / case, after / case
        old = json.loads((a / 'report.json').read_text())
        new = legacy(json.loads((b / 'report.json').read_text()))
        md = (b / 'report.md').read_text().replace('  proven-extracted 0.00%', '')
        same_json = (a / 'report.json').read_bytes() == (json.dumps(new, indent=2) + '\n').encode()
        same_md = (a / 'report.md').read_text() == md
        print(f'{case}: JSON {"identical" if same_json else "DIFFERS"}; Markdown {"identical" if same_md else "DIFFERS"}')
        failures += not (same_json and same_md)
        if not (same_json and same_md):
            import difflib
            for x, y in [(json.dumps(old, indent=2), json.dumps(new, indent=2)), ((a/'report.md').read_text(), md)]:
                if x != y:
                    print('\n'.join(list(difflib.unified_diff(x.splitlines(), y.splitlines(), n=1))[:30]))
    assert not failures, f'{failures} v1.6 cases changed'
    print('PASS: five v1.6 cases unchanged except the empty extracted column')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('baseline', type=Path)
    parser.add_argument('current', type=Path)
    args = parser.parse_args()
    compare(args.baseline, args.current)
