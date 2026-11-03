# usage: v171-typecheck.sh TREE TIER FSTAR_VERSION [JOBS]  (container, amd64; cwd becomes the receipt base)
# Runs one grammar-compliant command per module: fstar.exe [--include DIR]* FILE. No cache, lax, admit or hint flags.
tree="$1"; tier="$2"; version="$3"; jobs="${4:-3}"
base="$V171_OUT/$tree"; cd "$base"
F="$V171_TOOLCHAIN/fstar-bin-$version/fstar"; [ -d "$F" ] || F="$V171_TOOLCHAIN/fstar-bin/fstar"
export PATH="$F/bin:$F/lib/fstar/z3-4.13.3/bin:$PATH"
name="typecheck-$tier-$version"; part="$base/$name.parts"; rm -rf "$part"; mkdir -p "$part"
fstar.exe --version > "$base/$name.fstar-version.txt" 2>&1; z3 --version >> "$base/$name.fstar-version.txt" 2>&1
includes=(--include libs/rust_primitives --include libs/core --include libs/hax-lib --include libs/libcrux-models --include "proofs/fstar/$tier")
export PART="$part"; export TIMEOUT="${V171_TIMEOUT:-1500}"
printf '%s\n' "${includes[@]}" > "$part/includes.txt"
run_module() {
  file="$1"; key="$(basename "$file")"
  mapfile -t inc < "$PART/includes.txt"
  { echo "+ fstar.exe ${inc[*]} $file"; start=$(date +%s); timeout "$TIMEOUT" fstar.exe "${inc[@]}" "$file" 2>&1; code=$?; echo "exit=$code"; } > "$PART/$key.out" 2>&1
  code=$(tail -n 1 "$PART/$key.out" | sed 's/exit=//'); echo "$file	$code	$(( $(date +%s) - start ))" > "$PART/$key.result"
}
export -f run_module
ls proofs/fstar/$tier/*.fst | LC_ALL=C sort | xargs -P "$jobs" -I{} bash -c 'run_module {}'
: > "$base/$name.log"; : > "$base/$name.results.tsv"
for f in $(ls proofs/fstar/$tier/*.fst | LC_ALL=C sort); do
  cat "$part/$(basename $f).out" >> "$base/$name.log"; cat "$part/$(basename $f).result" >> "$base/$name.results.tsv"
done
