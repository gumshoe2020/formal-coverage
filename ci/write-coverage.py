#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
"""Update committed coverage numbers from a measured report; never invent credit."""
import argparse
import json
from pathlib import Path
from coverage_metrics import snapshot
from coverage_section import END, START, measured_section

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('report', type=Path)
p.add_argument('--check', action='store_true', help='verify baseline and numeric document are current')
p.add_argument('--baseline', type=Path, default=Path('coverage/baseline.json'))
p.add_argument('--coverage', type=Path, default=Path('COVERAGE.md'))
a = p.parse_args()
r = json.loads(a.report.read_text())
b = snapshot(r)
encoded = json.dumps(b, indent=2, sort_keys=True) + '\n'
start, end = START, END
proven_pct = b['proven'] / b['total'] * 100
ratio = 'n/a' if b['proof_ratio'] is None else f"{b['proof_ratio']:.2f}"
section = measured_section(b, r)
baseline = a.baseline
page = a.coverage
text = page.read_text()
updated = text[:text.index(start)] + section + text[text.index(end)+len(end):]
if a.check:
    if baseline.read_text() != encoded or text != updated:
        p.error('baseline.json or COVERAGE.md differs from measured report')
    print('coverage baseline and document match measurement')
else:
    baseline.parent.mkdir(parents=True, exist_ok=True)
    baseline.write_text(encoded)
    page.write_text(updated)
    print(f'coverage: proven={b["proven"]} tested={b["tested"]} unverified={b["unverified"]} proven_share={proven_pct:.4f}% ratio={ratio}')
