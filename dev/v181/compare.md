# formal-coverage v1.8.1 paired replay

Generated 2026-11-21T21:29:12.048539-05:00.

Baseline: v1.8.0 at `e74b116`; candidate: v1.8.1. Bertie: `f646099cee002c1d998b0e8c160886ef838f4f69`, copied read-only from bertie-deps to `target/v181/bertie`, existing F* receipt retained byte-for-byte. Both tools use the same temporary v1.8 proof config and compiled-files measurement, 5–15 unchanged. A second candidate config has `patches = []`. No extraction or verifier executed.

scratch-server `2fa5eb78a6f96a8433229dc64ec4b82c3e5f9bc2` and Vest `ce6705fad1af476e828feae6621d0bcf4d0c1665` use the same archived exports, retained receipts, whole-crate sizes and targets as v1.8 M4 / scripts/v15_replay.py (scratch aarch64-apple-darwin, Vest aarch64-unknown-linux-gnu). External checkouts remain untouched.

## Ratios and every component

| Project / mode | Version | Scope | Standalone | Inline | Total proof | Annotations | cfg standalone | cfg inline | Patches | Files | Models¹ | Proven | Extracted | Bounded | Runtime | Ratio | Extracted share² |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Bertie / separate | v1.8.0 | security | 0 | 350 | 350 | 350 | 0 | 0 | 0 | 0 | 18 | 0 | 696 | 0 | 5468 | 0.502873563 | 12.728603% |
| Bertie / separate | v1.8.0 | all | 0 | 350 | 350 | 350 | 0 | 0 | 0 | 0 | 18 | 0 | 696 | 0 | 5468 | 0.502873563 | 12.728603% |
| Bertie / separate | v1.8.1 | security | 197 | 312 | 509 | 268 | 197 | 57 | 0 | 0 | 18 | 0 | 549 | 0 | 5309 | 0.927140255 | 10.340930% |
| Bertie / separate | v1.8.1 | all | 197 | 312 | 509 | 268 | 197 | 57 | 0 | 0 | 18 | 0 | 549 | 0 | 5309 | 0.927140255 | 10.340930% |
| Bertie / separate | v1.8.1 patches=[] | security | 197 | 312 | 509 | 268 | 197 | 57 | 0 | 0 | 18 | 0 | 549 | 0 | 5309 | 0.927140255 | 10.340930% |
| Bertie / separate | v1.8.1 patches=[] | all | 197 | 312 | 509 | 268 | 197 | 57 | 0 | 0 | 18 | 0 | 549 | 0 | 5309 | 0.927140255 | 10.340930% |
| Bertie / runtime | v1.8.0 | security | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 18 | 0 | 824 | 0 | 5818 | 0.000000000 | 14.162943% |
| Bertie / runtime | v1.8.0 | all | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 18 | 0 | 824 | 0 | 5818 | 0.000000000 | 14.162943% |
| Bertie / runtime | v1.8.1 | security | 197 | 0 | 197 | 0 | 197 | 0 | 0 | 0 | 18 | 0 | 647 | 0 | 5621 | 0.304482226 | 11.510407% |
| Bertie / runtime | v1.8.1 | all | 197 | 0 | 197 | 0 | 197 | 0 | 0 | 0 | 18 | 0 | 647 | 0 | 5621 | 0.304482226 | 11.510407% |
| Bertie / runtime | v1.8.1 patches=[] | security | 197 | 0 | 197 | 0 | 197 | 0 | 0 | 0 | 18 | 0 | 647 | 0 | 5621 | 0.304482226 | 11.510407% |
| Bertie / runtime | v1.8.1 patches=[] | all | 197 | 0 | 197 | 0 | 197 | 0 | 0 | 0 | 18 | 0 | 647 | 0 | 5621 | 0.304482226 | 11.510407% |
| scratch-spec-lemmas / separate | v1.8.0 | security | 4732 | 198 | 4930 | 0 | 0 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 1160 | 11.053811659 | 0.000000% |
| scratch-spec-lemmas / separate | v1.8.0 | all | 4732 | 198 | 4930 | 0 | 0 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 2801 | 11.053811659 | 0.000000% |
| scratch-spec-lemmas / separate | v1.8.1 | security | 4732 | 198 | 4930 | 0 | 0 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 1160 | 11.053811659 | 0.000000% |
| scratch-spec-lemmas / separate | v1.8.1 | all | 4732 | 198 | 4930 | 0 | 0 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 2801 | 11.053811659 | 0.000000% |
| scratch-spec-lemmas / runtime | v1.8.0 | security | 4732 | 0 | 4732 | 0 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 1358 | 7.347826087 | 0.000000% |
| scratch-spec-lemmas / runtime | v1.8.0 | all | 4732 | 0 | 4732 | 0 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 2999 | 7.347826087 | 0.000000% |
| scratch-spec-lemmas / runtime | v1.8.1 | security | 4732 | 0 | 4732 | 0 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 1358 | 7.347826087 | 0.000000% |
| scratch-spec-lemmas / runtime | v1.8.1 | all | 4732 | 0 | 4732 | 0 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 2999 | 7.347826087 | 0.000000% |
| vest / separate | v1.8.0 | security | 23462 | 2405 | 25867 | 0 | 0 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 7960 | 14.755847119 | 0.000000% |
| vest / separate | v1.8.0 | all | 102886 | 6901 | 109787 | 0 | 0 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 80100 | 62.628066172 | 0.000000% |
| vest / separate | v1.8.1 | security | 23462 | 2405 | 25867 | 0 | 0 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 7960 | 14.755847119 | 0.000000% |
| vest / separate | v1.8.1 | all | 102886 | 6901 | 109787 | 0 | 0 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 80100 | 62.628066172 | 0.000000% |
| vest / runtime | v1.8.0 | security | 23462 | 0 | 23462 | 0 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 10365 | 8.192039106 | 0.000000% |
| vest / runtime | v1.8.0 | all | 102886 | 0 | 102886 | 0 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 87001 | 35.923882682 | 0.000000% |
| vest / runtime | v1.8.1 | security | 23462 | 0 | 23462 | 0 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 10365 | 8.192039106 | 0.000000% |
| vest / runtime | v1.8.1 | all | 102886 | 0 | 102886 | 0 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 87001 | 35.923882682 | 0.000000% |

