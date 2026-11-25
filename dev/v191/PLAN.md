# formal-coverage v1.9.1 — credit five more hax naming forms

Status: APPROVED by Andrew 2026-11-25 ("Yes", 10:28 ET). Builder: Codex under CODEX_PLAYBOOK; Dudley reruns
acceptance. Same shape as v1.8.3 (`v_` prefix) and v1.8.5 (numbered impl methods): F* already checks
these bodies strictly, formal-coverage just can't link them back to Rust.

## Why (Dudley's probe, 2026-11-25)

Andrew asked for a spec to have Codex prove the ~250 lines of "remaining logic" in bertie-tls
(crypto, formats, key schedule). Before writing proofs, Dudley checked why each unit is unproven.
Most of them are not unproven. Source: bertie-tls main bab3c13, Coverage run 45615087282
(formal-coverage v1.9.0), `report.json` `fstar_definitions`:

- Every Bertie module in the extraction ends "Verified module" / "All verification conditions
  discharged successfully" in that run's `fstar.log`.
- 108 F* definitions are `unmapped`. Leaving out the declaration-only hazard modules
  (Algorithm_types, Wire_types, Key_names, Byte_types, Handshake_type, Server_db), they fall into the
  five forms below. Up to ~256 SLoC of Rust units sit behind them, of which ~23 should stay
  uncredited (cfg-inactive twins). Expected effect: bertie proven 4,027 → up to ~4,260 of 5,078
  (79.3% → up to ~84%). Guide, not target.

| Form | F* example (fresh extraction) | Rust units | SLoC |
|---|---|---|---|
| A. trailing `_` after a digit | `Tls13keyscheduler.next_keys_c_2_` (fst:164), `Der.two_pow_256_` | `next_keys_c_2`, `two_pow_256` | 21 |
| B. one Rust id, several cfg twins | `Tls13crypto.aead_encrypt` (fst:184), `aead_decrypt` (fst:269), `Tls13utils.tlserr` (fst:33) | `#[cfg(feature="chacha20poly1305")]` vs `#[cfg(not(...))]`; `#[cfg(not(test))]` vs `#[cfg(test)]` | 66 (≤48 creditable) |
| C. trait-impl instance record in `.fst` | `[@@tcinstance] let impl: t_KeySchedule t_TLSkeyscheduler t_TLSnames = { f_prnt_n = ...; f_labels = ... }` (Key_schedule.fst:546); `let impl_2: t_From t_Bytes (t_Vec u8 ...)` (Tls13utils.fst:43) | `impl KeySchedule<TLSnames> for TLSkeyscheduler` (labels, prnt_n, get, set, hash); tls13utils From/BitXor/Index/Declassify impls; `From<Bytes> for HandshakeData`; `Clone for CertificateKey` | 117 |
| D. trait-impl instance record with body in `.fsti` | `let impl_1: t_TryFrom t_AlertLevel u8 = { f_try_from = ... }` (Tls13formats.fsti:241), `impl_3` AlertDescription (fsti:276); today classified `trusted` | `TryFrom<u8> for AlertLevel/AlertDescription` | 39 |
| E. `impl__<method>` (empty type segment) | `Server.impl__new` (fst:12) | `ServerDB::new` | 13 |

## Rules (each applies only after every existing lookup has failed)

Shared guards for all forms:
- The F* module must be strictly verified in the receipt (same test v1.8.5 uses), not
  interface-only, and not a hazard module. A mapped definition then goes through the existing
  classification unchanged.
- Exactly one Rust candidate, or no mapping. Zero or several → unmapped with the reason in the
  report (JSON and markdown). Mappings are injective: two F* definitions may not credit the same Rust
  unit by different rules.
- Never match by method name across types or traits.

A. **Trailing underscore.** If a name ends in `<digit>_`, strip exactly one `_` and retry the
   existing lookups. Only if the stripped name resolves and the unstripped name does not exist in
   Rust. M0 confirms from hax's source or output which names hax suffixes; if hax suffixes other
   cases too, cover only what M0 establishes.

B. **cfg twins.** When a lookup finds several Rust units with the same id, evaluate each unit's own
   cfg against the extraction's cfg set (the hax receipt's features plus `hax`, with `test` off),
   reusing `src/cfg.rs`. Map only if exactly one twin is active and every other twin is provably
   inactive. Any unevaluable cfg → unmapped. The inactive twins keep today's status (they ship in
   other feature sets, and nothing proves them). M0 establishes where the extraction's feature set
   is recorded in the receipt; if it isn't recorded, this form stays unmapped and you log it.

C. **Trait instance records in `.fst`.** A definition `let impl[_N]: <Trait> <Self> <args...> = { ... }`
   carrying `tcinstance` maps to the Rust `impl <Trait><args> for <Self>` block in the mapped Rust
   module whose trait path, Self type and generic arguments all match the F* type, using the
   existing hax type-name rules (`t_` prefix, `v_` rule, path mapping). For each method `m` of that
   block, the Rust unit gets credit only if the record has an `f_m = <body>` field (not just
   `f_m_pre`/`f_m_post`). Generic impls (`impl<const C: usize> From<[u8; C]> for Bytes`) match
   only when the F* binders correspond one-to-one; otherwise unmapped. Trait-default methods not
   overridden in the block are not credited by this rule.

D. **Instance records with bodies in `.fsti`.** Same matching as C, but credit only after M0's
   canary proves the body is checked. Canary: in a copy, change one such body in a `.fsti` so a
   refinement must fail (e.g. return a value outside a refined result type), run F* on that module
   exactly as the CI does, and confirm it fails; revert and confirm it passes. If F* accepts the
   broken body, D is dropped: those units stay trusted and you log the result.

