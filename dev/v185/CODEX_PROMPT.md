# formal-coverage v1.8.5 — numbered hax impl methods (M1 + M2)

Repo: this directory (~/src/formal-coverage), branch `main`. `dev/v185/PLAN.md` is authoritative;
read it fully first, then `src/fstar.rs` (`fn mapped` and its tests) and `dev/v184/replay.py`.

## Build
- M1, then M2, exactly as PLAN.md describes. One or more small commits per milestone, messages
  prefixed `v185 M1:` / `v185 M2:`.
- Inputs for M2 are already staged in `target/v185-inputs/` (gitignored). Don't regenerate the F*
  evidence and don't run hax or F*; replay against the staged receipt. If a staged input looks wrong,
  stop and write it up in NOTES.md.

## Non-negotiables
- PLAN.md's non-negotiables and rule, as written. In particular: no source-order check, no matching by
  method name across types, any doubt → unmapped with a visible reason.
- No toolchain pin/override (RUSTUP_TOOLCHAIN unset), no `#[ignore]`, no weakened or edited existing
  assertions, never `--no-verify`. No push, no tag. Never modify anything outside this repo.
- If replay check (b) or (c) fails — any bertie unit changes in a way the rule doesn't explain, or any
  unit loses a tier — stop and write it up in NOTES.md and compare.md; don't adjust the rule to fit.

## Acceptance (record each exit code, timestamped, in dev/v185/PROGRESS.md)
1. `cargo test --no-fail-fast` → 0
2. `cargo clippy --all-targets -- -D warnings` → 0
3. `cargo fmt --check` → 0
4. the local CI gate that `.git/hooks/pre-push` runs (run that command; don't write its absolute path into committed files) → 0
5. `python3 dev/v185/replay.py` from an empty `target/v185` → exit 0, every check PASS
6. `target/debug/formal-coverage --version` → `formal-coverage 1.8.5`

Keep dev/v185/PROGRESS.md (milestone checklist) and dev/v185/NOTES.md (decisions, blockers,
timestamped). Resolve ambiguity by choosing reasonably and logging it; don't stop to ask.
