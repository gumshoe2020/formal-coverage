# v1.8.5 paired replay

Baseline: tag `v1.8.4`; candidate: v1.8.5 working tree.
Bertie: staged 9617409 with its own unchanged [ship] configuration.
Original CI receipt/log bytes and all receipt-bound hashes verified; no hax or F* executed.
Both binaries use current stable with RUSTUP_TOOLCHAIN unset: rustc 1.100.0 (0ef1bc386 2026-11-09).

Checks are Codex-run; Andrew/Dudley independent acceptance remains outstanding.

(a) Baseline units exactly equal CI units: PASS.

## Bertie separate

Before: 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078 security runtime SLoC (74.36% proven-extracted).

After: 4027 proven-extracted, 281 tested, 652 unverified, 118 trusted / 5078 security runtime SLoC (79.30% proven-extracted).

| Rust unit | Old → new | SLoC | Numbered definition |
| --- | --- | ---: | --- |
| `bertie_tls::tls13crypto::HashAlgorithm::hash_len` | unverified → proven-extracted | 8 | `Bertie_tls.Tls13crypto.impl_4__hash_len` |
| `bertie_tls::tls13crypto::HashAlgorithm::hmac_tag_len` | unverified → proven-extracted | 4 | `Bertie_tls.Tls13crypto.impl_4__hmac_tag_len` |
| `bertie_tls::tls13crypto::AeadAlgorithm::key_len` | unverified → proven-extracted | 8 | `Bertie_tls.Tls13crypto.impl_2__key_len` |
| `bertie_tls::tls13crypto::AeadAlgorithm::iv_len` | unverified → proven-extracted | 8 | `Bertie_tls.Tls13crypto.impl_2__iv_len` |
| `bertie_tls::tls13crypto::Algorithms::new` | tested → proven-extracted | 17 | `Bertie_tls.Tls13crypto.impl_5__new` |
| `bertie_tls::tls13crypto::Algorithms::validate` | unverified → proven-extracted | 41 | `Bertie_tls.Tls13crypto.impl_5__validate` |
| `bertie_tls::tls13crypto::Algorithms::hash` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__hash` |
| `bertie_tls::tls13crypto::Algorithms::aead` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__aead` |
| `bertie_tls::tls13crypto::Algorithms::signature` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__signature` |
| `bertie_tls::tls13crypto::Algorithms::kem` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__kem` |
| `bertie_tls::tls13crypto::Algorithms::psk_mode` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__psk_mode` |
| `bertie_tls::tls13crypto::Algorithms::zero_rtt` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__zero_rtt` |
| `bertie_tls::tls13crypto::Algorithms::ciphersuite` | tested → proven-extracted | 14 | `Bertie_tls.Tls13crypto.impl_5__ciphersuite` |
| `bertie_tls::tls13crypto::Algorithms::supported_group` | tested → proven-extracted | 11 | `Bertie_tls.Tls13crypto.impl_5__supported_group` |
| `bertie_tls::tls13crypto::Algorithms::signature_algorithm` | unverified → proven-extracted | 7 | `Bertie_tls.Tls13crypto.impl_5__signature_algorithm` |
| `bertie_tls::tls13crypto::Algorithms::check` | unverified → proven-extracted | 7 | `Bertie_tls.Tls13crypto.impl_5__check` |
| `bertie_tls::tls13formats::ContentType::try_from_u8` | unverified → proven-extracted | 9 | `Bertie_tls.Tls13formats.impl_4__try_from_u8` |
| `bertie_tls::tls13utils::Bytes::prefix` | tested → proven-extracted | 10 | `Bertie_tls.Tls13utils.impl_11__prefix` |
| `bertie_tls::tls13utils::Bytes::declassify` | tested → proven-extracted | 8 | `Bertie_tls.Tls13utils.impl_11__declassify` |
| `bertie_tls::tls13utils::Bytes::into_raw` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_11__into_raw` |
| `bertie_tls::tls13utils::Bytes::as_raw` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_11__as_raw` |
| `bertie_tls::tls13utils::Bytes::declassify_array` | tested → proven-extracted | 9 | `Bertie_tls.Tls13utils.impl_3__declassify_array` |
| `bertie_tls::tls13utils::Bytes::new` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_17__new` |
| `bertie_tls::tls13utils::Bytes::new_alloc` | unverified → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__new_alloc` |
| `bertie_tls::tls13utils::Bytes::zeroes` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_17__zeroes` |
| `bertie_tls::tls13utils::Bytes::len` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__len` |
| `bertie_tls::tls13utils::Bytes::push` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__push` |
| `bertie_tls::tls13utils::Bytes::extend_from_slice` | unverified → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__extend_from_slice` |
| `bertie_tls::tls13utils::Bytes::append` | unverified → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__append` |
| `bertie_tls::tls13utils::Bytes::from_slice` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__from_slice` |
| `bertie_tls::tls13utils::Bytes::raw_slice` | unverified → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__raw_slice` |
| `bertie_tls::tls13utils::Bytes::slice_range` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__slice_range` |
| `bertie_tls::tls13utils::Bytes::slice` | unverified → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__slice` |
| `bertie_tls::tls13utils::Bytes::concat` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_17__concat` |
| `bertie_tls::tls13utils::Bytes::concat_array` | unverified → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_17__concat_array` |
| `bertie_tls::tls13utils::Bytes::update_slice` | tested → proven-extracted | 13 | `Bertie_tls.Tls13utils.impl_17__update_slice` |
| `bertie_tls::tls13utils::AppData::new` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_5__new` |
| `bertie_tls::tls13utils::AppData::into_raw` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_5__into_raw` |
| `bertie_tls::tls13utils::AppData::as_raw` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_5__as_raw` |

