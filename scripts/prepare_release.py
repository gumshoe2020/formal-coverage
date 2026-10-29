#!/usr/bin/env python3
"""Regenerate v1.4.1 evidence from pinned archives; no verifier or target build runs."""
import argparse
import io
import json
from pathlib import Path
import shutil
import subprocess
import tarfile
import tomllib

from evidence_support import REPO, sha, write, source_identity, replay, validate_receipts, portable


def archive(source, commit, path):
    data = subprocess.check_output(['git', '-C', str(source), 'archive', commit])
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    return sha(data)


def stable_coverage(before, after):
    # The release fence is stronger than equality of rounded percentages.
    for origin in ['first_party', 'third_party', 'all']:
        for side in ['security_relevant', 'everything_else']:
            a, b = [r['summary'][origin][side] for r in [before, after]]
            assert a['sloc'] == b['sloc'], (origin, side, a, b)
            for status in set(a['statuses']) | set(b['statuses']):
                assert a['statuses'].get(status, 0) == b['statuses'].get(status, 0), (origin, side, status)
    assert before['summary']['proof_code'] == after['summary']['proof_code']


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--scratch-server', type=Path, default=REPO.parent / 'scratch-server')
    p.add_argument('--survey', type=Path, default=REPO.parent / 'coverage-survey')
    p.add_argument('--crate-cache', type=Path, required=True)
    p.add_argument('--pins', type=Path, required=True)
    p.add_argument('--scratch-only', action='store_true')
    p.add_argument('--survey-only', action='store_true')
    args = p.parse_args()
    pins = dict(line.split() for line in args.pins.read_text().splitlines() if line.strip())
    index = dict(version='1.4.1', tool_sources=source_identity(), survey=[], scratch=[], required=[])
    binary = REPO / 'target/release/formal-coverage'
    scratch_runs = json.loads((REPO / 'docs/scratch-server/runs.json').read_text())
    assert {r['branch']: r['commit'] for r in scratch_runs} == pins
    crates = REPO / 'logs/evidence/crates'
    crates.mkdir(parents=True, exist_ok=True)
    if args.survey_only:
        index['scratch'] = json.loads((REPO / 'logs/release-evidence.json').read_text())['scratch']
        for case in index['scratch']:
            case['report_sha256'] = sha((REPO / case['report']).read_bytes())
    for row in ([] if args.survey_only else scratch_runs):
        name = row['branch']; path = REPO / f'logs/evidence/archives/scratch-{name}.tar'
        digest = archive(args.scratch_server, pins[name], path)
        with tarfile.open(path) as tar:
            lock = tomllib.loads(tar.extractfile('Cargo.lock').read().decode())
        dependencies = []
        entries = []
        for package in lock['package']:
            if not package.get('source', '').startswith('registry+'):
                assert not package.get('source'), package
                continue
            filename = f'{package["name"]}-{package["version"]}.crate'
            destination = crates / filename
            if not destination.exists():
                shutil.copyfile(args.crate_cache / filename, destination)
            assert sha(destination.read_bytes()) == package['checksum']
            dependencies.append(str(destination.relative_to(REPO)))
            entries.append(f'"{package["name"]}@{package["version"]}" = "vendor/{package["name"]}-{package["version"]}"')
        config = REPO / f'logs/evidence/configs/{name}.toml'
        config.parent.mkdir(exist_ok=True)
        config.write_text((REPO / 'examples/scratch-server/formal-coverage.toml').read_text()+'\n[dependency_sources]\n'+'\n'.join(entries)+'\n')
        case = dict(name='scratch-'+name, commit=pins[name], archive=str(path.relative_to(REPO)), archive_sha256=digest,
                    source=f'target/release-scratch/{name}', config=str(config.relative_to(REPO)),
                    vendor_archives=dependencies, isolate_workspace=True, cargo_home='logs/evidence/empty-cargo',
                    target='aarch64-apple-darwin', evidence=[['verus', 'docs/scratch-server/'+n] for n in row['receipts']],
                    report=f'docs/scratch-server/{name}.json')
        previous = json.loads((REPO / case['report']).read_text())
        current = replay(REPO, binary, case, REPO / 'logs/release-generated' / case['name'])
        stable_coverage(previous, current)
        validate_receipts(REPO, case, current)
        write(REPO / case['report'], current)
        case['report_sha256'] = sha((REPO / case['report']).read_bytes())
        index['scratch'].append(case)
        print(case['name'], 'regenerated with identical coverage', flush=True)
    for name in ([] if args.scratch_only else ['vest', 'anvil', 'firecracker']):
        out = REPO / 'runs/v1.4' / name
        row = json.loads((out / ('release-summary.json' if name == 'firecracker' else 'summary.json')).read_text())
        old_path = out / row.get('report_artifact', 'report.json')
        previous = json.loads(old_path.read_text())
        path = REPO / f'logs/evidence/archives/{name}.tar'
        digest = archive(args.survey / name, row['commit'], path)
        if name == 'anvil':
            if not (out / 'Cargo.lock').exists():
                from verify_runs import EXPORTS
                shutil.copyfile(EXPORTS / name / 'Cargo.lock', out / 'Cargo.lock')
        case = dict(name=name, commit=row['commit'], archive=str(path.relative_to(REPO)), archive_sha256=digest,
                    source=f'target/release-survey/{name}', isolate_workspace=True, cargo_home=f'runs/v1.4/{name}/cargo-home', target=row['target'],
                    evidence=[[r['tool'], str(Path(r['log']).relative_to(REPO)) if Path(r['log']).is_absolute() else r['log']] for r in previous['evidence_runs']],
                    report=f'runs/v1.4/{name}/release/report.json')
        if name == 'anvil':
            case['lock'] = 'runs/v1.4/anvil/Cargo.lock'
        if name == 'vest':
            case['config'] = 'docs/survey/vest.toml'
        current = replay(REPO, binary, case, REPO / 'runs/v1.4' / name / 'release')
        stable_coverage(previous, current)
        validate_receipts(REPO, case, current)
        case['summary'] = current['summary']
        case['report_sha256'] = sha((REPO / case['report']).read_bytes())
        from collections import Counter
        case['receipt_counts'] = dict(Counter('accepted' if r['accepted'] else 'rejected' for r in current['evidence_runs']))
        from verify_runs import compact_run
        row.update(summary=current['summary'], evidence_runs=[compact_run(r) for r in current['evidence_runs']],
                   receipt_counts=case['receipt_counts'], report_artifact='release/report.json',
                   report_sha256=case['report_sha256'])
        write(out / 'release-summary.json', portable(row))
        if name == 'firecracker':
            credits = portable(json.loads((out / 'release-credits.json').read_text()))
            credits.update(report_artifact='release/report.json', report_sha256=case['report_sha256'])
            write(out / 'release-credits.json', credits)
        index['survey'].append(case)
        print(name, 'regenerated with identical coverage', flush=True)
    required = set()
    for case in index['scratch'] + index['survey']:
        required.update([case['archive'], case['report']])
        required.update(case.get('vendor_archives', []))
        for key in ['config', 'lock']:
            if case.get(key):
                required.add(case[key])
        for _, log in case['evidence']:
            required.update([log, log + '.evidence.json'])
    index['required'] = sorted(required)
    if not args.scratch_only:
        path = REPO / 'docs/survey/results.json'
        document = json.loads(path.read_text())
        document['projects'] = [json.loads((REPO / 'runs/v1.4' / name / 'release-summary.json').read_text()) for name in ['vest', 'anvil', 'firecracker']]
        write(path, portable(document))
    write(REPO / 'logs/release-evidence.json', index)
    write(REPO / 'docs/survey/tool-sources.json', index['tool_sources'])


if __name__ == '__main__':
    main()
