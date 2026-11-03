# Synthetic hax/F* receipt fixtures

These small Rust and F* files test receipt policy. F* text and successful logs
are synthetic: no verifier execution is claimed. `tests/fstar_evidence.rs`
copies them under logs/v1.7/, builds receipts and exercises clean, lax, admit,
patch alteration, interface-only, stale Rust and self-consistency variants.
It also mutates every validation boundary and asserts zero credit.

Run `cargo test --frozen --test fstar_evidence`, redirecting the complete log
to logs/v1.7/. This carries receipt-policy acceptance when Bertie's toolchain
cannot complete; it is not evidence that F* checked Bertie or these fixtures.
