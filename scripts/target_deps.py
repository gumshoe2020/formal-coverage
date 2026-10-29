#!/usr/bin/env python3
"""Offline v1.2 scratch-server replay and literal acceptance-C set comparison.

Exports pinned commits under this repo's target/, reuses retained source-bound
receipts, never builds or runs scratch-server code, and never uses the network.
Returns 1 if the literal C comparison differs; artifacts are still written.
"""
import io
import json
import os
from pathlib import Path
import re
import subprocess
import tarfile

REPO = Path(__file__).resolve().parents[1]
SCRATCH = REPO.parent / 'scratch-server'
EXPORTS = REPO / 'target/v1.2-scratch'
LOGS = REPO / 'logs'
ENV = {**os.environ, 'CARGO_HOME': str(REPO / '.cargo-home'),
       'TMPDIR': str(REPO / '.tmp'), 'CARGO_NET_OFFLINE': 'true'}


def capture(command):
    return subprocess.check_output(command, cwd=REPO, env=ENV, text=True).strip()


def main():
    (REPO / '.tmp').mkdir(exist_ok=True)
    EXPORTS.mkdir(parents=True, exist_ok=True)
    LOGS.mkdir(exist_ok=True)
    host = next(line[6:] for line in capture(['rustc', '-vV']).splitlines()
                if line.startswith('host: '))
    records = json.loads((REPO / 'docs/scratch-server/runs.json').read_text())
    results = []
    comparison = None
    for row in records:
        branch = row['branch']
        destination = EXPORTS / branch
        destination.mkdir(exist_ok=True)
        archive = subprocess.check_output(['git', '-C', str(SCRATCH), 'archive', row['commit']])
        with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
            tar.extractall(destination, members=[m for m in tar.getmembers()
                                                if m.isfile() or m.isdir()], filter='data')
        # Early branches predate their own workspace; isolate these exported manifests
        # from the enclosing tool workspace without changing dependencies or Rust sources.
        import tomllib
        manifest = destination / 'Cargo.toml'
        if 'workspace' not in tomllib.loads(manifest.read_text()):
            manifest.write_text(manifest.read_text() + '\n[workspace]\n')
        (destination / 'formal-coverage.toml').write_text(
            (REPO / 'examples/scratch-server/formal-coverage.toml').read_text())
        output = LOGS / f'v1.2-scratch-{branch}.json'
        markdown = LOGS / f'v1.2-scratch-{branch}.md'
        command = [str(REPO / 'target/release/formal-coverage'), 'report', str(destination),
                   '--target', host, '--json', str(output), '--markdown', str(markdown)]
        for receipt in row['receipts']:
            command.extend(['--verus-log', str(REPO / 'docs/scratch-server' / receipt)])
        with (LOGS / f'v1.2-scratch-{branch}.log').open('w') as log:
            subprocess.run(command, cwd=REPO, env=ENV, stdout=log,
                           stderr=subprocess.STDOUT, check=True)
        report = json.loads(output.read_text())
        previous = json.loads((REPO / 'docs/scratch-server' / f'{branch}.json').read_text())
        identity = lambda r: [(u['id'], u['file'], u['line'], u['sloc'], u['category'],
                               u['security'], u['status']) for u in r['units']]
        assert identity(report) == identity(previous), f'{branch}: changed first-party attribution'
        assert report['summary']['proof_code'] == previous['summary']['proof_code']
        assert all(run['accepted'] for run in report['evidence_runs'])
        assert not any(e['first_party'] for e in report['escape_hatches'])
        data = {key: report[key] for key in ['closure', 'closure_reason', 'target',
                                            'cargo_version', 'dev_dependencies_excluded', 'summary']}
        data.update(branch=branch, commit=row['commit'], receipts=row['receipts'])
        data['runtime_dependencies'] = [{k: d[k] for k in ['name', 'version', 'sloc',
                                       'size_known', 'unsafe_sites']} for d in report['dependencies']]
        data['build_dependencies'] = [{k: d[k] for k in ['name', 'version', 'sloc',
                                     'size_known', 'unsafe_sites']} for d in report['build_dependencies']]
        if report['closure_reason']:
            assert {(d['name'], d['version']) for d in report['dependencies']} == {
                (d['name'], d['version']) for d in previous['dependencies']}
            assert any('unknown dependency SLoC' in line for line in subprocess.run(
                [str(REPO / 'target/release/formal-coverage'), 'gate', str(destination),
                 '--target', host, '--min-first-party-security-proven', '0'],
                cwd=REPO, env=ENV, capture_output=True, text=True).stderr.splitlines())
            print(f'{branch}: FALLBACK: {report["closure_reason"]}', flush=True)
        else:
            assert all(d['size_known'] and d['unsafe_sites'] is not None
                       for d in report['dependencies'] + report['build_dependencies'])
            print(f'{branch}: runtime={len(report["dependencies"])}; '
                  f'build={len(report["build_dependencies"])}; '
                  f'dev excluded={report["dev_dependencies_excluded"]}', flush=True)
        results.append(data)
        if branch == 'spec-lemmas':
            assert row['commit'].startswith('2fa5eb78a6f9')
            tree_command = ['cargo', 'tree', '-e', 'normal', '--target', host,
                            '--frozen', '--prefix', 'none', '--manifest-path',
                            str(destination / 'Cargo.toml')]
            tree_log = LOGS / 'v1.2-scratch-spec-lemmas-tree.log'
            with tree_log.open('w') as log:
                code = subprocess.run(tree_command, cwd=REPO, env=ENV, stdout=log,
                                      stderr=subprocess.STDOUT).returncode
            # Remove workspace packages by exact name@version, rather than name alone.
            import tomllib
            workspace = set()
            for manifest in [destination / 'Cargo.toml',
                             *sorted((destination / 'crates').glob('*/Cargo.toml'))]:
                package = tomllib.loads(manifest.read_text())['package']
                workspace.add(f'{package["name"]}@{package["version"]}')
            expected = {f'{name}@{version}' for name, version in
                        re.findall(r'^(\S+) v(\S+)', tree_log.read_text(), re.M)} - workspace
            actual = {f'{d["name"]}@{d["version"]}' for d in report['dependencies']}
            comparison = dict(command=tree_command, exit_code=code,
                              tree=sorted(expected), runtime=sorted(actual),
                              tree_only=sorted(expected - actual),
                              metadata_runtime_only=sorted(actual - expected))
            first = report['summary']['first_party']['security_relevant']
            assert (first['statuses']['proven'], first['sloc']) == (644, 1363)
            assert 'proven 47.25%' in markdown.read_text().splitlines()[0]
            assert report['summary']['third_party']['security_relevant']['sloc'] < 1626171
            assert expected - actual <= {
                f'{d["name"]}@{d["version"]}' for d in report['build_dependencies']}
            comparison['first_line'] = markdown.read_text().splitlines()[0]
            comparison['third_party_security_sloc'] = report['summary']['third_party']['security_relevant']['sloc']
            comparison['equal'] = code == 0 and actual == expected
    artifact = REPO / 'docs/scratch-server/v1.2-results.json'
    artifact.write_text(json.dumps(dict(results=results, acceptance_c=comparison), indent=2) + '\n')
    print('Acceptance C literal equality:', comparison['equal'])
    print('Tree only:', ', '.join(comparison['tree_only']) or 'none')
    print('Metadata runtime only:', ', '.join(comparison['metadata_runtime_only']) or 'none')
    print('First-party security: 644/1363 = 47.25%; third-party security SLoC:',
          comparison['third_party_security_sloc'])
    return 0 if comparison['equal'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
