# SPDX-License-Identifier: Apache-2.0
"""Render the measured-coverage section of COVERAGE.md from a snapshot and report."""
from coverage_metrics import proof_depth

START = '<!-- measured coverage: begin -->'
END = '<!-- measured coverage: end -->'


def measured_section(b, r):
    rows = []
    for key in ['proven', 'tested', 'unverified']:
        rows.append(f"| {key} | {b[key]} | {100*b[key]/b['total']:.2f}% |")
    ratio = 'n/a' if b['proof_ratio'] is None else f"{b['proof_ratio']:.2f}"
    section = START + '\n\n| Security runtime | Lines | Share |\n| --- | ---: | ---: |\n' + '\n'.join(rows)
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
    section += depth_table(proof_depth(r))
    section += '\n' + END
    return section


def depth_table(depth):
    if depth is None:
        return ''
    proven = sum(depth.values())
    rows = [('safety-only', 'safety_only'),
            ('functional, checked only against own code', 'functional_own_code_only'),
            ('functional', 'functional')]
    share = lambda n: 0.0 if proven == 0 else 100 * n / proven
    table = '\n### Proof depth\n\n| Contract depth | Lines | Share of proven lines |\n| --- | ---: | ---: |\n'
    table += ''.join(f"| {label} | {depth[key]} | {share(depth[key]):.2f}% |\n" for label, key in rows)
    return table + ('\nSafety-only means proven free of panics, overflows and out-of-bounds access, including '
                    'length and range facts; it says nothing about what the code computes.\n')
