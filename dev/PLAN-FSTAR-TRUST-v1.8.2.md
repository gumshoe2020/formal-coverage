# formal-coverage v1.8.2 — F* credit for code F* actually checks

Approved by Andrew 2026-11-22 ("let's fix the tool"). Version 1.8.1 → 1.8.2. Report schema may gain
fields; existing fields keep their meaning.

## Background (verified against the bertie-tls main 9212e97 coverage artifact)

Two independent defects make the tool under-credit F*-verified Rust in bertie-tls:

**D1. ProverIf-only attributes demote F* status.** `src/fstar.rs` (`sites()`, ~line 303) marks any
unit containing the identifier `opaque`, `replace` or `replace_body` as `trusted`, whatever the
attribute's backend. In hax, `hax_lib::proverif::replace(...)`, `proverif::replace_body(...)` and
attributes wrapped in `cfg_attr(feature = "hax-pv", ...)` affect only the ProverIf extraction; the
F* extraction keeps and checks the real Rust body. Example: `handshake_data::to_two_inner`,
`to_four_inner` and `HandshakeData::len` have real F* bodies in a strict root module
(`Bertie_tls.Tls13formats.Handshake_data`, "Verified module") but report `trusted`. bertie-tls has
42 ProverIf replace sites in 9 files. Its F* extraction (`hax-driver.py`, `cargo_hax_into`) does NOT
enable feature `hax-pv`; only the ProverIf extraction does.

**D2. Inherent impl methods never map to F* definitions.** `mapped()` (~line 464) builds
`<module lowercased>::<fstar name>`. hax names inherent methods `impl_<Type>__<method>`
(e.g. `Bertie_tls.Tls13formats.Handshake_data.impl_HandshakeData__next_handshake_message`), while
the Rust unit id is `bertie_tls::tls13formats::handshake_data::HandshakeData::next_handshake_message`.
Result: 35 `impl_X__method` definitions are `unmapped` in bertie-tls, and no impl method in the
whole crate has ever been credited as proven (112 unverified, 21 trusted, 0 proven).

The artifact for 9212e97 (report.json, report.md, F* logs) is at
`target/v182/ci-9212e97/`. Use it to check your replay matches CI before changing code.

## Non-negotiables
1. Fail toward trusted. Anything the rules below don't positively identify as non-F* stays trusted
   exactly as today. Unconditional `hax_lib::opaque`, `hax_lib::fstar::replace`,
   `fstar::replace_body`, bare `opaque`/`replace` with an unrecognised path: unchanged.
2. Non-F* trust stays visible. ProverIf-only sites are still listed in the report with file, line,
   affected units and SLoC, in their own clearly named category, and counted separately in the
   markdown summary. They just stop setting a unit's status to `trusted`.
3. A unit freed from D1 is classified by its F* evidence like any other unit (fresh, has body,
   module success, contract kind). It is not automatically proven.
4. D2 mapping must be unique, as `mapped()` already requires; ambiguity → unmapped as today.
   Trait impls (`impl`, `impl_N`, `impl_N'`) stay out of scope.
5. Only bertie-tls F* statuses and the totals derived from them may change. Verus, scratch-server,
   Vest and fixture figures must not change except where a test fixture is added for this release.

## Rules

**R1 (backend path).** A trust identifier whose attribute path has a backend segment other than
`fstar` directly before it (`proverif`, `coq`, `ssprove`, e.g. `hax_lib::proverif::replace`,
`proverif::replace_body`) is non-F* trust for that backend.

**R2 (inactive feature).** New optional config:
```toml
[fstar.extraction]
inactive_features = ["hax-pv"]
reason = "..."   # required when inactive_features is non-empty; config error otherwise
```
A trust identifier inside `cfg_attr(feature = "X", ...)` with X listed is non-F* trust (kind
names the feature). Only the plain `feature = "X"` predicate qualifies; `all`/`any`/`not` or any
other predicate → trusted as today. No config → unchanged behaviour. The report records the
declared features and reason.

**R3 (scope attributes).** The impl/module-scope path (`u.scope_attributes.contains(&kind)`) must
apply R1/R2 the same way: an impl-level ProverIf-only attribute does not demote its methods; an
impl-level unconditional `hax_lib::opaque` still does.

**R4 (impl mapping).** In `mapped()`, if the direct id finds nothing and the name matches
`impl_<Type>__<method>` (split at the first `__` after the `impl_` prefix), try
`<module path>::<Type>::<method>`. Same uniqueness rule.

## Milestones

**M1 — D1 (R1-R3).** Tests in `tests/fstar_evidence.rs` (or a new test file), each asserting
unit status and trust-item category:
- `#[hax_lib::proverif::replace("...")]` on a fn → not trusted; listed as ProverIf trust
- `#[cfg_attr(feature = "hax-pv", proverif::replace("..."))]` → same (R1 alone suffices)
- `#[hax_lib::proverif::replace_body("(0)")]` on an impl method → same
- `#[cfg_attr(feature = "hax-pv", hax_lib::opaque)]` with `inactive_features = ["hax-pv"]` → not
  trusted; without the config → trusted; with `cfg_attr(all(feature = "hax-pv", test), ...)` → trusted
- unconditional `#[hax_lib::opaque]`, `#[hax_lib::fstar::replace(...)]` → trusted (unchanged)
- impl-level ProverIf replace → methods not demoted; impl-level `hax_lib::opaque` → demoted
- `inactive_features` without `reason` → config error

**M2 — D2 (R4).** Tests: `impl_Foo__bar` maps to `crate::m::Foo::bar`; ambiguous or missing stays
unmapped; `impl_1` stays unmapped.

**M3 — Bertie replay and docs.**
- Copy bertie-tls at 9212e97 to `target/v182/bertie` (from `~/src/bertie-der/bertie`:
  `git worktree add` is NOT allowed; use `git -C ~/src/bertie-der/bertie archive 9212e97 | tar -x -C target/v182/bertie`).
  Add `[fstar.extraction] inactive_features = ["hax-pv"]` with a reason citing hax-driver.py to the
  copy's formal-coverage.toml for the "after" run only.
- Run `python3 scripts/verify-fstar.py` in the copy (Docker image `fc-verify-fstar-amd64:v17` is
  present; output under the copy's `target/ci-logs/fstar`; redirect to a log, read `tail -n 30`).
- Baseline check: v1.8.1 (`git stash`/worktree of 7a60142 built into `target/v182/baseline`) on the
  copy must reproduce `target/v182/ci-9212e97/report.json` unit statuses exactly. If it doesn't,
  HONEST-FINDING.
- Candidate run on the same F* log. Write `dev/v182/compare.md`: every unit whose status or category
  changed (id, old → new status, SLoC, which rule), totals proven before/after, the list of ProverIf
  trust items, and confirmation that nothing else changed. Also run the existing fixture replays
  (`dev/v181/replay.py fixtures` pattern, adapted into `dev/v182/`) before/after and show no change.
- Docs: update `docs/evidence.md` (~line 548, the "explicit syntactic trust, including inactive
  cfgs" policy) and the config docs in README for `[fstar.extraction]`; bump Cargo.toml to 1.8.2.

## Expected result (a guide, not a target)
At least `to_two_inner`, `to_four_inner`, `HandshakeData::len` and most `HandshakeData` methods
become proven-extracted. Do not tune rules to hit a number.

## Out of scope
Trait-impl mapping; re-running the survey; changing anything in bertie-tls itself; any release,
push, tag or public mirror step.
