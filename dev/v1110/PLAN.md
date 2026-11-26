# formal-coverage v1.11.0: honest contract depth

Status: AUTHORIZED by Andrew 2026-11-25 19:25 ET ("Let's fix the tool first. We need honest
measurements"). He confirmed the core rule at 19:28 ET: "Isn't length bounding a safety property?"
Builder: Codex under CODEX_PLAYBOOK; Dudley reruns acceptance. Local only: no push, no tag.

## Why (Dudley's probe, 2026-11-25, bertie-tls report.json from CI run 45718095868 at 863ed07)

The tool labels each proven F* contract `safety-only` or `functional`. Two defects make
`functional` overstate what is proven:

1. **Size facts are counted as functional.** `contract()` in `src/fstar.rs` (~line 796) calls any
   `ensures` that mentions a word other than `fun/result/True` "functional". About 675 of bertie's
   983 "functional" lines only constrain lengths or integer ranges, e.g.
   `hash_len: result <=. mk_usize 64`, `parse_client_hello: trunc_len <=. len client_hello`,
   `bytes2: len result =. 2`, `encode_length_u16: len lenb - 2 =. len bytes`. Those are what callers
   need to prove absence of panics, so they are **safety facts**.
2. **The self-consistency flag never fires.** It is `false` on all 309 units. The `external` test
   treats any dotted word not starting with `Core.`, `Prims.` or `Bertie.` as an outside spec, but
   current hax output uses `Bertie_tls.`, `Core_models.`, `Alloc.`, `Rust_primitives.` and
   `Hax_lib.`, so type ascriptions alone make every postcondition look "external". Five handshake
   builders (`client_hello`, `server_hello`, `server_certificate`, `certificate_verify`, `finished`,
   167 lines) are proven only to round-trip through bertie's own parsers, which is exactly the
   pattern that hid the ECDSA DER bug, and nothing flags them.

Dudley's hand classification of the 4,111 proven lines (11-25 local report): ~3,800 safety,
~167 functional by self-consistency only, ~141 functional against an independent spec or a direct
value equality (DER encode/decode via `cfg(hax)` spec functions, 81; `check_eq` family, `eq_slice`,
`check`, 60). Guide, not target.

## Rules

Classification applies to `ensures` bodies exactly as `contract()` sees them today.

R1. **Normalize first.** Strip: the binder `fun x ->`; `let x:T = x in` rebinding; type
ascriptions `<: T`; `#T` implicit arguments; `(decreases ...)` clauses; `b2t`; match scaffolding on
the result (`match result <: T with | Ok p -> BODY | _ -> true`, including
`Result_Err _ -> true`): keep each non-trivial arm body. Split the remaining body into atoms on
`&&`, `/\`, `||`, `\/`, `==>`, `if/then/else` and match arms.

R2. **Size atom.** An atom is a size atom when it is a comparison (`=. <. <=. >. >=. <>. = < <= >
>= <>`) and both sides are built only from: length functions (identifiers whose last segment is
`len`, `length` or matches `impl_<N>__len` / `impl__len`, plus `Seq.length`); integer literals
and `mk_usize/mk_u8/mk_u16/mk_u32/mk_u64/mk_isize N`; integer-typed variables (a parameter, the
result or a pattern binder whose type in the signature or the matched tuple is a machine integer or
`Hax_lib.Int.t_Int`); `from_machine`/`Rust_primitives.Hax.Int.from_machine`; arithmetic
(`+ - * / % +! -! *!`); range fields `f_start`/`f_end`. A literal `true` atom is a size atom.

R3. **Contract kind.**
- `safety-only`: no `ensures`, a trivially true one (as today), or every atom is a size atom.
  Record the reason in a new `contract_reason` field: `no-postcondition`, `trivial` or `size-only`.
- `functional`: at least one atom is not a size atom. Reason `value`.
- If normalization or atom parsing fails, the kind is `safety-only` with reason `unparsed`, and the
  definition is listed in a new report section "Contracts the classifier could not read". Never
  round up.

R4. **Self-consistency** (only for `functional`). Collect the function identifiers the postcondition
calls, other than the definition itself, length functions, and names in the non-spec namespaces
`Core.`, `Core_models.`, `Prims.`, `FStar.`, `Alloc.`, `Rust_primitives.`, `Hax_lib.`,
`Lib.`. Resolve each through the existing F*→Rust mapping. The contract is self-consistent when at
least one resolves to a first-party **runtime** unit and none resolves to first-party **proof code**
(`Category::ProofCode`, e.g. `cfg(hax)` spec functions such as `parser_sound`, `encoder_correct`,
`be_value`, `zero_prefix`). Unresolvable names are listed for review and count as not proof code.
Fix the namespace bug by deriving the crate's own prefix from the crate name
(`bertie_tls` → `Bertie_tls.`) instead of a hard-coded `Bertie.`.

R5. **Tiers and sizes don't move.** This release changes only contract fields and the new report
sections. Every unit keeps its tier, status and SLoC; proven share and the proof ratio are
identical to v1.10.0 on the same inputs.

R6. **Report.** In `report.md` and the kit's COVERAGE.md section (`ci/coverage_section.py`), add a
"Proof depth" table under the proven tier, over proven + proven-extracted runtime SLoC:
`safety-only` / `functional, self-consistent only` / `functional`, each with lines and share of
proven lines, plus one sentence: "Safety-only means proven free of panics, overflows and
out-of-bounds access, including length and range facts; it says nothing about what the code
computes." Add the same three numbers to `report.json` `summary` and to `ci/coverage_metrics.py`
output. **Badges do not change.** The ratchet does not gate on depth.

## Controls (bab3c13 staged inputs; every one must classify as listed)

| Unit (Rust) | Expected kind | Self-consistent |
|---|---|---|
| `tls13formats::client_hello`, `server_hello`, `server_certificate`, `certificate_verify`, `finished` | functional | true |
| `der::decode_ecdsa_signature`, `der::encode_ecdsa_signature` (if present in bab3c13), `der::encode_uint`, `der::skip_zeros`, `der::uint_to_32_bytes` | functional | false |
| `tls13utils::check_eq`, `check_eq1`, `check_eq_slice`, `check_eq_option`, `check_eq_inner`, `eq_slice` | functional | false |
| `tls13formats::parse_client_hello`, `check_extension`, `check_server_extension` | safety-only (size-only) | n/a |
| `tls13utils::length_u24_encoded`, `encode_length_u16`, `bytes2`, `Bytes::len`, `Bytes::concat`, `check_eq_with_slice` | safety-only (size-only) | n/a |
| `tls13crypto::HashAlgorithm::hash_len`, `tls13cert::has_oid`, `tls13record::padlen` | safety-only (size-only) | n/a |

If a control's postcondition in the staged extraction differs from what this table assumes,
don't bend the rules: record it as `HONEST-FINDING` in NOTES.md with the postcondition text.
Unit fixtures in `fixtures/hax/` must cover every rule, including an `unparsed` case and a
functional case that calls a proof-code spec function (expected: not self-consistent).

## Milestones

- **M1 classifier.** R1-R4 in `src/fstar.rs` with unit tests per rule; fixtures updated; the old
  heuristic removed, not left beside the new one.
- **M2 report.** R6 in the Rust report, report.json, kit section and metrics, with kit tests.
  Docs updated where they define `safety-only`/`functional` (`docs/evidence.md` ~line 575, README).
  CHANGELOG entry. Version 1.11.0.
- **M3 replay.** `dev/v1110/replay.py` (model on `dev/v191/replay.py`) runs v1.10.0 and v1.11.0 on
  `target/v1110-inputs/bertie` and checks: (a) every unit's tier/status/SLoC identical; (b) proven
  share and proof ratio identical; (c) every control as in the table; (d) depth totals add up to
  proven SLoC; (e) zero `unparsed` contracts, or each one listed in compare.md with its text.
  Writes `dev/v1110/compare.md`: before/after depth table, then every contract whose kind or
  self-consistency changed (unit, SLoC, old → new, one-line postcondition excerpt).

## Inputs (staged by Dudley; gitignored)
`target/v1110-inputs/bertie/` = copy of `target/v191-inputs/bertie/` (bertie-tls bab3c13 with F*
evidence byte-identical to Coverage run 45615087282). `target/v1110-inputs/ci-863ed07/` = artifacts
of the latest main run (863ed07), reference only; it lacks the fresh extraction, so don't measure
from it.

## Out of scope
Badge changes, ratchet changes, ProVerif/SSProve credit, the proof ratio formula, any bertie-tls
change. The bertie-tls re-measure is a separate PR after Andrew sees compare.md.

## Amendment A1 (Dudley, 2026-11-25 ~20:00 ET, after the M3 HONEST-FINDING)

The builder's stop was correct; both findings are spec errors, resolved here. These supersede the
matching parts of R2, R4, R6 and the controls table.

A1.1 (supersedes R2's "integer-typed variables"). Only variables typed `usize`, `isize` or
`Hax_lib.Int.t_Int` (including refinements of them) are size operands. Fixed-width data integers
(`u8`..`u128`, `i8`..`i128`) are values: an atom that compares one of them to anything other than
a length function or a literal is a value atom. Control: `tls13utils::check_eq1` (`b1 b2: u8`,
`Ok -> b1 =. b2`) is `functional`, self-consistent false. Every other control keeps its result.

A1.2 (R4 stays as written; label changes). The DER spec predicates (`be_value`, `der_uint_valid`,
`parser_sound`, `encoder_correct`, `der_sig_valid`, `parser_complete`) are plain runtime `pub fn`s
in bertie bab3c13, so under R4 those contracts check against first-party runtime code. The tool is
right to say so; the fix belongs in bertie-tls (gate spec-only predicates with `cfg(hax)`), which is
out of scope here. Expected for the controls `der::decode_ecdsa_signature`,
`der::encode_ecdsa_signature`, `der::encode_uint`, `der::uint_to_32_bytes`: functional,
self-consistent **true**. Because "self-consistent" now covers both "round-trips through its own
parser" and "checked against other first-party runtime code", rename the depth row everywhere
(report.md, report.json key, kit section, compare.md, docs, CHANGELOG) from
"functional, self-consistent only" to "functional, checked only against own code" (json key
`functional_own_code_only`). The per-definition field may keep its name.
The one-sentence explanation under the table gains: "Checked only against own code means the
postcondition relates the result to other runtime functions of the same project (for example its
own parser), not to a separate specification."
