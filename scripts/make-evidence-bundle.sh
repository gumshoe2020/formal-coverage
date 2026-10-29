#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 - "$@" <<'PY'
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile

root = Path.cwd()
destination = Path(sys.argv[1]) if len(sys.argv) > 1 else root.parent / 'release'
destination.mkdir(parents=True, exist_ok=True)
index = json.loads((root / 'logs/release-evidence.json').read_text())
paths = set(index['required']) | {'logs/release-evidence.json'}
for base in ['logs/evidence', '.cargo-home']:
    paths.update(str(p.relative_to(root)) for p in (root / base).rglob('*') if p.is_file())
for project in ['vest', 'anvil', 'firecracker']:
    base = root / 'runs/v1.4' / project
    paths.update(str(p.relative_to(root)) for p in base.iterdir() if p.is_file())
    for folder in ['cargo-home', 'm2b', 'release']:
        paths.update(str(p.relative_to(root)) for p in (base / folder).rglob('*') if p.is_file())
    # Retained verifier build locks are small and independently hashed by manifests.
    paths.update(str(p.relative_to(root)) for p in (base / 'verus-lockfiles').rglob('*') if p.is_file())
paths.update(str(p.relative_to(root)) for p in (root / 'logs').glob('v1.1-c-*') if p.is_file())
for path in paths:
    assert not Path(path).is_absolute() and '..' not in Path(path).parts
bundle = destination / 'formal-coverage-evidence-v1.4.1.tar.zst'
with tempfile.TemporaryFile(mode='w+') as listing:
    listing.write(''.join(p + '\n' for p in sorted(paths)))
    listing.seek(0)
    tar = subprocess.Popen(['tar', '-cf', '-', '-T', '/dev/stdin'], stdin=listing, stdout=subprocess.PIPE)
    try:
        subprocess.run(['zstd', '-q', '-T0', '-f', '-o', str(bundle)], stdin=tar.stdout, check=True)
    finally:
        tar.stdout.close()
    assert tar.wait() == 0
hasher = hashlib.sha256()
with bundle.open('rb') as source:
    for block in iter(lambda: source.read(1024 * 1024), b''):
        hasher.update(block)
bundle.with_suffix(bundle.suffix + '.sha256').write_text(f'{hasher.hexdigest()}  {bundle.name}\n')
print(f'{bundle.name}: {bundle.stat().st_size} bytes; {len(paths)} relative artifact paths')
PY
