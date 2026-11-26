#!/usr/bin/env bash
# Run both test suites the way a git hook would, pointed at a decoy repository, and
# check the decoy is byte-unchanged. Usage: dev/v1122/decoy_suite.sh [git-dir-only]
set -euo pipefail
cd "$(dirname "$0")/../.."
decoy=$(mktemp -d "${TMPDIR:-/tmp}/fc-decoy.XXXXXX")
trap 'rm -rf "$decoy"' EXIT
export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
git -C "$decoy" init -q -b main
echo tracked > "$decoy/keep.txt"
git -C "$decoy" add .
git -C "$decoy" -c user.name=Decoy -c user.email=decoy@example.invalid commit -qm decoy
unset GIT_CONFIG_GLOBAL GIT_CONFIG_NOSYSTEM
fingerprint() { (cd "$decoy" && find . -type f | LC_ALL=C sort | xargs shasum); }
before=$(fingerprint)
hook_env=(GIT_DIR="$decoy/.git")
[ "${1:-}" = git-dir-only ] || hook_env+=(GIT_WORK_TREE="$decoy")
echo "hook environment: ${hook_env[*]}"
status=0
env "${hook_env[@]}" cargo test --locked || status=1
env "${hook_env[@]}" python3 -m unittest discover -s ci/tests || status=1
if [ "$before" = "$(fingerprint)" ]; then
  echo "decoy unchanged: $(git -C "$decoy" rev-parse HEAD)"
else
  echo "DECOY CHANGED:"; diff <(echo "$before") <(fingerprint) || true; status=1
fi
exit $status
