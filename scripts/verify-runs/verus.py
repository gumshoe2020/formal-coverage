#!/usr/bin/env python3
"""Anvil module fallback after a full-run resource failure; keep rejected receipts."""
from collections import Counter
import json
from pathlib import Path
import re
import shlex
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_runs as v



def verus_result(data, module):
    project = data.rsplit('Checking verifiable-controllers ', 1)[-1] if 'Checking verifiable-controllers ' in data else ''
    totals = re.findall(r'verification results:: (\d+) verified, (\d+) errors', project)
    verified, errors = map(int, totals[-1]) if totals else (None, None)
    observed = re.findall(r'^note: verifying module ([\w:]+)', project, re.M)
    fresh = bool(project and observed and all(m == module or m.startswith(module + '::') for m in observed))
    return verified, errors, observed, fresh


def save(rows):
    v.write(v.RUNS / 'anvil/modules.json', dict(
        scope='every root library module, including framework, four controllers and composition',
        counts=dict(Counter(r['outcome'] for r in rows)), modules=rows, invocation_mode='fresh-partial',
        count_semantics='Counts are exact project-run summary counts where emitted; null means no project summary. Dependency summaries are not Anvil counts.'))


def main():
    active = subprocess.check_output(['docker', 'ps', '--filter', 'name=fc-verify-firecracker-kani', '--format', '{{.Names}}'], text=True)
    assert not active.strip(), 'Do not overlap heavy Anvil and Kani runs'
    path = v.RUNS / 'anvil/modules.json'
    modules = re.findall(r'^pub mod (\w+);', (v.EXPORTS / 'anvil/src/lib.rs').read_text(), re.M)
    rows = json.loads(path.read_text())['modules'] if path.exists() else [
        dict(module=m, outcome='pending', verified=None, errors=None, log=None) for m in modules]
    if path.exists() and json.loads(path.read_text()).get('invocation_mode') != 'fresh-partial':
        rows = [dict(module=r['module'], outcome='pending', verified=None, errors=None, log=None, previous_attempts=r.get('previous_attempts', []) + [{k: val for k, val in r.items() if k != 'previous_attempts'}]) for r in rows]
    manifest = json.loads((v.RUNS / 'anvil/manifest.json').read_text())
    spent = sum(s['seconds'] for s in manifest['steps'] if s['label'].startswith('verus'))
    available = json.loads((v.RUNS / 'anvil/available-modules.json').read_text())
    save(rows)
    for row in rows:
        if row['outcome'] != 'pending':
            continue
        row['selected_descendants'] = [m for m in available if m == row['module'] or m.startswith(row['module'] + '::')]
        if not row['selected_descendants']:
            row.update(outcome='skipped', reason='not in the pinned verifier selectable-module inventory (Rust-only/cfg-excluded source); earlier selector errors retained')
            save(rows)
            continue
        if spent >= 21600:
            row.update(outcome='skipped', reason='six-hour verifier budget exhausted')
            save(rows)
            continue
        label = 'verus-fresh-' + row['module']
        command = ['cargo', 'verus', 'focus', '--lib', '--', '--rlimit', '100', '--verify-module', row['module']]
        script = ('export RUSTUP_TOOLCHAIN=1.98.1 CARGO_NET_OFFLINE=true; '
                  'cargo clean -p verifiable-controllers --target-dir /out/target/verus-partial && ' + shlex.join(command))
        pending = sum(m['outcome'] == 'pending' for m in rows)
        timeout = max(1, min(1800, int((21600 - spent) / pending)))
        run = v.container('anvil', 'fc-verify-verus:781a460f', label, script, timeout)
        spent += run['seconds']
        data = (v.RUNS / 'anvil' / run['log']).read_text()
        # Dependency summaries can appear before the project is checked. Only
        # parse the suffix after Cargo names this actual package.
        verified, errors, observed, fresh = verus_result(data, row['module'])
        if run['exit_code'] == 124:
            outcome = 'timeout'
        elif 'SIGKILL' in data or run['exit_code'] == 137:
            outcome = 'OOM'
        elif run['exit_code'] == 0 and fresh and verified is not None and errors == 0:
            outcome = 'verified'
        else:
            outcome = 'errors'
        reason = {'timeout': 'container wall-time limit', 'OOM': 'SIGKILL; probable resource exhaustion', 'errors': 'nonzero verifier exit or missing successful project summary; see log', 'verified': None}[outcome]
        row.update(outcome=outcome, reason=reason, observed_modules=observed, fresh_compile=fresh, phase='verification' if fresh else 'setup', verified=verified, errors=errors,
                   log=run['log'], exit_code=run['exit_code'], seconds=run['seconds'])
        v.receipt('anvil', label, 'verus', command, 'Verus 0.2026.10.12.781a460', run['exit_code'])
        save(rows)
    logs = [('verus', Path(str(p).removesuffix('.evidence.json'))) for p in sorted((v.RUNS / 'anvil').glob('verus*.log.evidence.json'))]
    v.host_report('anvil', logs, v.RUNS / 'anvil/cargo-home')


if __name__ == '__main__':
    main()
