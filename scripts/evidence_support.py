"""Portable paths, source provenance and offline evidence replay (no target builds)."""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import subprocess
import tarfile

REPO = Path(__file__).resolve().parents[1]
BUNDLE = 'formal-coverage-evidence-v1.4.1.tar.zst'
FETCH = f'Missing {BUNDLE}; fetch it and its .sha256 from https://github.com/gumshoe2020/formal-coverage/releases/tag/v1.4.1 and extract it in the checkout.'


def arguments(description, scratch=False):
    parser = argparse.ArgumentParser(description=description)
    parser.add_argument('--evidence-root', type=Path, default=Path(os.environ.get('FC_EVIDENCE_ROOT', REPO)))
    parser.add_argument('--binary', type=Path, default=Path(os.environ.get('FC_BINARY', REPO / 'target/release/formal-coverage')))
    if scratch:
        parser.add_argument('--scratch-server', type=Path, default=Path(os.environ.get('FC_SCRATCH_SERVER', REPO.parent / 'scratch-server')))
    return parser


def require_bundle(root):
    marker = root / 'logs/release-evidence.json'
    if not marker.is_file():
        raise SystemExit(FETCH)
    index = json.loads(marker.read_text())
    if any(not (root / p).is_file() for p in index['required']):
        raise SystemExit(FETCH)
    return index


def sha(data):
    return hashlib.sha256(data).hexdigest()


def source_identity(root=REPO):
    paths = ['Cargo.toml', 'Cargo.lock', 'registry/upstream.toml'] + sorted(str(p.relative_to(root)) for p in (root / 'src').rglob('*.rs'))
    return {p: sha((root / p).read_bytes()) for p in paths}


def check_tool(index):
    assert index['tool_sources'] == source_identity(), 'tool source differs from report provenance; regenerate release evidence'


def write(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + '\n')


def portable(value, root=REPO):
    """Remove machine-specific prefixes from generated report fields only."""
    if isinstance(value, dict):
        return {portable(k, root): portable(v, root) for k, v in value.items()}
    if isinstance(value, list):
        return [portable(v, root) for v in value]
    if isinstance(value, str):
        value = value.replace(str(root.resolve()) + '/', '')
        # Cargo resolves source roots to absolute paths, even with relative CLI inputs.
        import re
        value = re.sub(r'/private/tmp/[^\s\"`]+', '<temporary-path>', value)
        value = re.sub(r'/tmp/[^\s\"`]+', '<temporary-path>', value)
        value = re.sub(r'/Us' r'ers/[^\s\"`]+', '<local-path>', value)
        return value
    return value


def extract(archive, destination):
    destination.mkdir(parents=True, exist_ok=True)
    with tarfile.open(archive) as tar:
        tar.extractall(destination, members=[m for m in tar.getmembers() if m.isfile() or m.isdir()], filter='data')


def replay(root, binary, case, output):
    destination = root / case['source']
    assert sha((root / case['archive']).read_bytes()) == case['archive_sha256'], case['name']
    extract(root / case['archive'], destination)
    # Export-only metadata isolation and generated Anvil lock: no Rust source changes.
    for archive in case.get('vendor_archives', []):
        extract(root / archive, destination / 'vendor')
    if case.get('lock'):
        (destination / 'Cargo.lock').write_bytes((root / case['lock']).read_bytes())
    if case.get('config'):
        (destination / 'formal-coverage.toml').write_bytes((root / case['config']).read_bytes())
    if case.get('isolate_workspace'):
        import tomllib
        manifest = destination / 'Cargo.toml'
        if 'workspace' not in tomllib.loads(manifest.read_text()):
            manifest.write_text(manifest.read_text() + '\n[workspace]\n')
    output.mkdir(parents=True, exist_ok=True)
    command = [str(binary.resolve()), 'report', case['source'], '--target', case['target'],
               '--json', str(output / 'report.json'), '--markdown', str(output / 'report.md')]
    for tool, log in case['evidence']:
        command += [f'--{tool}-log', log]
    env = {**os.environ, 'CARGO_HOME': str((root / case['cargo_home']).resolve()),
           'TMPDIR': str((root / '.tmp').resolve()), 'CARGO_NET_OFFLINE': 'true', 'RUSTUP_TOOLCHAIN': 'stable'}
    (root / '.tmp').mkdir(exist_ok=True)
    with (output / 'report.log').open('w') as log:
        subprocess.run(command, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, check=True)
    report = portable(json.loads((output / 'report.json').read_text()), root)
    write(output / 'report.json', report)
    (output / 'report.md').write_text(portable((output / 'report.md').read_text(), root))
    return report


def validate_receipts(root, case, report):
    with tarfile.open(root / case['archive']) as tar:
        sources = {m.name: sha(tar.extractfile(m).read()) for m in tar.getmembers() if m.isfile() and m.name.endswith('.rs')}
    assert set(report['source_files']) <= set(sources)
    assert report['source_files'], case['name']
    assert len(report['evidence_runs']) == len(case['evidence'])
    for (tool, log), run in zip(case['evidence'], report['evidence_runs']):
        receipt = json.loads((root / (log + '.evidence.json')).read_text())
        assert receipt['files'] == sources, log
        assert receipt['log_sha256'] == sha((root / log).read_bytes()), log
        assert run['tool'] == tool and run['log'] == log
        if not run['accepted']:
            assert run['reasons'] and not run['credited_units']
