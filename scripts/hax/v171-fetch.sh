# Network step: download exactly the shipped lock's crates (no lock changes allowed).
set -eu
cd "$V171_SOURCES/scratch-server"
cargo fetch --locked
