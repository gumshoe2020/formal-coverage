# usage: v171-metadata.sh TREE. Offline: resolve the workspace and dump the closure/feature views.
set -u
tree="$1"; ws="$V171_OUT/$tree/ws"; res="$V171_OUT/$tree/resolve"
mkdir -p "$res"; cd "$ws" || exit 1
cp -n Cargo.lock "$V171_OUT/$tree/shipped-input.Cargo.lock"
proof='--no-default-features --features defensive,std'
cargo metadata --offline --format-version 1 > "$res/metadata.json" 2> "$res/metadata.log"; echo "$?" > "$res/metadata.exit"
cargo tree --offline -p bertie $proof -e normal,build --prefix none --format '{p}' > "$res/closure-proof-features.txt" 2> "$res/closure-proof-features.log"; echo "$?" > "$res/closure-proof-features.exit"
RUSTFLAGS='--cfg hax' cargo tree --offline -p bertie $proof -e normal,build --prefix none --format '{p}' > "$res/closure-proof-features-hax-cfg.txt" 2> "$res/closure-proof-features-hax-cfg.log"; echo "$?" > "$res/closure-proof-features-hax-cfg.exit"
RUSTFLAGS='--cfg hax' cargo tree --offline -p bertie -e normal,build --prefix none --format '{p}' > "$res/closure-default-features-hax-cfg.txt" 2> "$res/closure-default-features-hax-cfg.log"; echo "$?" > "$res/closure-default-features-hax-cfg.exit"
cargo tree --offline -p bertie -e normal,build --prefix none --format '{p}' > "$res/closure-default-features.txt" 2> "$res/closure-default-features.log"; echo "$?" > "$res/closure-default-features.exit"
cargo tree --offline -p bertie $proof -e features,normal,build > "$res/features-proof.txt" 2> "$res/features-proof.log"; echo "$?" > "$res/features-proof.exit"
cargo tree --offline -p scratch-server -e features,normal,build -i bertie > "$res/features-shipped.txt" 2> "$res/features-shipped.log"; echo "$?" > "$res/features-shipped.exit"
cargo tree --offline -p scratch-server -e normal,build --prefix none --format '{p}' > "$res/closure-scratch-server.txt" 2> "$res/closure-scratch-server.log"; echo "$?" > "$res/closure-scratch-server.exit"
