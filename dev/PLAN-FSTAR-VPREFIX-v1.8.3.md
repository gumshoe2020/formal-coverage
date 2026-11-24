# formal-coverage v1.8.3 — map hax `v_`-prefixed F* names back to Rust units

Approved by Andrew 2026-11-24 ("Yes, please" to speccing the fix). Version 1.8.2 → 1.8.3.
Report schema may gain fields; existing fields keep their meaning.

## Background (verified against the bertie-tls main c66a986 coverage artifact)

hax renames a Rust value whose name starts with an uppercase letter by prefixing `v_`, because
F* requires lowercase-initial `let` names. Rust `fn XTR` in
`bertie_tls::tls13keyscheduler::key_schedule` extracts as
`Bertie_tls.Tls13keyscheduler.Key_schedule.v_XTR`; Rust `const fn U8` in `tls13utils` extracts
as `Bertie_tls.Tls13utils.v_U8`.

`mapped()` in `src/fstar.rs` (~line 641) builds `<module lowercased>::<fstar name>` and then tries
only the `impl_<Type>__<method>` form added in v1.8.2. It never strips `v_`, so these definitions
are `unmapped` and their Rust units keep their non-F* status even though F* checks them.

In the c66a986 artifact (`target/v183/ci-c66a986/report.json`), 5 `.fst` definitions are
`unmapped` with a `v_` prefix: `v_XPD`, `v_XTR`, `v_U8`, `v_U16`, `v_U32`. Their Rust units:

| Unit | runtime SLoC | status today |
| --- | ---: | --- |
| key_schedule::XPD (key_schedule.rs:295) | 69 | unverified |
| key_schedule::XTR (key_schedule.rs:376) | 57 | unverified |
| tls13utils::U8 | 5 | tested |
| tls13utils::U16 | 5 | tested |
| tls13utils::U32 | 5 | unverified |

`Bertie_tls.Tls13keyscheduler.Key_schedule` is a strict root module ("Verified module" in its F*
log, listed in the run's `verified_files`; the local extraction of it has no `push-options`,
`admit` or `lax`). Note `tls13utils` also has `pub struct U8` (hax: `t_U8`) beside `const fn U8`;
the struct is a type, not a `v_` value, but the uniqueness rule below must still hold.

The artifact (report.json, report.md, F* logs incl. `fstar/fstar.log.evidence.json`) is at
`target/v183/ci-c66a986/`. Use it to check that your baseline replay matches CI before changing code.

## Non-negotiables
1. Fail toward unmapped. Anything the rule below doesn't positively identify maps exactly as today.
2. Direct match first. The existing direct id lookup and the v1.8.2 `impl_` rule run unchanged and
   win; the `v_` rule is tried only when they find nothing.
3. Unique or nothing. The stripped name must match exactly one unit, as `mapped()` already requires;
   zero or several → unmapped.
4. A newly mapped unit is classified by its F* evidence like any other unit (fresh, has body, module
   success, contract kind, trust sites). It is not automatically proven.
5. Only bertie-tls F* statuses and the totals derived from them may change. Verus, scratch-server,
   Vest and fixture figures must not change except where a test fixture is added for this release.

## Rule

**R1 (`v_` prefix).** If the direct id and the `impl_` rule find nothing, and the F* name is
`v_<Rest>` where `<Rest>` is non-empty and its first character is an ASCII uppercase letter, try
`<module path>::<Rest>`. Apply the same strip to the method part of the `impl_` form
(`impl_<Type>__v_<Method>` → `<module path>::<Type>::<Method>`, same uppercase condition).
Do not strip `v_` when the remainder starts lowercase, a digit or `_`.

Before implementing, confirm the hax naming rule from hax's own source or docs (record the
reference in NOTES). If hax also prefixes in cases R1 doesn't cover, log them in NOTES and leave them
unmapped; do not widen R1 in this release.

## Milestones

**M1 — R1 with tests** (`tests/fstar_evidence.rs` or the `mapped()` unit tests in `src/fstar.rs`):
- `v_XTR` maps to `crate::m::XTR`
- `v_xtr`, `v_`, `v_1A`, `v__A` stay unmapped
- direct id `crate::m::v_Foo` present → direct wins (`v_Foo` maps to it, not to `Foo`)
- two units with id `crate::m::Foo` → `v_Foo` unmapped
- `impl_Foo__v_Bar` maps to `crate::m::Foo::Bar`; `impl_Foo__bar` still maps as in v1.8.2
- a mapped unit inside a non-verified (lax/failed) module is not proven

**M2 — Bertie replay and docs.**
- Copy bertie-tls at c66a986 to `target/v183/bertie` with
  `git -C ~/src/bertie-tls-flake-check archive c66a986 | tar -x -C target/v183/bertie`
  (no worktrees; never modify that checkout).
- F* evidence: reuse the CI F* log/evidence in `target/v183/ci-c66a986/fstar/` if the tool can be
  pointed at it. Only if it can't, run `python3 scripts/verify-fstar.py` in the copy (Docker image
  `fc-verify-fstar-amd64:v17`; redirect to a log, read `tail -n 30`) and log why.
- Baseline check: v1.8.2 (tag `v1.8.2`, built into `target/v183/baseline`) on the copy must
  reproduce `target/v183/ci-c66a986/report.json` unit statuses and the proven/tested/unverified
  totals exactly (proven 3,635 of 5,402 security lines, 67.29%). If not, HONEST-FINDING.
- Candidate run on the same inputs. Write `dev/v183/compare.md`: every unit whose status changed
  (id, old → new, SLoC, rule), proven totals before/after, and confirmation nothing else changed.
  Also list every `.fst` definition still `unmapped` after the change, grouped by apparent reason
  (trait impl, no Rust unit, generated helper, other); inventory only, no fixes.
- Run the existing fixture replays (adapt `dev/v182/replay.py` into `dev/v183/`) before/after and
  show no change.
- Docs: note the `v_` rule wherever docs describe F*-to-Rust name mapping (grep `impl_` in
  `docs/` and README); bump Cargo.toml to 1.8.3; CHANGELOG entry if the repo keeps one.

## Expected result (a guide, not a target)
XTR and XPD become proven-extracted (+126 lines, ~67.3% → ~69.6%); U8/U16/U32 may add up to 15
more. Do not tune the rule to hit a number.

## Out of scope
Trait-impl mapping; other unmapped names; changing anything in bertie-tls; any push, tag, release
bundle or public mirror step.
