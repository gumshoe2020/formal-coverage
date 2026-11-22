#!/usr/bin/env python3
"""Fail-closed paired audit and reviewable v1.8.1 comparison."""
from copy import deepcopy
from datetime import datetime
import hashlib
import json
from pathlib import Path
from cfghax import audit

REPO=Path(__file__).resolve().parents[2]
OUT=REPO/'target/v181'
NEW={'fstar_cfg_hax_sloc','fstar_cfg_hax_inline_sloc'}
METRICS=['standalone_proof_sloc','inline_proof_sloc','proof_sloc','fstar_annotation_sloc',*sorted(NEW),'fstar_patch_sloc','fstar_file_sloc','fstar_model_sloc','excluded_patch_hunks','proven_runtime_sloc','proven_extracted_runtime_sloc','proven_bounded_runtime_sloc','runtime_sloc','ratio','status']


def read(path): return json.loads((OUT/path).read_text())
def ensure(ok,message):
    if not ok: raise AssertionError('HONEST-FINDING: '+message)


def identity(old,new,path='root'):
    if isinstance(old,dict):
        ensure(isinstance(new,dict),path+' type')
        ensure(old.keys() <= new.keys(),path+' lost fields')
        for key in new.keys()-old.keys(): ensure(key in NEW and new[key]==0,path+'.'+key+' unexpected new field')
        for key in old: identity(old[key],new[key],path+'.'+key)
    elif isinstance(old,list):
        ensure(len(old)==len(new),path+' list length')
        for i,(a,b) in enumerate(zip(old,new)): identity(a,b,f'{path}[{i}]')
    else: ensure(old==new,f'{path}: {old!r} -> {new!r}')


def markdown_without_columns(text):
    lines=text.splitlines();active=False
    for i,line in enumerate(lines):
        if '| cfg(hax) standalone | cfg(hax) inline |' in line: active=True
        elif active and not line.startswith('|'): active=False
        if active:
            parts=line.split('|'); del parts[3:5]; lines[i]='|'.join(parts)
    return '\n'.join(lines)+'\n'


def summary_expected(before,units,defaults=False):
    s=deepcopy(before)
    sec=lambda u: u.get('default_security',u['security']) if defaults else u['security']
    s['proof_code']={}
    for key,selected in [('total',lambda u:True),('security_relevant',sec),('everything_else',lambda u:not sec(u))]:
        proof=[u for u in units if u['category']=='proof-code' and selected(u)]
        s['proof_code'][key]=dict(sloc=sum(u['sloc'] for u in proof),units=len(proof),verified=sum(u['status']=='verified' for u in proof),unverified=sum(u['status']!='verified' for u in proof),hazard_units=sum(u['status']=='hazard' for u in proof))
    tests=[u for u in units if u['category']=='test']
    if tests:
        s['test_code']=dict(sloc=sum(u['sloc'] for u in tests),units=len(tests),security_reachable_sloc=sum(u['sloc'] for u in tests if sec(u)),security_reachable_units=sum(bool(sec(u)) for u in tests))
    else: s.pop('test_code',None)
    for side,security in [('security_relevant',True),('everything_else',False)]:
        runtime=[u for u in units if u['category']=='runtime' and sec(u)==security]
        totals=s['first_party'][side]
        totals['sloc']=sum(u['sloc'] for u in runtime)
        totals['statuses']={status:sum(u['sloc'] for u in runtime if u['status']==status) for status in totals['statuses']}
        all_totals=s['all'][side]
        all_totals['sloc']=totals['sloc']+s['third_party'][side]['sloc']
        all_totals['statuses']={status:totals['statuses'][status]+s['third_party'][side]['statuses'][status] for status in totals['statuses']}
    return s


