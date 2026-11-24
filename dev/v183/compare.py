#!/usr/bin/env python3
"""Fail closed on any change outside R1 mappings and their derived totals."""
from collections import Counter, defaultdict
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import re
import subprocess
from baseline import check, ensure, read, ROOT, OUT


def equal(a, b, where):
    ensure(a == b, 'unexpected change: ' + where)


def except_fields(a, b, fields, where):
    equal({k: v for k, v in a.items() if k not in fields},
          {k: v for k, v in b.items() if k not in fields}, where)


def stripped_id(definition):
    module, name = definition['name'].rsplit('.', 1)
    path = module.lower().replace('.', '::')
    match = re.fullmatch(r'v_([A-Z].*)', name)
    if match:
        return path + '::' + match[1]
    match = re.fullmatch(r'impl_(.+?)__v_([A-Z].*)', name)
    return path + '::' + match[1] + '::' + match[2] if match else None


def ratios(a, b, mode, changes, where='proof_ratio'):
    selected_mode = ('runtime' if '.runtime_proven_modes.runtime.' in where else
                     'separate' if '.runtime_proven_modes.separate.' in where else mode)
    if isinstance(a, dict):
        equal(a.keys(), b.keys(), where + ' keys')
        except_fields(a, b, {'proven_extracted_runtime_sloc', 'ratio', 'status',
                            *[k for k, v in a.items() if isinstance(v, (dict, list))]}, where)
        if 'proven_extracted_runtime_sloc' in a:
            # Numerator, Verus and bounded denominators stay fixed; only F* grows.
            relevant = [u for u in changes[selected_mode]
                        if (a.get('crate_name') is None or u['crate_name'] == a['crate_name'])
                        and (a.get('module') is None or u['module'] == a['module'])]
            if where == 'proof_ratio' or where.endswith('.security_relevant'):
                relevant = [u for u in relevant if u['security']]
            equal(b['proven_extracted_runtime_sloc'] - a['proven_extracted_runtime_sloc'],
                  sum(u['sloc'] for u in relevant), where + ' R1 denominator delta')
            denominator = b['proven_runtime_sloc'] + b['proven_extracted_runtime_sloc']
            expected = b['proof_sloc'] / denominator if denominator else None
            equal(b['ratio'], expected, where + ' arithmetic')
            expected_status = ('n/a' if expected is None else 'below' if expected < 5
                               else 'above' if expected > 15 else 'within')
            equal(b['status'], expected_status, where + ' range status')
        for k, v in a.items():
            if isinstance(v, (dict, list)):
                ratios(v, b[k], mode, changes, where + '.' + k)
    elif isinstance(a, list):
        equal(len(a), len(b), where + ' length')
        for i, (x, y) in enumerate(zip(a, b)):
            ratios(x, y, mode, changes, f'{where}[{i}]')
    else:
        equal(a, b, where)


def reason(d, report):
    module, name = d['name'].rsplit('.', 1)
    if re.fullmatch(r'impl(?:_\d+\'?)?(?:__.*)?', name):
        return 'trait impl', 'hax impl/disambiguator name; no unique inherent Rust ID'
    if name.startswith('impl_'):
        return 'other', 'unsupported impl spelling; no unique matching ID'
    rust = module.lower().replace('.', '::') + '::' + name
    if name.endswith('_') and any(u['id'] == rust[:-1] for u in report['units']):
        return 'generated helper', 'hax adds trailing underscore (outside R1)'
    if not any(u['id'] == rust for u in report['units']):
        return 'no Rust unit', 'no exact inventoried Rust ID (may be cfg/backend or macro generated)'
    return 'other', 'ambiguous or unsupported mapping'


lines = ['# v1.8.3 paired replay', '',
         'Baseline: tag `v1.8.2`; candidate: v1.8.3. Bertie: `c66a986` archive under `target/v183/bertie`.',
         'Both binaries use the same original CI log/receipt and reconstructed, hash-identical extraction/library inputs.',
         'Bertie uses compiled-files, CI target x86_64-unknown-linux-gnu; local dependency fallbacks are identical before/after.',
         'Controls reuse v1.8.2 replay settings. No external checkout is modified.',
         'Fresh fallback verification also passed all 18 strict implementation modules (exit 0); paired reports retain original CI evidence.', '']
lines += [check(read(OUT / 'bertie-separate-before.json')), '']
subprocess.run(['python3', str(ROOT / 'dev/v183/controls.py')], check=True)
lines += ['## Controls', '', *[f'- {line}' for line in (OUT / 'controls.txt').read_text().splitlines()], '']
all_unmapped = {}
changes = {}
for ghost_mode in ['separate', 'runtime']:
    a = read(OUT / f'bertie-{ghost_mode}-before.json')
    b = read(OUT / f'bertie-{ghost_mode}-after.json')
    changes[ghost_mode] = [v for u, v in zip(a['units'], b['units']) if u['status'] != v['status']]
