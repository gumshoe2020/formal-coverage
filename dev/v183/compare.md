# v1.8.3 paired replay

Baseline: tag `v1.8.2`; candidate: v1.8.3. Bertie: `c66a986` archive under `target/v183/bertie`.
Both binaries use the same original CI log/receipt and reconstructed, hash-identical extraction/library inputs.
Bertie uses compiled-files, CI target x86_64-unknown-linux-gnu; local dependency fallbacks are identical before/after.
Controls reuse v1.8.2 replay settings. No external checkout is modified.
Fresh fallback verification also passed all 18 strict implementation modules (exit 0); paired reports retain original CI evidence.

CI baseline PASS: all 510 unit entries (506 distinct IDs) match; 3635 proven-extracted, 452 tested, 1197 unverified, 118 trusted / 5402 security SLoC.

## Controls

- scratch-spec-lemmas separate: exact JSON and Markdown equality PASS
- scratch-spec-lemmas runtime: exact JSON and Markdown equality PASS
- vest separate: exact JSON and Markdown equality PASS
- vest runtime: exact JSON and Markdown equality PASS
- fixture-classification separate: exact JSON and Markdown equality PASS
- fixture-classification runtime: exact JSON and Markdown equality PASS
- fixture-doc_comments separate: exact JSON and Markdown equality PASS
- fixture-doc_comments runtime: exact JSON and Markdown equality PASS
- fixture-evidence separate: exact JSON and Markdown equality PASS
- fixture-evidence runtime: exact JSON and Markdown equality PASS
- fixture-ghost separate: exact JSON and Markdown equality PASS
- fixture-ghost runtime: exact JSON and Markdown equality PASS
- fixture-hax separate: exact JSON and Markdown equality PASS
- fixture-hax runtime: exact JSON and Markdown equality PASS
- fixture-inventory separate: exact JSON and Markdown equality PASS
- fixture-inventory runtime: exact JSON and Markdown equality PASS

## Bertie, separate

Security proven-extracted: 3635/5402 (67.29%) → 3776/5402 (69.90%).
5 changed unit statuses; all sizes, categories, inventories, trust, hazards, Verus and non-F* evidence unchanged.

| Unit | Old → new | Runtime SLoC | Rule |
| --- | --- | ---: | --- |
| `bertie_tls::tls13keyscheduler::key_schedule::XPD` | unverified → proven-extracted | 69 | R1 uppercase `v_` |
| `bertie_tls::tls13keyscheduler::key_schedule::XTR` | unverified → proven-extracted | 57 | R1 uppercase `v_` |
| `bertie_tls::tls13utils::U8` | tested → proven-extracted | 5 | R1 uppercase `v_` |
| `bertie_tls::tls13utils::U16` | tested → proven-extracted | 5 | R1 uppercase `v_` |
| `bertie_tls::tls13utils::U32` | unverified → proven-extracted | 5 | R1 uppercase `v_` |

Only F* mappings/classifications, credited-unit lists and status-derived summaries/ratios differ; all other report fields are equal.

## Bertie, runtime

Security proven-extracted: 3950/5772 (68.43%) → 4091/5772 (70.88%).
5 changed unit statuses; all sizes, categories, inventories, trust, hazards, Verus and non-F* evidence unchanged.

| Unit | Old → new | Runtime SLoC | Rule |
| --- | --- | ---: | --- |
| `bertie_tls::tls13keyscheduler::key_schedule::XPD` | unverified → proven-extracted | 69 | R1 uppercase `v_` |
| `bertie_tls::tls13keyscheduler::key_schedule::XTR` | unverified → proven-extracted | 57 | R1 uppercase `v_` |
| `bertie_tls::tls13utils::U8` | tested → proven-extracted | 5 | R1 uppercase `v_` |
| `bertie_tls::tls13utils::U16` | tested → proven-extracted | 5 | R1 uppercase `v_` |
| `bertie_tls::tls13utils::U32` | unverified → proven-extracted | 5 | R1 uppercase `v_` |

Only F* mappings/classifications, credited-unit lists and status-derived summaries/ratios differ; all other report fields are equal.

