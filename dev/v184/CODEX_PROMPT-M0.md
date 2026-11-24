# formal-coverage v1.8.4 — M0 only: make every gate pass on current stable Rust

Repo: this directory (~/src/formal-coverage), branch `main`. `dev/v184/PLAN.md` is the v1.8.4 spec; do ONLY milestone M0 now. Do not start M1/M2.

## Problem (verified 2026-11-24 on rustc 1.100.0, the active default toolchain)
`cargo test --no-fail-fast` fails exactly one test:
`tests/size_modes.rs` `missing_dep_info_falls_back_to_whole_crate_for_that_dependency_only`, panicking at line 141
(`.find(|p| p.is_dir()).unwrap()` on `None`). The test hard-codes `target/formal-coverage/*/debug/deps/` to find the
`other-*.d` dep-info file; cargo 1.100 places dep-info elsewhere. The tool itself still finds dep-info: the test's first
assertion (`first.size_fallbacks.is_empty()`) passes. The tool derives the path in `src/size.rs` (~line 200) from the
artifact's `.rmeta` path: `rmeta.with_file_name(format!("{stem}.d"))`. All other tests, `cargo clippy --all-targets -- -D warnings`
and `cargo fmt --check` already pass.

## Task
Fix the TEST so it locates the `other` crate's `.d` file the same way the tool does (from cargo's reported artifact path,
or by searching under the fixture's target dir for the `.d` next to the `other` rmeta), not by a hard-coded layout.
Keep the test's intent and every existing assertion: exactly one `other` dep-info file is removed, `other` falls back to
whole-crate with reason starting "dep-info file missing", `dep` stays compiled-files, banner text unchanged.

## Non-negotiables
- Change test code only. If you conclude the tool itself is wrong on 1.100, stop and write why in dev/v184/NOTES.md; do not change src/.
- No toolchain pin, no `RUSTUP_TOOLCHAIN` override, no `rust-toolchain` file, no `#[ignore]`, no weakened assertions.
- Do not bump the version, edit docs/CHANGELOG, or touch other tests.
- Commit on `main` with a clear message (one commit). Do not push. Never use `--no-verify`.

## Acceptance (run all, with no RUSTUP_TOOLCHAIN set, and paste exit codes into dev/v184/PROGRESS.md)
1. `cargo test --no-fail-fast` → exit 0
2. `cargo clippy --all-targets -- -D warnings` → exit 0
3. `cargo fmt --check` → exit 0
4. `.git/hooks/pre-push` gate: `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` → exit 0

Keep `dev/v184/PROGRESS.md` (M0 checklist) and `dev/v184/NOTES.md` (decisions, timestamped). Resolve ambiguity by choosing reasonably and logging it; don't stop to ask.