¹ Models are trusted assumptions, excluded from proof. Annotations and cfg inline may overlap; inline total is their union. cfg standalone is part of standalone proof. ² Extracted share is extracted runtime / runtime; security rows give the requested security proven-extracted percentage. New component fields display zero for the baseline, which did not expose them.

## Invariants and independent evidence

- Bertie separate: accepted receipt and credit membership identical; only rule-1 categories/statuses and rule-2 sizes change. patches=[]: all unit/evidence/numeric figures identical; zero-eligible patch inputs and 105 excluded-hunk records disappear.
- Bertie runtime: accepted receipt and credit membership identical; only rule-1 categories/statuses and rule-2 sizes change. patches=[]: all unit/evidence/numeric figures identical; zero-eligible patch inputs and 105 excluded-hunk records disappear.
- scratch-spec-lemmas separate: PASS every preexisting JSON field identical, schema 4 unchanged; the two new ratio fields are zero. Markdown identical after removing the two added zero-component columns.
- scratch-spec-lemmas runtime: PASS every preexisting JSON field identical, schema 4 unchanged; the two new ratio fields are zero. Markdown identical after removing the two added zero-component columns.
- vest separate: PASS every preexisting JSON field identical, schema 4 unchanged; the two new ratio fields are zero. Markdown identical after removing the two added zero-component columns.
- vest runtime: PASS every preexisting JSON field identical, schema 4 unchanged; the two new ratio fields are zero. Markdown identical after removing the two added zero-component columns.

Independent scanner `dev/v181/cfghax.py` matches 18 units / 197 standalone SLoC and 57 potential cfg inline lines in both source scans. Separate: 268 annotations + 57 cfg inline − 13 shared lines = 312 inline; 197 + 312 = 509 proof; 509/549 = 0.927140255. Runtime mode retains rule-2 lines in runtime, annotation/inline cost zero; standalone remains 197. Four helpers in cfg(all(test,hax)) remain test / unverified with their v1.8.0 sizes (57 SLoC total): test classification takes precedence over both proof rules. No cfg(hax) hazard; actual gate audit confirms no new failure reason. Bertie was and remains below 5–15; no gate threshold changed.

## Every moved unit