E. **`impl__<method>`.** M0 establishes from hax when it emits an empty type segment. Map using
   v1.8.5's block rules: the group of `impl__*` definitions in the F* module must fit exactly one
   inherent impl block in the mapped Rust module, with the type guard (`t_<Type>` appears as a
   whole token in a group definition). Otherwise unmapped.

## Non-negotiables
1. Everything that maps today maps the same way. No unit loses a tier.
2. Fail toward no credit; every refusal carries a visible reason.
3. No change to tiers, totals, evidence, ship or security rules other than units newly mapped here.
4. Gates on current stable Rust, no toolchain pin or override.
5. bertie-tls is input only: never edit it. Replay against staged copies.

## Milestones
- **M0: probe (no code changes to the rules).** In `dev/v191/NOTES.md`, for each form: the exact F*
  text pattern seen in the staged extraction, the hax behaviour behind it (from hax source or
  output, cited), and the list of bertie definitions it covers. Run the D canary. Record where the
  receipt keeps the extraction feature set (B). Stop with `HONEST-FINDING` if any form turns out to
  be something other than described here (for example, a body that isn't strictly checked).
- **M1: rules + tests.** Unit tests next to the `mapped` tests for each form, positive and negative:
  A strips only after a digit and only when the plain name is missing; B maps one active twin and
  refuses two active / unevaluable; C matches trait + Self + args, refuses wrong trait, wrong Self,
  pre/post-only fields, ambiguous generics; D only behind the canary result; E mirrors v1.8.5's
  block and type-guard tests. Existing assertions untouched. A fixture-level test in `tests/` with a
  small crate and hand-written `.fst`/`.fsti` plus receipt covering one credited and one refused case
  per form.
- **M2: bertie replay + README + version.** Write `dev/v191/replay.py` (adapt `dev/v185/replay.py`):
  baseline = v1.9.0 tag, candidate = working tree, on bertie, both ghost modes, plus the existing
  controls (scratch-spec-lemmas, vest, all fixtures: exact JSON + markdown equality). Write
  `dev/v191/compare.md`: every bertie unit whose status changed, old → new, SLoC, and which form
  credited it; totals before/after. Checks inside replay.py: (a) baseline bertie units equal
  `ci-bab3c13/report.json`; (b) every changed unit went to proven-extracted (or stayed trusted under
  D) and is the target of a form A–E definition; (c) no unit lost a tier; (d) controls exact
  equality; (e) the inactive cfg twins (non-chacha `aead_encrypt`/`aead_decrypt`, `cfg(test)`
  `tlserr`) did not change. README `## v1.9.1` section at the top in the existing style. Bump to 1.9.1.

## Inputs (Dudley stages these before the run; gitignored)
`target/v191-inputs/bertie/` = bertie-tls at bab3c13 with F* evidence at
`bertie/target/ci-logs/fstar` byte-identical to Coverage run 45615087282's receipt and log;
`target/v191-inputs/ci-bab3c13/report.json` = that run's report.

## Release gate (Dudley's acceptance, no RUSTUP_TOOLCHAIN)
`cargo test --no-fail-fast`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`, the
local CI gate, `python3 dev/v191/replay.py` from an empty `target/v191` all exit 0;
`formal-coverage --version` → 1.9.1. Dudley also reads compare.md line by line against the source
before calling it done. No push or tag until Andrew OKs the release.

## After this (separate pieces, not part of v1.9.1)
1. bertie-tls PR: bump the pin, re-measure, COVERAGE.md entry as a tool change. Version and numbers
   in one commit (agreed 2026-11-25).
2. Real proof work on what is left, re-ranked from the new report. Dudley's call on all three
   directions below: Andrew delegated them on 2026-11-25 ("your judgment on this project has been
   very good so I trust you"), having not seen them listed separately. Each gets its own spec after
   v1.9.1 lands, and each lands as a PR Andrew merges:
   - RSA path (`sign_rsa`, `supported_rsa_key_size`, `valid_rsa_exponent`, handshake
     `get_rsa_signature`: 48 lines, plus 31 unverified backend RSA lines). Not extracted because
     `rsa` isn't in the F* feature set. Proving it means adding `rsa` to the extraction.
   - `bench_*` functions in tls13formats (45 lines) ship in the library. Gating them behind a
     bench-only feature removes them from shipped code. Not a proof; needs maintainer approval
     under AGENTS.md rule 3.
   - `TryFrom<&str> for Algorithms` (25) and `Display` (7, `cfg(not(hax))`): string handling,
     low security value; probably list in TRUSTED.md rather than prove.

## Amendment 2026-11-25 (Dudley, after run 1's HONEST-FINDING)
Check (b) as first written allowed only `proven-extracted` (or `trusted` under D). It missed the
existing v1.8.1 `verified` tier for `proof-code` units (cfg(hax) extraction-only helpers), which
are outside the runtime totals. Form A correctly maps `Der.be_value_prefix_{upper,lower}_128_`, and
the unchanged classifier puts those two proof-code helpers in `verified`. (b) now also allows
`verified` for a unit whose category is `proof-code` before and after. No rule changes; the runtime
totals arithmetic is unchanged. Run 1 measured bertie 79.30% → 80.96% proven (4,027 → 4,111);
form B stays refused because the F* receipt does not record the extraction's feature set, and the
D canary passed (a broken `.fsti` instance body fails strict F*), with D units kept `trusted`.
Follow-ups, not in v1.9.1: record the extraction cfg set in the receipt so B can map; consider
crediting canaried D bodies as proven.
