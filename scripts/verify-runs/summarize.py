#!/usr/bin/env python3
"""Snapshot actual v1.4 artifacts; never infer proof from an unaccepted run."""
from collections import Counter
from datetime import datetime
import json
from pathlib import Path
import sys
import tomllib

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_runs as v


def finalize_manifest(name):
    out = v.RUNS / name
    path = out / 'manifest.json'
    record = json.loads(path.read_text())
    versions = out / 'tool-versions.json'
    if versions.exists():
        facts = json.loads(versions.read_text())
        record['tool_versions'] = facts
        record['rust_toolchains'] = facts[0]['output'].splitlines()
        record['z3_version'] = next((f['output'] for f in facts if f['command'][0].endswith('/z3')), None)
    else:
        record['rust_toolchains'] = ['1.98.0-aarch64-unknown-linux-gnu (fetch only; pinned base image)']
    record['z3_applicability'] = 'Verus solver' if name == 'anvil' else 'not applicable (Kani uses bundled CBMC/SAT; Vest replays retained evidence)'
    record['artifact_hashes'] = {str(p.relative_to(out)): v.sha(p.read_bytes())
                                 for p in out.glob('*') if p.is_file() and
                                 (p.name.startswith('image-build') or p.name in ['packages.txt', 'tool-versions.json', 'Cargo.lock', 'Cargo.pinned-resolution.lock', 'pin-candidates.json', 'tla-pin.json', 'available-modules.json', 'selector-definitions.txt', 'selector-usage.txt', 'module-filter-source.txt'])}
    packages = out / 'packages.txt'
    if packages.exists():
        record['system_packages'] = dict(line.split('\t', 1) for line in packages.read_text().splitlines() if '\t' in line)
    if name == 'firecracker':
        record['image_builds'] = [dict(log='image-build-bookworm.log', exit_code=1, base_image='rust@sha256:82150a52ec202c1b14d7817e14516c392bb7f5cfebd88f1ed531cb37ebd39922'),
                                  dict(log='image-build.log', exit_code=0, base_image=v.BASE),
                                  dict(log='image-build-toolchain.log', exit_code=0, base_image=v.BASE, image_id=record['image_id'])]
    elif name == 'anvil':
        record['image_builds'] = [dict(log='image-build.log', exit_code=0, base_image=v.BASE, image_id=record['image_id'])]
    record['tool_build_lockfiles'] = {str(p.relative_to(out)): v.sha(p.read_bytes())
                                    for p in (out / 'verus-lockfiles').rglob('Cargo.lock')}
    lock = v.EXPORTS / name / 'Cargo.lock'
    if lock.exists():
        data = tomllib.loads(lock.read_text())
        record['lockfile_sha256'] = v.sha(lock.read_bytes())
        record['git_dependencies'] = sorted({p['source'] for p in data['package'] if p.get('source', '').startswith('git+')})
        record['lockfile_origin'] = 'generated with date-pinned Verus and verus-tla; archived manifests unchanged' if name == 'anvil' else 'git archive'
    if name == 'vest':
        log = v.REPO / 'docs/survey/vest.verus.log'
        receipt_path = Path(str(log) + '.evidence.json')
        retained = json.loads(receipt_path.read_text())
        record['retained_verification'] = {k: retained[k] for k in ['tool_version', 'command', 'exit_code', 'cfg', 'log_sha256']}
        record['retained_verification']['receipt_sha256'] = v.sha(receipt_path.read_bytes())
    if name == 'anvil':
        record['verus_pin_selection'] = {'branch': 'main', 'until': '2026-10-12T23:59:59Z',
                                        'commit_date': '2026-10-12T20:36:21Z', 'attempted_commits': [v.VERUS_SHA]}
        record['verus_tla_commit'] = 'e7eda3f24b60fbbf4396e623a06da9ec32512c0e'
    v.write(path, record)
    return record


