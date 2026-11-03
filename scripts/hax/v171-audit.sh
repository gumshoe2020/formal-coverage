#!/usr/bin/env bash
# usage: v171-audit.sh TREE   (host, repository root). Tree check + lock audit v2 + full lock diff.
set -u
d="runs/v1.7.1/$1"; r="$d/resolve"
python3 scripts/hax/v171_workspace.py check "$d/ws" > /dev/null
diff -u "$d/shipped-input.Cargo.lock" "$d/ws/Cargo.lock" > "$d/lock.diff"
python3 scripts/hax/lock_audit_v2.py "$d/shipped-input.Cargo.lock" "$d/ws/Cargo.lock" "$d/tree-check.json" "$r/metadata.json" \
  "$r/closure-proof-features.txt" "$r/closure-proof-features-hax-cfg.txt" \
  "$r/closure-default-features.txt" "$r/closure-default-features-hax-cfg.txt" > "$d/lock-audit-v2.json" 2> "$d/lock-audit-v2.err"
echo "$?" > "$d/lock-audit-v2.exit"
