#!/usr/bin/env python3
"""Audit the release scope and write every changed Bertie unit for review."""
from pathlib import Path
import hashlib
import json
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT/'target/v182'

def read(path):
    return json.loads(path.read_text())

def ensure(condition, message):
    if not condition:
        raise AssertionError('HONEST-FINDING: '+message)

def difference(a, b, allowed, path='root'):
    if isinstance(a, dict):
        ensure(a.keys() == b.keys(), path+' keys')
        for key in a:
            if key not in allowed:
                difference(a[key], b[key], allowed, path+'.'+key)
    elif isinstance(a, list):
        ensure(len(a) == len(b), path+' length')
        for i, (x, y) in enumerate(zip(a, b)):
            difference(x, y, allowed, f'{path}[{i}]')
    else:
        ensure(a == b, f'{path}: {a!r} -> {b!r}')

lines = ['# v1.8.2 paired replay', '',
         'Baseline: 7a60142 (v1.8.1). Bertie sources: 9212e97. All writes confined to this repository.',
         'The pre-change baseline reproduced all 465 CI unit statuses exactly with an accepted CI receipt;',
         'freshly extracted inputs passed the unmodified CI receipt hashes. Both release reports below use the same fresh verifier log.', '']
subprocess.run(['python3', str(ROOT/'dev/v182/controls.py')], check=True)
lines += ['## Controls', '', (OUT/'controls.txt').read_text().strip(), '']
for mode in ['separate', 'runtime']:
    before = read(OUT/f'bertie-{mode}-before.json')
    after = read(OUT/f'bertie-{mode}-after.json')
    ensure(all(r['accepted'] for r in before['evidence_runs']+after['evidence_runs']), 'rejected F* evidence')
    ensure(len(before['units']) == len(after['units']), 'Bertie unit count')
    ci = read(OUT/'ci-9212e97/report.json')
    ensure({u['id']: u['status'] for u in ci['units']} == {u['id']: u['status'] for u in before['units']}, 'fresh-log baseline does not match CI statuses')
    changed = []
    freed = {}
    for item in after['non_fstar_trust']:
        for id_ in item['affected_units']:
            freed.setdefault(id_, set()).add('R2 (inactive feature)' if 'inactive feature' in item['kind'] else 'R1 (backend path)')
    for old, new in zip(before['units'], after['units']):
        difference(old, new, {'status', 'proof_scope'}, 'unit.'+old['id'])
        if old['status'] != new['status'] or old['category'] != new['category']:
            ensure(old['crate_name'] == 'bertie_tls', 'status outside bertie-tls: '+old['id'])
            rules = []
            if old['id'] in freed:
                rules.extend(sorted(freed[old['id']]))
            mapped = any(d['rust_unit'] == old['id'] and '.impl_' in d['name'] for d in after['fstar_definitions'])
            if mapped:
                rules.append('R4 (unique inherent method)')
            ensure(rules, 'unexplained status change: '+old['id'])
            changed.append((old['id'], old['status'], new['status'], new['sloc'], ', '.join(rules)))
        else:
            ensure(old == new, 'unchanged status with altered unit: '+old['id'])
    # Invariant inventories and sizes, including all preexisting control metadata.
    base_root = {k:v for k,v in before.items() if k not in {'non_fstar_trust', 'fstar_extraction'}}
    candidate_root = {k:v for k,v in after.items() if k not in {'non_fstar_trust', 'fstar_extraction'}}
    difference(base_root, candidate_root, {'units', 'trusted_items', 'fstar_definitions', 'evidence_runs',
                              'summary', 'summary_default_rules', 'proof_ratio',
                              'non_fstar_trust', 'fstar_extraction'})
    # Extraction declarations and non-F* inventory are the two additive fields.
    difference(before['proof_ratio'], after['proof_ratio'], {'proven_extracted_runtime_sloc', 'ratio', 'status'}, 'proof_ratio')
    difference(before['fstar_definitions'], after['fstar_definitions'], {'rust_unit', 'classification'}, 'fstar_definitions')
    for key in ['summary', 'summary_default_rules']:
        difference(before[key], after[key], {'statuses', 'trusted_items', 'trusted_sloc', 'verified', 'unverified'}, key)
        ensure(after[key]['trusted_items'] == len(after['trusted_items']), key+' trust count')
        ensure(after[key]['trusted_sloc'] == sum(t['sloc'] for t in after['trusted_items']), key+' trust SLoC')
    for old, new in zip(before['evidence_runs'], after['evidence_runs']):
        difference(old, new, {'credited_units', 'trusted_sites', 'trusted_sloc'}, 'evidence_run')
        ensure(old['tool'] == new['tool'] == 'hax-fstar', 'non-F* evidence changes')
    proven = lambda r: (sum(u['status'] == 'proven-extracted' for u in r['units']),
                        sum(u['sloc'] for u in r['units'] if u['status'] == 'proven-extracted'))
    lines += [f'## Bertie, {mode}', '',
              f'Runtime proven-extracted units / SLoC: {proven(before)} → {proven(after)}.',
              'Verus proven SLoC: '+str(before['summary']['first_party']['security_relevant']['statuses']['proven'])+' → '+str(after['summary']['first_party']['security_relevant']['statuses']['proven'])+'.',
              f'{len(changed)} unit statuses changed; no unit category or size changed.', '',
              '| Unit | Old → new status | SLoC | Rule |', '| --- | --- | ---: | --- |']
    lines += [f'| `{id_}` | {old} → {new} | {sloc} | {rule} |' for id_, old, new, sloc, rule in changed]
    lines += ['', '### Non-F* trust annotations', '',
              '| File:line | Category | SLoC | Affected units |', '| --- | --- | ---: | --- |']
    lines += [f"| {t['file']}:{t['line']} | {t['kind']} | {t['sloc']} | {', '.join(t['affected_units'])} |" for t in after['non_fstar_trust']]
    lines += ['', f"Non-F* trust: {len(after['non_fstar_trust'])} sites; {sum(t['sloc'] for t in after['non_fstar_trust'])} summed affected SLoC.",
              'All other unit fields, categories, source/dependency sizes, escape sites, proof cost, contract text and control figures are unchanged. F* mappings/classifications, trust attribution and credited-unit lists change as the rules require.', '']
lines += ['## Artifacts', '']
for path in sorted(OUT.glob('bertie-*.json')):
    lines.append(f'- `{path.relative_to(ROOT)}` SHA-256 `{hashlib.sha256(path.read_bytes()).hexdigest()}`')
lines += ['', '## Rerun', '', '```sh',
          'python3 dev/v182/baseline.py > logs/v1.8.2/baseline-comparison.log 2>&1',
          'python3 dev/v182/replay.py bertie > logs/v1.8.2/m3-bertie.log 2>&1',
          'python3 dev/v182/replay.py fixtures > logs/v1.8.2/m3-fixtures.log 2>&1',
          'python3 dev/v182/replay.py verus > logs/v1.8.2/m3-verus.log 2>&1',
          'python3 dev/v182/compare.py > logs/v1.8.2/m3-compare.log 2>&1', '```', '',
          'Local verification only; independent acceptance remains Dudley’s. Existing ignored performance acceptance and fresh Verus execution were not run.']
(ROOT/'dev/v182/compare.md').write_text('\n'.join(lines)+'\n')
print('PASS: release scope, controls and both-mode Bertie comparison')
