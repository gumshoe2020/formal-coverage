#!/usr/bin/env bash
# Replace the absolute repository root with <repo> in the tracked v1.7.1 text outputs.
set -eu
root="$PWD"
git ls-files -z -c -o --exclude-standard runs/v1.7.1 logs/v1.7.1 2>/dev/null | xargs -0 grep -lI -F "$root" 2>/dev/null | python3 -c '
import sys
root = sys.argv[1]
for name in sys.stdin.read().split():
    text = open(name).read()
    open(name, "w").write(text.replace(root, "<repo>"))
' "$root" || true
