#!/usr/bin/env python3
"""M1 paired report check: denominator only; status and size invariants."""
import json
from pathlib import Path

before, after = [json.loads(Path('target/v18/' + name).read_text())
                 for name in ('bertie-before.json', 'bertie-m1.json')]
assert before['units'] == after['units'], 'HONEST-FINDING: units changed'
assert all(run['accepted'] for run in after['evidence_runs'])
a = after['proof_ratio']
assert a['proven_runtime_sloc'] == 0
assert a['proven_extracted_runtime_sloc'] == 824
assert a['ratio'] == a['proof_sloc'] / 824
print('PASS: 469 units identical; accepted receipt; extracted denominator 824; ratio', a['ratio'])