(b) Every changed unit is an eligible numbered target: PASS.
(c) No unit lost a tier; unchanged unit fields and totals arithmetic: PASS.

## Bertie runtime

Before: 4091 proven-extracted, 447 tested, 792 unverified, 118 trusted / 5448 security runtime SLoC (75.09% proven-extracted).

After: 4382 proven-extracted, 285 tested, 663 unverified, 118 trusted / 5448 security runtime SLoC (80.43% proven-extracted).

| Rust unit | Old → new | SLoC | Numbered definition |
| --- | --- | ---: | --- |
| `bertie_tls::tls13crypto::HashAlgorithm::hash_len` | unverified → proven-extracted | 9 | `Bertie_tls.Tls13crypto.impl_4__hash_len` |
| `bertie_tls::tls13crypto::HashAlgorithm::hmac_tag_len` | unverified → proven-extracted | 4 | `Bertie_tls.Tls13crypto.impl_4__hmac_tag_len` |
| `bertie_tls::tls13crypto::AeadAlgorithm::key_len` | unverified → proven-extracted | 8 | `Bertie_tls.Tls13crypto.impl_2__key_len` |
| `bertie_tls::tls13crypto::AeadAlgorithm::iv_len` | unverified → proven-extracted | 8 | `Bertie_tls.Tls13crypto.impl_2__iv_len` |
| `bertie_tls::tls13crypto::Algorithms::new` | tested → proven-extracted | 17 | `Bertie_tls.Tls13crypto.impl_5__new` |
| `bertie_tls::tls13crypto::Algorithms::validate` | unverified → proven-extracted | 41 | `Bertie_tls.Tls13crypto.impl_5__validate` |
| `bertie_tls::tls13crypto::Algorithms::hash` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__hash` |
| `bertie_tls::tls13crypto::Algorithms::aead` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__aead` |
| `bertie_tls::tls13crypto::Algorithms::signature` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__signature` |
| `bertie_tls::tls13crypto::Algorithms::kem` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__kem` |
| `bertie_tls::tls13crypto::Algorithms::psk_mode` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__psk_mode` |
| `bertie_tls::tls13crypto::Algorithms::zero_rtt` | tested → proven-extracted | 3 | `Bertie_tls.Tls13crypto.impl_5__zero_rtt` |
| `bertie_tls::tls13crypto::Algorithms::ciphersuite` | tested → proven-extracted | 17 | `Bertie_tls.Tls13crypto.impl_5__ciphersuite` |
| `bertie_tls::tls13crypto::Algorithms::supported_group` | tested → proven-extracted | 14 | `Bertie_tls.Tls13crypto.impl_5__supported_group` |
| `bertie_tls::tls13crypto::Algorithms::signature_algorithm` | unverified → proven-extracted | 10 | `Bertie_tls.Tls13crypto.impl_5__signature_algorithm` |
| `bertie_tls::tls13crypto::Algorithms::check` | unverified → proven-extracted | 11 | `Bertie_tls.Tls13crypto.impl_5__check` |
| `bertie_tls::tls13formats::ContentType::try_from_u8` | unverified → proven-extracted | 9 | `Bertie_tls.Tls13formats.impl_4__try_from_u8` |
| `bertie_tls::tls13utils::Bytes::prefix` | tested → proven-extracted | 12 | `Bertie_tls.Tls13utils.impl_11__prefix` |
| `bertie_tls::tls13utils::Bytes::declassify` | tested → proven-extracted | 10 | `Bertie_tls.Tls13utils.impl_11__declassify` |
| `bertie_tls::tls13utils::Bytes::into_raw` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_11__into_raw` |
| `bertie_tls::tls13utils::Bytes::as_raw` | tested → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_11__as_raw` |
| `bertie_tls::tls13utils::Bytes::declassify_array` | tested → proven-extracted | 9 | `Bertie_tls.Tls13utils.impl_3__declassify_array` |
| `bertie_tls::tls13utils::Bytes::new` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_17__new` |
| `bertie_tls::tls13utils::Bytes::new_alloc` | unverified → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_17__new_alloc` |
| `bertie_tls::tls13utils::Bytes::zeroes` | tested → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__zeroes` |
| `bertie_tls::tls13utils::Bytes::len` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_17__len` |
| `bertie_tls::tls13utils::Bytes::push` | tested → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__push` |
| `bertie_tls::tls13utils::Bytes::extend_from_slice` | unverified → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__extend_from_slice` |
| `bertie_tls::tls13utils::Bytes::append` | unverified → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__append` |
| `bertie_tls::tls13utils::Bytes::from_slice` | tested → proven-extracted | 4 | `Bertie_tls.Tls13utils.impl_17__from_slice` |
| `bertie_tls::tls13utils::Bytes::raw_slice` | unverified → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__raw_slice` |
| `bertie_tls::tls13utils::Bytes::slice_range` | tested → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__slice_range` |
| `bertie_tls::tls13utils::Bytes::slice` | unverified → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__slice` |
| `bertie_tls::tls13utils::Bytes::concat` | tested → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__concat` |
| `bertie_tls::tls13utils::Bytes::concat_array` | unverified → proven-extracted | 5 | `Bertie_tls.Tls13utils.impl_17__concat_array` |
| `bertie_tls::tls13utils::Bytes::update_slice` | tested → proven-extracted | 15 | `Bertie_tls.Tls13utils.impl_17__update_slice` |
| `bertie_tls::tls13utils::AppData::new` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_5__new` |
| `bertie_tls::tls13utils::AppData::into_raw` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_5__into_raw` |
| `bertie_tls::tls13utils::AppData::as_raw` | tested → proven-extracted | 3 | `Bertie_tls.Tls13utils.impl_5__as_raw` |

