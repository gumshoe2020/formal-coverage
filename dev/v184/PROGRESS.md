# v1.8.4 M0 progress

- [x] Read M0 spec; confirmed main and default stable Rust/Cargo 1.100.0 with RUSTUP_TOOLCHAIN unset.
- [x] Replace the failing test's layout assumption with discovery of other rmeta artifacts and adjacent dep-info paths; preserve every assertion.
- [x] `cargo test --no-fail-fast` — exit 0.
- [x] `cargo clippy --all-targets -- -D warnings` — exit 0.
- [x] `cargo fmt --check` — exit 0.
- [x] `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` — exit 0.
- [x] Reviewed M0 diff; this log is included in the single M0 commit on main. No push.

At the M0 handoff, only the named test and these audit notes were in scope; M1/M2 were unstarted.

Validation uses the active default stable Rust/Cargo 1.100.0 with RUSTUP_TOOLCHAIN unset. Focused regression test also passed (exit 0). These are Codex-run checks; independent acceptance by Andrew/Dudley remains outstanding.

# v1.8.4 M1 progress

- [x] Read authoritative Rule, Non-negotiables, Milestones; M1 only.
- [x] Add optional schema-3 `[ship]` with package validation and a required non-empty reason.
- [x] Reuse dependency artifact/dep-info machinery for first-party file scope.
- [x] Keep excluded source files visible in JSON and markdown, outside coverage/evidence inputs.
- [x] Fixture acceptance: normal closure, bins, features, fallbacks, proven/proof totals, config errors.
- [x] `cargo test --no-fail-fast` — exit 0 (`/tmp/formal-coverage-v184-test.log`).
- [x] `cargo clippy --all-targets -- -D warnings` — exit 0 (`/tmp/formal-coverage-v184-clippy.log`).
- [x] `cargo fmt --check` — exit 0 (`/tmp/formal-coverage-v184-fmt.log`).
- [x] `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` — exit 0 (`/tmp/formal-coverage-v184-ci.log`, all checks passed).
- [x] No-`[ship]` CLI identity against c944fd4: JSON and markdown both byte-identical; both `diff -u` commands exit 0.
- [x] Reviewed implementation and audit diff; committed on main (332fc8a implementation/tests, followed by audit notes). Normal pre-commit hooks passed; no push. Independent acceptance remains outstanding.

## M1 no-`[ship]` identity commands (repository root)

RUSTUP_TOOLCHAIN is unset; rustc/cargo 1.100.0. `examples/scratch-server` contains only a config, so both binaries analyze the same existing archived scratch-server workspace with that exact config. All successful commands below returned exit 0; both diffs produced no output.

```sh
git worktree add --detach /tmp/formal-coverage-v184-before c944fd4
cargo build --manifest-path /tmp/formal-coverage-v184-before/Cargo.toml
/tmp/formal-coverage-v184-before/target/debug/formal-coverage report "$PWD/target/v1.5-scratch/spec-lemmas" --config "$PWD/examples/scratch-server/formal-coverage.toml" --json /tmp/formal-coverage-v184-before.json --markdown /tmp/formal-coverage-v184-before.md > /tmp/formal-coverage-v184-before.stdout
target/debug/formal-coverage report "$PWD/target/v1.5-scratch/spec-lemmas" --config "$PWD/examples/scratch-server/formal-coverage.toml" --json /tmp/formal-coverage-v184-after.json --markdown /tmp/formal-coverage-v184-after.md > /tmp/formal-coverage-v184-after.stdout
diff -u /tmp/formal-coverage-v184-before.json /tmp/formal-coverage-v184-after.json
diff -u /tmp/formal-coverage-v184-before.md /tmp/formal-coverage-v184-after.md
```

The baseline binary was initially built via `cargo run --manifest-path /tmp/formal-coverage-v184-before/Cargo.toml -- report ...`; its first literal examples directory run failed (exit 1, missing entry point), then the corrected archived-workspace command above succeeded (exit 0). Before-code build and final after-code build both completed (exit 0). No output normalization or schema-field removal was used.

# v1.8.4 M2 progress

- [x] Read M2/Expected; main, stable rustc 1.100.0, RUSTUP_TOOLCHAIN unset.
- [x] Rebuild v1.8.3 baseline and candidate; all 16 no-ship control pairs pass byte-exact JSON and Markdown equality.
- [x] Bertie no-ship exact JSON + Markdown equality in both modes; original receipt hashes and accepted v183 tag baseline verified.
- [x] Bertie ship inventory, retained-unit identity, totals and compare.md (completed by M1b replay below).
- [x] README ship documentation and parsing regression; Cargo.toml/Cargo.lock version 1.8.4.
- [x] Acceptance commands and exit codes; normal commits on main, no push/tag (M1b final results below).

## M2 acceptance commands (current stable 1.100.0; RUSTUP_TOOLCHAIN unset)

- `cargo test --no-fail-fast` — exit 0 (`target/v184-test.log`).
- `cargo clippy --all-targets -- -D warnings` — exit 0 (`target/v184-clippy.log`).
- `cargo fmt --check` — exit 0 (`target/v184-fmt.log`).