| Unit | Source | Old category / status / SLoC (separate) | New category / status / SLoC | Runtime SLoC old → new |
| --- | --- | --- | --- | --- |
| `bertie_tls::der::pow256` | `src/der.rs:205` | runtime / proven-extracted / 8 | proof-code / verified / 10 | 10 → 10 |
| `bertie_tls::der::pow256_mono` | `src/der.rs:217` | runtime / proven-extracted / 6 | proof-code / verified / 10 | 10 → 10 |
| `bertie_tls::der::pow256_is_pow2` | `src/der.rs:229` | runtime / proven-extracted / 6 | proof-code / verified / 10 | 10 → 10 |
| `bertie_tls::der::pow256_32_is_bound` | `src/der.rs:241` | runtime / proven-extracted / 4 | proof-code / verified / 6 | 6 → 6 |
| `bertie_tls::der::zero_prefix` | `src/der.rs:249` | runtime / proven-extracted / 3 | proof-code / verified / 4 | 4 → 4 |
| `bertie_tls::der::shifted_eq` | `src/der.rs:255` | runtime / proven-extracted / 3 | proof-code / verified / 9 | 9 → 9 |
| `bertie_tls::der::be_value_prefix_bound` | `src/der.rs:266` | runtime / proven-extracted / 6 | proof-code / verified / 10 | 10 → 10 |
| `bertie_tls::der::be_value_prefix_lower_bound` | `src/der.rs:278` | runtime / proven-extracted / 6 | proof-code / verified / 10 | 10 → 10 |
| `bertie_tls::der::be_value_shift` | `src/der.rs:290` | runtime / proven-extracted / 10 | proof-code / verified / 19 | 19 → 19 |
| `bertie_tls::der::agree` | `src/der.rs:464` | runtime / proven-extracted / 3 | proof-code / verified / 4 | 4 → 4 |
| `bertie_tls::der::be_value_prefix_upper_128` | `src/der.rs:470` | runtime / unverified / 6 | proof-code / unverified / 10 | 10 → 10 |
| `bertie_tls::der::be_value_prefix_lower_128` | `src/der.rs:482` | runtime / unverified / 6 | proof-code / unverified / 10 | 10 → 10 |
| `bertie_tls::der::uint_range` | `src/der.rs:494` | runtime / proven-extracted / 11 | proof-code / verified / 15 | 15 → 15 |
| `bertie_tls::der::uint_len_unique` | `src/der.rs:512` | runtime / proven-extracted / 10 | proof-code / verified / 13 | 13 → 13 |
| `bertie_tls::der::be_value_prefix_injective` | `src/der.rs:527` | runtime / proven-extracted / 6 | proof-code / verified / 10 | 10 → 10 |
| `bertie_tls::der::uint_unique` | `src/der.rs:539` | runtime / proven-extracted / 5 | proof-code / verified / 8 | 8 → 8 |
| `bertie_tls::der::agree_elim` | `src/der.rs:549` | runtime / proven-extracted / 2 | proof-code / verified / 5 | 5 → 5 |
| `bertie_tls::der::der_roundtrip` | `src/der.rs:594` | runtime / proven-extracted / 14 | proof-code / verified / 34 | 34 → 34 |

## Preserved hax-gated tests

| Unit | Source | Category | Status | SLoC before → after |
| --- | --- | --- | --- | ---: |
| `bertie_tls::der::tests::from_hex` | `src/der.rs:639` | test | unverified | 6 → 6 |
| `bertie_tls::der::tests::der_sig_valid_for_some_values` | `src/der.rs:646` | test | unverified | 13 → 13 |
| `bertie_tls::der::tests::known_answers_from_x690` | `src/der.rs:662` | test | unverified | 29 → 29 |
| `bertie_tls::der::tests::spec_distinguishes_values` | `src/der.rs:692` | test | unverified | 9 → 9 |

## Proof-code table

| Ghost mode | Version | Scope | SLoC | Units | Verified | Unverified | Hazard units |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: |
| separate | v1.8.0 | total | 0 | 0 | 0 | 0 | 0 |
| separate | v1.8.0 | security_relevant | 0 | 0 | 0 | 0 | 0 |
| separate | v1.8.0 | everything_else | 0 | 0 | 0 | 0 | 0 |
| separate | v1.8.1 | total | 197 | 18 | 16 | 2 | 0 |
| separate | v1.8.1 | security_relevant | 197 | 18 | 16 | 2 | 0 |
| separate | v1.8.1 | everything_else | 0 | 0 | 0 | 0 | 0 |
| runtime | v1.8.0 | total | 0 | 0 | 0 | 0 | 0 |
| runtime | v1.8.0 | security_relevant | 0 | 0 | 0 | 0 | 0 |
| runtime | v1.8.0 | everything_else | 0 | 0 | 0 | 0 | 0 |
| runtime | v1.8.1 | total | 197 | 18 | 16 | 2 | 0 |
| runtime | v1.8.1 | security_relevant | 197 | 18 | 16 | 2 | 0 |
| runtime | v1.8.1 | everything_else | 0 | 0 | 0 | 0 | 0 |

