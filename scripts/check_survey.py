#!/usr/bin/env python3
"""Audit portable survey archives, logs, receipts, source provenance and published totals offline."""
from collections import Counter
import json
from pathlib import Path
import re
import shlex
import tarfile

from evidence_support import arguments, require_bundle, check_tool, sha, validate_receipts
from verify_runs import compact_run, outcome_row, dependency_row, kani_result


def check_steps(root, out, manifest):
    for step in manifest.get('steps', []):
        assert sha((out / step['log']).read_bytes()) == step['log_sha256'], step['log']
        command = step['command']
        mounts = [command[i + 1] for i, arg in enumerate(command) if arg == '--mount']
        export = f'<temporary-export>/{out.name}'
        assert mounts == [f'type=bind,source={export},target={export},readonly',
                          f'type=bind,source=<checkout>/runs/v1.4/{out.name},target=/out']
        assert command[:2] == ['docker', 'run'] and '--rm' in command
        assert command[command.index('--platform') + 1] == 'linux/arm64'
        assert command[command.index('--workdir') + 1] == export
        assert all(token not in command for token in ['--privileged', '-v'])
        assert re.fullmatch(r'sha256:[a-f0-9]{64}', step['image_id'])
        assert step['image_id'] in command
        assert step['seconds'] <= step['timeout_seconds'] + 60


def main():
    args = arguments(__doc__).parse_args()
    root = args.evidence_root.resolve()
    index = require_bundle(root)
    check_tool(index)
    assert index['tool_sources'] == json.loads((root / 'docs/survey/tool-sources.json').read_text())
    document = json.loads((root / 'docs/survey/results.json').read_text())
    rows = {r['project']: r for r in document['projects']}
    assert set(rows) == {'vest', 'anvil', 'firecracker'}
    survey = (root / 'docs/survey.md').read_text()
    for case in index['survey']:
        name = case['name']; row = rows[name]; out = root / 'runs/v1.4' / name
        assert sha((root / case['archive']).read_bytes()) == case['archive_sha256']
        manifest = json.loads((out / 'manifest.json').read_text())
        assert row['commit'] == case['commit'] == manifest['commit']
        assert manifest['archive_sha256'] == case['archive_sha256']
        assert manifest['platform'] == 'linux/aarch64' and '@sha256:' in manifest['base_image']
        check_steps(root, out, manifest)
        for path, digest in {**manifest.get('artifact_hashes', {}), **manifest.get('tool_build_lockfiles', {})}.items():
            assert sha((out / path).read_bytes()) == digest, path
        for path in ['m3-manifest.json', 'm2b-manifest.json', 'release-rerun-manifest.json']:
            if (out / path).exists():
                check_steps(root, out, json.loads((out / path).read_text()))
        report = json.loads((root / case['report']).read_text())
        assert sha((root / case['report']).read_bytes()) == case['report_sha256']
        assert report['summary'] == row['summary'] == case['summary']
        assert report['closure'] == row['closure'] == 'cargo metadata (runtime)'
        assert report['target'] == row['target'] == 'aarch64-unknown-linux-gnu'
        assert all(d['size_known'] for d in report['dependencies'] + report['build_dependencies'])
        assert row['dependency_count'] == len(report['dependencies'])
        assert row['dependency_sloc'] == sum(d['sloc'] for d in report['dependencies'])
        assert row['build_dependency_count'] == len(report['build_dependencies'])
        assert row['build_dependency_sloc'] == sum(d['sloc'] for d in report['build_dependencies'])
        validate_receipts(root, case, report)
        assert row['evidence_runs'] == [compact_run(r) for r in report['evidence_runs']]
        assert row['receipt_counts'] == case['receipt_counts'] == dict(Counter('accepted' if r['accepted'] else 'rejected' for r in report['evidence_runs']))
        assert outcome_row(name, row) in survey and dependency_row(row) in survey
        for run in report['evidence_runs']:
            if not run['accepted']:
                assert run['reasons'] and not run['credited_units']
        if name == 'vest':
            units = [u for u in report['units'] if u['crate_name'] == 'vest_lib' and u['category'] == 'runtime']
            for key, security, expected in [('security_runtime', True, (2961, 10555)), ('runtime', False, (2961, 11916))]:
                group = [u for u in units if not security or u['security']]
                assert (sum(u['sloc'] for u in group if u['status'] == 'proven'), sum(u['sloc'] for u in group)) == expected
                assert row['vest_lib'][key] == dict(proven=expected[0], sloc=expected[1])
            assert row['receipt_counts'] == {'accepted': 1}
        elif name == 'anvil':
            modules = json.loads((out / 'modules.json').read_text())
            assert row['verifier_outcome'] == modules
            assert row['receipt_counts'] == {'rejected': 53}
            assert sum(m['verified'] or 0 for m in modules['modules']) == 935
            for module in modules['modules']:
                if module['outcome'] == 'verified':
                    text = (out / module['log']).read_text().rsplit('Checking verifiable-controllers ', 1)[-1]
                    totals = re.findall(r'verification results:: (\d+) verified, (\d+) errors', text)
                    assert totals and tuple(map(int, totals[-1])) == (module['verified'], 0)
                    assert module['fresh_compile'] and module['observed_modules']
                    assert all(m in module['selected_descendants'] for m in module['observed_modules'])
        else:
            credits = json.loads((out / 'release-credits.json').read_text())
            assert (credits['credited'], credits['passing']) == (35, 38)
            assert credits['runtime_targets_credited'] == 0
            assert not any(u['category'] == 'runtime' and u['status'] in ['proven', 'proven-bounded'] for u in report['units'])
            assert len({(e['file'], e['line'], e['kind']) for e in report['escape_hatches'] if e['first_party'] and e['kind'] == 'unsafe'}) == 412
            by_log = {r['log']: r for r in report['evidence_runs']}
            by_id = {u['id']: u for u in report['units']}
            for h in credits['harnesses']:
                if h['outcome'] == 'pass':
                    run = by_log[h['report_log']]
                    assert h['credited'] == (run['accepted'] and h['id'] in run['credited_units'])
                    if h['credited']:
                        assert by_id[h['id']]['status'] == 'proven-bounded'
                        assert 'dyn calls restricted to type-compatible vtable targets' in by_id[h['id']]['proof_scope']
            rerun = json.loads((out / 'release-rerun-manifest.json').read_text())['steps'][0]
            assert ['--network', 'none'] == rerun['command'][rerun['command'].index('--network'):][:2]
            expected = 'devices::virtio::iovec::verification::verify_write_to_iovec_0'
            assert kani_result((out / rerun['log']).read_text(), expected, rerun['exit_code'])['passed']
        print(name, 'archives, pinned mounts, log hashes, receipts, outcomes and published totals passed')
    assert len(index['survey']) == 3


if __name__ == '__main__':
    main()