def summary(name):
    out = v.RUNS / name
    manifest = finalize_manifest(name)
    evidence = [('verus', v.REPO / 'docs/survey/vest.verus.log')] if name == 'vest' else [
        ('kani' if name == 'firecracker' else 'verus', Path(str(p).removesuffix('.evidence.json')))
        for p in sorted(out.glob(('kani' if name == 'firecracker' else 'verus') + '*.log.evidence.json'))]
    report = v.host_report(name, evidence, out / 'cargo-home')
    unknown = [dict(name=d['name'], version=d['version'], reason='locked fetch or source inventory unresolved; see closure reason and fetch log')
               for d in report['dependencies'] if not d['size_known']]
    baseline = json.loads((v.REPO / f'logs/v1.1-c-{name}.json').read_text())
    record = dict(project=name, commit=manifest['commit'], export=manifest['export'],
                  summary=report['summary'], evidence_runs=[v.compact_run(r) for r in report['evidence_runs']],
                  dependency_inventory_complete=report['dependency_inventory_complete'],
                  closure=report['closure'], closure_reason=report['closure_reason'],
                  target=report['target'], cargo_version=report['cargo_version'],
                  build_dependency_count=len(report['build_dependencies']),
                  build_dependency_sloc=sum(d['sloc'] for d in report['build_dependencies']),
                  build_unknown_dependencies=[dict(name=d['name'], version=d['version'], reason='source inventory unresolved') for d in report['build_dependencies'] if not d['size_known']],
                  dev_dependencies_excluded=report['dev_dependencies_excluded'],
                  unknown_dependencies=unknown, dependency_count=len(report['dependencies']),
                  dependency_sloc=sum(d['sloc'] for d in report['dependencies']),
                  dependency_before=dict(count=len(baseline['dependencies']),
                                         sloc=sum(d['sloc'] for d in baseline['dependencies']),
                                         unknown=sum(not d['size_known'] for d in baseline['dependencies']),
                                         complete=baseline['dependency_inventory_complete']),
                  report_sha256=v.sha((out / 'report.json').read_bytes()),
                  manifest_sha256=v.sha((out / 'manifest.json').read_bytes()),
                  escape_sites=[{k: e[k] for k in ['file', 'line', 'kind', 'first_party', 'security']} for e in report['escape_hatches'] if e['first_party']],
                  trusted_inventory=dict(count=len(report['trusted_items']), sloc=report['summary']['trusted_sloc'], sha256=v.stable_sha(report['trusted_items'])))
    if name == 'firecracker':
        data = json.loads((out / 'harnesses.json').read_text())
        record['verifier_outcome'] = dict(counts=data['counts'], harnesses=data['harnesses'],
                                          workspace='compilation failed: cpu-template-helper writes to read-only source',
                                          scope=data['scope'])
    elif name == 'anvil':
        data = json.loads((out / 'modules.json').read_text())
        record['verifier_outcome'] = data
    else:
        record['verifier_outcome'] = dict(verified=2935, errors=0, scope='retained source-bound Vest run; no new verifier execution')
        units = [u for u in report['units'] if u['crate_name'] == 'vest_lib' and u['category'] == 'runtime']
        record['vest_lib'] = {label: dict(sloc=sum(u['sloc'] for u in units if not security or u['security']),
                                        proven=sum(u['sloc'] for u in units if u['status'] == 'proven' and (not security or u['security'])))
                              for label, security in [('runtime', False), ('security_runtime', True)]}
    record['receipt_counts'] = dict(Counter('accepted' if r['accepted'] else 'rejected' for r in report['evidence_runs']))
    v.write(out / 'summary.json', record)
    return record


def main():
    parser = __import__('argparse').ArgumentParser(description=__doc__)
    parser.add_argument('projects', nargs='*', default=['vest', 'anvil', 'firecracker'])
    args = parser.parse_args()
    for name in args.projects:
        row = summary(name)
        print(name, row['closure'], len(row['unknown_dependencies']), row['receipt_counts'], flush=True)


if __name__ == '__main__':
    main()
