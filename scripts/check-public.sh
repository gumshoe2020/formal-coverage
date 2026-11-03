#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 - "$@" <<'PY'
from pathlib import Path
import re
import subprocess
import sys

if len(sys.argv) not in (2, 3):
    raise SystemExit('usage: scripts/check-public.sh DENYLIST [EXTRACTED_BUNDLE]')
patterns = [line for line in Path(sys.argv[1]).read_bytes().splitlines() if line]
patterns.append(b'-----' + b'BEGIN')
# Bundle-mode masks: exact substrings removed before matching, so any other
# hit on the same line (including other armor markers) still counts. Armor
# masks keep their closing dashes, which may also open a following marker.
masks = [re.compile(re.escape(b'/Us' b'ers/runner/work/')), re.compile(re.escape(b'api.github.com/us' b'ers/')),
         re.compile(b'-----' b'BEGIN PGP SIGNATURE(?=-----)'),
         re.compile(b'-----' b'BEGIN PGP PUBLIC KEY BLOCK(?=-----)')]
excluded_files = {'logs/evidence/archives/firecracker.tar'}
if len(sys.argv) == 3:
    root = Path(sys.argv[2])
    if not root.is_dir():
        raise SystemExit('extracted bundle must be a directory')
    print('Excluding */cargo-home/registry/src/** (byte-identical third-party sources)', flush=True)
    print('Excluding */cargo-home/git/checkouts/** (byte-identical third-party git dependency sources)', flush=True)
    print('Masking substring /Us' 'ers/runner/work/ (upstream CI build paths)', flush=True)
    print('Masking substring api.github.com/us' 'ers/ (GitHub API URLs in pinned metadata)', flush=True)
    print('Excluding logs/evidence/archives/firecracker.tar (pinned public upstream source, unmodified)', flush=True)
    print('Masking substrings -----' 'BEGIN PGP SIGNATURE----- and -----' 'BEGIN PGP PUBLIC KEY BLOCK----- '
          '(public signature/key material; other armor markers still count)', flush=True)
    files = sorted(p for p in root.rglob('*') if p.is_file()
                   and 'cargo-home/registry/src/' not in p.relative_to(root).as_posix()
                   and 'cargo-home/git/checkouts/' not in p.relative_to(root).as_posix()
                   and p.relative_to(root).as_posix() not in excluded_files)
else:
    files = [Path(p.decode()) for p in subprocess.check_output(['git', 'ls-files', '-z']).split(b'\0') if p]
hits = 0
for name in files:
    if not name:
        continue
    path = Path(name)
    if not path.is_file():
        continue
    for number, line in enumerate(path.read_bytes().splitlines(), 1):
        scanned = line
        if len(sys.argv) == 3:
            for mask in masks:
                scanned = mask.sub(b'', scanned)
        if any(pattern.lower() in scanned.lower() for pattern in patterns):
            print(f'{path}:{number}:{line.decode(errors="replace")}')
            hits += 1
print(f'Public scan: {hits} matching lines')
raise SystemExit(1 if hits else 0)
PY
