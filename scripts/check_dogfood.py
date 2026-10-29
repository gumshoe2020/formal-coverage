#!/usr/bin/env python3
"""Recompute artifact totals and bind verifier receipts to exact git archives."""
import hashlib
import io
import json
from pathlib import Path
import re
import subprocess
import tarfile

from evidence_support import arguments, require_bundle, check_tool, validate_receipts
args = arguments(__doc__, scratch=True).parse_args()
ROOT = args.evidence_root.resolve()
index = require_bundle(ROOT)
check_tool(index)
STATUS = ['proven', 'proven-bounded', 'proven-upstream', 'tested', 'unverified', 'hazard', 'trusted']
runs = json.loads((ROOT/'docs/scratch-server/runs.json').read_text())
cases = {c['name'].removeprefix('scratch-'): c for c in index['scratch']}
assert {r['branch']: r['commit'] for r in runs} == {b: c['commit'] for b, c in cases.items()}
text = (ROOT/'docs/scratch-server.md').read_text()
assert all(s in text for s in ['SECURITY-RELEVANT', 'EVERYTHING ELSE', 'Proof-code SLoC', 'v1 security proven'])
reports = {}
for run in runs:
    branch = run['branch']
    report = json.loads((ROOT/'docs/scratch-server'/f'{branch}.json').read_text())
    reports[branch] = report
    assert report['schema'] == 2 and report['units'] and report['dependency_inventory_complete']
    assert all(d['size_known'] and d['unsafe_sites'] is not None for d in report['dependencies'])
    assert all(u['category'] in ['runtime', 'proof-code'] for u in report['units'])
    assert all(u['status'] in (['verified', 'unverified', 'hazard'] if u['category'] == 'proof-code' else STATUS) for u in report['units'])
    assert all(d['status'] == 'unverified' for d in report['dependencies'])
    assert all(c['confirmation'] == 'unconfirmed' for c in report['upstream_claims'])
    assert report['overrides'] and report['approximation'].startswith('Approximate')
    proof = [u for u in report['units'] if u['category'] == 'proof-code']
    for security, key in [(True, 'security_relevant'), (False, 'everything_else'), (None, 'total')]:
        units = [u for u in proof if security is None or u['security'] == security]
        totals = report['summary']['proof_code'][key]
        assert totals == dict(sloc=sum(u['sloc'] for u in units), units=len(units), verified=sum(u['status'] == 'verified' for u in units), unverified=sum(u['status'] != 'verified' for u in units), hazard_units=sum(u['status'] == 'hazard' for u in units))
    for security, key in [(True, 'security_relevant'), (False, 'everything_else')]:
        groups = {'first_party': [u for u in report['units'] if u['security'] == security and u['category'] == 'runtime'], 'third_party': [d for d in report['dependencies'] if d['security'] == security]}
        groups['all'] = groups['first_party'] + groups['third_party']
        for origin, units in groups.items():
            summary = report['summary'][origin][key]
            assert summary['sloc'] == sum(u['sloc'] for u in units), (branch, origin, key)
            assert set(summary['statuses']) == set(STATUS)
            assert all(summary['statuses'][s] == sum(u['sloc'] for u in units if u['status'] == s) for s in STATUS)
            assert summary['unknown_dependency_sizes'] == 0
        assert report[key] == report['summary']['all'][key]['sloc']
    # The current dependency table is the separately retained v1.2 measurement.
    # These bundled full reports preserve the earlier vendor/lockfile denominator.
    # The first-party comparison table below is shared by both measurements.
    old = json.loads((ROOT/'docs/scratch-server'/f'{branch}.v1.json').read_text())
    assert old['schema'] == 1
    assert old['tool_commit'] == '4685da4841e9f9bf2e74442d2e9d24e82317ead3'
    for side in ['security_relevant', 'everything_else']:
        a, b = old['summary']['third_party'][side], report['summary']['third_party'][side]
        assert a['sloc'] == b['sloc']
        assert all(a['statuses'].get(s, 0) == b['statuses'].get(s, 0) for s in STATUS)
    for security, key in [(True, 'security_relevant'), (False, 'everything_else')]:
        assert old['summary']['first_party'][key]['sloc'] == sum(u['sloc'] for u in report['units'] if u['security'] == security)
    a = report['summary']['first_party']['security_relevant']
    b = old['summary']['first_party']['security_relevant']
    pct = lambda g: 100*g['statuses']['proven']/g['sloc'] if g['sloc'] else 0
    proof_totals=report['summary']['proof_code']['total']
    verified,units=proof_totals['verified'],proof_totals['units']
    comparison = f'| {branch} | {a["statuses"]["proven"]}/{a["sloc"]} | {pct(a):.2f}% | {proof_totals["sloc"]} | {verified}/{units} ({100*verified/units if units else 0:.2f}%) | {len(run["roots"])} | {b["statuses"]["proven"]}/{b["sloc"]} | {pct(b):.2f}% |'
    assert comparison in text
    case = cases[branch]
    archive = (ROOT / case['archive']).read_bytes()
    assert hashlib.sha256(archive).hexdigest() == case['archive_sha256']
    validate_receipts(ROOT, case, report)
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        sources = {m.name: tar.extractfile(m).read() for m in tar.getmembers() if m.isfile() and m.name.endswith('.rs')}
        has_verifier = 'verify.sh' in tar.getnames()
        roots = re.findall(r'^"\$VERUS" --crate-type lib (\S+)$',tar.extractfile('verify.sh').read().decode(),re.M) if has_verifier else []
    assert run['roots'] == roots
    assert set(report['source_files']) == set(sources)
    if not has_verifier:
        assert run['verify_exit'] is None
        assert not any(u['status'] in ['proven', 'verified'] for u in report['units'])
    else:
        assert run['verify_exit'] == 0
        assert len(run['receipts']) == len(roots) == len(report['evidence_runs'])
        credited = set()
        counts = []
        for index, (name, source, evidence_run) in enumerate(zip(run['receipts'], roots, report['evidence_runs'])):
            logfile=ROOT/'docs/scratch-server'/name
            log=logfile.read_bytes()
            receipt=json.loads(Path(str(logfile)+'.evidence.json').read_text())
            assert receipt['exit_code']==0 and receipt['tool']=='verus' and receipt['tool_version'].strip()
            assert receipt['command'][1:] == ['--crate-type','lib',source]
            assert receipt['log_sha256'] == hashlib.sha256(log).hexdigest()
            results=re.findall(r'^verification results:: ([1-9][0-9]*) verified, 0 errors$',log.decode(),re.M)
            assert len(results)==1 and 'error:' not in log.decode()
            counts.extend(results)
            assert receipt['files']=={name:hashlib.sha256(data).hexdigest() for name,data in sources.items()}
            assert evidence_run['accepted'] and Path(evidence_run['log']).name == name
            scope={source}
            # Independently inspect the known scratch-server plain declarations, never #[path].
            if source=='crates/transport-core/src/lib.rs':
                assert b'pub mod aead;' in sources[source]
                scope.add('crates/transport-core/src/aead.rs')
                assert any('http' in reason and 'excluded path/cfg/verifier scope' in reason for reason in evidence_run['reasons'])
            if source=='crates/http-subset/src/lib.rs' and b'mod intent;' in sources[source]:
                scope.add('crates/http-subset/src/intent.rs')
            own=[u for u in report['units'] if u['id'] in evidence_run['credited_units']]
            assert len(own)==len(evidence_run['credited_units']) and own
            assert all(u['file'] in scope and u['status'] in ['proven','verified'] and u['proof_scope'] for u in own)
            credited.update(evidence_run['credited_units'])
        script_log=(ROOT/'docs/scratch-server'/f'{branch}.verify-script.log').read_text()
        assert 'error:' not in script_log
        assert re.findall(r'^verification results:: ([1-9][0-9]*) verified, 0 errors$',script_log,re.M)==counts
        assert credited=={u['id'] for u in report['units'] if u['status'] in ['proven','verified']}
        assert any('accepted caller-supplied run receipt' in w for w in report['warnings'])
    print(f'{branch}: runtime/proof totals, v1 comparison, evidence and archive identities passed')

