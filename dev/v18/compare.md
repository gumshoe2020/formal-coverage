# formal-coverage v1.8 paired replay

Generated 2026-11-21T20:23:23.994470-05:00.

Bertie source: read-only `bertie-deps` at `f646099cee002c1d998b0e8c160886ef838f4f69`, copied under `target/v18/bertie` along with the existing `target/ci-logs/fstar/fstar.log` and receipt. Baseline: pre-v1.8 commit `218e190`, release binary `target/v18/baseline`; candidate: v1.8.0 release binary `target/v18/candidate`. All other source checkouts remain untouched. No extraction or verifier was executed.

Bertie uses CI’s `report . --config CONFIG --fstar-log target/ci-logs/fstar/fstar.log --json OUT.json --markdown OUT.md`, in compiled-files mode. Baseline CONFIG is the unchanged original; candidate CONFIG is a temp copy under `target/v18/` plus the plan’s `[fstar.proof]` section. Both retain 5–15 and accept the identical receipt. Candidate also runs runtime ghost mode.

scratch-server `2fa5eb78a6f96a8433229dc64ec4b82c3e5f9bc2` and Vest `ce6705fad1af476e828feae6621d0bcf4d0c1665` use the same archived exports, configs, retained receipts, whole-crate sizes and target triples as the v1.6 replays (`scripts/v15_replay.py`). Targets: scratch aarch64-apple-darwin, Vest aarch64-unknown-linux-gnu. This is a report-only replay; whole-crate sizing does not invoke cargo check. Earlier numeric release figures may differ because v1.7.3 removed docs; the paired control here is v1.7.3 on identical inputs.

## Ratios and components

| Project / mode / version | Scope | Standalone | Inline | F* annotations¹ | Patches | Files | Models² | Proven | Extracted³ | Bounded | Runtime | Ratio |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Bertie / separate / v1.7.3 | security | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5818 | n/a |
| Bertie / separate / v1.7.3 | all | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 5818 | n/a |
| Bertie / separate / v1.8.0 | security | 0 | 350 | 350 | 0 | 0 | 18 | 0 | 696 | 0 | 5468 | 0.502873563 |
| Bertie / separate / v1.8.0 | all | 0 | 350 | 350 | 0 | 0 | 18 | 0 | 696 | 0 | 5468 | 0.502873563 |
| Bertie / runtime / v1.8.0 | security | 0 | 0 | 0 | 0 | 0 | 18 | 0 | 824 | 0 | 5818 | 0.000000000 |
| Bertie / runtime / v1.8.0 | all | 0 | 0 | 0 | 0 | 0 | 18 | 0 | 824 | 0 | 5818 | 0.000000000 |
| scratch-spec-lemmas / separate / v1.7.3 | security | 4732 | 198 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 1160 | 11.053811659 |
| scratch-spec-lemmas / separate / v1.7.3 | all | 4732 | 198 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 2801 | 11.053811659 |
| scratch-spec-lemmas / separate / v1.8.0 | security | 4732 | 198 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 1160 | 11.053811659 |
| scratch-spec-lemmas / separate / v1.8.0 | all | 4732 | 198 | 0 | 0 | 0 | 0 | 446 | 0 | 0 | 2801 | 11.053811659 |
| scratch-spec-lemmas / runtime / v1.7.3 | security | 4732 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 1358 | 7.347826087 |
| scratch-spec-lemmas / runtime / v1.7.3 | all | 4732 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 2999 | 7.347826087 |
| scratch-spec-lemmas / runtime / v1.8.0 | security | 4732 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 1358 | 7.347826087 |
| scratch-spec-lemmas / runtime / v1.8.0 | all | 4732 | 0 | 0 | 0 | 0 | 0 | 644 | 0 | 0 | 2999 | 7.347826087 |
| vest / separate / v1.7.3 | security | 23462 | 2405 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 7960 | 14.755847119 |
| vest / separate / v1.7.3 | all | 102886 | 6901 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 80100 | 62.628066172 |
| vest / separate / v1.8.0 | security | 23462 | 2405 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 7960 | 14.755847119 |
| vest / separate / v1.8.0 | all | 102886 | 6901 | 0 | 0 | 0 | 0 | 1753 | 0 | 0 | 80100 | 62.628066172 |
| vest / runtime / v1.7.3 | security | 23462 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 10365 | 8.192039106 |
| vest / runtime / v1.7.3 | all | 102886 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 87001 | 35.923882682 |
| vest / runtime / v1.8.0 | security | 23462 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 10365 | 8.192039106 |
| vest / runtime / v1.8.0 | all | 102886 | 0 | 0 | 0 | 0 | 0 | 2864 | 0 | 0 | 87001 | 35.923882682 |