Existing Bertie gate failures remain: `first-party security proof ratio … is below the acceptance range` and `unattributed file-scope hazard can invalidate runtime proofs`. `bash dev/v181/gates.sh` verifies identical failure reasons in both modes, including patches=[] and baseline/no-new-escape checks. The new proof-code hazard gate stays clear.


## Every changed unit size

| Mode | Unit | Source | Before | After | Reason |
| --- | --- | --- | ---: | ---: | --- |
| separate | `bertie_tls::der::pow256` | `src/der.rs:205` | 8 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::pow256_mono` | `src/der.rs:217` | 6 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::pow256_is_pow2` | `src/der.rs:229` | 6 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::pow256_32_is_bound` | `src/der.rs:241` | 4 | 6 | annotations included in standalone proof |
| separate | `bertie_tls::der::zero_prefix` | `src/der.rs:249` | 3 | 4 | annotations included in standalone proof |
| separate | `bertie_tls::der::shifted_eq` | `src/der.rs:255` | 3 | 9 | annotations included in standalone proof |
| separate | `bertie_tls::der::be_value_prefix_bound` | `src/der.rs:266` | 6 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::be_value_prefix_lower_bound` | `src/der.rs:278` | 6 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::be_value_shift` | `src/der.rs:290` | 10 | 19 | annotations included in standalone proof |
| separate | `bertie_tls::der::encode_uint` | `src/der.rs:336` | 14 | 9 | cfg inline lines leave runtime (overlap counted once) |
| separate | `bertie_tls::der::encode_ecdsa_signature` | `src/der.rs:357` | 22 | 14 | cfg inline lines leave runtime (overlap counted once) |
| separate | `bertie_tls::der::uint_to_32_bytes` | `src/der.rs:388` | 22 | 12 | cfg inline lines leave runtime (overlap counted once) |
| separate | `bertie_tls::der::decode_ecdsa_signature` | `src/der.rs:419` | 41 | 33 | cfg inline lines leave runtime (overlap counted once) |
| separate | `bertie_tls::der::agree` | `src/der.rs:464` | 3 | 4 | annotations included in standalone proof |
| separate | `bertie_tls::der::be_value_prefix_upper_128` | `src/der.rs:470` | 6 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::be_value_prefix_lower_128` | `src/der.rs:482` | 6 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::uint_range` | `src/der.rs:494` | 11 | 15 | annotations included in standalone proof |
| separate | `bertie_tls::der::uint_len_unique` | `src/der.rs:512` | 10 | 13 | annotations included in standalone proof |
| separate | `bertie_tls::der::be_value_prefix_injective` | `src/der.rs:527` | 6 | 10 | annotations included in standalone proof |
| separate | `bertie_tls::der::uint_unique` | `src/der.rs:539` | 5 | 8 | annotations included in standalone proof |
| separate | `bertie_tls::der::agree_elim` | `src/der.rs:549` | 2 | 5 | annotations included in standalone proof |
| separate | `bertie_tls::der::der_sig_unique` | `src/der.rs:556` | 15 | 2 | cfg inline lines leave runtime (overlap counted once) |
| separate | `bertie_tls::der::der_roundtrip` | `src/der.rs:594` | 14 | 34 | annotations included in standalone proof |

## Rule-2 source lines

| Unit | File | cfg inline source lines |
| --- | --- | --- |
| `bertie_tls::der::encode_uint` | `src/der.rs` | 349, 350, 351, 352, 353 |
| `bertie_tls::der::encode_ecdsa_signature` | `src/der.rs` | 373, 374, 375, 376, 377, 378, 379, 380 |
| `bertie_tls::der::uint_to_32_bytes` | `src/der.rs` | 400, 401, 403, 404, 405, 406, 407, 408, 413, 414 |
| `bertie_tls::der::decode_ecdsa_signature` | `src/der.rs` | 453, 454, 455, 456, 457, 458, 459, 460 |
| `bertie_tls::der::der_sig_unique` | `src/der.rs` | 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579, 580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591 |

## Fixture identity checks

| Fixture | Ghost mode | Units | Result |
| --- | --- | ---: | --- |
| classification | separate | 4 | all existing fields identical; added fields zero |
| classification | runtime | 4 | all existing fields identical; added fields zero |
| doc_comments | separate | 1 | all existing fields identical; added fields zero |
| doc_comments | runtime | 1 | all existing fields identical; added fields zero |
| evidence | separate | 6 | all existing fields identical; added fields zero |
| evidence | runtime | 6 | all existing fields identical; added fields zero |
| ghost | separate | 8 | all existing fields identical; added fields zero |
| ghost | runtime | 8 | all existing fields identical; added fields zero |
| hax | separate | 3 | all existing fields identical; added fields zero |
| hax | runtime | 3 | all existing fields identical; added fields zero |
| inventory | separate | 4 | all existing fields identical; added fields zero |
| inventory | runtime | 4 | all existing fields identical; added fields zero |

## Reproduction and limits

Source copy/config preparation and exact baseline archive/build commands are in dev/NOTES.md. Baseline build: `cargo build --release --frozen --manifest-path target/v181/Cargo.toml --target-dir target/v181/baseline-build`; copy release binary to target/v181/baseline. Candidate: `cargo build --release --frozen`, copy release binary to target/v181/candidate. Redirect both builds under logs/v1.8.1.

```sh
python3 dev/v181/replay.py bertie > logs/v1.8.1/m3-bertie.log 2>&1
python3 dev/v181/replay.py verus > logs/v1.8.1/m3-verus.log 2>&1
python3 dev/v181/replay.py fixtures > logs/v1.8.1/m3-fixtures.log 2>&1
bash dev/v181/gates.sh > logs/v1.8.1/m3-final-gates.log 2>&1
python3 dev/v18/patches.py target/v181/bertie target/v181/bertie-separate-after.json target/v181/patches.json > logs/v1.8.1/m3-patches.log 2>&1
python3 dev/v181/compare.py > logs/v1.8.1/m3-compare.log 2>&1
cargo test > logs/v1.8.1/m3-test.log 2>&1
cargo clippy --all-targets -- -D warnings > logs/v1.8.1/m3-clippy.log 2>&1
cargo fmt --check > logs/v1.8.1/m3-fmt.log 2>&1
```

Report schema stays 4; configuration stays 2/3. Two additive ratio fields are requested by the plan; all existing control fields and figures are identical, and added fields are zero. Unit cfg metadata is omitted when zero/false. Markdown adds only the two required component columns on controls. No fresh F*/Verus execution or independent acceptance; one existing ignored performance test remains unrun. No push, tag, bundle, host install or external messages.

## Artifact SHA-256

- `target/v181/baseline`: `8bb02cc0e41d028eb61ba08ee777d8aa916c47d2d9deaf86fd6059ef687bcda3`.
- `target/v181/candidate`: `9b26ad599a02d4cfeb5e91c89ef02e6783e7fa2e25c46b63b279ed41dd3595a5`.
- `target/v181/bertie-proof.toml`: `9b901f913e66469b18b35320bee19989d17cce14653b99ac554138abd84ae20a`.
- `target/v181/bertie-no-patches.toml`: `497eca810e0871d3b3a7b8e5207bba936ebc7d912a12312c23916fced68feb2f`.
- `target/v181/bertie/target/ci-logs/fstar/fstar.log.evidence.json`: `cd054c8d09c1cc831dea020ff521ffa4d4c8bf096475a6bdc934066d4b70e8e3`.
- `target/v181/independent-separate.json`: `cd7493493d4823f6aee69206bd12ca9eea4807a4e5558a8fe35b2c60706148d4`.
- `target/v181/independent-runtime.json`: `963880d7259ba4ba444dbc095b0cf5487784eb49f8753ad3cf5d5bb1a428ea02`.
- `target/v181/bertie-separate-before.json`: `1b8f2321f910933762324075c7d6a19d24732dfd1b4b2ed3605a8a504a7596ef`.
- `target/v181/bertie-separate-after.json`: `2c7d4efcc8787af3e064ea5c679b4fb71ffddd5b39d35c01a148529682045a6c`.
- `target/v181/bertie-runtime-before.json`: `21dba08b7efd70c8050db38c271601c5aa01ca61a71635e4f20393ab14e5168f`.
- `target/v181/bertie-runtime-after.json`: `ca95469c1221cc9c8df721ee7d7a7378c1eb38013d0296a48bee2a628d27eed5`.
