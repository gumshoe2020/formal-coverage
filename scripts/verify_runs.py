#!/usr/bin/env python3
"""Fenced v1.4 run primitives. Third-party execution is exclusively in Docker.

Exports are immutable git archives at a stable /tmp path. Each container gets
exactly that read-only source and its single writable runs/v1.4/project mount.
No shell commands from the target are executed on the host.
"""
import argparse
from datetime import datetime
import hashlib
import io
import json
import os
import re
from pathlib import Path
import subprocess
import tarfile
import time

REPO = Path(__file__).resolve().parents[1]
RUNS = REPO / 'runs/v1.4'
EXPORTS = Path(os.environ.get('FC_EXPORTS', '/tmp/fc-verify-v1.4'))
BASE = 'rust@sha256:620dbcd124499c59e2406d3741574b5c5838cf9eb9656f0c3a03948f79b02959'
VERUS_SHA = '781a460f35066e4bbc914b7fe3feb219ca9c850f'
KANI_COMMAND = ['cargo', 'kani', '-Z', 'unstable-options', '-Z', 'stubbing', '-Z',
                'function-contracts', '-Z', 'restrict-vtable', '-j', '--output-format',
                'terse', '--harness-timeout', '40m', '--workspace']


def sha(data):
    return hashlib.sha256(data).hexdigest()


def write(path, data):
    path.write_text(json.dumps(data, indent=2) + '\n')


def note(message):
    with (REPO / 'dev/NOTES.md').open('a') as out:
        out.write(f'\n{datetime.now().astimezone().isoformat()} (v1.4): {message}\n')


def initialize(name):
    out = RUNS / name
    out.mkdir(parents=True, exist_ok=True)
    source = REPO.parent / 'coverage-survey' / name
    commit = subprocess.check_output(['git', '-C', str(source), 'rev-parse', 'HEAD'], text=True).strip()
    data = subprocess.check_output(['git', '-C', str(source), 'archive', commit])
    target = EXPORTS / name
    target.mkdir(parents=True, exist_ok=True)
    with tarfile.open(fileobj=io.BytesIO(data)) as archive:
        members = [m for m in archive.getmembers() if m.isfile() or m.isdir()]
        if (target / 'Cargo.toml').exists():
            for member in members:
                if member.isfile():
                    assert (target / member.name).read_bytes() == archive.extractfile(member).read(), member.name
        else:
            archive.extractall(target, members=members, filter='data')
    path = out / 'manifest.json'
    if path.exists():
        record = json.loads(path.read_text())
        assert record['commit'] == commit and record['archive_sha256'] == sha(data)
        record['export'] = str(target)
        return record
    record = dict(project=name, commit=commit, archive_sha256=sha(data), export=str(target),
                  platform='linux/aarch64', base_image=BASE, image_id=None,
                  verus_commit=VERUS_SHA if name == 'anvil' else None,
                  kani_version='0.67.0' if name == 'firecracker' else None,
                  z3_version=None, rust_toolchains=[], steps=[])
    write(path, record)
    return record


def container(name, image, label, script, timeout=21600):
    """Run one recorded process; timeout stops the container, not just Docker CLI."""
    manifest = initialize(name)
    out = RUNS / name
    target = Path(manifest['export'])
    identity = subprocess.check_output(['docker', 'image', 'inspect', image, '--format', '{{.Id}}'], text=True).strip()
    cname = f'fc-verify-{name}-{label}'
    command = ['docker', 'run', '--rm', '--name', cname, '--platform', 'linux/arm64',
               '--mount', f'type=bind,source={target},target={target},readonly',
               '--mount', f'type=bind,source={out},target=/out',
               '--workdir', str(target), '--env', 'CARGO_HOME=/out/cargo-home',
               '--env', 'CARGO_TARGET_DIR=/out/target', '--env', 'TMPDIR=/out/tmp',
               '--env', 'CARGO_BUILD_JOBS=2', identity, 'bash', '-c',
               'mkdir -p /out/tmp; ' + script]
    start = time.monotonic()
    with (out / f'{label}.log').open('w') as log:
        try:
            result = subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, timeout=timeout)
            code = result.returncode
        except subprocess.TimeoutExpired:
            subprocess.run(['docker', 'stop', '--time', '10', cname], stdout=log, stderr=subprocess.STDOUT, check=False)
            code = 124
    record = dict(label=label, command=command, script=script, image_id=identity,
                  exit_code=code, seconds=time.monotonic() - start,
                  timeout_seconds=timeout, log=f'{label}.log',
                  log_sha256=sha((out / f'{label}.log').read_bytes()))
    manifest['steps'].append(record)
    manifest['image_id'] = identity
    write(out / 'manifest.json', manifest)
    note(f'{name}/{label}: exit {code}, {record["seconds"]:.2f} s; image {identity}; log {out.relative_to(REPO)}/{label}.log.')
    print(f'{name}/{label}: exit {code}, {record["seconds"]:.2f} s', flush=True)
    return record


