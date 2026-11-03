#!/usr/bin/env python3
"""Audit v1.6 paired retained-evidence reports and write portable measured results.

Input directories are produced by v15_replay.py, with identical config/size mode
and --ghost-lines runtime/separate. No verifier or application is executed here.
"""
import argparse
import json
from pathlib import Path
from evidence_support import source_identity, write
from v16_regression import CASES


def reconcile(runtime, separate):
    assert runtime['ghost_lines'] == 'runtime'
    assert separate['ghost_lines'] == 'separate'
    assert runtime['evidence_runs'] == separate['evidence_runs'], 'receipt acceptance/scope changed'
    assert runtime['escape_hatches'] == separate['escape_hatches']
    assert runtime['trusted_items'] == separate['trusted_items']
    assert runtime['dependencies'] == separate['dependencies']
    assert runtime['source_files'] == separate['source_files']
    assert len(runtime['units']) == len(separate['units'])
    new = {'sloc', 'runtime_sloc', 'inline_proof_sloc'}
    for a, b in zip(runtime['units'], separate['units']):
        assert {k: v for k, v in a.items() if k not in new} == {k: v for k, v in b.items() if k not in new}, a['id']
        assert a['inline_proof_sloc'] == 0
        assert a['sloc'] - b['sloc'] == b['inline_proof_sloc'], a['id']
        if a['category'] == 'runtime':
            assert a['runtime_sloc'] - b['runtime_sloc'] == b['inline_proof_sloc']
        else:
            assert b['inline_proof_sloc'] == 0
    for key in ['proof_ratio']:
        a, b = runtime[key], separate[key]
        scopes = [(a, b), (a['all_first_party'], b['all_first_party'])]
        scopes += list(zip(a['crates'], b['crates'])) + list(zip(a['modules'], b['modules']))
        for x, y in scopes:
            assert x['standalone_proof_sloc'] == y['standalone_proof_sloc']
            assert x['runtime_sloc'] - y['runtime_sloc'] == y['inline_proof_sloc']
            assert y['proof_sloc'] == y['standalone_proof_sloc'] + y['inline_proof_sloc']
            for z in [x, y]:
                expected = z['proof_sloc'] / z['proven_runtime_sloc'] if z['proven_runtime_sloc'] else None
                assert z['ratio'] == expected
        assert a['runtime_proven_modes'] == b['runtime_proven_modes']
        assert a['runtime_proven_modes']['runtime']['security_relevant']['proven_runtime_sloc'] == a['proven_runtime_sloc']
        assert b['runtime_proven_modes']['separate']['security_relevant']['proven_runtime_sloc'] == b['proven_runtime_sloc']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('runtime', type=Path)
    parser.add_argument('separate', type=Path)
    parser.add_argument('--out', required=True, type=Path)
    args = parser.parse_args()
    result = {'tool_sources': source_identity(), 'method': 'offline retained-receipt replay; whole-crate; unchanged v1.4.2 configs; no Docker or verifier execution', 'cases': {}}
    index = json.loads(Path('logs/release-evidence.json').read_text())
    pins = {c['name']: {k: c[k] for k in ['archive', 'archive_sha256', 'target']} for c in index['scratch'] + index['survey']}
    for name in CASES:
        a = json.loads((args.runtime / name / 'report.json').read_text())
        b = json.loads((args.separate / name / 'report.json').read_text())
        reconcile(a, b)
        if name in ['scratch-spec-lemmas', 'spec-lemmas-host']:
            assert a['proof_ratio']['standalone_proof_sloc'] == 4732
            assert a['proof_ratio']['proven_runtime_sloc'] == 644
            assert a['proof_ratio']['runtime_sloc'] == 1363
            assert a['proof_ratio']['range'] == {'min': 1.0, 'max': 10.0, 'source': 'default', 'reason': None}
        proof = lambda r: r['proof_ratio'] if name in ['scratch-spec-lemmas', 'spec-lemmas-host'] else {k: v for k, v in r['proof_ratio'].items() if k != 'modules'}
        result['cases'][name] = {'pin': pins.get(name, {'commit': '2fa5eb78a6f96a8433229dc64ec4b82c3e5f9bc2', 'target': a['target']}), 'size_mode': a['size_mode'], 'target': a['target'], 'closure': a['closure'], 'config_schema': a['config_schema'], 'receipts': {'accepted': sum(r['accepted'] for r in a['evidence_runs']), 'rejected': sum(not r['accepted'] for r in a['evidence_runs'])}, 'runtime': proof(a), 'separate': proof(b)}
        print(f'{name}: sanity PASS; runtime {a["proof_ratio"]["ratio"]}; separate {b["proof_ratio"]["ratio"]}; {b["proof_ratio"]["status"]}')
    write(args.out, result)
    print('M4: all paired line, ratio and evidence invariants PASS')


if __name__ == '__main__':
    main()
