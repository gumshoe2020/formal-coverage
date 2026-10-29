#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
python3 - "$@" <<'PY'
from pathlib import Path
import subprocess
import sys

if len(sys.argv) != 2:
    raise SystemExit('usage: scripts/check-public.sh DENYLIST')
patterns = [line for line in Path(sys.argv[1]).read_bytes().splitlines() if line]
patterns.append(b'-----' + b'BEGIN')
files = subprocess.check_output(['git', 'ls-files', '-z']).split(b'\0')
hits = 0
for name in files:
    if not name:
        continue
    path = Path(name.decode())
    if not path.is_file():
        continue
    for number, line in enumerate(path.read_bytes().splitlines(), 1):
        if any(pattern.lower() in line.lower() for pattern in patterns):
            print(f'{path}:{number}:{line.decode(errors="replace")}')
            hits += 1
raise SystemExit(1 if hits else 0)
PY
