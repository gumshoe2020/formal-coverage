# usage: v171-patches.sh TREE  (inside the container; runs Bertie's own patches.sh apply on the fresh extraction)
tree="$1"; dir="$V171_OUT/$tree/ws/bertie/proofs/fstar"; res="$V171_OUT/$tree/patches"
mkdir -p "$res"; cd "$dir"
snapshot() { find extraction-lax extraction-panic-free -type f \( -name '*.fst' -o -name '*.fsti' \) | LC_ALL=C sort | xargs sha256sum; }
snapshot > "$res/pre.sha256"
command -v patch > "$res/patch-path.txt" 2>&1; patch --version 2>&1 | head -n 1 >> "$res/patch-path.txt"
bash patches.sh apply > "$res/apply.log" 2>&1; echo "$?" > "$res/apply.exit"
snapshot > "$res/post.sha256"
find . -name '*.rej' -o -name '*.orig' | LC_ALL=C sort > "$res/rejects.txt"
