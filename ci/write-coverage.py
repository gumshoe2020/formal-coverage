#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
"""Update committed coverage numbers from a measured report; never invent credit."""
import argparse
import json
from pathlib import Path
from coverage_metrics import snapshot

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('report', type=Path)
p.add_argument('--check', action='store_true', help='verify baseline and numeric document are current')
p.add_argument('--baseline', type=Path, default=Path('coverage/baseline.json'))
p.add_argument('--coverage', type=Path, default=Path('COVERAGE.md'))
a = p.parse_args()
r = json.loads(a.report.read_text())
b = snapshot(r)
encoded = json.dumps(b, indent=2, sort_keys=True) + '\n'
start = '<!-- measured coverage: begin -->'
end = '<!-- measured coverage: end -->'
proven_pct = b['proven'] / b['total'] * 100
rows = []
for key in ['proven', 'tested', 'unverified']:
    rows.append(f"| {key} | {b[key]} | {100*b[key]/b['total']:.2f}% |")
ratio = 'n/a' if b['proof_ratio'] is None else f"{b['proof_ratio']:.2f}"
section = start + '\n\n| Security runtime | Lines | Share |\n| --- | ---: | ---: |\n' + '\n'.join(rows)
section += f"\n\nTotal: **{b['total']}** first-party security runtime SLoC. Proven includes explicitly labelled\n`proven-extracted` hax credit ({b['statuses'].get('proven-extracted',0)} lines); Verus proven: {b['statuses'].get('proven',0)} lines.\n"
proof = r['proof_ratio']
proven_runtime = proof['proven_runtime_sloc'] + proof['proven_extracted_runtime_sloc']
section += f"\nProof-to-code ratio: **{ratio}** ({b['proof_sloc']} proof SLoC / {proven_runtime} proven + proven-extracted runtime SLoC); informational, never blocking.\n"
section += (f"\nProof components: {proof['fstar_annotation_sloc']} hax annotation SLoC, "
            f"{proof['fstar_cfg_hax_sloc']} standalone `cfg(hax)` proof-code SLoC, "
            f"and {proof['fstar_cfg_hax_inline_sloc']} inline `cfg(hax)` SLoC "
            "(overlaps count once). "
            f"F* models: {proof['fstar_model_sloc']} SLoC of trusted assumptions, excluded from the ratio.\n")
if proof['modules']:
    section += '\n| Module | Proven runtime | Proof SLoC | Proof / proven runtime |\n| --- | ---: | ---: | ---: |\n'
for mod in proof['modules']:
    proven = mod['proven_runtime_sloc'] + mod['proven_extracted_runtime_sloc']
    val = 'n/a' if mod['ratio'] is None else f"{mod['ratio']:.2f}"
    if proven or mod['proof_sloc']:
        section += f"| {mod['crate_name']}::{mod['module']} | {proven} | {mod['proof_sloc']} | {val} |\n"
section += '\n' + end
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
