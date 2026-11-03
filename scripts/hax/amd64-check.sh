set -u
out="$1"
fstar_bin="${2:-$out/fstar-bin}"
export HOME="$out/home" PATH="$fstar_bin/fstar/bin:$fstar_bin/fstar/lib/fstar/z3-4.13.3/bin:$PATH"
export FSTAR_HOME="$fstar_bin/fstar"
export WORKSPACE_ROOT="$out" HAX_HOME="$out/hax" HAX_PROOF_LIBS_HOME="$out/hax/proof-libs/fstar" HAX_LIBS_HOME="$out/hax/hax-lib/proofs/fstar/extraction"
find "$fstar_bin" -name '*z3*' -type f > "$out/z3-paths.log"
fstar.exe --version > "$out/fstar-version.log" 2>&1
for tree in upstream shipped; do
  cd "$out/work/$tree" || exit 1
  for kind in extraction panic-free; do
    if [ ! -f "typecheck-native-$kind.log" ]; then
      cp "typecheck-$kind.log" "typecheck-native-$kind.log"
      cp "typecheck-$kind.exit" "typecheck-native-$kind.exit"
    fi
  done
  # `make clean`'s `rm *.fst` deletes exactly the root files `make verify`
  # needs; PROOFS.md documents plain `./hax-driver.py typecheck`, with no
  # --clean, run against the restored archived/extracted .fst snapshot. A
  # --clean chained straight into verify leaves zero ROOTS and "succeeds"
  # by checking nothing (caught once in this run; do not repeat it).
  rm -rf proofs/fstar/extraction/.cache/* proofs/fstar/extraction-panic-free/.cache/*
  cp -a archived-models/extraction/. proofs/fstar/extraction/
  cp -a archived-models/extraction-panic-free/. proofs/fstar/extraction-panic-free/
  timeout 1800 python3 hax-driver.py typecheck > typecheck-extraction.log 2>&1
  echo "$?" > typecheck-extraction.exit
  timeout 1800 make -C proofs/fstar/extraction-panic-free > typecheck-panic-free.log 2>&1
  echo "$?" > typecheck-panic-free.exit
done
z3 --version > "$out/z3-version.log" 2>&1
