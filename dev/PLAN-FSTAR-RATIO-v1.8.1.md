# PLAN: formal-coverage v1.8.1, `cfg(hax)` code counts as proof

Authoritative for the v1.8.1 build. Approved by Andrew 2026-11-21. Builds on v1.8
(`dev/PLAN-FSTAR-RATIO.md`, commits 218e190..e74b116); every v1.8 rule still holds unless
this plan says otherwise.

## Problem
Code gated by `cfg(hax)` exists only when cargo-hax extracts the crate for F*. It is never
compiled into the shipped binary. Today the tool inventories it as runtime code. In Bertie at
`f646099`, ~115 SLoC in `src/der.rs` (DER lemmas, `Prop` helpers, `pow256` facts) are
counted as proven-extracted *runtime*, inflating both the proven share and the ratio
denominator. Verus already solves this for `proof fn`/`spec fn`: those units are
`Category::ProofCode`, counted in `standalone_proof_sloc`, excluded from runtime totals.
v1.8.1 gives `cfg(hax)` code the same treatment.

## Rules (Andrew's decision, non-negotiable)
1. **Items.** An item (fn, const, struct, impl, mod, …) whose own `#[cfg(...)]` attributes
   or any enclosing item's `#[cfg(...)]` can only hold when `hax` is set becomes
   `Category::ProofCode`, in **both** ghost-line modes (as Verus proof fns are).
   "Can only hold when hax is set" = the same algebra as `cfg_implies_test` in `src/lib.rs`,
   with `hax` in place of `test`: `cfg(hax)`, `cfg(all(hax, …))`, `cfg(any(...))` only if every
   arm implies hax. `cfg(not(hax))`, `cfg_attr(hax, …)` and feature flags (`feature = "hax"`,
   `hax-pv`, …) do **not** qualify. Implement as one shared function with unit tests per case.
   Test classification takes precedence: anything the tool classifies as test today stays test.
2. **Statements/blocks inside runtime units.** A statement, expression statement or block
   inside a runtime fn whose own `#[cfg(...)]` implies hax (rule 1 algebra) is inline proof:
   its lines move from runtime to inline proof in `--ghost-lines separate` (default) and stay
   runtime in `runtime` mode, exactly like v1.8 hax annotations. Lines shared with real runtime
   code stay runtime. A line counted by both this rule and the v1.8 annotation rule counts once.
3. **Annotations inside ProofCode units** are part of that unit's standalone proof SLoC. They
   are not also counted in `fstar_annotation_sloc` (which stays runtime-units-only, per v1.8).
4. **Scope.** Rules 1–2 apply only to first-party crates the tool already treats as hax
   projects (whatever `src/hax.rs` / `src/fstar.rs` / config use to decide today; name the rule
   in NOTES). Non-hax crates: zero change.
5. **Status.** Follow the Verus proof-code convention. A `cfg(hax)` ProofCode unit that the
   accepted F* receipt would credit (today it would be `proven-extracted`) is reported
   `verified`, as a credited Verus proof fn is. Every other such unit keeps its current status.
   No change to receipt grammar, evidence acceptance, thresholds or the 5–15 range.
   If any `cfg(hax)` unit is `hazard`, or the proof-code hazard gate
   (`report.rs` "proof-code hazard can invalidate runtime proofs") would newly fail on Bertie,
   that is an **HONEST-FINDING**: stop.
6. `use` items gated by `cfg(hax)` are not units today; leave them uncounted.

## Allowed changes (everything else must be identical)
- Bertie: units matched by rule 1 move runtime → proof-code (category, and status per rule 5);
  lines matched by rule 2 move runtime → inline proof in separate mode. Report each.
- Bertie security runtime SLoC, proven-extracted SLoC, proven share, ratio and proof-code
  table change accordingly. No other Bertie unit may change status or size.
- scratch-server, Vest and every fixture except new ones: every report field identical to v1.8.0
  (schema included, unless you bump it; if you bump, justify in NOTES and say so in compare.md).

## Report
- Proof-code table and ratio show `cfg(hax)` proof-code with the existing proof-code fields;
  add `fstar_cfg_hax_sloc` (standalone, rule 1) and `fstar_cfg_hax_inline_sloc` (rule 2) to
  `proof_ratio` (overall, per-crate, per-module) so the parts stay visible. Markdown breakdown
  table gains these two columns.
- Version 1.8.1 in Cargo metadata. `docs/v1.8.md` gets a short v1.8.1 section; README updated
  where it describes what counts as proof.

## Milestones
- **M1 Classification.** Shared `cfg_implies_hax` + rule 1 + rule 5. Tests: plain, `all`,
  `any` (both arms / one arm), `not`, `cfg_attr`, feature flag, nested under `#[cfg(hax)] mod`,
  non-hax crate untouched, credited → `verified`, uncredited keeps status.
  Accept: Bertie replay lists every reclassified unit; an independent scanner
  (`dev/v181/cfghax.py`, separate code, not the tool's) finds the same unit set and the same
  SLoC per unit, or every difference is explained line by line.
- **M2 Inline statements.** Rule 2 + rule 3. Tests: cfg(hax) block, cfg(hax) statement,
  shared line stays runtime, overlap with annotation counted once, runtime mode unchanged.
  Accept: the independent scanner also counts rule-2 lines in Bertie and matches exactly.
- **M3 Replay and docs.** Same replay procedure and inputs as v1.8 M4 (`dev/v18/replay.py`,
  read-only `bertie-deps` at `f646099` copied under `target/`, existing receipt, temp config
  with the v1.8 `[fstar.proof]` section). Baseline = v1.8.0 at `e74b116`; candidate = v1.8.1.
  Write `dev/v181/compare.md`: before/after for Bertie (proven share security %, ratio, all
  components, every moved unit with old/new category/status/size, every changed size) and the
  identity checks for scratch-server, Vest and fixtures in both ghost modes.
  Also report Bertie's result **with `patches = []`** in the temp config (the stale upstream
  patches are being deleted from bertie-tls): figures must be identical to the patched run,
  since v1.8 counted those patches as 0.

## Out of scope
Changing Bertie or bertie-tls; any tier/receipt/threshold change; ProVerif/SSProve;
push, tag, release.
