#!/usr/bin/env python3
"""Require v1.8.1 replay unit statuses to match the pinned CI artifact."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ci = json.loads((ROOT / 'target/v182/ci-9212e97/report.json').read_text())
replay = json.loads((ROOT / 'target/v182/baseline-ci-replay.json').read_text())
old = {u['id']: u['status'] for u in ci['units']}
new = {u['id']: u['status'] for u in replay['units']}
differences = [(key, old.get(key), new.get(key)) for key in sorted(old.keys() | new.keys())
               if old.get(key) != new.get(key)]
result = {'ci_units': len(old), 'replay_units': len(new),
          'receipt_accepted': [r['accepted'] for r in replay['evidence_runs']],
          'status_differences': differences}
(ROOT / 'target/v182/baseline-comparison.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
assert replay['evidence_runs'] and all(r['accepted'] for r in replay['evidence_runs'])
assert not differences, 'HONEST-FINDING: v1.8.1 baseline statuses differ from CI'