¹ Annotations are a subset of inline proof, counted once. ² Models are trusted assumptions, excluded from proof/ratio. ³ The old report has no extracted denominator field: its displayed 0 means excluded from the old ratio, not zero evidence credit. Bertie’s baseline has 824 extracted security/all-first-party runtime SLoC.

## Invariants and independent counts

- Bertie: PASS; all 469 identities/statuses and receipt credit membership unchanged; exactly 101 annotation-only size changes. Separate security runtime 5,818 → 5,468; extracted runtime 824 → 696. Runtime mode preserves every baseline size/status.
- Independent annotation scanner: exact 350 security / 350 all-first-party SLoC; per-unit source lines retained in `target/v18/annotations-final.json`. No line-by-line differences.
- Independent patch audit: 1322 raw added code lines, 105 hunks, 8 modules; exact eligible match of zero per module. All target legacy `Bertie.*`, absent from the `Bertie_tls.*` receipt. Names were not rewritten. Every excluded hunk/reason is in JSON and Markdown reports.
- Hand-written file cost 0 (none configured); models 18 SLoC, excluded from ratio. Cost moves Bertie from n/a to 350/696 = 0.502873563; below the unchanged 5–15 range. No threshold or tier adjustment.
- scratch-spec-lemmas, separate: PASS; every pre-existing report field identical except schema 3 → 4. No extracted units, no size changes; ratio and every Verus figure unchanged.
- scratch-spec-lemmas, runtime: PASS; every pre-existing report field identical except schema 3 → 4. No extracted units, no size changes; ratio and every Verus figure unchanged.
- vest, separate: PASS; every pre-existing report field identical except schema 3 → 4. No extracted units, no size changes; ratio and every Verus figure unchanged.
- vest, runtime: PASS; every pre-existing report field identical except schema 3 → 4. No extracted units, no size changes; ratio and every Verus figure unchanged.

Unknown hax paths retained for review: `hax_lib::Abstraction`, `hax_lib::ToInt`, `hax_lib::fstar::verification_status`, `hax_lib::proverif::before`, `hax_lib::proverif::replace`, `hax_lib::proverif::replace_body`, `hax_lib::pv_constructor`.

## Unit status census (all inventory categories)

| Project | Units | Status counts (same before/after) | Tier changes | Size changes |
| --- | ---: | --- | ---: | ---: |
| Bertie | 469 | hazard=2, proven-extracted=47, tested=80, trusted=118, unverified=222 | 0 | 101 |
| scratch-spec-lemmas | 360 | proven=38, tested=73, unverified=49, verified=200 | 0 | 0 |
| vest | 19498 | hazard=7813, proven=176, tested=1912, trusted=33, unverified=5793, verified=3771 | 0 | 0 |

## Fixture replay

Whole-crate paired static reports; generated receipt variants are covered by the full Rust test suite. These runs do not execute fixture Rust or verifiers.

| Fixture | Units | Tier changes | Size changes |
| --- | ---: | ---: | ---: |
| classification | 4 | 0 | 0 |
| doc_comments | 1 | 0 | 0 |
| evidence | 6 | 0 | 0 |
| ghost | 8 | 0 | 0 |
| hax | 3 | 0 | 1 |
| inventory | 4 | 0 | 0 |

## Independent patch additions per target module