def receipt(name, label, tool, command, version, code, **extra):
    out = RUNS / name
    target = EXPORTS / name
    log = out / f'{label}.log'
    # Rejected runs get truthful receipts too, so validators retain their reasons.
    data = dict(tool=tool, command=command, tool_version=version, exit_code=code,
                cfg=[], files={str(p.relative_to(target)): sha(p.read_bytes()) for p in target.rglob('*.rs')},
                log_sha256=sha(log.read_bytes()), harnesses=[], covered_units={})
    data.update(extra)
    write(Path(str(log) + '.evidence.json'), data)


def host_report(name, evidence=(), cargo_home=None, artifact_dir=None):
    initialize(name)
    out = Path(artifact_dir) if artifact_dir else RUNS / name
    assert out.resolve().is_relative_to((RUNS / name).resolve())
    out.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, CARGO_HOME=str(cargo_home or REPO / '.cargo-home'),
               TMPDIR=str(REPO / '.tmp'), CARGO_NET_OFFLINE='true', RUSTUP_TOOLCHAIN='stable')
    command = [str(REPO / 'target/release/formal-coverage'), 'report', str(EXPORTS / name),
               '--target', 'aarch64-unknown-linux-gnu', '--json', str(out / 'report.json'),
               '--markdown', str(out / 'report.md')]
    if name == 'vest':
        command += ['--config', str(REPO / 'docs/survey/vest.toml')]
    for tool, log in evidence:
        command += [f'--{tool}-log', str(log)]
    with (out / 'report.log').open('w') as log:
        subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    return json.loads((out / 'report.json').read_text())





def kani_result(text, expected, exit_code):
    """Account for Kani's substring filters without duplicating harness counts."""
    checked = re.findall(r'^Checking harness (.+?)\.\.\.$', text, re.M)
    successes = len(re.findall(r'^VERIFICATION:- SUCCESSFUL$', text, re.M))
    summaries = re.findall(r'^Complete - (\d+) successfully verified harnesses, (\d+) failures, (\d+) total\.$', text, re.M)
    complete = (len(summaries) == 1 and tuple(map(int, summaries[0])) == (len(checked), 0, len(checked)))
    passed = (exit_code == 0 and expected in checked and len(set(checked)) == len(checked)
              and successes == len(checked) and complete and 'VERIFICATION:- FAILED' not in text)
    return dict(passed=passed, checked_harnesses=checked)


def stable_sha(value):
    return sha(json.dumps(value, sort_keys=True, separators=(',', ':')).encode())


def compact_run(run):
    """Keep outcomes/reasons; bind large attribution lists without duplicating them."""
    result = {k: run[k] for k in ['tool', 'log', 'accepted', 'reasons', 'cfg', 'trusted_sloc']}
    for key in ['credited_units', 'verified_files', 'trusted_sites', 'escape_sites']:
        result[key + '_count'] = len(run[key])
        result[key + '_sha256'] = stable_sha(run[key])
    return result



def dependency_row(row):
    before = row['dependency_before']
    prior = f'{before["count"]} / {before["unknown"]}'
    if not before['complete']:
        prior += ' (inventory incomplete)'
    return (f'| {row["project"].capitalize()} | {prior} | {row["dependency_count"]} / {row["dependency_sloc"]:,} | '
            f'{row["build_dependency_count"]} / {row["build_dependency_sloc"]:,} |')


def outcome_row(name, row):
    """Exact audit-table row shared by the survey writer and checker."""
    if name == 'vest':
        outcome = 'Retained vest_lib Verus: 2,935 verified, 0 errors; fresh execution skipped'
    elif name == 'anvil':
        counts = row['verifier_outcome']['counts']
        outcome = 'Modules: ' + ', '.join(f'{status}={counts.get(status, 0)}' for status in ['verified', 'errors', 'timeout', 'OOM', 'skipped'])
        outcome += '; named [scopes](../runs/v1.4/anvil/modules.json)'
    else:
        counts = row['verifier_outcome']['counts']
        outcome = 'Harnesses: ' + ', '.join(f'{status}={counts.get(status, 0)}' for status in ['pass', 'fail', 'timeout', 'not-compiled-on-aarch64', 'skipped'])
        artifact = 'release-credits.json' if row.get('report_artifact', '').startswith('release') else 'credits.json'
        outcome += f'; named [harnesses](../runs/v1.4/firecracker/{artifact})'
        if 'bounded_credit' in row:
            credit = row['bounded_credit']
            outcome += f'; bounded harness credit={credit["credited"]}/{credit["passing"]}, assuming source stubs/contracts and type-compatible vtable targets; runtime targets={credit["runtime_targets_credited"]}'
    receipts = row['receipt_counts']
    return (f'| {name.capitalize()} verifier outcome | {outcome}; receipts accepted={receipts.get("accepted", 0)}, '
            f'rejected={receipts.get("rejected", 0)} |')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['export', 'container'])
    parser.add_argument('project', choices=['vest', 'anvil', 'firecracker'])
    parser.add_argument('--image', default=BASE)
    parser.add_argument('--label', default='probe')
    parser.add_argument('--script')
    parser.add_argument('--timeout', type=int, default=21600)
    args = parser.parse_args()
    if args.action == 'export':
        print(json.dumps(initialize(args.project), indent=2))
    else:
        assert args.script
        container(args.project, args.image, args.label, args.script, args.timeout)


if __name__ == '__main__':
    main()
