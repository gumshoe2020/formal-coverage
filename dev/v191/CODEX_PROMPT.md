# formal-coverage v1.9.1 — five more hax naming forms (M0 + M1 + M2)

Repo: this directory (~/src/formal-coverage), branch `main`. `dev/v191/PLAN.md` is authoritative;
read it fully first, then `src/fstar.rs` (`fn mapped`, the v1.8.5 numbered-impl rule and their
tests), `src/cfg.rs`, and `dev/v185/replay.py`.

## Build
- M0, M1, then M2, exactly as PLAN.md describes. Small commits per milestone, messages prefixed
  `v191 M0:` / `v191 M1:` / `v191 M2:`, trailer `Assisted-by: Codex (GPT-6.1)` or the model you
  actually are.
- Inputs are already staged in `target/v191-inputs/` (gitignored). Don't regenerate F* evidence and
  don't run hax. The only F* you run is M0's canary, on a copy under `target/v191/canary/`, using the
  pinned toolchain the staged bertie checkout's `scripts/verify-fstar.py` uses
  (the `BERTIE_DOCKER_TOOLS` toolchain directory), one module only.
- If a staged input looks wrong, stop and write it up in NOTES.md.

## Non-negotiables
- PLAN.md's rules and non-negotiables as written. Any doubt → unmapped with a visible reason.
- No toolchain pin/override (RUSTUP_TOOLCHAIN unset), no `#[ignore]`, no weakened or edited
  existing assertions, never `--no-verify`. No push, no tag. Never modify anything outside this
  repo; bertie-tls is input only.
- Stop with `HONEST-FINDING` in NOTES.md (and compare.md if it exists) if: M0 finds a form isn't
  what PLAN.md describes; replay check (b), (c) or (e) fails; or a bertie figure changes in a way the
  rules don't explain. Don't adjust a rule to fit the numbers.
- Logs go to files under `logs/` or `target/`; read only tails or grep.

## Acceptance (record each exit code, timestamped, in dev/v191/PROGRESS.md)
1. `cargo test --no-fail-fast` → 0
2. `cargo clippy --all-targets -- -D warnings` → 0
3. `cargo fmt --check` → 0
4. the local CI gate that `.git/hooks/pre-push` runs (run that command; don't write its absolute
   path into committed files) → 0
5. `python3 dev/v191/replay.py` from an empty `target/v191` → exit 0, every check PASS
6. `target/debug/formal-coverage --version` → `formal-coverage 1.9.1`

Keep dev/v191/PROGRESS.md (milestone checklist) and dev/v191/NOTES.md (decisions, M0 findings,
blockers, timestamped). Resolve other ambiguity by choosing reasonably and logging it. When done,
write `V1.9.1-READY` as the last line of PROGRESS.md and stop. Dudley checks "done" by rerunning
acceptance.