Focused `cargo test --test readme` — exit 0. Initial fmt check returned 1 for wrapping in the new test; `cargo fmt` corrected it before the successful acceptance run. No src behavior changes or ignores added. Independent acceptance remains Andrew/Dudley’s.

- `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` — exit 0 (`target/v184-ci.log`, all checks passed).
- `env PATH="$PWD/target/debug:$PATH" formal-coverage --version` — exit 0, prints `formal-coverage 1.8.4`. Local build selected explicitly; no global installation modified.

- `python3 dev/v184/controls.py` — exit 0; all 16 pairs PASS.

## Previous M2 STOP — acceptance 1c failed (superseded by M1b below)

- `python3 dev/v184/replay.py` final clean run — exit 143 (intentionally stopped on retained-unit finding). **Not PASS.**
- Direct retained-unit identity assertion on separate-mode reports — exit 1: seven retained units (81 security runtime SLoC) change `tested` → `unverified`.
- Dropped inventory verifies 324 security runtime SLoC (303 unverified, 21 tested), but observed tested/unverified totals also include the forbidden retained-tier shift.
- `dev/v184/compare.md` records the failure, all dropped files, changed retained units, and artifact hashes. No successful full comparison/reproduction is claimed.
- README/version/parsing test committed as `bf0a590`; its normal pre-commit hook exited 0. Rust acceptance gates and version command above passed.
- No src repair, push, tag, toolchain override, new ignore, or hook bypass. Existing ignored performance acceptance was not run. Independent acceptance remains outstanding; M2 is incomplete.

## 2026-11-24T19:16:00.077105+00:00 — M1b/M2 fresh acceptance

RUSTUP_TOOLCHAIN unset; all commands run by Codex. Independent acceptance remains outstanding.

- `cargo test --no-fail-fast` — exit 0 (`target/v184-m1b-test.log`).
- `cargo clippy --all-targets -- -D warnings` — exit 0 (`target/v184-m1b-clippy.log`).
- `cargo fmt --check` — exit 0 (`target/v184-m1b-fmt.log`).
- `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` — exit 0 (`target/v184-m1b-ci.log`).
- `target/debug/formal-coverage --version` — exit 0 (`target/v184-m1b-version.log`).

## 2026-11-24T19:18:39.207372+00:00 — final M1b gates after additional proof regression

Added a non-shipped Kani harness regression and a valid dev-dependency for the member integration test. All seven ship tests pass (exit 0). The evidence fixture itself also passes `cargo test --workspace --offline` (exit 0; `target/v184-m1b-fixture-runtime.log`). Repeating acceptance on the final tests; source is unchanged from the running clean replay. RUSTUP_TOOLCHAIN remains unset.

- `cargo test --no-fail-fast` — exit 0 (`target/v184-m1b-final-test.log`).
- `cargo clippy --all-targets -- -D warnings` — exit 0 (`target/v184-m1b-final-clippy.log`).
- `cargo fmt --check` — exit 0 (`target/v184-m1b-final-fmt.log`).
- `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` — exit 0 (`target/v184-m1b-final-ci.log`).
- `target/debug/formal-coverage --version` — exit 0 (`target/v184-m1b-final-version.log`).

- Repair/tests/README committed on main as `12f83c3`; normal pre-commit hook — exit 0. Clean M2 replay remains running.

## 2026-11-24T19:57:55.794239+00:00 — replay interim results

- Bertie no-ship exact JSON + Markdown equality — PASS in separate and runtime modes.
- Bertie shipped separate retained-unit full identity — PASS; 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078 (74.36%).
- Full replay exit code and remaining controls/runtime ship comparison are pending.

## 2026-11-24T20:02:01.767558+00:00 — M1b/M2 final replay acceptance

- `python3 dev/v184/replay.py` — exit 0 (`target/v184-m1b-replay.log`); runner recreated `target/v184` from empty before rebuilding/replaying. RUSTUP_TOOLCHAIN unset throughout.
- All 16 control pairs: exact JSON + Markdown equality PASS.
- Bertie no-ship exact JSON + Markdown equality, original CI/tag baseline, accepted evidence, retained-unit identity (all 448 entries), dropped would-be units, metadata, dependencies, third-party totals and arithmetic: PASS in both ghost modes.
- Separate shipped: 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078 (74.36%). Runtime shipped: 4091 proven-extracted, 447 tested, 792 unverified, 118 trusted / 5448 (75.09%).
- Removed runtime: 303 unverified + 21 tested = 324 SLoC in both modes. All 16 excluded files remain listed in Not shipped.
- `target/debug/formal-coverage --version` — exit 0, `formal-coverage 1.8.4`; final test/clippy/fmt/CI exits above are all 0.
- Success report regenerated as `dev/v184/compare.md`; previous failure detail remains in `dev/v184/NOTES.md`. Archived Bertie config restored byte-for-byte.
- M1b repair committed as `12f83c3`; audit/report commit follows on main using the normal hook. No push/tag, toolchain pin/override, new ignore, existing assertion changes, or hook bypass. Existing untracked run logs are excluded from commits.

M1b implementation and M2 replay are complete for Codex-run checks. Independent Andrew/Dudley acceptance is still outstanding; no new F*/Verus/Kani verifier execution is claimed.
