# usage: v171-rootlock.sh TREE  (container; let cargo derive the lock for Bertie's standalone report root, offline)
tree="$1"; root="$V171_OUT/$tree/root"
export RUSTUP_TOOLCHAIN=1.98.0 RUSTUP_HOME=/usr/local/rustup CARGO_NET_OFFLINE=true
cd "$root"; cp Cargo.lock "$V171_OUT/$tree/root-lock.workspace.Cargo.lock"
cargo metadata --offline --format-version 1 > /dev/null 2> "$V171_OUT/$tree/root-lock.err"; echo "$?" > "$V171_OUT/$tree/root-lock.exit"
cp Cargo.lock "$V171_OUT/$tree/root-lock.derived.Cargo.lock"
