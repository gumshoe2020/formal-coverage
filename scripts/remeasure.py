#!/usr/bin/env python3
"""Replay retained source-bound scratch receipts through the release tool, without verifier scripts."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time

REPO = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('exports', type=Path)
    args = parser.parse_args()
    assert args.exports.resolve().is_relative_to(Path('/tmp').resolve())
    records = json.loads((REPO/'docs/scratch-server/runs.json').read_text())
    results = []
    env = {**os.environ, 'CARGO_HOME': str(REPO/'.cargo-home'), 'RUSTUP_TOOLCHAIN': 'stable'}
    for row in records:
        branch = row['branch']
        target = args.exports/branch
        assert (target/'export-commit.txt').read_text().strip() == row['commit']
        output = REPO/'logs'/f'v1.1-c-scratch-{branch}.json'
        command = [str(REPO/'target/release/formal-coverage'), 'report', str(target), '--json', str(output)]
        for name in row['receipts']:
            command.extend(['--verus-log', str(REPO/'docs/scratch-server'/name)])
        start = time.monotonic()
        with (REPO/'logs'/f'v1.1-c-scratch-{branch}.log').open('w') as log:
            subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
        current = json.loads(output.read_text())
        previous = json.loads((REPO/'docs/scratch-server'/f'{branch}.json').read_text())
        assert all(run['accepted'] for run in current['evidence_runs'])
        assert not [e for e in current['escape_hatches'] if e['first_party']]
        assert not current['trusted_items']
        # Compare individual first-party units, not just rounded percentages.
        identity = lambda r: [(u['id'], u['file'], u['line'], u['sloc'], u['category'], u['security'], u['status']) for u in r['units']]
        assert identity(current) == identity(previous), f'{branch}: changed unit attribution; investigate'
        for origin in ['all', 'first_party', 'third_party']:
            for side in ['security_relevant', 'everything_else']:
                a, b = current['summary'][origin][side], previous['summary'][origin][side]
                assert a['sloc'] == b['sloc']
                assert all(a['statuses'].get(k, 0) == b['statuses'].get(k, 0) for k in a['statuses'].keys() | b['statuses'].keys())
        assert current['summary']['proof_code'] == previous['summary']['proof_code']
        unsafe_set = lambda r: {(e['file'],e['line'],e['kind']) for e in r['escape_hatches'] if not e['first_party'] and e['kind'] == 'unsafe'}
        assert unsafe_set(current) == unsafe_set(previous), f'{branch}: dependency unsafe sites changed'
        old_deps = {(d['name'],d['version']): d for d in previous['dependencies']}
        changes = []
        for d in current['dependencies']:
            old = old_deps[(d['name'],d['version'])]
            count = len({(e['file'],e['line']) for e in current['escape_hatches'] if not e['first_party'] and e['kind'] == 'unsafe' and e['unit'] == f"{d['name']}@{d['version']}"})
            assert d['unsafe_sites'] == count
            if old['unsafe_sites'] != d['unsafe_sites']:
                changes.append(dict(name=d['name'],version=d['version'],before=old['unsafe_sites'],distinct=d['unsafe_sites']))
        dependency_unsafe = {label: dict(before=sum(d['unsafe_sites'] for d in previous['dependencies'] if d['security'] == security),distinct=sum(d['unsafe_sites'] for d in current['dependencies'] if d['security'] == security)) for label,security in [('security_relevant',True),('everything_else',False)]}
        results.append({'branch': branch, 'commit': row['commit'], 'seconds': time.monotonic()-start, 'receipts': row['receipts'], 'summary': current['summary'], 'first_party_hazards': 0, 'dependency_unsafe': dependency_unsafe, 'dependency_site_count_changes': changes})
        print(f'{branch}: runtime/proof unchanged, {len(changes)} dependency counts deduplicated; all {len(row["receipts"])} retained receipts accepted', flush=True)
    stages = {r['branch']: r for r in results}
    for branch in ['header-policy', 'spec-lemmas']:
        security = stages[branch]['summary']['first_party']['security_relevant']
        assert (security['statuses']['proven'], security['sloc']) == (644, 1363)
    (REPO/'logs/v1.1-c-scratch-results.json').write_text(json.dumps(results, indent=2)+'\n')


if __name__ == '__main__':
    main()
