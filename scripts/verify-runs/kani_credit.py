#!/usr/bin/env python3
"""Replay unchanged M1 logs through Andrew's M2b policy; preserve M1 artifacts."""
from collections import Counter
import json
from pathlib import Path
import shutil
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_runs as v


def main():
    out = v.RUNS / 'firecracker'
    raw = json.loads((out / 'harnesses.json').read_text())
    current = out / 'm2b'
    current.mkdir(exist_ok=True)
    records = []
    for h in raw['harnesses']:
        if h['outcome'] != 'pass':
            continue
        original = out / h['log']
        log = current / h['log']
        shutil.copyfile(original, log)
        r = json.loads(Path(str(original) + '.evidence.json').read_text())
        # Kani's logged command ID omits the crate. Correct caller metadata only;
        # no command, log, source bytes, unwind bounds or model are rewritten.
        r['harnesses'] = [r['command'][r['command'].index('--harness') + 1]]
        v.write(Path(str(log) + '.evidence.json'), r)
        records.append(dict(id=h['id'], outcome=h['outcome'], original_log=h['log'], log=str(log.relative_to(out))))
    logs = [('kani', Path(str(p).removesuffix('.evidence.json'))) for p in sorted(out.glob('kani*.log.evidence.json'))]
    logs += [('kani', current / Path(row['log']).name) for row in records]
    report = v.host_report('firecracker', logs, out / 'cargo-home', current)
    units = {u['id']: u for u in report['units']}
    runs = {r['log']: r for r in report['evidence_runs']}
    for row in records:
        run = runs[str(out / row['log'])]
        unit = units[row['id']]
        row.update(credited=run['accepted'] and row['id'] in run['credited_units'],
                   status=unit['status'], proof_scope=unit['proof_scope'], reasons=run['reasons'],
                   log_sha256=v.sha((out / row['log']).read_bytes()),
                   receipt_sha256=v.sha(Path(str(out / row['log']) + '.evidence.json').read_bytes()))
    passing = len(records)
    for h in raw['harnesses']:
        if h['outcome'] != 'pass':
            records.append(dict(id=h['id'], outcome=h['outcome'], original_log=h['log'], log=h['log'], credited=False, status=units[h['id']]['status'], proof_scope=None, reasons=[h['reason']]))
    data = dict(passing=passing, credited=sum(r['credited'] for r in records), harnesses=records,
                runtime_targets_credited=sum(u['category']=='runtime' and u['status']=='proven-bounded' for u in report['units']),
                assumptions=['source stubs replace named functions', 'verified stubs assume contracts',
                             'proof-for-contract scopes cover requires/ensures', 'type-compatible vtable restriction'],
                report_artifact='m2b/report.json', report_sha256=v.sha((current / 'report.json').read_bytes()))
    assert not any(u['status']=='proven' for u in report['units'])
    v.write(out / 'credits.json', data)
    # Compact current snapshot; original M1 manifest/summary/harness inventory are immutable.
    row = json.loads((out / 'summary.json').read_text())
    row.update(evidence_runs=[v.compact_run(r) for r in report['evidence_runs']], summary=report['summary'],
               report_artifact=data['report_artifact'], report_sha256=data['report_sha256'],
               receipt_counts=dict(Counter('accepted' if r['accepted'] else 'rejected' for r in report['evidence_runs'])),
               bounded_credit=dict(passing=data['passing'], credited=data['credited'], assumptions=data['assumptions'],
                                   runtime_targets_credited=data['runtime_targets_credited']))
    v.write(out / 'm2b-summary.json', row)
    print(f'credited {data["credited"]}/{data["passing"]}; runtime targets {data["runtime_targets_credited"]}', flush=True)


if __name__ == '__main__':
    main()
