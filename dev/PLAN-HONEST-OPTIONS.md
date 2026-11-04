# PLAN v1.7.2 — the tool reports F* proofs honestly

## Background
Andrew (2026-11-03): "We do want the tool to be honest." The bertie-der pilot (`projects/bertie-der`,
REPORT.md "M5") found two places where v1.7.1 reports something false about a proof that F* accepted:

1. `unsafe_options()` (`src/fstar.rs`) accepts only `--fuel`, `--ifuel`, `--z3rlimit` in pragmas.
   `Bertie.Der.fst` has three `#push-options` with `--split_queries always`, so all 32 bodies in the
   module are classified `hazard` and the fix earns no credit, although F* checks it with 0 errors.
   `--split_queries` changes how a verification condition is sent to Z3 (one query per goal); every
   goal must still be proved, so it does not weaken soundness.
2. The declaration parser names every `let rec f` as `<Module>.rec` (11 times in `Bertie.Der.fst`),
   so recursive helpers show up as interface-only trusted values.

Honest means both directions: never credit what was not proved, never withhold credit for what was.
The fix stays fail-closed.

## Non-negotiables
- The option check stays a **whitelist**. Add exactly one option: `--split_queries` with value
  `always`, `on_failure` or `no`. Any other value, a missing value, or any other new option stays a hazard.
- Before adding it, quote the `--split_queries` description from F*'s own source or help text for
  the F* version in the toolchain (`runs/v1.7/bertie`, offline) into NOTES.md, with the path. If that
  text suggests it can weaken soundness, STOP and write `HONEST-FINDING` in PROGRESS.md.
- A rejected option must say which one: the escape kind names the rejected option text (e.g.
  `unsupported F* option: --foo bar`), not only the generic phrase. Checks fail visibly.
- Whole-file hazard on any unaccepted pragma stays as it is (conservative; out of scope).
- `admit`/`assume`/`--lax`/`--admit_smt_queries` detection is unchanged and still fires inside a
  region that also uses `--split_queries`.
- No release bundle, no push, no tag, no edits outside this repo except new files under
  `projects/bertie-der/evidence/m5-v1.7.2/`. Do not edit `projects/bertie-der` sources or scripts.
- Docker: Dudley's acceptance rerun is using Docker until `pgrep -f dudley-rerun-m5` is empty. Do
  M1-M2 first; do not start a container until that check is empty. No new F* typecheck runs:
  reuse the existing receipts/logs.

## Milestones
- **M1 Option whitelist.** Accept `--split_queries {always,on_failure,no}`; name the rejected option
  in the escape kind. Unit tests: the existing hazard list still flags; new negatives
  (`--split_queries`, `--split_queries maybe`, `--split_queries always --admit_smt_queries true`,
  `--split_queries always --lax`) flag; new positives (each valid value, combined with fuel/ifuel/
  z3rlimit) pass. Acceptance: `cargo test` green, `cargo clippy -- -D warnings` clean.
- **M2 `let rec` names.** `let rec f` (and `let rec f ... and g`) yields names `f` and `g`. Unit
  tests on literal snippets copied from `Bertie.Der.fst`. Acceptance: `cargo test` green.
- **M3 Regression on existing runs.** Re-run the report step only (no extraction, no typecheck) for
  `runs/v1.7.1/upstream` and `runs/v1.7.1/shipped` with the new binary. Every number that changes
  is listed in NOTES.md with the cause (expected: only `let rec` naming effects). Any change you can't
  explain is a finding, reported not fixed. If receipts bind the old tool binary hash, write down
  exactly how that is handled; do not edit receipts by hand.
- **M4 bertie-der re-measure.** Same report-only re-run on the bertie-der M5 evidence (`fix/ecdsa-der`
  vs `ef65f76`). Write `projects/bertie-der/evidence/m5-v1.7.2/SUMMARY.md`: the numbers table
  v1.7.1 vs v1.7.2 (proven-extracted SLoC and %, credited units, classification of each `der` unit),
  and whether the expectation (`der` credited proven-extracted with functional postconditions) now
  holds. If it does not, say why from the evidence; do not tune the tool to make it hold.

## Acceptance (Dudley reruns)
`cargo test`, `cargo clippy -- -D warnings`, the M3 and M4 commands from NOTES.md, and a check that
`git diff` touches only `src/`, tests, `dev/`, `docs/` if the option rule is documented there, and
`projects/bertie-der/evidence/m5-v1.7.2/`.
