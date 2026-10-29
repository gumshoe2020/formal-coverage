#!/usr/bin/env python3
"""Measure read-only survey clones via /tmp archives; execute only pinned Verus, never project scripts."""
import argparse
from datetime import datetime
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
import time

REPO = Path(__file__).resolve().parents[1]
SURVEY = REPO.parent/'coverage-survey'
TOOLS = REPO.parent/'scratch-server/.tools'
VERUS = TOOLS/'verus-arm64-macos/verus'
LOGS = REPO/'logs'


def digest(data):
    return hashlib.sha256(data).hexdigest()


def export(name, base):
    source = SURVEY/name
    commit = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    data = subprocess.check_output(['git', '-C', str(source), 'archive', commit])
    target = base/name
    target.mkdir(parents=True, exist_ok=True)
    with tarfile.open(fileobj=io.BytesIO(data)) as archive:
        members = [m for m in archive.getmembers() if m.isfile() or m.isdir()]
        if (target/'Cargo.toml').exists():
            # Reusing a verifier export must never silently overwrite changed source.
            for member in members:
                if member.isfile():
                    assert digest((target/member.name).read_bytes()) == digest(archive.extractfile(member).read()), member.name
        else:
            archive.extractall(target, members=members, filter='data')
    return target, commit


def verify(name, target, commit, reuse):
    root = 'vest_lib/src/lib.rs' if name == 'vest' else 'src/lib.rs'
    command = [str(VERUS), '--crate-type', 'lib', root]
    log = LOGS/f'v1.1-c-{name}-verus.log'
    record_path = LOGS/f'v1.1-c-{name}-run.json'
    if reuse:
        record = json.loads(record_path.read_text())
        assert record['command'] == command and record['commit'] == commit
        assert record['log_sha256'] == digest(log.read_bytes())
        return record
    env = {**os.environ, 'CARGO_HOME': str(TOOLS/'cargo'), 'RUSTUP_HOME': str(TOOLS/'rustup')}
    version = subprocess.run([str(VERUS), '--version'], env=env, capture_output=True, text=True, check=True)
    start = time.monotonic()
    with log.open('w') as output:
        try:
            result = subprocess.run(command, cwd=target, env=env, stdout=output, stderr=subprocess.STDOUT, timeout=600)
            code = result.returncode
        except subprocess.TimeoutExpired:
            code = 124
    record = dict(command=command, exit_code=code, seconds=time.monotonic()-start, commit=commit,
                  log_sha256=digest(log.read_bytes()), tool_version=version.stdout+version.stderr)
    record_path.write_text(json.dumps(record, indent=2)+'\n')
    sidecar = Path(str(log)+'.evidence.json')
    if code == 0:
        receipt = dict(tool='verus', tool_version=record['tool_version'], command=command, exit_code=code,
                       cfg=[], files={str(p.relative_to(target)): digest(p.read_bytes()) for p in target.rglob('*.rs')},
                       log_sha256=record['log_sha256'], harnesses=[], covered_units={})
        sidecar.write_text(json.dumps(receipt, indent=2)+'\n')
    elif sidecar.exists():
        sidecar.unlink()
    return record


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--directory', type=Path)
    parser.add_argument('--reuse-verifier-runs', action='store_true', help='Require intact logs from the recorded runs and identical archive sources')
    args = parser.parse_args()
    base = args.directory or Path(tempfile.mkdtemp(prefix='formal-coverage-survey-', dir='/tmp'))
    assert base.resolve().is_relative_to(Path('/tmp').resolve())
    LOGS.mkdir(exist_ok=True)
    env = {**os.environ, 'CARGO_HOME': str(REPO/'.cargo-home'), 'RUSTUP_TOOLCHAIN': 'stable'}
    results = []
    for name in ['vest', 'anvil', 'firecracker']:
        target, commit = export(name, base)
        verification = verify(name, target, commit, args.reuse_verifier_runs) if name != 'firecracker' else None
        output = LOGS/f'v1.1-c-{name}.json'
        command = [str(REPO/'target/release/formal-coverage'), 'report', str(target), '--json', str(output), '--markdown', str(LOGS/f'v1.1-c-{name}.md')]
        if name == 'vest':
            command.extend(['--config', str(SURVEY/'configs/vest.toml')])
        if verification and verification['exit_code'] == 0:
            command.extend(['--verus-log', str(LOGS/f'v1.1-c-{name}-verus.log')])
        start = time.monotonic()
        with (LOGS/f'v1.1-c-{name}-report.log').open('w') as log:
            subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
        seconds = time.monotonic()-start
        report = json.loads(output.read_text())
        sites = report['escape_hatches']
        assert len(sites) == len({(e['file'], e['line'], e['kind']) for e in sites})
        if name == 'firecracker':
            assert not any(e['kind'] in ['assume', 'kani::assume'] for e in sites)
            assert report['summary']['proof_code']['total']['units'] > 0
        if name == 'vest':
            assert verification['exit_code'] == 0
            assert len(report['evidence_runs']) == 1 and report['evidence_runs'][0]['accepted']
            assert any(u['crate_name'] == 'vest_lib' and u['status'] == 'proven' for u in report['units'])
        record = dict(project=name, commit=commit, verification=verification, report_seconds=seconds,
                      summary=report['summary'], evidence_runs=report['evidence_runs'],
                      # Full reports (including affected units) remain under logs/. Keep every distinct site here.
                      escape_sites=[{k:e[k] for k in ['file','line','kind','first_party','security']} for e in sites],
                      trusted_items=report['trusted_items'],
                      dependency_inventory_complete=report['dependency_inventory_complete'])
        if name == 'vest':
            units = [u for u in report['units'] if u['crate_name'] == 'vest_lib' and u['category'] == 'runtime']
            record['vest_lib'] = {label: dict(sloc=sum(u['sloc'] for u in units if not security or u['security']),
                                            proven=sum(u['sloc'] for u in units if u['status'] == 'proven' and (not security or u['security']))) for label,security in [('runtime',False),('security_runtime',True)]}
        results.append(record)
        print(f'{name}: exit={verification["exit_code"] if verification else "inventory only"}; {len(sites)} distinct escapes; report {seconds:.2f}s', flush=True)
    document = dict(measured_at=datetime.now().astimezone().isoformat(), exports=str(base), projects=results)
    (LOGS/'v1.1-c-survey-results.json').write_text(json.dumps(document, indent=2)+'\n')


if __name__ == '__main__':
    main()
