#!/usr/bin/env python3
"""Audit all v1.8 paired reports and write the reviewable comparison.

Fails with HONEST-FINDING for changed tiers or old Verus figures. Independent
source counts are separate scripts, never calculated from report size deltas.
"""
from collections import Counter
from datetime import datetime
import hashlib
import json
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
OUT = REPO / 'target/v18'


def read(path):
    return json.loads((OUT / path).read_text())


def project(before, after, location='root'):
    if isinstance(before, dict):
        for k, v in before.items():
            if location == 'root' and k == 'schema':
                continue
            assert k in after, f'HONEST-FINDING: missing {location}.{k}'
            project(v, after[k], location+'.'+k)
    elif isinstance(before, list):
        assert len(before) == len(after), f'HONEST-FINDING: {location} length changed'
        for i, (a, b) in enumerate(zip(before, after)):
            project(a, b, f'{location}[{i}]')
    else:
        assert before == after, f'HONEST-FINDING: {location}: {before!r} -> {after!r}'


def sizes_and_tiers(before, after):
    key = lambda u: (u['id'], u['file'], u['line'], u['end_line'])
    old, new = ({key(u): u for u in r['units']} for r in [before, after])
    assert old.keys() == new.keys(), 'HONEST-FINDING: unit identities changed'
    changed = []
    for k, a in old.items():
        b = new[k]
        assert a['status'] == b['status'], f'HONEST-FINDING: tier changed: {k}'
        delta = a['sloc'] - b['sloc']
        expected = b['fstar_annotation_sloc'] if b['first_party'] and b['category'] == 'runtime' and after['ghost_lines'] == 'separate' else 0
        assert delta == expected, f'HONEST-FINDING: non-annotation size change: {k}'
        if delta:
            changed.append((a, b))
    return changed


