# usage (in container, via dev/v172/docker.sh): report.sh TREE FSTAR_VERSION TOOL_BINARY OUT_SUFFIX
# Report step only, same arguments as scripts/hax/v171-report.sh; writes report$SUFFIX.{json,md,log,exit}.
tree="$1"; version="$2"; tool="$3"; suffix="$4"; base="$V171_OUT/$tree"
export RUSTUP_TOOLCHAIN=1.98.0 RUSTUP_HOME=/usr/local/rustup
cd "$base"
"$tool" report "$base/root" --target aarch64-unknown-linux-gnu \
  --size-mode compiled-files --check-target-dir "$base/check-target" \
  --fstar-log "$base/typecheck-extraction-$version.log" \
  --json "$base/report$suffix.json" --markdown "$base/report$suffix.md" > "$base/report$suffix.log" 2>&1
echo "$?" > "$base/report$suffix.exit"
