set -u
out="$1"
export RUSTUP_TOOLCHAIN=1.98.0
export HOME="$out/home" CARGO_HOME="$out/cargo-home" RUSTUP_HOME=/usr/local/rustup
for tree in upstream shipped; do
  work="$out/work/$tree"
  "$out/tool-target/release/formal-coverage" report "$work" --target aarch64-unknown-linux-gnu --size-mode compiled-files --check-target-dir "$out/check-target/$tree" --fstar-log "$work/typecheck-extraction.log" --fstar-log "$work/typecheck-panic-free.log" --json "$work/report.json" --markdown "$work/report.md" > "$work/report.log" 2>&1
  echo "$?" > "$work/report.exit"
done
