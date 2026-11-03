# usage: v171-toolbuild.sh  (container, hax image; builds this repository's tool from a git-archive export, offline, pinned toolchain)
export RUSTUP_TOOLCHAIN=1.98.0 RUSTUP_HOME=/usr/local/rustup CARGO_NET_OFFLINE=true
export CARGO_HOME="$V171_OUT/tool-cargo"
[ -d "$CARGO_HOME" ] || cp -a "$V171_TOOLCHAIN/tool-cargo" "$CARGO_HOME"
cd "$V171_SOURCES/tool"
cargo build --locked --offline --release --target-dir "$V171_OUT/tool-target" > "$V171_OUT/tool-build.log" 2>&1
echo "$?" > "$V171_OUT/tool-build.exit"
sha256sum "$V171_OUT/tool-target/release/formal-coverage" > "$V171_OUT/tool-binary.sha256"
