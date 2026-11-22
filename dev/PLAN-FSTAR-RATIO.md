# PLAN: formal-coverage v1.8, proof-to-code ratio for hax/F*

Authoritative for the v1.8 build. Approved by Andrew 2026-11-21.

## Problem
For hax/F* projects such as Bertie, `proof_ratio` is always `n/a`, for two reasons:
1. **Denominator.** `src/ratio.rs::metrics` counts only `status == "proven"` (and
   `proven-bounded` separately) as proven runtime. All of Bertie's credit is
   `proven-extracted`, so `proven_runtime_sloc` is 0 and the ratio is `None`.
2. **Numerator.** Proof lines are counted only for Verus: `Category::ProofCode` units and
   inline Verus ghost syntax (`src/ghost.rs`). hax/F* proof lives in other places
   (listed below), and none of it is counted.

## What counts (Andrew's decision, non-negotiable)
**Denominator:** proven runtime includes `proven-extracted` units. Report
`proven_extracted_runtime_sloc` as its own field next to `proven_runtime_sloc` and
`proven_bounded_runtime_sloc`, so the parts stay visible. The ratio divides by
proven + proven-extracted. `proven-bounded` keeps its current treatment.

**Counts as proof (numerator):**
- **A. Inline hax-lib annotations in Rust**, measured like inline Verus ghost lines: the
  token span of these attributes/macros, after doc-line exclusion (v1.7.3 rule):
  `hax_lib::requires`, `ensures`, `decreases`, `loop_invariant`, `fstar` (the attribute
  and the `fstar!` macro, plus `fstar::before`/`after`/`replace` style variants),
  `opaque`, `attributes`, `lemma`, `assert`, `assume`, `implies`, `forall`, `exists`,
  `int`/`prop` helpers, both `#[hax_lib::x]` and `#[hax::x]` path spellings and
  `use hax_lib as …` aliases where the code uses them. Implement this as a **whitelist** of
  paths in one place, with a test per entry. In `--ghost-lines separate` mode
  (default) these lines move from the unit's runtime size to inline proof, exactly as
  Verus ghost lines do; in `runtime` mode they stay runtime. Count them only for runtime
  units, under the same first-party/security filters as today's ratio.
  `assume` counts as proof lines but must ALSO keep (or get) the existing escape-hatch
  or trusted treatment; counting it as proof must never raise a tier.
- **B. Hand-written edits to verified extraction.** Added lines (`+` lines, excluding
  `+++` headers, blank lines and comment-only lines) in patch files listed in config,
  **only** for hunks whose target file is a module the accepted F* receipt verified
  strictly. Hunks targeting lax, admitted, hazard or interface-only modules, or modules
  absent from the receipt, count 0 and are listed in the report with the reason.
- **C. Hand-written F* files listed in config as proof** (`.fst`/`.fsti`), nonblank
  non-comment lines, only if the receipt shows them verified. Bertie lists none today;
  support exists so future hand-written lemmas count.

**Does not count as proof:**
- Generated hax extraction output (any file the receipt or config marks as extraction).
- Files listed in config as **models/assumptions** (Bertie: `proofs/fstar/libcrux-models/*.fsti`).
  Report their line count as `fstar_model_sloc`, a separate "trusted assumptions" figure.
  Never part of the ratio.
- ProVerif (`hax_lib::proverif`, `pv_constructor`, `.pvl`) and SSProve. F* only in v1.8.
  Do not count them anywhere in the ratio.

## Config (new, optional; absent = today's behavior for these inputs)
In `formal-coverage.toml`:
```toml
[fstar.proof]
patches = ["proofs/fstar/extraction-panic-free.patch"]
files = []                                     # hand-written proof .fst/.fsti (C)
models = ["proofs/fstar/libcrux-models/*.fsti"] # assumptions, reported, not counted
```
Globs relative to the workspace root. A listed path that matches nothing is an error.
`deny_unknown_fields` like the existing config. Document it in README.

## Report
- JSON `proof_ratio` gains `fstar_annotation_sloc`, `fstar_patch_sloc`, `fstar_file_sloc`,
  `fstar_model_sloc`, `proven_extracted_runtime_sloc`, and a list of excluded patch hunks
  (file, target module, reason). Existing fields keep their names and meaning, except
  that `proof_sloc`/`ratio` now include the F* parts and the denominator includes
  proven-extracted. Per-crate and per-module rows get the same fields.
- Markdown shows the breakdown in one short table.
- Bump schema/version as the repo's conventions require; v1.8.0 in Cargo metadata.

## Non-negotiables
- No tier, receipt-grammar, threshold or evidence-acceptance change. Every unit's status
  must be identical before and after on Bertie, scratch-server and the fixtures.
- Coverage **sizes** may change only by moving hax annotation lines from runtime to
  inline proof in separate mode. Report every changed unit size; no other size change.
- Verus results unchanged: ratio and all figures for Verus fixtures, scratch-server and Vest
  stay identical, unless a unit is `proven-extracted` (then say so explicitly).
- Patch counting is tied to the accepted receipt. No receipt → F* patch/file lines are 0.
- Whitelist, not heuristics. Unknown `hax_lib::` paths are not counted; list the
  distinct unknown paths seen in the report so the whitelist can be reviewed.

## Milestones
- **M1 Denominator.** Add proven-extracted to the denominator, the new field and tests.
  Accept: unit tests; Bertie replay shows a nonzero denominator; Verus fixtures unchanged.
- **M2 Annotations (A).** Whitelist plus span measurement; separate/runtime modes; tests per
  whitelist entry, a ProVerif negative test, and an alias test.
  Accept: an independent scanner (a separate script under `dev/v18/`, not the tool's code)
  counts the annotation lines in Bertie at `f646099` and matches the tool's figure exactly,
  or every difference is explained line by line.
- **M3 Patches, files, models (B, C, config).** Config parsing, receipt-tied hunk filter,
  model reporting. Tests: a hunk to a verified module counts; lax/absent/interface-only
  hunks count 0 with a reason; a missing glob errors; no receipt gives 0.
  Accept: an independent count of Bertie's panic-free patch `+` lines per target module
  matches.
- **M4 Replay and docs.** Bertie report-only replay at `f646099` (checkout
  `~/src/bertie-deps`, existing receipt `target/ci-logs/fstar/fstar.log`,
  the exact CI command in `dev/v173/compare.md` with Bertie's config plus the `[fstar.proof]`
  section, written to a temp copy of the config, never committed into Bertie). Also replay
  scratch-server and Vest the way earlier releases did (see dev/NOTES.md). Write
  `dev/v18/compare.md` (before and after for all three: ratio, components, statuses
  unchanged, changed sizes) and `docs/v1.8.md` (short, like docs/v1.7.3.md). README updated.

## Out of scope
ProVerif/SSProve counting; changing Bertie itself (Dudley re-baselines bertie-tls
afterward); changing the configured 5–15 range; push, tag or release.