## Remaining unmapped .fst definitions

62 definitions, identical in both modes. Apparent reasons only; no additional mapping fixes.

### trait impl (54)

| Definition | Location | Apparent reason |
| --- | --- | --- |
| `Bertie_tls.Server.impl__new` | `fresh/Bertie_tls.Server.fst:12` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13cert.impl` | `fresh/Bertie_tls.Tls13cert.fst:13` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13cert.impl_1` | `fresh/Bertie_tls.Tls13cert.fst:21` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_4__hash_len` | `fresh/Bertie_tls.Tls13crypto.fst:50` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_4__hmac_tag_len` | `fresh/Bertie_tls.Tls13crypto.fst:56` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_2__key_len` | `fresh/Bertie_tls.Tls13crypto.fst:172` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_2__iv_len` | `fresh/Bertie_tls.Tls13crypto.fst:178` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__new` | `fresh/Bertie_tls.Tls13crypto.fst:814` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__validate` | `fresh/Bertie_tls.Tls13crypto.fst:832` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__hash` | `fresh/Bertie_tls.Tls13crypto.fst:867` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__aead` | `fresh/Bertie_tls.Tls13crypto.fst:870` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__signature` | `fresh/Bertie_tls.Tls13crypto.fst:873` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__kem` | `fresh/Bertie_tls.Tls13crypto.fst:876` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__psk_mode` | `fresh/Bertie_tls.Tls13crypto.fst:879` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__zero_rtt` | `fresh/Bertie_tls.Tls13crypto.fst:882` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__ciphersuite` | `fresh/Bertie_tls.Tls13crypto.fst:885` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__supported_group` | `fresh/Bertie_tls.Tls13crypto.fst:927` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__signature_algorithm` | `fresh/Bertie_tls.Tls13crypto.fst:966` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13crypto.impl_5__check` | `fresh/Bertie_tls.Tls13crypto.fst:990` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13formats.Handshake_data.impl_1` | `fresh/Bertie_tls.Tls13formats.Handshake_data.fst:294` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13formats.impl_4__try_from_u8` | `fresh/Bertie_tls.Tls13formats.fst:1566` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` | `fresh/Bertie_tls.Tls13keyscheduler.Key_schedule.fst:546` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_2` | `fresh/Bertie_tls.Tls13utils.fst:43` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_11__into_raw` | `fresh/Bertie_tls.Tls13utils.fst:60` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_11__as_raw` | `fresh/Bertie_tls.Tls13utils.fst:63` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__new` | `fresh/Bertie_tls.Tls13utils.fst:89` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__new_alloc` | `fresh/Bertie_tls.Tls13utils.fst:94` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__zeroes` | `fresh/Bertie_tls.Tls13utils.fst:99` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__len` | `fresh/Bertie_tls.Tls13utils.fst:104` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_11__prefix` | `fresh/Bertie_tls.Tls13utils.fst:107` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_12` | `fresh/Bertie_tls.Tls13utils.fst:124` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_13` | `fresh/Bertie_tls.Tls13utils.fst:141` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_14` | `fresh/Bertie_tls.Tls13utils.fst:159` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__push` | `fresh/Bertie_tls.Tls13utils.fst:206` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__extend_from_slice` | `fresh/Bertie_tls.Tls13utils.fst:219` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__append` | `fresh/Bertie_tls.Tls13utils.fst:240` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__from_slice` | `fresh/Bertie_tls.Tls13utils.fst:261` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__raw_slice` | `fresh/Bertie_tls.Tls13utils.fst:267` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__slice_range` | `fresh/Bertie_tls.Tls13utils.fst:272` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__slice` | `fresh/Bertie_tls.Tls13utils.fst:281` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__concat` | `fresh/Bertie_tls.Tls13utils.fst:294` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__concat_array` | `fresh/Bertie_tls.Tls13utils.fst:296` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_17__update_slice` | `fresh/Bertie_tls.Tls13utils.fst:316` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_5__new` | `fresh/Bertie_tls.Tls13utils.fst:472` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_5__into_raw` | `fresh/Bertie_tls.Tls13utils.fst:475` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_5__as_raw` | `fresh/Bertie_tls.Tls13utils.fst:478` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_6` | `fresh/Bertie_tls.Tls13utils.fst:482` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_7` | `fresh/Bertie_tls.Tls13utils.fst:501` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_8` | `fresh/Bertie_tls.Tls13utils.fst:521` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_9` | `fresh/Bertie_tls.Tls13utils.fst:545` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl` | `fresh/Bertie_tls.Tls13utils.fst:563` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_1` | `fresh/Bertie_tls.Tls13utils.fst:571` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_11__declassify` | `fresh/Bertie_tls.Tls13utils.fst:578` | hax impl/disambiguator name; no unique inherent Rust ID |
| `Bertie_tls.Tls13utils.impl_3__declassify_array` | `fresh/Bertie_tls.Tls13utils.fst:613` | hax impl/disambiguator name; no unique inherent Rust ID |

