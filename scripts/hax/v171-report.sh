# usage: v171-report.sh TREE FSTAR_VERSION  (container, hax image; mirrors v1.7 measure.sh with the v1.7 tool binary)
tree="$1"; version="$2"; base="$V171_OUT/$tree"
export RUSTUP_TOOLCHAIN=1.98.0 RUSTUP_HOME=/usr/local/rustup
cd "$base"
logs=(); for l in typecheck-extraction-$version.log; do [ -f "$l" ] && logs+=(--fstar-log "$base/$l"); done
"$V171_OUT/tool-target/release/formal-coverage" report "$base/root" --target aarch64-unknown-linux-gnu \
  --size-mode compiled-files --check-target-dir "$base/check-target" "${logs[@]}" \
  --json "$base/report.json" --markdown "$base/report.md" > "$base/report.log" 2>&1
echo "$?" > "$base/report.exit"