def metrics_expected(old,units,current,mode,range_):
    m={k:deepcopy(old[k]) for k in METRICS if k in old}
    for k in ['standalone_proof_sloc','inline_proof_sloc','proof_sloc','fstar_annotation_sloc',*NEW,'proven_runtime_sloc','proven_extracted_runtime_sloc','proven_bounded_runtime_sloc','runtime_sloc']: m[k]=0
    for u in units:
        if u['category']=='proof-code':
            m['standalone_proof_sloc']+=u['sloc']
            if u.get('fstar_cfg_hax'): m['fstar_cfg_hax_sloc']+=u['sloc']
        elif u['category']=='runtime':
            full=u['sloc']+(u['inline_proof_sloc'] if current=='separate' else 0)
            inline=u['ghost_sloc'] if mode=='separate' else 0
            runtime=full-inline
            m['inline_proof_sloc']+=inline;m['runtime_sloc']+=runtime
            if mode=='separate':
                m['fstar_annotation_sloc']+=u['fstar_annotation_sloc']
                m['fstar_cfg_hax_inline_sloc']+=u.get('fstar_cfg_hax_inline_sloc',0)
            for status,field in [('proven','proven_runtime_sloc'),('proven-extracted','proven_extracted_runtime_sloc'),('proven-bounded','proven_bounded_runtime_sloc')]:
                if u['status']==status:m[field]+=runtime
    m['proof_sloc']=m['standalone_proof_sloc']+m['inline_proof_sloc']+m['fstar_patch_sloc']+m['fstar_file_sloc']
    denominator=m['proven_runtime_sloc']+m['proven_extracted_runtime_sloc']
    m['ratio']=m['proof_sloc']/denominator if denominator else None
    m['status']='n/a' if m['ratio'] is None else 'below' if m['ratio'] < range_['min'] else 'above' if m['ratio'] > range_['max'] else 'within'
    return m


def bertie(before,after,mode):
    independent=audit(OUT/'bertie',before,after,inline=True)
    (OUT/f'independent-{mode}.json').write_text(json.dumps(independent,indent=2)+'\n')
    ensure(len(before['units'])==len(after['units']),'Bertie unit set')
    changes=[]
    for a,b in zip(before['units'],after['units']):
        expected=deepcopy(a)
        if a['category'] == 'test':
            ensure(a == b, 'test category/status/size changed '+a['id'])
        if b.get('fstar_cfg_hax'):
            expected.update(category='proof-code',status='verified' if a['status']=='proven-extracted' else a['status'],sloc=a['sloc']+a['inline_proof_sloc'],runtime_sloc=0,inline_proof_sloc=0,ghost_sloc=0,fstar_annotation_sloc=0,fstar_cfg_hax=True)
        else:
            for k in ['sloc','runtime_sloc','inline_proof_sloc','ghost_sloc','fstar_annotation_sloc','fstar_cfg_hax_inline_sloc']:
                if k in b:expected[k]=b[k]
        ensure(expected==b,'unexpected unit change '+a['id'])
        if a['sloc']!=b['sloc']: changes.append((a,b))
    for key in before.keys() | after.keys():
        if key not in ['units','summary','summary_default_rules','proof_ratio','security_relevant','everything_else']:
            ensure(before.get(key)==after.get(key),'Bertie root '+key)
    for key,defaults in [('summary',False),('summary_default_rules',True)]:
        ensure(summary_expected(before[key],after['units'],defaults)==after[key],key+' unexpected change')
    for key,security in [('security_relevant',True),('everything_else',False)]:
        ensure(after[key]==sum(u['sloc'] for u in after['units'] if u['category']=='runtime' and u['security']==security)+sum(d['sloc'] for d in after['dependencies'] if d['security']==security),key+' size')
    p,q=before['proof_ratio'],after['proof_ratio']
    for key in ['range','gate_enabled','unknown_hax_paths']:ensure(p[key]==q[key],'ratio '+key)
    ensure(q['range']['min']==5 and q['range']['max']==15,'range changed')
    def check(old,new,units,m):
        actual={k:new[k] for k in METRICS}
        ensure(metrics_expected(old,units,mode,m,q['range'])==actual,'ratio metrics')
    units=after['units'];security=[u for u in units if u['security']]
    check(p,q,security,mode);check(p['all_first_party'],q['all_first_party'],units,mode)
    for kind in ['crates','modules']:
        ensure(len(p[kind])==len(q[kind]),kind+' row count')
        for a,b in zip(p[kind],q[kind]):
            ensure(a['crate_name']==b['crate_name'] and a['module']==b['module'],kind+' identity')
            subset=[u for u in units if u['crate_name']==b['crate_name'] and (kind=='crates' or u['module']==('' if b['module']=='<unattributed>' else b['module']))]
            check(a,b,subset,mode)
    for m in ['separate','runtime']:
        for scope,selected in [('security_relevant',security),('all_first_party',units)]:
            check(p['runtime_proven_modes'][m][scope],q['runtime_proven_modes'][m][scope],selected,m)
    ensure(all(r['accepted'] for r in after['evidence_runs']),'receipt not accepted')
    return independent,changes


