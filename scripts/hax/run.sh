#!/usr/bin/env bash
# Run only inside the prepared Docker image, with the documented mounts.
set -u
out="$1"
sources="$2"
export RUSTUP_TOOLCHAIN=nightly-2025-11-08
export HOME="$out/home" CARGO_HOME="$out/cargo-home" RUSTUP_HOME="$out/rustup"
export PATH="$CARGO_HOME/bin:$out/fstar-src/fstar/out/bin:$out/z3/bin:$PATH"
eval "$(opam env --switch=hax)"
export HAX_HOME="$out/hax" HAX_PROOF_LIBS_HOME="$out/hax/proof-libs/fstar"
export HAX_LIBS_HOME="$out/hax/hax-lib/proofs/fstar/extraction"
export FSTAR_HOME="$out/fstar-src/fstar/out"
export WORKSPACE_ROOT="$out"
mkdir -p "$out/work"
for tree in upstream shipped; do
  work="$out/work/$tree"
  mkdir -p "$work"
  cp -a "$sources/$tree/." "$work/"
  cp "$out/shipped.Cargo.lock" "$work/Cargo.lock"
  cd "$work" || exit 1
  mkdir -p "$work/archived-models"
  cp -a proofs/fstar/extraction proofs/fstar/extraction-panic-free "$work/archived-models/"
  cargo metadata --locked --format-version 1 > "$work/metadata-locked.log" 2>&1
  echo "$?" > "$work/metadata-locked.exit"
  # Extend the copied shipped lock for Bertie's workspace/dev targets, never
  # resolve from an empty lock. Audit the retained library versions afterwards.
  cargo metadata --format-version 1 > "$work/metadata.json" 2> "$work/metadata.log"
  echo "$?" > "$work/metadata.exit"
  python3 "$out/audit-lock.py" "$out/shipped.Cargo.lock" "$work/Cargo.lock" > "$work/lock-audit.json" 2> "$work/lock-audit.log"
  audit=$?
  echo "$audit" > "$work/lock-audit.exit"
  if [ "$audit" != 0 ]; then
    echo 'Lock extension changed shipped dependency versions; extraction skipped' > "$work/extract.log"
    echo 125 > "$work/extract.exit"
  else
    find proofs/fstar/extraction -maxdepth 1 -type f \( -name '*.fst' -o -name '*.fsti' \) -delete
    python3 hax-driver.py extract-fstar > "$work/extract.log" 2>&1
    echo "$?" > "$work/extract.exit"
  fi
  mkdir -p "$work/fresh"
  # Never mistake the tracked extraction for a fresh extraction on failure.
  if [ "$(cat "$work/extract.exit")" = 0 ]; then
    cp proofs/fstar/extraction/*.fst* "$work/fresh/"
  fi
  (cd proofs/fstar && bash patches.sh apply) > "$work/patches.log" 2>&1
  echo "$?" > "$work/patches.exit"
  # Restore the explicitly labelled archived snapshots on failed extraction/patch.
  # They remain uncredited models and cannot populate fresh/.
  if [ "$(cat "$work/extract.exit")" != 0 ]; then
    cp -a "$work/archived-models/extraction/." proofs/fstar/extraction/
  fi
  if [ "$(cat "$work/patches.exit")" != 0 ]; then
    cp -a "$work/archived-models/extraction-panic-free/." proofs/fstar/extraction-panic-free/
  fi
  # Both commands are attempted without lax/admit even if extraction failed.
  python3 hax-driver.py typecheck --clean > "$work/typecheck-extraction.log" 2>&1
  echo "$?" > "$work/typecheck-extraction.exit"
  make -C proofs/fstar/extraction-panic-free clean > "$work/panic-free-clean.log" 2>&1
  make -C proofs/fstar/extraction-panic-free > "$work/typecheck-panic-free.log" 2>&1
  echo "$?" > "$work/typecheck-panic-free.exit"
done
rustc -vV > "$out/rust-version.log" 2>&1
cargo hax --version > "$out/hax-version.log" 2>&1
fstar.exe --version > "$out/fstar-version.log" 2>&1
z3 --version > "$out/z3-version.log" 2>&1
