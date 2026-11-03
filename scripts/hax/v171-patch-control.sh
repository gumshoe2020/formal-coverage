# usage: v171-patch-control.sh TREE  (container; apply the patches to the *committed* snapshot to separate hax drift from environment problems)
tree="$1"; res="$V171_OUT/$tree/patch-control"
rm -rf "$res"; mkdir -p "$res"; cp -r "$V171_SOURCES/$tree/proofs/fstar" "$res/fstar"
cd "$res/fstar"; bash patches.sh apply > "$res/apply.log" 2>&1; echo "$?" > "$res/apply.exit"
find extraction-lax extraction-panic-free -type f \( -name '*.fst' -o -name '*.fsti' \) | LC_ALL=C sort | xargs sha256sum > "$res/post.sha256"
