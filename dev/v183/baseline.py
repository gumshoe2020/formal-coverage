#!/usr/bin/env python3
"""Check original CI hashes, all unit statuses, and first-party status totals."""
from collections import Counter
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'target/v183'


def ensure(condition, message):
    if not condition:
        raise AssertionError('HONEST-FINDING: ' + message)


def read(path):
    return json.loads(path.read_text())


def check(replay):
    ci = read(OUT / 'ci-c66a986/report.json')
    receipt = read(OUT / 'ci-evidence/fstar.log.evidence.json')
    ensure((OUT / 'ci-c66a986/fstar/fstar.log.evidence.json').read_bytes()
           == (OUT / 'ci-evidence/fstar.log.evidence.json').read_bytes(), 'CI receipt altered')
    for field in ['fresh', 'checked', 'patches']:
        for name, expected in receipt[field].items():
            ensure(hashlib.sha256((OUT / 'ci-evidence' / name).read_bytes()).hexdigest()
                   == expected, f'CI {field} hash: {name}')
    ensure(replay['evidence_runs'] and all(r['accepted'] for r in replay['evidence_runs']),
           'baseline rejects CI evidence')
    projection = lambda report: Counter((u['id'], u['status']) for u in report['units'])
    ensure(projection(ci) == projection(replay), 'v1.8.2 baseline unit statuses differ from CI')
    ensure(ci['summary']['first_party'] == replay['summary']['first_party'],
           'v1.8.2 baseline first-party totals differ from CI')
    totals = replay['summary']['first_party']['security_relevant']
    ensure(totals['sloc'] == 5402 and totals['statuses']['proven-extracted'] == 3635,
           'baseline differs from pinned 3635/5402')
    return (f"CI baseline PASS: all {len(replay['units'])} unit entries "
            f"({len({u['id'] for u in replay['units']})} distinct IDs) match; "
            f"{totals['statuses']['proven-extracted']} proven-extracted, "
            f"{totals['statuses']['tested']} tested, {totals['statuses']['unverified']} unverified, "
            f"{totals['statuses']['trusted']} trusted / {totals['sloc']} security SLoC.")


if __name__ == '__main__':
    result = check(read(OUT / 'bertie-separate-before.json'))
    (OUT / 'baseline-comparison.txt').write_text(result + '\n')
    print(result)
