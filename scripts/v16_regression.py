#!/usr/bin/env python3
"""v1.6 M2: compare all five v15_replay cases against v1.5 at 887b730.

Only the explicitly enumerated new fields and report additions are removed.
No rounding, reordering of arrays, normalization, or old-value changes allowed.
"""
import argparse
import json
from pathlib import Path

NEW_REPORT_KEYS = ['ghost_lines', 'unparseable_files', 'proof_ratio_range',
                   'no_proof_ratio_gate', 'proof_ratio']
NEW_UNIT_KEYS = ['runtime_sloc', 'inline_proof_sloc', 'ghost_sloc']
CASES = ['scratch-spec-lemmas', 'spec-lemmas-host', 'vest', 'anvil', 'firecracker']


def legacy_json(report):
    assert report['ghost_lines'] == 'runtime'
    report = {k: v for k, v in report.items() if k not in NEW_REPORT_KEYS}
    report['units'] = [{k: v for k, v in u.items() if k not in NEW_UNIT_KEYS} for u in report['units']]
    return report


def legacy_markdown(text):
    suffix = ' Ghost lines: `runtime`;'
    lines = text.split('\n')
    assert lines[0].endswith(suffix)
    lines[0] = lines[0][:-len(suffix)]
    text = '\n'.join(lines)
    # New ratio section, including the paired ghost-mode figures. The existing
    # Units section and all subsequent bytes must remain unchanged.
    marker = '## Proof ratio\n'
    if marker in text:
        start = text.index(marker)
        end = text.index('\n## Units\n', start)
        text = text[:start] + text[end:]
    return text


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('baseline', type=Path)
    p.add_argument('current', type=Path)
    args = p.parse_args()
    failures = 0
    for case in CASES:
        a, b = args.baseline / case, args.current / case
        old = (a / 'report.json').read_bytes()
        new = (json.dumps(legacy_json(json.loads((b / 'report.json').read_text())), indent=2) + '\n').encode()
        old_md = (a / 'report.md').read_bytes()
        new_md = legacy_markdown((b / 'report.md').read_text()).encode()
        okay = old == new and old_md == new_md
        print(f'{case}: JSON {"identical" if old == new else "DIFFERS"}, Markdown {"identical" if old_md == new_md else "DIFFERS"}')
        failures += not okay
        if not okay:
            import difflib
            for x, y in [(old, new), (old_md, new_md)]:
                if x != y:
                    print('\n'.join(list(difflib.unified_diff(x.decode().splitlines(), y.decode().splitlines(), n=1))[:40]))
    print('M2: PASS (5/5)' if not failures else f'M2: FAIL ({failures})')
    return bool(failures)


if __name__ == '__main__':
    raise SystemExit(main())
