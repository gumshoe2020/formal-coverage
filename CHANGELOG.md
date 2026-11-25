# Changelog

## 1.10.0 — 2026-11-25

- Coverage CI: opt-in `floor: "recorded"` mode. A PR's ratchet compares its
  measurement with the one recorded for its merge-base commit on main, read from a
  data branch (`coverage-data`, file `baselines/<sha>.json`), so PRs no longer
  commit `coverage/baseline.json` or the measured section of `COVERAGE.md` and two
  PRs stop conflicting there. A missing recorded file fails closed, naming the
  commit; `bootstrap_baseline` covers the first adoption only.
- New `publish-baseline` action records each main push append-only (existing
  files are never overwritten) and refreshes `COVERAGE-MEASURED.md` on the data
  branch. Only that job needs `contents: write`; main is never written.
- Ratchet rules, arithmetic and the exceptions table are unchanged. The default
  `floor: "committed"` mode is byte-identical to 1.9.1 (checked by replay in
  [dev/v1100](dev/v1100/PROGRESS.md)).

## 1.9.1 — 2026-11-25

- Map five more hax naming forms back to Rust, each only when exactly one Rust
  candidate fits and the F* module is strictly verified: trailing numeric escapes
  (`name_2_`), typed trait instance records with explicit method bodies, interface
  instance records (kept trusted, guarded by a canary showing F* checks the body),
  and empty-segment inherent impl names (`impl__new`).
- Cfg twins stay unmapped with a visible reason when the receipt lacks the
  extraction cfg set. Ambiguity, unsupported generics or a failed type guard
  leave a definition unmapped with the reason in JSON and Markdown.
- Existing mappings retain precedence; new mappings cannot share a Rust unit
  with another definition. Interface records require the F* version checked by
  the [M0 canary](dev/v191/NOTES.md).
- On bertie-tls: 79.30% to 80.96% proven. See the
  [paired replay](dev/v191/compare.md).

## 1.9.0 — 2026-11-24

- Add a reusable, Python standard-library coverage CI kit extracted from bertie-tls:
  baseline/document checks, exact-rational git ratchet, configurable honest badge
  thresholds and informational proof-cost ranges.
- Add SHA-pinned composite coverage and main-only badge publication actions;
  the action pin also selects the measurement tool. Git state is the floor;
  missing base baselines fail closed. Only the publish job needs contents: write.
- Add kit tests, CI build/test workflow and documented consumer workflow.
- Measurement behavior and report format are unchanged from 1.8.5. Proof cost
  stays informational in the [CI kit](ci/README.md). See [Use in CI](README.md#use-in-ci).

## 1.8.5 — 2026-11-24

- Map numbered `impl_<N>__method` definitions only when the module's whole method
  group selects exactly one inherent Rust impl and the F* text mentions its
  `t_Type` token. Different numbered groups cannot select the same block; hax
  numbers do not encode source order. Trait impls are excluded.
- Keep ambiguous groups and missing type tokens unmapped, with reasons in JSON
  and Markdown. Preserve direct, named-type and uppercase value mappings.
- Apply existing evidence rules: strictly checked bodies can earn
  proven-extracted credit; interface-only methods remain trusted. See the
  [paired replay](dev/v185/compare.md).

## 1.8.4 — 2026-11-24

- Add optional `[ship]` scope for selected packages, normal workspace runtime
  dependencies and Cargo features. Exclude unrelated members and cfg-disabled
  files from all coverage and proof-cost totals; keep the full workspace scope
  when `[ship]` is absent.
- Retain tests and evidence from the full first-party inventory when assigning
  shipped units' tiers. Missing dep-info or a failed `cargo check` keeps uncertain
  code in scope with a warning.
- List excluded files, whole-file SLoC and would-be unit tiers under **Not
  shipped**. Scope is file-level; inline cfg-disabled code can still count.
  See the [paired replay](dev/v184/compare.md).

## 1.8.1 — 2026-11-21

- Classify first-party hax items whose own or enclosing cfg implies hax as
  standalone proof code in both ghost modes. Accepted F* credit marks them
  `verified`; uncredited units retain their status.
- Move cfg(hax) statements and blocks inside runtime functions to inline proof
  in `separate` mode; keep them runtime in `runtime` mode. Keep shared runtime
  lines runtime and count overlapping annotations once.
- Add `fstar_cfg_hax_sloc` and `fstar_cfg_hax_inline_sloc` to proof ratios and
  both-mode figures. Receipt grammar, evidence acceptance and thresholds are
  unchanged. See the [paired replay](dev/v181/compare.md).

## 1.8 — 2026-11-21

- Include proven-extracted runtime in the proof-ratio denominator and whitelisted
  hax/F* annotations in inline proof. Report annotations, strictly verified
  patch additions and hand-written F* proof files separately; report models as
  trusted assumptions outside the ratio.
- Use JSON report schema 4; configuration remains schema 2/3. Add
  `fstar_annotation_sloc`, `fstar_patch_sloc`, `fstar_file_sloc`, `fstar_model_sloc`,
  `proven_extracted_runtime_sloc` and `excluded_patch_hunks` to proof ratios and
  crate/module rows, with distinct `unknown_hax_paths` for review.
- Tier rules, receipt grammar and thresholds are unchanged. See the
  [release notes](docs/v1.8.md) and [paired replay](dev/v18/compare.md).

## 1.7.3 — 2026-11-21

- Exclude lines containing only Rust doc comments or explicit `doc` attributes
  from SLoC, including inline Verus ghost sizing. Other attributes and lines
  with real code still count.
- Correct measured sizes and coverage denominators; status-weighted totals and
  percentages can change. Tier rules, receipt grammar and thresholds are
  unchanged. See the [release notes](docs/v1.7.3.md) and
  [Bertie comparison](dev/v173/compare.md).

## 1.7 — 2026-11-02

- Add repeatable `--fstar-log LOG` for source-bound hax/F* receipts and a separate
  proven-extracted column that trusts hax extraction and never enters Verus
  proven coverage or its gates.
- Keep patched models and interface-only contracts trusted, without Rust proof
  credit. Report checked postconditions and a labelled self-consistency
  heuristic; agreement with one's own parser does not establish external format
  conformance. See the [receipt contract](docs/evidence.md#haxf-receipts-v17)
  and [Bertie survey](docs/survey.md).

## 1.6 — 2026-10-31

- Add the proof cost ratio and default `--ghost-lines separate` mode for supported
  Verus ghost constructs; keep mixed lines runtime and offer `runtime` mode.
  Show both-mode proven runtime figures, standalone/inline proof, security and
  all-first-party headlines, crate/module rows and separate bounded runtime.
  Unattributed root proof contributes to crate and overall totals.
- Add the default inclusive 1.0–10.0 range, schema-3 `[proof_ratio]` configuration,
  `--proof-ratio-range MIN:MAX` overrides and `--no-proof-ratio-gate`. Report the
  range, source, reason and status; a zero denominator is `n/a`.
- Fail the first-party security ratio gate outside the range; module exceptions
  are warnings. Retain coverage, completeness, hazard and input-reason checks.
  The ratio measures syntactic cost, not specification quality or difficulty.