(b) Every changed unit is an eligible numbered target: PASS.
(c) No unit lost a tier; unchanged unit fields and totals arithmetic: PASS.

## Controls

- scratch-spec-lemmas separate: exact JSON + Markdown equality PASS
- scratch-spec-lemmas runtime: exact JSON + Markdown equality PASS
- vest separate: exact JSON + Markdown equality PASS
- vest runtime: exact JSON + Markdown equality PASS
- fixture-classification separate: exact JSON + Markdown equality PASS
- fixture-classification runtime: exact JSON + Markdown equality PASS
- fixture-doc_comments separate: exact JSON + Markdown equality PASS
- fixture-doc_comments runtime: exact JSON + Markdown equality PASS
- fixture-evidence separate: exact JSON + Markdown equality PASS
- fixture-evidence runtime: exact JSON + Markdown equality PASS
- fixture-ghost separate: exact JSON + Markdown equality PASS
- fixture-ghost runtime: exact JSON + Markdown equality PASS
- fixture-hax separate: exact JSON + Markdown equality PASS
- fixture-hax runtime: exact JSON + Markdown equality PASS
- fixture-inventory separate: exact JSON + Markdown equality PASS
- fixture-inventory runtime: exact JSON + Markdown equality PASS

(d) All v1.8.4 controls exact JSON + Markdown equality: PASS.

## Rerun

`python3 dev/v185/replay.py` rebuilds both binaries and replaces target/v185.
Requires staged target/v185-inputs, release archives and populated control Cargo caches.
Host target is used for replay; check (a) compares every full CI unit without normalization.
