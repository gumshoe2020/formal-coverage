#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
mkdir -p target/v181/gate-audit/src logs/v1.8.1
cat > target/v181/gate-audit/Cargo.toml <<'TOML'
[package]
name = "v181-gate-audit"
version = "0.0.0"
edition = "2021"
[workspace]
[dependencies]
formal-coverage = { path = "../../.." }
serde_json = "1"
TOML
cp dev/v181/gate-audit.rs target/v181/gate-audit/src/main.rs
cp Cargo.lock target/v181/gate-audit/Cargo.lock
for mode in separate runtime; do
    cargo run --offline --manifest-path target/v181/gate-audit/Cargo.toml --target-dir target/v181/gate-build -- \
        "target/v181/bertie-$mode-before.json" "target/v181/bertie-$mode-after.json" "target/v181/bertie-$mode-no-patches.json" \
        > "logs/v1.8.1/m3-gates-$mode.log" 2>&1
done
