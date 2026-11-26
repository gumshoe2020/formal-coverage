# v1.11.0 notes

- 2026-11-25 (Claude Sonnet 5.5 took over from Codex; tree was clean at efe1f92, nothing of M1 existed).
  Baseline binary: worktree of tag v1.10.0 under target/v1110/baseline-src.

## HONEST-FINDING: staged controls that the rules classify differently from PLAN.md's table

Probe: v1.11.0 classifier (rules R1-R4 as written) on `target/v1110-inputs/bertie`, `--ghost-lines separate`.
No rule was adjusted to fit; the numbers are what the rules give.

1. `tls13utils::check_eq1` -> `safety-only` / `size-only`; table says `functional`.
   Signature `check_eq1 (b1 b2: u8)`, postcondition `Result_Ok _ -> b1 =. b2`. Both sides are
   integer-typed parameters (`u8`), which R2 lists as size-atom operands, so the atom is a size atom.
   (`check_eq`, `check_eq_slice`, `check_eq_option`, `check_eq_inner`, `eq_slice` do come out functional.)
2. `der::encode_uint`, `der::decode_ecdsa_signature`, `der::uint_to_32_bytes`, `der::encode_ecdsa_signature`
   -> `functional`, self-consistent **true**; table says false. They call `be_value`, `der_uint_valid`,
   `parser_sound`, `encoder_correct` (and `parser_complete`, `der_sig_valid`). In bertie bab3c13 these
   are plain `pub fn`s in `src/der.rs`, not under `#[cfg(hax)]`, so the tool categorizes them `runtime`
   (status proven-extracted). Only `zero_prefix`, `pow256` etc. are `proof-code`. R4 requires a
   first-party runtime callee and no first-party proof-code callee, so R4 says self-consistent.
   `skip_zeros` (calls `zero_prefix`, proof code) is correctly false.
   `uint_to_32_bytes` also lists unresolved `two_pow_256_` (the hax name of `two_pow_256`; not mapped).
3. Postcondition text is as PLAN assumed otherwise; all other controls match.

Decision needed from Andrew/Dudley (not mine): (1) should integer equality between two machine-integer
variables count as a value fact? (2) should R4 treat spec predicates that are executable runtime
Rust (`be_value`, `parser_sound`...) as proof code, e.g. by recognizing spec-only functions that are
reachable only from contracts? Either needs a rule change.

## Interpretations made while implementing R1-R4 (logged, no rule bent)

- R2 `impl_<N>__len`: the real names are `impl_Bytes__len`, `impl_HandshakeData__len`,
  `Alloc.Vec.impl_1__len`; accepted any `impl_*__len`. Length-function arguments must be variables
  (params, binders, result) or their projections; a call as argument (`len (concat a b)`) is a value atom.
- R2 variable typing: parameters from the signature binders, the `fun` binder from `Prims.Pure T`
  or the `let x:T = x in` ascription, match-arm binders from the result type (`t_Result`/`t_Option`
  payload, tuple split on `&`). `(x: usize{...})` refinements count as integers.
- R1: only rebinding `let x:T = y in` is scaffolding; any other `let`, `forall`, `exists` => `unparsed`.
  A match whose scrutinee is a variable is scaffolding; any other scrutinee is itself an atom.
- R4: `Category::ProofCode` taken literally (including cfg(hax) units that were runtime). Qualified
  names resolve only when they carry the module's own crate prefix (`Bertie_tls.`, derived from the
  module name); other prefixes are listed unresolved.
- Unparsed contracts include non-first-party F* library models extracted with bertie
  (Alloc, Core_models, Rust_primitives); they are listed, not hidden.

## Assertions edited because they encoded the old `functional` heuristic

- `src/fstar.rs` test `contracts_and_comment_masking`: asserted the old `contract()` tuple
  (`own` flag for "same-module name and no external namespace"; `External.Spec.valid` not own).
  The old function is gone; the test now checks the new classifier (`certificate_verify` is functional and
  its calls include `parse_certificate_verify`; `Prims.l_True` post is safety-only). Self-consistency
  itself is now covered by `tests/fstar_evidence.rs::contract_depth_follows_each_classification_rule`
  (self-consistent via runtime callee; not self-consistent via a `cfg(hax)` proof-code callee).
- `tests/fstar_evidence.rs::seven_required_cases` unchanged: `round_trip` is still functional and self-consistent.

## M2 decisions

- Depth numbers are computed once in `src/ratio.rs` (`proof_depth`) with the same per-unit runtime
  SLoC helper as the proof ratio, over first-party security-relevant units: its total equals the
  headline denominator (proven + proven-extracted runtime SLoC). Exposed as `summary.proof_depth`.
- `coverage_metrics.snapshot()` is NOT extended: recorded baselines on the data branch are exact
  snapshots (`snapshot_keys`, `committed != measured`), so a new key would invalidate every recorded
  floor and change ratchet behaviour. The depth is a separate `coverage_metrics.proof_depth(report)`
  (validates it adds up to the proven runtime lines) used by the COVERAGE.md section.
- Verus-proven units have no F* contract; they count as `safety-only` (doubt lowers). Bertie has none.
- A unit with several credited definitions takes the lowest depth.
- Markdown: the definitions table gains a "Contract reason" column; new section
  "Contracts the classifier could not read" (printed whenever F* definitions exist; "None." if empty).

## 2026-11-26T00:50:47+00:00 — HONEST-FINDING / replay

- FAIL (c) [separate] control mismatches: der::decode_ecdsa_signature, der::encode_ecdsa_signature, der::encode_uint, der::uint_to_32_bytes, tls13utils::check_eq1
- FAIL (c) [runtime] control mismatches: der::decode_ecdsa_signature, der::encode_ecdsa_signature, der::encode_uint, der::uint_to_32_bytes, tls13utils::check_eq1

- 2026-11-26T01:01:19Z M4: A1 applied; the M3 HONEST-FINDING is resolved by Amendment A1. All five controls now match; replay exits 0.
