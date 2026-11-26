# formal-coverage v1.11.0: Amendment A1 (follow-up to M1-M3)

Repo: this directory (~/src/formal-coverage), branch `main`. Read `dev/v1110/PLAN.md` in full,
especially the new section "Amendment A1" at the end, and `dev/v1110/NOTES.md` (the M3
HONEST-FINDING it resolves). M1-M3 are committed (9b13447, 62b49a5, e12f810).

## Build (one milestone, M4)
- Implement A1.1 (data-integer operands are values) with a unit test that covers a `u8` equality
  (functional) and a `usize` bound (safety-only, size-only).
- Implement A1.2's rename everywhere it appears, including tests and docs.
- Update `dev/v1110/replay.py`'s control table to A1's expectations and rerun it; regenerate
  `dev/v1110/compare.md`.
- Commit as `v1110 M4: amendment A1 (data-integer operands, own-code label)`, trailer
  `Assisted-by: <the model you actually are>`.

## Non-negotiables
Same as `dev/v1110/CODEX_PROMPT.md`: no tier/status/SLoC changes, proof ratio and proven share
identical to v1.10.0; no toolchain override, no `#[ignore]`, no `--no-verify`, no push, no tag,
nothing outside this repo; logs to files, read tails. Stop with `HONEST-FINDING` in NOTES.md if a
control still doesn't match A1, or a check other than (c) fails. Don't bend a rule to fit numbers.

## Acceptance (record exit codes, timestamped, in dev/v1110/PROGRESS.md)
1. `cargo test --no-fail-fast` → 0
2. `cargo clippy --all-targets -- -D warnings` → 0
3. `cargo fmt --check` → 0
4. the local CI gate `.git/hooks/pre-push` runs → 0
5. `python3 dev/v1110/replay.py` from an empty `target/v1110` (worktree may stay) → 0, all PASS
6. `target/debug/formal-coverage --version` → `formal-coverage 1.11.0`

When done, write `V1.11.0-READY` as the last line of dev/v1110/PROGRESS.md and stop.
