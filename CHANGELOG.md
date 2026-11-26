# Changelog

## 1.12.2 — 2026-11-26

- Run from a git hook (for example a pre-push `cargo test`), the
  test suites inherited the hook's `GIT_DIR`, which overrides `git -C`. A
  fixture's `git add`/`commit` then committed onto the real branch, deleting
  every tracked file. Every test git command now goes through one helper per
  language (`tests/common/mod.rs`, `ci/tests/gitenv.py`). The helper removes the
  variables `git rev-parse --local-env-vars` lists, ignores global and system
  git config and commits as a fixed test identity. Kit scripts and kit code the
  tests run get the same cleaned environment. New regression tests point
  `GIT_DIR` (alone, and with `GIT_WORK_TREE` and `GIT_INDEX_FILE`) at a decoy
  repository and check that the decoy stays byte-identical.
- The baseline, receipt and badge publishers now clear inherited repository
  variables before running git in their temporary directories. A hook's
  `GIT_DIR` can no longer redirect a publisher's checkout or commit into the
  caller's repository. The publishers retain their configured HTTP auth headers.

## 1.12.1 — 2026-11-26

- `ci/run.py` (and so the coverage action) now refuses to compute a ratchet
  verdict when the checkout has uncommitted changes: tracked files modified,
  staged or deleted, or untracked files that are not git-ignored. It exits 1
  naming the workspace, the fix (commit first) and up to ten paths. Before, a
  local run on uncommitted edits measured them but took the change set from
  `base..HEAD`, so with HEAD at the base it saw no touched security code and
  printed `ratchet PASS`. This applies in both enforcement modes and both
  floor modes. The action's own `logs/ci-kit/build.log` and `coverage.log`
  are the only exemption. Anything a workflow writes into the checkout before
  the coverage action (prover outputs, receipts, dependency checkouts) must be
  git-ignored or written outside the checkout.

## 1.12.0 — 2026-11-26

- Recorded-floor fetches and publish actions now reset inherited GitHub HTTP
  headers before setting the job token, preventing duplicate Authorization
  headers when checkout persists credentials.

- The coverage action now defaults to reporting the ratchet without enforcing it.
  Existing projects that rely on the ratchet must add `enforce-ratchet: 'true'`
  when they adopt 1.12.0.

- At equal proven share, touched security code can now pass the ratchet by
  increasing proven security runtime lines in the two functional contract
  categories combined. Unverified lines still cannot increase. Recorded-mode
  PRs can use this rule once their main merge base has a depth-bearing record
  written by this version.

- Opt-in reuse of main's strict per-module F* checks for identical extracted
  inputs. Receipts bind the module/interface and full transitive project/library
  closure, tool identities, exact options, main commit and strict log slice.
  Fresh extraction/dependency analysis remain mandatory; `.checked` files never
  provide evidence. Main still checks every root strictly and uncached.
- Add the PR selector, dependency-context builder and main-only
  `publish-receipts` action. Data-branch records are append-only: identical bytes
  are a no-op, conflicting bytes fail. Publication uses GITHUB_TOKEN only.
- `report --fstar-reuse` independently verifies every reused record against
  current inputs and main's fetched data-branch record before granting credit.
  Mismatches fail closed with module/field diagnostics. Markdown labels fresh
  and reused module evidence and its main commit; equal coverage JSON stays
  identical. Added trust: main's protected receipt publisher and artifact chain.
  Default evidence rules, coverage schema and runtime dependencies are unchanged.

## 1.11.0 — 2026-11-25

- Honest contract depth. `safety-only` now covers every F* postcondition that only
  states lengths or integer ranges (about two thirds of what 1.10.0 called
  `functional` on bertie-tls), not just a missing one; `functional` needs a value fact.
  New `contract_reason` per definition (`no-postcondition`, `trivial`, `size-only`,
  `value`, `unparsed`); unreadable postconditions stay `safety-only` and are listed in
  a new report section.
- The self-consistency flag now works with current hax namespaces and is decided by
  resolving the functions a contract calls: first-party runtime callee and no proof-code
  callee. `contract_unresolved` lists names that did not resolve.
- New "Proof depth" table (Markdown, `summary.proof_depth`, kit COVERAGE.md section,
  `coverage_metrics.proof_depth`). No unit changes tier, status or SLoC; the proven share,
  proof ratio, badges and ratchet are unchanged. Recorded baselines stay valid.

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
