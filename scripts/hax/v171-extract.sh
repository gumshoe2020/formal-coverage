# usage: v171-extract.sh TREE. hax-driver.py's exact `cargo hax` arguments, run from the workspace root.
# The only change is the rand_core pin: the workspace has three rand_core versions, and 0.9.5 is the one
# in Bertie's closure under every feature view (via rand 0.9.5).
set -u
tree="$1"; ws="$V171_OUT/$tree/ws"; res="$V171_OUT/$tree/extract"
mkdir -p "$res"; cd "$ws" || exit 1
export CARGO_NET_OFFLINE=true
snapshot() { find . -path ./target -prune -o -type f -print | LC_ALL=C sort | xargs sha256sum; }
snapshot > "$res/pre.sha256"
cargo hax --version > "$res/hax-version.log" 2>&1
cmd=(cargo hax -C -p bertie -p rand_core@0.9.5 --no-default-features --features defensive,std ';' into
  -i '-**::non_hax::** -bertie::stream::**' fstar
  --interfaces '+** +!bertie::tls13crypto::** +!bertie::tls13utils::** +!bertie::tls13cert::**')
printf '%q ' "${cmd[@]}" > "$res/command.txt"; echo >> "$res/command.txt"
"${cmd[@]}" > "$res/extract.log" 2>&1; echo "$?" > "$res/extract.exit"
snapshot > "$res/post.sha256"