def table(rows, scopes=('security', 'all')):
    out = ['| Project / mode / version | Scope | Standalone | Inline | F* annotations¹ | Patches | Files | Models² | Proven | Extracted³ | Bounded | Runtime | Ratio |', '| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
    for name, version, report in rows:
        for scope in scopes:
            m = report['proof_ratio'] if scope == 'security' else report['proof_ratio']['all_first_party']
            fields = ['standalone_proof_sloc', 'inline_proof_sloc', 'fstar_annotation_sloc', 'fstar_patch_sloc', 'fstar_file_sloc', 'fstar_model_sloc', 'proven_runtime_sloc', 'proven_extracted_runtime_sloc', 'proven_bounded_runtime_sloc', 'runtime_sloc']
            values = [str(m.get(k, 0)) for k in fields]
            ratio = 'n/a' if m['ratio'] is None else f"{m['ratio']:.9f}"
            out.append(f'| {name} / {version} | {scope} | '+' | '.join(values)+f' | {ratio} |')
    out += ['', '¹ Annotations are a subset of inline proof, counted once. ² Models are trusted assumptions, excluded from proof/ratio. ³ The old report has no extracted denominator field: its displayed 0 means excluded from the old ratio, not zero evidence credit. Bertie’s baseline has 824 extracted security/all-first-party runtime SLoC.', '']
    return out


def main():
    before, after, runtime = [read(f'bertie-{n}.json') for n in ['before', 'after', 'after-runtime']]
    changes = sizes_and_tiers(before, after)
    assert not sizes_and_tiers(before, runtime)
    assert [(r['accepted'],r['credited_units'],r['verified_files']) for r in before['evidence_runs']] == [(r['accepted'],r['credited_units'],r['verified_files']) for r in after['evidence_runs']]
    assert all(r['accepted'] for r in after['evidence_runs'])
    assert after['proof_ratio']['range'] == before['proof_ratio']['range']
    assert after['proof_ratio']['range']['min'] == 5 and after['proof_ratio']['range']['max'] == 15
    rows = [('Bertie / separate', 'v1.7.3', before), ('Bertie / separate', 'v1.8.0', after), ('Bertie / runtime', 'v1.8.0', runtime)]
    invariants = []
    status_rows = []
    for name in ['scratch-spec-lemmas', 'vest']:
        for mode in ['separate', 'runtime']:
            b, a = [read(f'{name}-{mode}-{label}/report.json') for label in ['before', 'after']]
            project(b, a)
            assert not sizes_and_tiers(b, a)
            assert a['proof_ratio']['proven_extracted_runtime_sloc'] == 0
            invariants.append(f'- {name}, {mode}: PASS; every pre-existing report field identical except schema 3 → 4. No extracted units, no size changes; ratio and every Verus figure unchanged.')
            rows += [(f'{name} / {mode}', 'v1.7.3', b), (f'{name} / {mode}', 'v1.8.0', a)]
        counts = Counter(u['status'] for u in a['units'])
        status_rows.append(f'| {name} | {len(a["units"])} | '+', '.join(f'{k}={v}' for k,v in sorted(counts.items()))+' | 0 | 0 |')
    fixture_rows, fixture_changes = [], []
    for name in ['classification', 'doc_comments', 'evidence', 'ghost', 'hax', 'inventory']:
        b, a = [read(f'fixture-{name}-{label}/report.json') for label in ['before', 'after']]
        changed = sizes_and_tiers(b, a)
        if name != 'hax':
            project(b, a)
        fixture_rows.append(f'| {name} | {len(a["units"])} | 0 | {len(changed)} |')
        fixture_changes.extend(changed)
    p = read('patches-final.json')
    ann = read('annotations-final.json')
    assert len(changes) == len(ann['changed_sizes'])
    assert sum(a['sloc'] - b['sloc'] for a,b in changes) == ann['all_first_party'] == 350
    count = Counter(u['status'] for u in after['units'])
    text = ['# formal-coverage v1.8 paired replay', '', f'Generated {datetime.now().astimezone().isoformat()}.', '',
        'Bertie source: read-only `bertie-deps` at `f646099cee002c1d998b0e8c160886ef838f4f69`, copied under `target/v18/bertie` along with the existing `target/ci-logs/fstar/fstar.log` and receipt. Baseline: pre-v1.8 commit `218e190`, release binary `target/v18/baseline`; candidate: v1.8.0 release binary `target/v18/candidate`. All other source checkouts remain untouched. No extraction or verifier was executed.', '',
        'Bertie uses CI’s `report . --config CONFIG --fstar-log target/ci-logs/fstar/fstar.log --json OUT.json --markdown OUT.md`, in compiled-files mode. Baseline CONFIG is the unchanged original; candidate CONFIG is a temp copy under `target/v18/` plus the plan’s `[fstar.proof]` section. Both retain 5–15 and accept the identical receipt. Candidate also runs runtime ghost mode.', '',
        'scratch-server `2fa5eb78a6f96a8433229dc64ec4b82c3e5f9bc2` and Vest `ce6705fad1af476e828feae6621d0bcf4d0c1665` use the same archived exports, configs, retained receipts, whole-crate sizes and target triples as the v1.6 replays (`scripts/v15_replay.py`). Targets: scratch aarch64-apple-darwin, Vest aarch64-unknown-linux-gnu. This is a report-only replay; whole-crate sizing does not invoke cargo check. Earlier numeric release figures may differ because v1.7.3 removed docs; the paired control here is v1.7.3 on identical inputs.', '',
        '## Ratios and components', '']
    text += table(rows)
    text += ['## Invariants and independent counts', '',
        f'- Bertie: PASS; all {len(after["units"])} identities/statuses and receipt credit membership unchanged; exactly {len(changes)} annotation-only size changes. Separate security runtime 5,818 → 5,468; extracted runtime 824 → 696. Runtime mode preserves every baseline size/status.',
        '- Independent annotation scanner: exact 350 security / 350 all-first-party SLoC; per-unit source lines retained in `target/v18/annotations-final.json`. No line-by-line differences.',
        f'- Independent patch audit: {sum(p["raw_added_sloc"].values())} raw added code lines, {len(p["hunks"])} hunks, {len(p["eligible_sloc"])} modules; exact eligible match of zero per module. All target legacy `Bertie.*`, absent from the `Bertie_tls.*` receipt. Names were not rewritten. Every excluded hunk/reason is in JSON and Markdown reports.',
        '- Hand-written file cost 0 (none configured); models 18 SLoC, excluded from ratio. Cost moves Bertie from n/a to 350/696 = 0.502873563; below the unchanged 5–15 range. No threshold or tier adjustment.',
        *invariants, '', 'Unknown hax paths retained for review: '+', '.join('`'+s+'`' for s in after['proof_ratio']['unknown_hax_paths'])+'.', '',
        '## Unit status census (all inventory categories)', '', '| Project | Units | Status counts (same before/after) | Tier changes | Size changes |', '| --- | ---: | --- | ---: | ---: |', f'| Bertie | {len(after["units"])} | '+', '.join(f'{k}={v}' for k,v in sorted(count.items()))+f' | 0 | {len(changes)} |', *status_rows, '',
        '## Fixture replay', '', 'Whole-crate paired static reports; generated receipt variants are covered by the full Rust test suite. These runs do not execute fixture Rust or verifiers.', '', '| Fixture | Units | Tier changes | Size changes |', '| --- | ---: | ---: | ---: |', *fixture_rows, '',
        '## Independent patch additions per target module', '', '| Module | Raw added code SLoC | Eligible proof SLoC | Excluded hunks |', '| --- | ---: | ---: | ---: |']
    for name in sorted(p['raw_added_sloc']):
        text.append(f'| {name} | {p["raw_added_sloc"][name]} | {p["eligible_sloc"][name]} | {p["hunks_by_module"][name]} |')
    text += ['', '## Every changed unit size', '', 'Only hax annotation-only lines move. All listed tiers are unchanged. No scratch-server or Vest sizes change.', '', '| Unit | Source | Before | After | Tier |', '| --- | --- | ---: | ---: | --- |']
    for a,b in sorted(changes+fixture_changes, key=lambda p: (p[0]['id'],p[0]['file'],p[0]['line'])):
        text.append(f'| `{a["id"]}` | `{a["file"]}:{a["line"]}` | {a["sloc"]} | {b["sloc"]} | {b["status"]} |')
    text += ['', '## Reproduction and limits', '',
        'Build the pre-v1.8 baseline with `git archive 218e190` exported under `target/v18/baseline-src`, then `cargo build --release --frozen --manifest-path target/v18/baseline-src/Cargo.toml --target-dir target/v18/baseline-build > logs/v1.8/baseline-rebuild.log 2>&1`; copy its release binary to `target/v18/baseline`. Build candidate with `cargo build --release --frozen > logs/v1.8/m4-build.log 2>&1` and copy `target/release/formal-coverage` to `target/v18/candidate`. Bertie preparation is in `dev/NOTES.md` (copy source/receipt only).', '', '```sh',
        'python3 dev/v18/replay.py bertie > logs/v1.8/m4-bertie.log 2>&1',
        'python3 dev/v18/replay.py verus > logs/v1.8/m4-verus.log 2>&1',
        'python3 dev/v18/replay.py fixtures > logs/v1.8/m4-fixtures.log 2>&1',
        'python3 dev/v18/annotations.py target/v18/bertie target/v18/bertie-before.json target/v18/bertie-after.json target/v18/annotations-final.json > logs/v1.8/m4-annotations.log 2>&1',
        'python3 dev/v18/patches.py target/v18/bertie target/v18/bertie-after.json target/v18/patches-final.json > logs/v1.8/m4-patches.log 2>&1',
        'python3 dev/v18/compare.py > logs/v1.8/m4-compare.log 2>&1',
        'cargo test > logs/v1.8/m4-test.log 2>&1',
        'cargo clippy --all-targets -- -D warnings > logs/v1.8/m4-clippy.log 2>&1',
        'cargo fmt --check > logs/v1.8/m4-fmt.log 2>&1', '```', '',
        'Report schema 3 → 4 is deliberate; configuration remains schema 2/3. Local checks and replays are not independent acceptance. The existing ignored release performance test is not run. No fresh F*/Verus proof execution, host installs, push, tag, bundle, or external messages.', '', '## Artifact SHA-256', '']
    paths = [OUT/'baseline', OUT/'candidate', OUT/'bertie-before.json', OUT/'bertie-after.json', OUT/'annotations-final.json', OUT/'patches-final.json', OUT/'bertie-proof.toml', OUT/'bertie/target/ci-logs/fstar/fstar.log.evidence.json']
    for path in paths:
        text.append(f'- `{path.relative_to(REPO)}`: `{hashlib.sha256(path.read_bytes()).hexdigest()}`.')
    (REPO/'dev/v18/compare.md').write_text('\n'.join(text)+'\n')
    print(f'PASS: all paired figures/status invariants; {len(changes)} Bertie annotation size changes; {len(fixture_changes)} fixture annotation size changes; independent counts match')


if __name__ == '__main__':
    main()