### no Rust unit (1)

| Definition | Location | Apparent reason |
| --- | --- | --- |
| `Bertie_tls.Tls13utils.e_update_at_usize_bytes_test` | `fresh/Bertie_tls.Tls13utils.fst:198` | no exact inventoried Rust ID (may be cfg/backend or macro generated) |

### generated helper (4)

| Definition | Location | Apparent reason |
| --- | --- | --- |
| `Bertie_tls.Der.two_pow_256_` | `fresh/Bertie_tls.Der.fst:6` | hax adds trailing underscore (outside R1) |
| `Bertie_tls.Der.be_value_prefix_upper_128_` | `fresh/Bertie_tls.Der.fst:285` | hax adds trailing underscore (outside R1) |
| `Bertie_tls.Der.be_value_prefix_lower_128_` | `fresh/Bertie_tls.Der.fst:295` | hax adds trailing underscore (outside R1) |
| `Bertie_tls.Tls13keyscheduler.next_keys_c_2_` | `fresh/Bertie_tls.Tls13keyscheduler.fst:164` | hax adds trailing underscore (outside R1) |

### other (3)

| Definition | Location | Apparent reason |
| --- | --- | --- |
| `Bertie_tls.Tls13crypto.aead_encrypt` | `fresh/Bertie_tls.Tls13crypto.fst:184` | ambiguous or unsupported mapping |
| `Bertie_tls.Tls13crypto.aead_decrypt` | `fresh/Bertie_tls.Tls13crypto.fst:269` | ambiguous or unsupported mapping |
| `Bertie_tls.Tls13utils.tlserr` | `fresh/Bertie_tls.Tls13utils.fst:33` | ambiguous or unsupported mapping |

## Artifacts

- `target/v183/bertie-runtime-after.json` SHA-256 `cf4b8237d5d72aa5fec57f5f02d25e40396abdb6644fa064dd8e9ac91a08cf2b`
- `target/v183/bertie-runtime-before.json` SHA-256 `c199df841e076c33408727e2b37bd2440fe09a4ffab7c82ef98c3a0b87253846`
- `target/v183/bertie-separate-after.json` SHA-256 `00bae81dac1e31321bf1e896c07a8bd83952ac864242ad9e5aed6e6ecfd0b34f`
- `target/v183/bertie-separate-before.json` SHA-256 `9883a2c2f7ee422cf2c9be665282452840da93865cd3e05350824325173b0d7a`

## Rerun

```sh
python3 dev/v183/replay.py bertie > logs/v1.8.3/m2-bertie.log 2>&1
python3 dev/v183/replay.py fixtures > logs/v1.8.3/m2-fixtures.log 2>&1
python3 dev/v183/replay.py verus > logs/v1.8.3/m2-verus.log 2>&1
python3 dev/v183/baseline.py > logs/v1.8.3/m2-baseline.log 2>&1
python3 dev/v183/compare.py > logs/v1.8.3/m2-compare.log 2>&1
```

Local checks only; independent acceptance remains Dudley’s. Existing ignored performance test and fresh Verus execution were not run.
