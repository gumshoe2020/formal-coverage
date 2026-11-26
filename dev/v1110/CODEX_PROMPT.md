# formal-coverage v1.11.0: honest contract depth (M1 + M2 + M3)

Repo: this directory (~/src/formal-coverage), branch `main`. `dev/v1110/PLAN.md` is authoritative;
read it fully first, then `src/fstar.rs` (`fn contract`, its call site near line 1172 and the
`proof_scope` formatting near line 1235), `src/ratio.rs`, `src/report.rs`, `ci/coverage_section.py`,
`ci/coverage_metrics.py` and `dev/v191/replay.py`.

## Build
- M1, M2, then M3, exactly as PLAN.md describes. Small commits per milestone, messages prefixed
  `v1110 M1:` / `v1110 M2:` / `v1110 M3:`, trailer `Assisted-by: Codex (GPT-6.1)` or the model you
  actually are.
- Inputs are already staged in `target/v1110-inputs/` (gitignored). Don't regenerate F* evidence,
  don't run hax or F*. Build the v1.10.0 baseline binary for the replay from a `git worktree` of
  tag `v1.10.0` under `target/v1110/`.
- If a staged input looks wrong, stop and write it up in NOTES.md.

## Non-negotiables
- PLAN.md's rules as written. Any doubt → the lower classification (`safety-only`, reason
  `unparsed`), listed visibly. Never round up.
- No unit may change tier, status or SLoC; proven share and proof ratio must not change.
- No toolchain pin/override (RUSTUP_TOOLCHAIN unset), no `#[ignore]`, no weakened or edited
  existing assertions except where an assertion encodes the old `functional` heuristic (list each
  such edit in NOTES.md with the reason). Never `--no-verify`. No push, no tag. Never modify
  anything outside this repo; bertie-tls is input only.
- Stop with `HONEST-FINDING` in NOTES.md (and compare.md if it exists) if: a control doesn't match
  PLAN.md's table and the rules explain why; replay check (a), (b) or (d) fails; or a gate fails
  that the baseline (v1.10.0 at HEAD before your first commit) doesn't fail. Don't adjust a rule
  to fit the numbers.
- Logs go to files under `logs/` or `target/`; read only tails or grep.

## Acceptance (record each exit code, timestamped, in dev/v1110/PROGRESS.md)
1. `cargo test --no-fail-fast` → 0
2. `cargo clippy --all-targets -- -D warnings` → 0
3. `cargo fmt --check` → 0
4. the local CI gate that `.git/hooks/pre-push` runs (run that command; don't write its absolute
   path into committed files) → 0
5. `python3 dev/v1110/replay.py` from an empty `target/v1110` (keep the worktree if you like) →
   exit 0, every check PASS
6. `target/debug/formal-coverage --version` → `formal-coverage 1.11.0`

Keep dev/v1110/PROGRESS.md (milestone checklist) and dev/v1110/NOTES.md (decisions, findings,
blockers, timestamped). Resolve other ambiguity by choosing reasonably and logging it. When done,
write `V1.11.0-READY` as the last line of PROGRESS.md and stop. Dudley checks "done" by rerunning
acceptance.