| Module | Raw added code SLoC | Eligible proof SLoC | Excluded hunks |
| --- | ---: | ---: | ---: |
| Bertie.Tls13api | 13 | 0 | 9 |
| Bertie.Tls13cert | 128 | 0 | 8 |
| Bertie.Tls13crypto | 32 | 0 | 7 |
| Bertie.Tls13formats | 963 | 0 | 41 |
| Bertie.Tls13formats.Handshake_data | 9 | 0 | 6 |
| Bertie.Tls13handshake | 15 | 0 | 8 |
| Bertie.Tls13record | 35 | 0 | 18 |
| Bertie.Tls13utils | 127 | 0 | 8 |

## Every changed unit size

Only hax annotation-only lines move. All listed tiers are unchanged. No scratch-server or Vest sizes change.

| Unit | Source | Before | After | Tier |
| --- | --- | ---: | ---: | --- |
| `bertie_tls::der::agree` | `src/der.rs:464` | 4 | 3 | proven-extracted |
| `bertie_tls::der::agree_elim` | `src/der.rs:549` | 5 | 2 | proven-extracted |
| `bertie_tls::der::be_value_prefix` | `src/der.rs:90` | 9 | 7 | proven-extracted |
| `bertie_tls::der::be_value_prefix_bound` | `src/der.rs:266` | 10 | 6 | proven-extracted |
| `bertie_tls::der::be_value_prefix_injective` | `src/der.rs:527` | 10 | 6 | proven-extracted |
| `bertie_tls::der::be_value_prefix_lower_128` | `src/der.rs:482` | 10 | 6 | unverified |
| `bertie_tls::der::be_value_prefix_lower_bound` | `src/der.rs:278` | 10 | 6 | proven-extracted |
| `bertie_tls::der::be_value_prefix_upper_128` | `src/der.rs:470` | 10 | 6 | unverified |
| `bertie_tls::der::be_value_shift` | `src/der.rs:290` | 19 | 10 | proven-extracted |
| `bertie_tls::der::decode_ecdsa_signature` | `src/der.rs:419` | 43 | 41 | proven-extracted |
| `bertie_tls::der::der_roundtrip` | `src/der.rs:594` | 34 | 14 | proven-extracted |
| `bertie_tls::der::der_sig_unique` | `src/der.rs:556` | 31 | 15 | proven-extracted |
| `bertie_tls::der::encode_ecdsa_signature` | `src/der.rs:357` | 25 | 22 | proven-extracted |
| `bertie_tls::der::encode_uint` | `src/der.rs:336` | 17 | 14 | proven-extracted |
| `bertie_tls::der::encoder_correct` | `src/der.rs:161` | 7 | 6 | proven-extracted |
| `bertie_tls::der::parser_complete` | `src/der.rs:181` | 17 | 2 | proven-extracted |
| `bertie_tls::der::pow256` | `src/der.rs:205` | 10 | 8 | proven-extracted |
| `bertie_tls::der::pow256_32_is_bound` | `src/der.rs:241` | 6 | 4 | proven-extracted |
| `bertie_tls::der::pow256_is_pow2` | `src/der.rs:229` | 10 | 6 | proven-extracted |
| `bertie_tls::der::pow256_mono` | `src/der.rs:217` | 10 | 6 | proven-extracted |
| `bertie_tls::der::shifted_eq` | `src/der.rs:255` | 9 | 3 | proven-extracted |
| `bertie_tls::der::skip_zeros` | `src/der.rs:313` | 17 | 11 | proven-extracted |
| `bertie_tls::der::two_pow_256` | `src/der.rs:84` | 4 | 3 | tested |
| `bertie_tls::der::uint_len_unique` | `src/der.rs:512` | 13 | 10 | proven-extracted |
| `bertie_tls::der::uint_range` | `src/der.rs:494` | 15 | 11 | proven-extracted |
| `bertie_tls::der::uint_to_32_bytes` | `src/der.rs:388` | 28 | 22 | proven-extracted |
| `bertie_tls::der::uint_unique` | `src/der.rs:539` | 8 | 5 | proven-extracted |
| `bertie_tls::der::zero_prefix` | `src/der.rs:249` | 4 | 3 | proven-extracted |
| `bertie_tls::tls13cert::check_tag` | `src/tls13cert.rs:151` | 14 | 11 | trusted |
| `bertie_tls::tls13cert::ecdsa_public_key` | `src/tls13cert.rs:378` | 10 | 9 | trusted |
| `bertie_tls::tls13cert::length` | `src/tls13cert.rs:101` | 18 | 15 | trusted |
| `bertie_tls::tls13cert::length_length` | `src/tls13cert.rs:67` | 14 | 11 | trusted |
| `bertie_tls::tls13cert::long_length` | `src/tls13cert.rs:51` | 14 | 11 | trusted |
| `bertie_tls::tls13cert::short_length` | `src/tls13cert.rs:86` | 14 | 11 | trusted |
| `bertie_tls::tls13crypto::Algorithms::check` | `src/tls13crypto.rs:979` | 11 | 7 | unverified |
| `bertie_tls::tls13crypto::Algorithms::ciphersuite` | `src/tls13crypto.rs:935` | 11 | 8 | tested |
| `bertie_tls::tls13crypto::Algorithms::signature_algorithm` | `src/tls13crypto.rs:966` | 10 | 7 | unverified |
| `bertie_tls::tls13crypto::Algorithms::supported_group` | `src/tls13crypto.rs:949` | 14 | 11 | tested |
| `bertie_tls::tls13crypto::HashAlgorithm::hash_len` | `src/tls13crypto.rs:153` | 9 | 8 | trusted |
| `bertie_tls::tls13formats::Transcript::transcript_hash_without_client_hello` | `src/tls13formats.rs:1420` | 17 | 16 | unverified |
| `bertie_tls::tls13formats::build_server_name` | `src/tls13formats.rs:59` | 11 | 8 | unverified |
| `bertie_tls::tls13formats::certificate_verify` | `src/tls13formats.rs:1131` | 34 | 26 | trusted |
| `bertie_tls::tls13formats::check_extension` | `src/tls13formats.rs:254` | 70 | 67 | unverified |
| `bertie_tls::tls13formats::check_extensions_slice` | `src/tls13formats.rs:352` | 11 | 10 | unverified |
| `bertie_tls::tls13formats::check_server_extension` | `src/tls13formats.rs:328` | 23 | 20 | unverified |
| `bertie_tls::tls13formats::check_server_extensions` | `src/tls13formats.rs:374` | 10 | 9 | unverified |
| `bertie_tls::tls13formats::client_hello` | `src/tls13formats.rs:552` | 85 | 72 | trusted |
| `bertie_tls::tls13formats::find_key_share` | `src/tls13formats.rs:150` | 12 | 11 | unverified |
| `bertie_tls::tls13formats::finished` | `src/tls13formats.rs:1218` | 18 | 10 | trusted |
| `bertie_tls::tls13formats::get_psk_extensions` | `src/tls13formats.rs:537` | 14 | 11 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::as_handshake_message` | `src/tls13formats/handshake_data.rs:186` | 18 | 15 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::find_handshake_message` | `src/tls13formats/handshake_data.rs:251` | 22 | 20 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::from_bytes` | `src/tls13formats/handshake_data.rs:152` | 9 | 6 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::len` | `src/tls13formats/handshake_data.rs:163` | 5 | 4 | trusted |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::next_handshake_message` | `src/tls13formats/handshake_data.rs:211` | 13 | 10 | unverified |
| `bertie_tls::tls13formats::handshake_data::from_bytes_inner` | `src/tls13formats/handshake_data.rs:132` | 12 | 9 | trusted |
| `bertie_tls::tls13formats::handshake_record` | `src/tls13formats.rs:1327` | 8 | 5 | unverified |
| `bertie_tls::tls13formats::parse_client_hello` | `src/tls13formats.rs:733` | 141 | 138 | trusted |
| `bertie_tls::tls13formats::server_certificate` | `src/tls13formats.rs:1059` | 28 | 20 | trusted |
| `bertie_tls::tls13formats::server_hello` | `src/tls13formats.rs:876` | 49 | 39 | trusted |
| `bertie_tls::tls13formats::set_client_hello_binder` | `src/tls13formats.rs:650` | 59 | 56 | trusted |
| `bertie_tls::tls13formats::supported_versions` | `src/tls13formats.rs:87` | 6 | 3 | unverified |
| `bertie_tls::tls13handshake::compute_psk_binder_zero_rtt` | `src/tls13handshake.rs:189` | 49 | 48 | unverified |
| `bertie_tls::tls13record::derive_iv_ctr` | `src/tls13record.rs:83` | 14 | 11 | trusted |
| `bertie_tls::tls13record::padlen` | `src/tls13record.rs:124` | 10 | 7 | trusted |
| `bertie_tls::tls13utils::<constC:usize>From<&[u8 ; C]>forBytes::from` | `src/tls13utils.rs:293` | 4 | 3 | tested |
| `bertie_tls::tls13utils::<constC:usize>From<[u8 ; C]>forBytes::from` | `src/tls13utils.rs:285` | 4 | 3 | tested |
| `bertie_tls::tls13utils::Bytes::append` | `src/tls13utils.rs:456` | 4 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::as_raw` | `src/tls13utils.rs:252` | 5 | 4 | tested |
| `bertie_tls::tls13utils::Bytes::concat` | `src/tls13utils.rs:508` | 4 | 3 | tested |
| `bertie_tls::tls13utils::Bytes::len` | `src/tls13utils.rs:440` | 4 | 3 | tested |
| `bertie_tls::tls13utils::Bytes::new_alloc` | `src/tls13utils.rs:427` | 4 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::prefix` | `src/tls13utils.rs:226` | 11 | 10 | trusted |
| `bertie_tls::tls13utils::Bytes::raw_slice` | `src/tls13utils.rs:487` | 5 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::slice` | `src/tls13utils.rs:501` | 5 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::slice_range` | `src/tls13utils.rs:494` | 5 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::zeroes` | `src/tls13utils.rs:433` | 5 | 4 | tested |
| `bertie_tls::tls13utils::From<&[u8]>forBytes::from` | `src/tls13utils.rs:270` | 4 | 3 | tested |
| `bertie_tls::tls13utils::_update_at_usize_bytes_test` | `src/tls13utils.rs:366` | 14 | 5 | unverified |
| `bertie_tls::tls13utils::bytes` | `src/tls13utils.rs:342` | 4 | 3 | trusted |
| `bertie_tls::tls13utils::bytes1` | `src/tls13utils.rs:347` | 4 | 3 | trusted |
| `bertie_tls::tls13utils::bytes2` | `src/tls13utils.rs:352` | 4 | 3 | trusted |
| `bertie_tls::tls13utils::check` | `src/tls13utils.rs:572` | 11 | 7 | trusted |
| `bertie_tls::tls13utils::check_eq` | `src/tls13utils.rs:664` | 7 | 4 | trusted |
| `bertie_tls::tls13utils::check_eq1` | `src/tls13utils.rs:590` | 11 | 8 | trusted |
| `bertie_tls::tls13utils::check_eq_option` | `src/tls13utils.rs:674` | 11 | 8 | trusted |
| `bertie_tls::tls13utils::check_eq_with_slice` | `src/tls13utils.rs:640` | 21 | 18 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u16` | `src/tls13utils.rs:903` | 6 | 3 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u16_slice` | `src/tls13utils.rs:891` | 11 | 8 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u24` | `src/tls13utils.rs:915` | 10 | 7 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u8` | `src/tls13utils.rs:879` | 7 | 4 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u8_slice` | `src/tls13utils.rs:868` | 10 | 7 | trusted |
| `bertie_tls::tls13utils::core::ops::Index<Range<usize>>forBytes::index` | `src/tls13utils.rs:400` | 5 | 4 | unverified |
| `bertie_tls::tls13utils::encode_length_u16` | `src/tls13utils.rs:732` | 16 | 13 | trusted |
| `bertie_tls::tls13utils::encode_length_u24` | `src/tls13utils.rs:754` | 17 | 14 | trusted |
| `bertie_tls::tls13utils::encode_length_u8` | `src/tls13utils.rs:711` | 15 | 12 | trusted |
| `bertie_tls::tls13utils::length_u16_encoded` | `src/tls13utils.rs:826` | 8 | 4 | trusted |
| `bertie_tls::tls13utils::length_u16_encoded_slice` | `src/tls13utils.rs:800` | 19 | 15 | trusted |
| `bertie_tls::tls13utils::length_u24_encoded` | `src/tls13utils.rs:841` | 20 | 16 | trusted |
| `bertie_tls::tls13utils::length_u8_encoded` | `src/tls13utils.rs:777` | 16 | 12 | trusted |
| `bertie_tls::tls13utils::tlserr` | `src/tls13utils.rs:53` | 5 | 4 | unverified |
| `hax_fixture::round_trip` | `src/lib.rs:3` | 2 | 1 | unverified |

## Reproduction and limits

Build the pre-v1.8 baseline with `git archive 218e190` exported under `target/v18/baseline-src`, then `cargo build --release --frozen --manifest-path target/v18/baseline-src/Cargo.toml --target-dir target/v18/baseline-build > logs/v1.8/baseline-rebuild.log 2>&1`; copy its release binary to `target/v18/baseline`. Build candidate with `cargo build --release --frozen > logs/v1.8/m4-build.log 2>&1` and copy `target/release/formal-coverage` to `target/v18/candidate`. Bertie preparation is in `dev/NOTES.md` (copy source/receipt only).

```sh
python3 dev/v18/replay.py bertie > logs/v1.8/m4-bertie.log 2>&1
python3 dev/v18/replay.py verus > logs/v1.8/m4-verus.log 2>&1
python3 dev/v18/replay.py fixtures > logs/v1.8/m4-fixtures.log 2>&1
python3 dev/v18/annotations.py target/v18/bertie target/v18/bertie-before.json target/v18/bertie-after.json target/v18/annotations-final.json > logs/v1.8/m4-annotations.log 2>&1
python3 dev/v18/patches.py target/v18/bertie target/v18/bertie-after.json target/v18/patches-final.json > logs/v1.8/m4-patches.log 2>&1
python3 dev/v18/compare.py > logs/v1.8/m4-compare.log 2>&1
cargo test > logs/v1.8/m4-test.log 2>&1
cargo clippy --all-targets -- -D warnings > logs/v1.8/m4-clippy.log 2>&1
cargo fmt --check > logs/v1.8/m4-fmt.log 2>&1
```

Report schema 3 → 4 is deliberate; configuration remains schema 2/3. Local checks and replays are not independent acceptance. The existing ignored release performance test is not run. No fresh F*/Verus proof execution, host installs, push, tag, bundle, or external messages.

## Artifact SHA-256

- `target/v18/baseline`: `fef45de51bfc0b6c65179c2a82c50dc3ef36460a8b3cc3a0eb99040d274bc329`.
- `target/v18/candidate`: `8bb02cc0e41d028eb61ba08ee777d8aa916c47d2d9deaf86fd6059ef687bcda3`.
- `target/v18/bertie-before.json`: `735e772a98dea6384f0c8ca79da232da3af20e2ca1a837a905515fb31e6c1c82`.
- `target/v18/bertie-after.json`: `1b8f2321f910933762324075c7d6a19d24732dfd1b4b2ed3605a8a504a7596ef`.
- `target/v18/annotations-final.json`: `3ff27e86bd1dc301b272a37df9615a21dfb69761674e8d222abd46dde21a3797`.
- `target/v18/patches-final.json`: `14ff623b3589f764e72ed7de45bb6aba8d103f598bf692a20812ac890b72fc7a`.
- `target/v18/bertie-proof.toml`: `9b901f913e66469b18b35320bee19989d17cce14653b99ac554138abd84ae20a`.
- `target/v18/bertie/target/ci-logs/fstar/fstar.log.evidence.json`: `cd054c8d09c1cc831dea020ff521ffa4d4c8bf096475a6bdc934066d4b70e8e3`.
