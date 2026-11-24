# formal-coverage v1.8.4 — M2: bertie-tls replay, README, version bump

Repo: this directory (~/src/formal-coverage), branch `main`. M0 (d40ebf4) and M1 (332fc8a, 9a23764) are done and accepted.
`dev/v184/PLAN.md` is authoritative; read the M2 bullet (incl. the README requirements) and "Expected". Do ONLY M2.

## Grounding (verified 2026-11-24)
- The v1.8.3 paired replay lives in `dev/v183/` (`replay.py`, `compare.py`, `controls.py`, `compare.md`); it reuses `scripts/v15_replay.py` and archives bertie `c66a986` under `target/v183/bertie` with the original CI log/receipt. Copy that pattern into `dev/v184/` (outputs under `target/v184/`). **Difference:** v183's `run()` forces `RUSTUP_TOOLCHAIN=1.97.1`. v1.8.4 must NOT set RUSTUP_TOOLCHAIN anywhere; everything runs on current stable (rustc 1.100.0).
- v1.8.3 CI baseline on bertie c66a986: 3635 proven-extracted, 452 tested, 1197 unverified, 118 trusted / 5402 security SLoC.
- bertie-tls `formal-coverage.toml` (at c66a986) has `[proof_ratio]`, `[fstar.proof]`, `[fstar.extraction]`; no `[ship]`. Shipped artifact: package `bertie-tls`, default features. Expected not shipped: `bogo_shim/`, `record/`, `simple_https_client/`, `simple_https_server/`, `integration_tests/`, `src/test_utils.rs` (~324 security SLoC: 303 unverified, 21 tested, 0 proven). Verify, don't assume.

## Build
1. **Replay** (baseline = tag `v1.8.3` binary, candidate = HEAD), both on stable 1.100:
   a. Controls without `[ship]` (all v183 controls: scratch, vest, fixtures; both ghost modes): exact JSON + markdown equality v1.8.3 vs candidate.
   b. Bertie c66a986 without `[ship]`: exact equality v1.8.3 vs candidate.
   c. Bertie c66a986 with `[ship] packages=["bertie-tls"], features=["default"], reason="..."` added to the ARCHIVED COPY's config only: list every dropped file with SLoC and would-be tier; prove every retained unit has identical tier and SLoC to (b); report new totals and proven %.
   Write `dev/v184/compare.md` like `dev/v183/compare.md`. If (a) or (b) differ, or (c) changes any retained unit, stop and write it up in NOTES.md; don't paper over it.
2. **README** per PLAN M2: new top section `## v1.8.4: code that doesn't ship isn't counted` (above v1.8.1) covering the rule, fail-toward-counting, and the "Not shipped" report section; document `[ship]` with an example in the `formal-coverage.toml` section (~line 196); fix any statement that all workspace members count as first-party; note gates run on current stable Rust. Add a test that every README TOML example containing `[ship]` parses, if none exists.
3. **Version**: bump `Cargo.toml` to `1.8.4` (and Cargo.lock). Don't tag, don't push.

## Non-negotiables
- Never modify bertie-tls itself or any checkout outside this repo; mutation only under `target/v184/`.
- No src/ behaviour changes in M2. If the replay exposes a bug, stop and write it up.
- No toolchain pin or override, no `#[ignore]`, never `--no-verify`. Small commits on `main`; no push, no tag.

## Acceptance (no RUSTUP_TOOLCHAIN; paste exit codes into dev/v184/PROGRESS.md)
1. `cargo test --no-fail-fast` → 0
2. `cargo clippy --all-targets -- -D warnings` → 0
3. `cargo fmt --check` → 0
4. `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` → 0
5. `python3 dev/v184/replay.py` reproduces `compare.md` from scratch; 1a/1b all PASS; 1c retained-units check PASS.
6. `formal-coverage --version` prints 1.8.4.

Keep `dev/v184/PROGRESS.md` (M2 checklist) and `dev/v184/NOTES.md` (timestamped). Resolve ambiguity by choosing reasonably and logging it; don't stop to ask.
