#!/usr/bin/env python3
"""Run the archived Firecracker harness inventory serially within six hours."""
from collections import Counter
import json
from pathlib import Path
import re
import shlex
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_runs as v


def inventory():
    report = v.host_report('firecracker', cargo_home=v.RUNS / 'firecracker/cargo-home')
    rows = []
    for unit in report['units']:
        if unit['category'] != 'proof-code':
            continue
        text = (v.EXPORTS / 'firecracker' / unit['file']).read_text().splitlines()
        head = '\n'.join(text[unit['line'] - 1:unit['end_line']]).split('fn ', 1)[0]
        if not re.search(r'#\[kani::proof(?:_for_contract\([^)]*\))?\]', head):
            continue
        unwind = re.search(r'#\[kani::unwind\((\d+)\)\]', head)
        rows.append(dict(id=unit['id'], file=unit['file'], line=unit['line'],
                         unwind=int(unwind[1]) if unwind else None,
                         outcome='not-compiled-on-aarch64' if '/x86_64/' in unit['file'] else 'pending',
                         reason='x86_64-only source' if '/x86_64/' in unit['file'] else None,
                         log=None))
    return rows


def save(rows):
    v.write(v.RUNS / 'firecracker/harnesses.json', dict(
        scope='aarch64; VMM package after workspace build-script EROFS',
        harnesses=rows, counts=dict(Counter(r['outcome'] for r in rows))))


def main():
    path = v.RUNS / 'firecracker/harnesses.json'
    rows = json.loads(path.read_text())['harnesses'] if path.exists() else inventory()
    manifest = json.loads((v.RUNS / 'firecracker/manifest.json').read_text())
    spent = sum(s['seconds'] for s in manifest['steps'] if s['label'].startswith('kani'))
    save(rows)
    for index, row in enumerate(rows):
        if row['outcome'] != 'pending':
            continue
        if spent >= 21600:
            row.update(outcome='skipped', reason='six-hour verifier budget exhausted')
            save(rows)
            continue
        label = f'kani-harness-{index:02}'
        command = [x for x in v.KANI_COMMAND if x not in ['-j', '--workspace']]
        command += ['--package', 'vmm', '--harness', row['id'].removeprefix('vmm::')]
        run = v.container('firecracker', 'fc-verify-kani:0.67.0', label, shlex.join(command),
                          timeout=min(2700, int(21600 - spent)))
        spent += run['seconds']
        log = v.RUNS / 'firecracker' / run['log']
        data = log.read_text()
        result = v.kani_result(data, row['id'].removeprefix('vmm::'), run['exit_code'])
        if result['passed']:
            outcome = 'pass'
            reason = None
        elif run['exit_code'] == 124 or re.search(r'timed out|TIMEOUT', data, re.I):
            outcome = 'timeout'
            reason = 'harness or container wall timeout'
        else:
            outcome = 'fail'
            reason = ('CBMC out of memory' if 'run out of memory' in data else 'nonzero verification/compilation exit; inspect retained log')
        row.update(outcome=outcome, reason=reason, checked_harnesses=result['checked_harnesses'], log=run['log'], exit_code=run['exit_code'], seconds=run['seconds'])
        # Full actual command, including all -Z flags, is intentionally retained.
        # Unsupported receipt policy is never bypassed by deleting flags.
        v.receipt('firecracker', label, 'kani', command, 'cargo-kani 0.67.0', run['exit_code'],
                  harnesses=[row['id']], covered_units={})
        save(rows)
    logs = [('kani', v.RUNS / 'firecracker' / r['log']) for r in rows if r['log']]
    logs.insert(0, ('kani', v.RUNS / 'firecracker/kani.log'))
    v.host_report('firecracker', logs, v.RUNS / 'firecracker/cargo-home')


if __name__ == '__main__':
    main()