def main():
    rows=[];moved=[];changes=[];invariants=[]
    for mode in ['separate','runtime']:
        before,after,no_patch=[read(f'bertie-{mode}-{label}.json') for label in ['before','after','no-patches']]
        scan,delta=bertie(before,after,mode)
        ensure(len(scan['moved']) == 18 and scan['standalone_sloc'] == 197, 'expected 18 proof units / 197 standalone SLoC')
        tests = [u for u in after['units'] if u['module']=='der::tests']
        ensure(len(tests)==4 and sum(u['sloc'] for u in tests)==57 and all(u['category']=='test' and u['status']=='unverified' for u in tests), 'four DER helpers must remain test/unverified / 57 SLoC')
        moved=scan['moved'];changes += [(mode,a,b) for a,b in delta]
        # Removing zero-eligible patches changes only patch-input/exclusion metadata.
        a,b=deepcopy(after),deepcopy(no_patch)
        for value in [a,b]:
            value['fstar_proof']['inputs']=[i for i in value['fstar_proof']['inputs'] if i['kind']!='patch']
            value['fstar_proof']['excluded_patch_hunks']=[]
            def clear(obj):
                if isinstance(obj,dict):
                    for key,val in obj.items():
                        if key=='excluded_patch_hunks':obj[key]=[]
                        else:clear(val)
                elif isinstance(obj,list):
                    for val in obj:clear(val)
            clear(value['proof_ratio'])
        ensure(a==b,'patches=[] changed figures beyond exclusion metadata')
        invariants.append(f'- Bertie {mode}: accepted receipt and credit membership identical; only rule-1 categories/statuses and rule-2 sizes change. patches=[]: all unit/evidence/numeric figures identical; zero-eligible patch inputs and 105 excluded-hunk records disappear.')
        rows += [(f'Bertie / {mode}','v1.8.0',before),(f'Bertie / {mode}','v1.8.1',after),(f'Bertie / {mode}','v1.8.1 patches=[]',no_patch)]
    for project in ['scratch-spec-lemmas','vest']:
        for mode in ['separate','runtime']:
            before,after=[read(f'{project}-{mode}-{label}/report.json') for label in ['before','after']]
            identity(before,after)
            for label in ['before','after']:
                path=OUT/f'{project}-{mode}-{label}/report.md'
                if label=='before':old_md=path.read_text()
                else:ensure(old_md==markdown_without_columns(path.read_text()),project+' Markdown figures')
            invariants.append(f'- {project} {mode}: PASS every preexisting JSON field identical, schema 4 unchanged; the two new ratio fields are zero. Markdown identical after removing the two added zero-component columns.')
            rows += [(f'{project} / {mode}','v1.8.0',before),(f'{project} / {mode}','v1.8.1',after)]
    fixture_rows=[]
    for name in ['classification','doc_comments','evidence','ghost','hax','inventory']:
        for mode in ['separate','runtime']:
            before,after=[read(f'fixture-{name}-{mode}-{label}/report.json') for label in ['before','after']]
            identity(before,after)
            old=(OUT/f'fixture-{name}-{mode}-before/report.md').read_text()
            new=(OUT/f'fixture-{name}-{mode}-after/report.md').read_text()
            ensure(old==markdown_without_columns(new),'fixture Markdown '+name+' '+mode)
            fixture_rows.append(f'| {name} | {mode} | {len(after["units"])} | all existing fields identical; added fields zero |')
    fields=['standalone_proof_sloc','inline_proof_sloc','proof_sloc','fstar_annotation_sloc','fstar_cfg_hax_sloc','fstar_cfg_hax_inline_sloc','fstar_patch_sloc','fstar_file_sloc','fstar_model_sloc','proven_runtime_sloc','proven_extracted_runtime_sloc','proven_bounded_runtime_sloc','runtime_sloc']
    lines=['# formal-coverage v1.8.1 paired replay','',f'Generated {datetime.now().astimezone().isoformat()}.','',
        'Baseline: v1.8.0 at `e74b116`; candidate: v1.8.1. Bertie: `f646099cee002c1d998b0e8c160886ef838f4f69`, copied read-only from bertie-deps to `target/v181/bertie`, existing F* receipt retained byte-for-byte. Both tools use the same temporary v1.8 proof config and compiled-files measurement, 5–15 unchanged. A second candidate config has `patches = []`. No extraction or verifier executed.','',
        'scratch-server `2fa5eb78a6f96a8433229dc64ec4b82c3e5f9bc2` and Vest `ce6705fad1af476e828feae6621d0bcf4d0c1665` use the same archived exports, retained receipts, whole-crate sizes and targets as v1.8 M4 / scripts/v15_replay.py (scratch aarch64-apple-darwin, Vest aarch64-unknown-linux-gnu). External checkouts remain untouched.','',
        '## Ratios and every component','',
        '| Project / mode | Version | Scope | Standalone | Inline | Total proof | Annotations | cfg standalone | cfg inline | Patches | Files | Models¹ | Proven | Extracted | Bounded | Runtime | Ratio | Extracted share² |',
        '| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |']
    for name,version,r in rows:
        for scope in ['security','all']:
            p=r['proof_ratio'] if scope=='security' else r['proof_ratio']['all_first_party']
            values=[str(p.get(f,0)) for f in fields]
            ratio='n/a' if p['ratio'] is None else f"{p['ratio']:.9f}"
            share=100*p['proven_extracted_runtime_sloc']/p['runtime_sloc'] if p['runtime_sloc'] else 0
            lines.append(f'| {name} | {version} | {scope} | '+' | '.join(values)+f' | {ratio} | {share:.6f}% |')
    lines += ['', '¹ Models are trusted assumptions, excluded from proof. Annotations and cfg inline may overlap; inline total is their union. cfg standalone is part of standalone proof. ² Extracted share is extracted runtime / runtime; security rows give the requested security proven-extracted percentage. New component fields display zero for the baseline, which did not expose them.','',
        '## Invariants and independent evidence','',*invariants,'',
        'Independent scanner `dev/v181/cfghax.py` matches 18 units / 197 standalone SLoC and 57 potential cfg inline lines in both source scans. Separate: 268 annotations + 57 cfg inline − 13 shared lines = 312 inline; 197 + 312 = 509 proof; 509/549 = 0.927140255. Runtime mode retains rule-2 lines in runtime, annotation/inline cost zero; standalone remains 197. Four helpers in cfg(all(test,hax)) remain test / unverified with their v1.8.0 sizes (57 SLoC total): test classification takes precedence over both proof rules. No cfg(hax) hazard; actual gate audit confirms no new failure reason. Bertie was and remains below 5–15; no gate threshold changed.','',
        '## Every moved unit','', '| Unit | Source | Old category / status / SLoC (separate) | New category / status / SLoC | Runtime SLoC old → new |','| --- | --- | --- | --- | --- |']
    for row in moved:
        # The last scan used runtime mode; take the separate pair for this table.
        a=next(u for u in read('bertie-separate-before.json')['units'] if u['id']==row['unit'])
        b=next(u for u in read('bertie-separate-after.json')['units'] if u['id']==row['unit'])
        runtime_old=next(u for u in read('bertie-runtime-before.json')['units'] if u['id']==row['unit'])
        runtime_new=next(u for u in read('bertie-runtime-after.json')['units'] if u['id']==row['unit'])
        lines.append(f"| `{a['id']}` | `{a['file']}:{a['line']}` | {a['category']} / {a['status']} / {a['sloc']} | {b['category']} / {b['status']} / {b['sloc']} | {runtime_old['sloc']} → {runtime_new['sloc']} |")
    lines += ['', '## Preserved hax-gated tests', '', '| Unit | Source | Category | Status | SLoC before → after |', '| --- | --- | --- | --- | ---: |']
    for a,b in zip(read('bertie-separate-before.json')['units'],read('bertie-separate-after.json')['units']):
        if a['category']=='test' and a['module']=='der::tests':
            lines.append(f"| `{a['id']}` | `{a['file']}:{a['line']}` | {b['category']} | {b['status']} | {a['sloc']} → {b['sloc']} |")
    lines += ['', '## Proof-code table', '', '| Ghost mode | Version | Scope | SLoC | Units | Verified | Unverified | Hazard units |', '| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |']
    for mode in ['separate','runtime']:
        for label,version in [('before','v1.8.0'),('after','v1.8.1')]:
            for scope in ['total','security_relevant','everything_else']:
                totals=read(f'bertie-{mode}-{label}.json')['summary']['proof_code'][scope]
                lines.append(f'| {mode} | {version} | {scope} | '+' | '.join(str(totals[k]) for k in ['sloc','units','verified','unverified','hazard_units'])+' |')
    lines += ['', 'Existing Bertie gate failures remain: `first-party security proof ratio … is below the acceptance range` and `unattributed file-scope hazard can invalidate runtime proofs`. `bash dev/v181/gates.sh` verifies identical failure reasons in both modes, including patches=[] and baseline/no-new-escape checks. The new proof-code hazard gate stays clear.', '']
    lines += ['', '## Every changed unit size','', '| Mode | Unit | Source | Before | After | Reason |','| --- | --- | --- | ---: | ---: | --- |']
    for mode,a,b in changes:lines.append(f"| {mode} | `{a['id']}` | `{a['file']}:{a['line']}` | {a['sloc']} | {b['sloc']} | "+('annotations included in standalone proof' if b.get('fstar_cfg_hax') else 'cfg inline lines leave runtime (overlap counted once)')+' |')
    scan=read('independent-separate.json')
    lines += ['', '## Rule-2 source lines','', '| Unit | File | cfg inline source lines |','| --- | --- | --- |']
    for row in scan['inline']:lines.append(f"| `{row['unit']}` | `{row['file']}` | "+', '.join(map(str,row['lines']))+' |')
    lines += ['', '## Fixture identity checks','', '| Fixture | Ghost mode | Units | Result |','| --- | --- | ---: | --- |',*fixture_rows,'',
        '## Reproduction and limits','',
        'Source copy/config preparation and exact baseline archive/build commands are in dev/NOTES.md. Baseline build: `cargo build --release --frozen --manifest-path target/v181/Cargo.toml --target-dir target/v181/baseline-build`; copy release binary to target/v181/baseline. Candidate: `cargo build --release --frozen`, copy release binary to target/v181/candidate. Redirect both builds under logs/v1.8.1.','', '```sh',
        'python3 dev/v181/replay.py bertie > logs/v1.8.1/m3-bertie.log 2>&1',
        'python3 dev/v181/replay.py verus > logs/v1.8.1/m3-verus.log 2>&1',
        'python3 dev/v181/replay.py fixtures > logs/v1.8.1/m3-fixtures.log 2>&1',
        'bash dev/v181/gates.sh > logs/v1.8.1/m3-final-gates.log 2>&1',
        'python3 dev/v18/patches.py target/v181/bertie target/v181/bertie-separate-after.json target/v181/patches.json > logs/v1.8.1/m3-patches.log 2>&1',
        'python3 dev/v181/compare.py > logs/v1.8.1/m3-compare.log 2>&1',
        'cargo test > logs/v1.8.1/m3-test.log 2>&1',
        'cargo clippy --all-targets -- -D warnings > logs/v1.8.1/m3-clippy.log 2>&1',
        'cargo fmt --check > logs/v1.8.1/m3-fmt.log 2>&1','```','',
        'Report schema stays 4; configuration stays 2/3. Two additive ratio fields are requested by the plan; all existing control fields and figures are identical, and added fields are zero. Unit cfg metadata is omitted when zero/false. Markdown adds only the two required component columns on controls. No fresh F*/Verus execution or independent acceptance; one existing ignored performance test remains unrun. No push, tag, bundle, host install or external messages.','', '## Artifact SHA-256','']
    for path in [OUT/'baseline',OUT/'candidate',OUT/'bertie-proof.toml',OUT/'bertie-no-patches.toml',OUT/'bertie/target/ci-logs/fstar/fstar.log.evidence.json',OUT/'independent-separate.json',OUT/'independent-runtime.json',OUT/'bertie-separate-before.json',OUT/'bertie-separate-after.json',OUT/'bertie-runtime-before.json',OUT/'bertie-runtime-after.json']:
        lines.append(f'- `{path.relative_to(REPO)}`: `{hashlib.sha256(path.read_bytes()).hexdigest()}`.')
    (REPO/'dev/v181/compare.md').write_text('\n'.join(lines)+'\n')
    print('PASS: Bertie allowed changes only; exact independent counts; patches=[] figures identical; scratch/Vest/fixtures identical in both modes; no schema change')

if __name__=='__main__':main()
