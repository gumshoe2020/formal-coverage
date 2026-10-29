#!/usr/bin/env python3
"""Add the release rerun to retained evidence without replacing historical M1/M2b artifacts."""
from collections import Counter
import json
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_runs as v


def main():
    out = v.RUNS / 'firecracker'
    old = json.loads((out / 'm2b/report.json').read_text())
    evidence = [(r['tool'], Path(r['log'])) for r in old['evidence_runs']]
    log = out / 'release-iovec-rerun.log'
    evidence.append(('kani', log))
    report = v.host_report('firecracker', evidence, out / 'cargo-home', out / 'release-rerun')
    data = json.loads((out / 'credits.json').read_text())
    rerun = json.loads((out / 'release-rerun-manifest.json').read_text())['steps'][0]
    assert v.kani_result(log.read_text(), 'devices::virtio::iovec::verification::verify_write_to_iovec_0', rerun['exit_code'])['passed']
    run = report['evidence_runs'][-1]
    by_id = {u['id']: u for u in report['units']}
    for h in data['harnesses']:
        h['report_log'] = str(out / h['log']) if h['log'] else None
        if h['id'].endswith('::verify_write_to_iovec_0'):
            h.update(outcome='pass', log=log.name, report_log=str(log), credited=run['accepted'] and h['id'] in run['credited_units'],
                     status=by_id[h['id']]['status'], proof_scope=by_id[h['id']]['proof_scope'], reasons=run['reasons'],
                     log_sha256=v.sha(log.read_bytes()), receipt_sha256=v.sha(Path(str(log)+'.evidence.json').read_bytes()))
    data.update(passing=sum(h['outcome']=='pass' for h in data['harnesses']), credited=sum(h['credited'] for h in data['harnesses']),
                report_artifact='release-rerun/report.json', report_sha256=v.sha((out/'release-rerun/report.json').read_bytes()))
    v.write(out / 'release-credits.json', data)
    row = json.loads((out / 'm2b-summary.json').read_text())
    outcomes = row['verifier_outcome']['harnesses']
    for h in outcomes:
        if h['id'].endswith('::verify_write_to_iovec_0'):
            h.update(outcome='pass', reason=None, log=log.name, exit_code=rerun['exit_code'], seconds=rerun['seconds'], checked_harnesses=['devices::virtio::iovec::verification::verify_write_to_iovec_0'])
    row['verifier_outcome']['counts'] = dict(Counter(h['outcome'] for h in outcomes))
    row.update(evidence_runs=[v.compact_run(r) for r in report['evidence_runs']], summary=report['summary'],
               report_artifact=data['report_artifact'], report_sha256=data['report_sha256'],
               receipt_counts=dict(Counter('accepted' if r['accepted'] else 'rejected' for r in report['evidence_runs'])),
               bounded_credit=dict(passing=data['passing'], credited=data['credited'], assumptions=data['assumptions'], runtime_targets_credited=0))
    assert report['summary']['first_party'] == old['summary']['first_party']
    assert not any(u['category']=='runtime' and u['status']=='proven-bounded' for u in report['units'])
    v.write(out / 'release-summary.json', row)
    path = v.REPO / 'docs/survey/results.json'
    document = json.loads(path.read_text())
    document['projects'] = [row if r['project']=='firecracker' else r for r in document['projects']]
    v.write(path, document)
    print('release rerun:', data['credited'], '/', data['passing'], row['receipt_counts'])


if __name__ == '__main__':
    main()