for mode in ['separate', 'runtime']:
    before = read(OUT / f'bertie-{mode}-before.json')
    after = read(OUT / f'bertie-{mode}-after.json')
    ensure(all(r['accepted'] for r in before['evidence_runs'] + after['evidence_runs']), 'rejected F* receipt')
    expected = deepcopy(before)
    equal(len(before['units']), len(after['units']), 'unit count')
    changed = []
    counts = Counter(u['id'] for u in after['units'])
    for i, (old, new) in enumerate(zip(before['units'], after['units'])):
        except_fields(old, new, {'status', 'proof_scope'}, 'unit ' + old['id'])
        if old != new:
            ensure(old['status'] != new['status'] and old['crate_name'] == 'bertie_tls'
                   and old['category'] == 'runtime' and new['status'] == 'proven-extracted',
                   'non-F* unit change: ' + old['id'])
            ensure(any(d['rust_unit'] == old['id'] and stripped_id(d) == old['id']
                       for d in after['fstar_definitions']), 'unexplained R1 credit: ' + old['id'])
            changed.append((old, new))
            expected['units'][i] = new
    equal(len(before['fstar_definitions']), len(after['fstar_definitions']), 'definition inventory')
    for i, (old, new) in enumerate(zip(before['fstar_definitions'], after['fstar_definitions'])):
        except_fields(old, new, {'rust_unit', 'classification'}, 'definition ' + old['name'])
        if old != new:
            ensure(old['rust_unit'] is None and new['rust_unit'] == stripped_id(old)
                   and counts[new['rust_unit']] == 1, 'non-R1 definition change: ' + old['name'])
            expected['fstar_definitions'][i] = new
    for key in ['summary', 'summary_default_rules']:
        for old, new in changed:
            security = old.get('default_security') if key == 'summary_default_rules' else old['security']
            if security is None:
                security = old['security']
            block = 'security_relevant' if security else 'everything_else'
            for origin in ['all', 'first_party']:
                statuses = expected[key][origin][block]['statuses']
                statuses[old['status']] -= old['sloc']
                statuses[new['status']] += new['sloc']
    equal(len(before['evidence_runs']), len(after['evidence_runs']), 'evidence run count')
    for i, (old, new) in enumerate(zip(before['evidence_runs'], after['evidence_runs'])):
        if old != new:
            ensure(old['tool'] == new['tool'] == 'hax-fstar', 'non-F* evidence change')
            except_fields(old, new, {'credited_units'}, 'F* receipt')
            equal(set(new['credited_units']), set(old['credited_units']) | {u['id'] for _, u in changed}, 'F* credit additions')
            equal([id_ for id_ in new['credited_units'] if id_ in old['credited_units']], old['credited_units'], 'old credit order')
            expected['evidence_runs'][i] = new
    ratios(before['proof_ratio'], after['proof_ratio'], mode, changes)
    expected['proof_ratio'] = after['proof_ratio']
    equal(expected, after, 'complete report outside audited R1 changes and arithmetic')
    total = lambda r: r['summary']['first_party']['security_relevant']
    a, b = total(before), total(after)
    lines += [f'## Bertie, {mode}', '',
              f"Security proven-extracted: {a['statuses']['proven-extracted']}/{a['sloc']} ({100*a['statuses']['proven-extracted']/a['sloc']:.2f}%) → {b['statuses']['proven-extracted']}/{b['sloc']} ({100*b['statuses']['proven-extracted']/b['sloc']:.2f}%).",
              f'{len(changed)} changed unit statuses; all sizes, categories, inventories, trust, hazards, Verus and non-F* evidence unchanged.', '',
              '| Unit | Old → new | Runtime SLoC | Rule |', '| --- | --- | ---: | --- |']
    lines += [f"| `{u['id']}` | {u['status']} → {v['status']} | {v['sloc']} | R1 uppercase `v_` |" for u, v in changed]
    lines += ['', 'Only F* mappings/classifications, credited-unit lists and status-derived summaries/ratios differ; all other report fields are equal.', '']
    unmapped = [d for d in after['fstar_definitions'] if d['file'].endswith('.fst') and d['classification'] == 'unmapped']
    all_unmapped[mode] = [(d['name'], d['file'], d['line']) for d in unmapped]
    if mode == 'separate':
        groups = defaultdict(list)
        for d in unmapped:
            group, explanation = reason(d, after)
            groups[group].append((d, explanation))
        inventory = []
        for group in ['trait impl', 'no Rust unit', 'generated helper', 'other']:
            inventory += [f'### {group} ({len(groups[group])})', '', '| Definition | Location | Apparent reason |', '| --- | --- | --- |']
            inventory += [f"| `{d['name']}` | `{d['file']}:{d['line']}` | {why} |" for d, why in groups[group]]
            inventory += ['']
equal(all_unmapped['separate'], all_unmapped['runtime'], 'unmapped inventory across modes')
lines += ['## Remaining unmapped .fst definitions', '',
          f"{len(all_unmapped['separate'])} definitions, identical in both modes. Apparent reasons only; no additional mapping fixes.", '', *inventory,
          '## Artifacts', '']
for path in sorted(OUT.glob('bertie-*.json')):
    lines.append(f'- `{path.relative_to(ROOT)}` SHA-256 `{hashlib.sha256(path.read_bytes()).hexdigest()}`')
lines += ['', '## Rerun', '', '```sh',
          'python3 dev/v183/replay.py bertie > logs/v1.8.3/m2-bertie.log 2>&1',
          'python3 dev/v183/replay.py fixtures > logs/v1.8.3/m2-fixtures.log 2>&1',
          'python3 dev/v183/replay.py verus > logs/v1.8.3/m2-verus.log 2>&1',
          'python3 dev/v183/baseline.py > logs/v1.8.3/m2-baseline.log 2>&1',
          'python3 dev/v183/compare.py > logs/v1.8.3/m2-compare.log 2>&1', '```', '',
          'Local checks only; independent acceptance remains Dudley’s. Existing ignored performance test and fresh Verus execution were not run.']
(ROOT / 'dev/v183/compare.md').write_text('\n'.join(lines) + '\n')
print('PASS: CI baseline, paired controls, both-mode R1 status changes, whole-report scope and unmapped inventory')
