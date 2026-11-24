# formal-coverage v1.8.5 — credit hax's numbered impl methods (`impl_<N>__method`)

Approved by Andrew 2026-11-24 ("Yes please", after Dudley's probe). Builder: Codex under
CODEX_PLAYBOOK; Dudley reruns acceptance. Small change, one rule.

## Background (bertie-tls main 9617409, CI run 45488257676, formal-coverage v1.8.4)

hax names an inherent-impl method `impl_<Type>__<method>` when the type has one inherent impl in
the module, and `impl_<N>__<method>` (a number) when it has more than one. v1.8.2 maps the first
form (`src/fstar.rs`, `fn mapped`). The numbered form is never mapped, so those F* definitions are
reported `unmapped` and their Rust units get no F* credit even when the body is strictly checked.

On bertie main: 39 numbered definitions with bodies in `.fst` files, all in modules whose strict F*
run ends "All verification conditions discharged successfully", none listed in
`interface-only.json`:
- `Bertie_tls.Tls13utils` 22 (`impl_3`, `impl_5`, `impl_11`, `impl_17`: Bytes, U8, U32, AppData blocks;
  `src/tls13utils.rs` has four `impl Bytes {` blocks at lines 230, 274, 470, 599 on 9617409)
- `Bertie_tls.Tls13crypto` 16 (`impl_2` AeadAlgorithm key_len/iv_len, `impl_4` HashAlgorithm
  hash_len/hmac_tag_len, `impl_5` Algorithms new/validate/hash/aead/.../check)
- `Bertie_tls.Tls13formats` 1 (`impl_4__try_from_u8`, ContentType)
Up to ~266 Rust runtime lines are affected (some names such as `new`, `declassify` exist on several types).

## Rule

When a definition name is `impl_<N>__<method>` (N = one or more ASCII digits, method non-empty),
and the direct and `impl_<Type>__` lookups found nothing:
1. Group all F* definitions in the same F* module sharing the prefix `impl_<N>__`; the group's method
   set M(N) is their method names (apply the existing `v_` uppercase rule to each name).
2. Candidates are the *inherent* (non-trait) impl blocks in the Rust module that F* module maps to
   (same module mapping `mapped` already uses). A block qualifies if every name in M(N) is a method of
   that block.
3. Map N to a block only if exactly one block qualifies. Zero or more than one: leave every
   definition in the group unmapped (as today), and record the reason in the report.
4. Injective: two different N in the same module may not map to the same block. If they do, neither maps.
5. Type guard: a block of type `T` qualifies only if at least one definition in the group mentions the
   hax type name `t_T` as a whole token in its F* definition text (signature or body; on bertie the token is in the
   signature for every group except `impl_4__try_from_u8`, where it appears a few lines in).
   This stops a one-method group like `impl_<N>__new` from landing on another type's block when its
   real block isn't in the extraction.
   Do NOT add a source-order check. hax's N is rustc's impl disambiguator and is not source order. On
   9617409 `src/tls13utils.rs` maps as: line 230 `impl Bytes` → `impl_11`, 274 `impl Bytes` → `impl_3`,
   470 `impl Bytes` → `impl_17`, 1011 `impl AppData` → `impl_5` (verified by Dudley from method sets).
   Note hax numbers each *block*, not each type: one type can have several N.
6. A mapped definition then goes through the existing classification unchanged (a body in `.fst` in a
   strictly verified module → proven-extracted; interface-only → trusted; failing module → no credit).
   Unit id for the credited Rust method is the method in that specific block, not by name alone
   (two blocks of the same type can't share a method name in Rust, but trait impls can — trait impls
   are never candidates). Units today carry `module::Type` but not which block they came from; if the
   Rust inventory doesn't keep impl-block spans, add them (smallest change), don't infer blocks from names.

This rule never credits a unit that has no matching F* body, and never matches by method name across
types: the block must contain the whole group.

## Non-negotiables
1. Everything that maps today maps the same way (direct, `impl_<Type>__`, `v_` rules unchanged).
2. Fail toward no credit: any doubt (ambiguity, failed type guard, parse failure) → unmapped, with a reason
   visible in the report (JSON and markdown), never a guess.
3. No change to tiers, totals, evidence or ship rules other than units newly mapped by this rule.
4. Gates on current stable Rust, no toolchain pin or override (same as v1.8.4).

## Milestones
- **M1: rule + tests.** Unit tests in `src/fstar.rs` next to the existing `mapped` tests: two inherent
  impls of one type with disjoint methods map correctly; a group whose names fit two blocks → unmapped;
  a group with a name in no block → unmapped; trait-impl methods never match; injectivity; type guard rejects a block whose type the group never mentions; `v_` inside a numbered name; all existing assertions
  untouched. A fixture-level test in `tests/` with a small crate + hand-written `.fst`/`.fsti` and
  receipt (follow the existing hax fixture's pattern) showing a numbered method credited
  proven-extracted end to end, and an interface-only numbered method staying trusted.
- **M2: bertie replay + README + version.** Inputs are staged (gitignored) in `target/v185-inputs/`:
  `bertie/` = bertie-tls at 9617409 (its own `[ship]` config) with the F* evidence at
  `bertie/target/ci-logs/fstar` (byte-identical to CI's receipt and log); `ci-9617409/report.json` is
  CI's v1.8.4 report for reference. Write `dev/v185/replay.py` (adapt `dev/v184/replay.py`): build the
  v1.8.4 tag as baseline and the candidate, run both on bertie, both ghost modes, plus the v1.8.4
  controls (scratch-spec-lemmas, vest, all fixtures: exact JSON + markdown equality). Write
  `dev/v185/compare.md`: every bertie unit whose status changed, with old → new and SLoC; totals
  before/after. Acceptance checks inside replay.py: (a) baseline bertie report equals
  `ci-9617409/report.json` units; (b) every changed unit went unmapped/tested/unverified →
  proven-extracted or trusted, and each one is the target of a numbered F* definition; (c) no unit lost
  a tier; (d) controls exact equality. README: a `## v1.8.5` section at the top in the existing
  per-version style. Bump to 1.8.5.

## Release gate (Dudley's acceptance, no RUSTUP_TOOLCHAIN)
`cargo test --no-fail-fast`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, the
local CI gate, `python3 dev/v185/replay.py` from an empty `target/v185` all exit 0;
`formal-coverage --version` → 1.8.5. No push or tag until Andrew OKs the release.

## Expected (guide, not target)
bertie proven 3,776 → roughly 4,000–4,040 of 5,078 (~79%).

## Follow-up (bertie-tls, separate PR)
Bump FORMAL_COVERAGE_COMMIT, re-measure, COVERAGE.md entry like earlier tool changes; baseline
regenerated from the measurement, not by hand.