# Compare actual classified units as well as aggregate headlines; fail without changing classification.
left, right = reports['header-policy'], reports['spec-lemmas']
for origin in ['first_party', 'all']:
    a, b = [r['summary'][origin]['security_relevant'] for r in [left, right]]
    assert (a['statuses']['proven'], a['sloc']) == (b['statuses']['proven'], b['sloc']), (origin, a, b)
    print(f'Stage 12/13 {origin}: {a["statuses"]["proven"]}/{a["sloc"]} runtime security SLoC invariant passed')
def runtime_units(report):
    return sorted((u['id'], u['file'], u['sloc'], u['security'], u['status']) for u in report['units'] if u['category'] == 'runtime' and u['security'])
assert runtime_units(left) == runtime_units(right)
assert right['summary']['proof_code']['total']['sloc'] > left['summary']['proof_code']['total']['sloc']
assert (right['summary']['first_party']['security_relevant']['statuses']['proven'], right['summary']['first_party']['security_relevant']['sloc']) == (644,1363)
intent=[u for u in right['units'] if u['file']=='crates/http-subset/src/intent.rs']
assert intent and sum(u['sloc'] for u in intent)==4410 and all(u['category']=='proof-code' and u['status']=='verified' for u in intent)
print(f'v1.1-b artifact acceptance passed ({len(runs)} branches).')
