#!/usr/bin/env python3
"""v1.5 acceptance F: whole-crate output equals v1.4.2 byte-for-byte, apart from new fields.

Compares replay directories written by scripts/v15_replay.py: the v1.4.2 binary's output
(baseline) against the v1.5 binary run with --size-mode whole-crate and the unchanged
v1.4.2 config. Only the constructs listed in NEW_JSON_* and legacy_markdown() are removed
from the v1.5 output before the byte comparison; anything else must match exactly.
"""
import argparse
import json
from pathlib import Path
import sys

# New top-level report keys (and the new schema number).
NEW_JSON_KEYS = ['size_mode', 'size_profile', 'binary_sha256', 'size_fallbacks',
                 'config_schema', 'inputs', 'summary_default_rules']
# New keys on each dependencies[] / build_dependencies[] entry.
NEW_DEPENDENCY_KEYS = ['size_mode', 'size_reason', 'whole_crate_sloc',
                       'default_security', 'default_sloc', 'default_size_unknown', 'override_reason']
# New key on units[] entries moved by an override (absent otherwise).
NEW_UNIT_KEYS = ['default_security']
# New warnings: one per schema-2 override entry.
NEW_WARNING_PREFIX = 'override has no reason: '
OLD_SCHEMA, NEW_SCHEMA = 2, 3


def legacy_json(report):
    assert report['schema'] == NEW_SCHEMA, report['schema']
    assert report['size_mode'] == 'whole-crate', report['size_mode']
    report = {k: v for k, v in report.items() if k not in NEW_JSON_KEYS}
    report['schema'] = OLD_SCHEMA
    for key in ['dependencies', 'build_dependencies']:
        report[key] = [{k: v for k, v in d.items() if k not in NEW_DEPENDENCY_KEYS} for d in report[key]]
    report['units'] = [{k: v for k, v in u.items() if k not in NEW_UNIT_KEYS} for u in report['units']]
    report['warnings'] = [w for w in report['warnings'] if not w.startswith(NEW_WARNING_PREFIX)]
    return report


def legacy_markdown(text):
    lines = text.split('\n')
    # 1. New first line (size mode, target, profile, binary sha256) and its blank line.
    assert lines[0].startswith('Size mode: `whole-crate`;'), lines[0]
    assert lines[1] == ''
    lines = lines[2:]
    out = []
    skipping = False
    for line in lines:
        # 2. New "(default rules only)" headline lines.
        if '(default rules only)' in line:
            continue
        # 3. New "## Inputs" section, up to the next section heading.
        if line == '## Inputs':
            skipping = True
            continue
        if skipping:
            if line.startswith('## '):
                skipping = False
            else:
                continue
        # 4. New schema-2 "override has no reason" warning bullets.
        if line.startswith('- ' + NEW_WARNING_PREFIX):
            continue
        # 5. "(with inputs)" label on the existing security headline lines.
        out.append(line.replace(' (with inputs)', ''))
    return '\n'.join(out)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('baseline', type=Path)
    parser.add_argument('current', type=Path)
    args = parser.parse_args()
    failures = 0
    for case in sorted(p.name for p in args.baseline.iterdir() if (p / 'report.json').is_file()):
        old_json = (args.baseline / case / 'report.json').read_bytes()
        old_md = (args.baseline / case / 'report.md').read_bytes()
        new = json.loads((args.current / case / 'report.json').read_text())
        new_json = (json.dumps(legacy_json(new), indent=2) + '\n').encode()
        new_md = legacy_markdown((args.current / case / 'report.md').read_text()).encode()
        same_json, same_md = new_json == old_json, new_md == old_md
        print(f'{case}: json {"identical" if same_json else "DIFFERS"}, markdown {"identical" if same_md else "DIFFERS"}')
        if not (same_json and same_md):
            failures += 1
            for name, a, b in [('json', old_json, new_json), ('md', old_md, new_md)]:
                if a != b:
                    import difflib
                    diff = difflib.unified_diff(a.decode().splitlines(), b.decode().splitlines(), 'v1.4.2', 'v1.5', lineterm='', n=1)
                    print('\n'.join(list(diff)[:40]))
    print('F: PASS' if not failures else f'F: FAIL ({failures} cases differ)')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
