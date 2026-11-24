# formal-coverage v1.8.4 — M1b: not-shipped tests must still count as test evidence; then finish M2

Repo: this directory (~/src/formal-coverage), branch `main` (HEAD 8cce556). Read `dev/v184/PLAN.md`, then `dev/v184/compare.md` and the last NOTES.md entry: M2 replay stopped because `[ship]` changed 7 retained bertie units (81 SLoC) from tested → unverified.

## Diagnosis (confirmed by Dudley, 2026-11-24)
`src/classify.rs` (~line 300) runs `crate::ship::apply` before evidence. `src/ship.rs` removes not-shipped units and source files, so `src/evidence.rs` (~line 376) builds `test_identifiers` only from the remaining `[test]` units, and the `tested` assignment (~line 425) loses references from tests in not-shipped files (e.g. bertie `tests/test_tls13api.rs`, `simple_https_client/tests/*`, `src/test_tls13traces_internal.rs`).

## Decision (Dudley, logged; consistent with Andrew's rule "if they are not shipped as part of the binary, they should not count towards coverage either way")
`[ship]` decides which code is COUNTED. It never removes EVIDENCE. Test code never ships, so tests anywhere in the first-party inventory keep conferring `tested` on shipped runtime units exactly as without `[ship]`. Same for proof evidence. Not-shipped units still leave every total and stay listed in "Not shipped".

## Build
1. Fix in src/ so evidence (test identifiers; and any other evidence derived from the inventory) is computed from the full first-party inventory, while totals use only the shipped scope. Smallest change that does it; no change when `[ship]` is absent.
2. Regression test in `tests/ship.rs`: a test in a not-shipped location (a non-shipped member's `tests/`, and a `#[cfg(test)]` file) calls a shipped lib function → that function stays `tested` with `[ship]`, and its tier equals the no-`[ship]` tier.
3. General invariant test: for the fixture, every unit retained under `[ship]` has the same status, tier and SLoC as without `[ship]`.
4. Then rerun M2 from an empty `target/v184`: `python3 dev/v184/replay.py` (it already exists; keep RUSTUP_TOOLCHAIN unset). Expected (guide, verify): bertie shipped separate = 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078 (74.36%); retained-units identity PASS in both ghost modes; all controls and bertie no-ship exact equality PASS. Regenerate `dev/v184/compare.md` as a success report (keep the failure history in NOTES.md, not compare.md).
5. If README text describes ship scope affecting evidence, make it say the rule above in one sentence.

## Non-negotiables
- No toolchain pin/override, no `#[ignore]`, no weakened or edited existing assertions, never `--no-verify`. Small commits on `main`; no push, no tag. Never modify anything outside this repo.
- If retained-units identity still fails for any other reason, stop and write it up; don't normalize tiers.

## Acceptance (no RUSTUP_TOOLCHAIN; exit codes into dev/v184/PROGRESS.md)
1. `cargo test --no-fail-fast` → 0
2. `cargo clippy --all-targets -- -D warnings` → 0
3. `cargo fmt --check` → 0
4. `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` → 0
5. `python3 dev/v184/replay.py` from empty `target/v184` → exit 0, every check PASS.
6. `target/debug/formal-coverage --version` → 1.8.4

Keep PROGRESS.md/NOTES.md (timestamped). Resolve ambiguity by choosing reasonably and logging it; don't stop to ask.
