# v1.9.1 paired replay

Check (b) amended 2026-11-25: strictly checked extraction-only proof-code helpers
may become verified under the unchanged v1.8.1 classifier; they contribute no runtime totals.

Baseline: tag `v1.9.0`; candidate: v1.9.1 working tree.
Bertie: staged bab3c13, Coverage run 45615087282, unchanged ship configuration.
CI receipt/log bytes and receipt-bound hashes verified. Replay runs neither hax nor F*.
Current stable, RUSTUP_TOOLCHAIN unset: rustc 1.100.0 (0ef1bc386 2026-11-09).

Codex-run checks; independent Andrew/Dudley acceptance remains outstanding.

(a) Baseline units exactly equal CI units: PASS.

## Bertie separate

Before: 4027 proven-extracted, 281 tested, 652 unverified, 118 trusted / 5078 security runtime SLoC (79.30% proven-extracted).

After: 4111 proven-extracted, 253 tested, 553 unverified, 161 trusted / 5078 security runtime SLoC (80.96% proven-extracted).

| Rust unit (source line) | Old → new | SLoC | Form and definition |
| --- | --- | ---: | --- |
| `bertie_tls::der::two_pow_256` (84) | tested → proven-extracted | 3 | A: `Bertie_tls.Der.two_pow_256_` |
| `bertie_tls::der::be_value_prefix_upper_128` (470) | unverified → verified | 10 | A: `Bertie_tls.Der.be_value_prefix_upper_128_` |
| `bertie_tls::der::be_value_prefix_lower_128` (482) | unverified → verified | 10 | A: `Bertie_tls.Der.be_value_prefix_lower_128_` |
| `bertie_tls::server::ServerDB::new` (19) | tested → proven-extracted | 13 | E: `Bertie_tls.Server.impl__new` |
| `bertie_tls::tls13cert::CloneforCertificateKey::clone` (38) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13cert.impl` |
| `bertie_tls::tls13formats::TryFrom<u8>forAlertLevel::try_from` (381) | unverified → trusted | 7 | D: `Bertie_tls.Tls13formats.impl_1` |
| `bertie_tls::tls13formats::TryFrom<u8>forAlertDescription::try_from` (395) | unverified → trusted | 32 | D: `Bertie_tls.Tls13formats.impl_3` |
| `bertie_tls::tls13keyscheduler::next_keys_c_2` (81) | unverified → proven-extracted | 18 | A: `Bertie_tls.Tls13keyscheduler.next_keys_c_2_` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::prnt_n` (91) | unverified → proven-extracted | 14 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::labels` (106) | unverified → proven-extracted | 21 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::get` (129) | unverified → proven-extracted | 3 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::set` (133) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::hash` (138) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13utils::core::ops::Index<usize>forBytes::index` (378) | unverified → trusted | 4 | D: `Bertie_tls.Tls13utils.impl_15` |
| `bertie_tls::tls13utils::From<Bytes>forAppData::from` (1047) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13utils.impl_9` |

15 changed source units.

(b) Every changed unit explained by form A–E: PASS.
(c) No tier loss; unchanged fields and totals arithmetic: PASS.
(e) Inactive cfg twins unchanged: PASS.

## Bertie runtime

Before: 4382 proven-extracted, 285 tested, 663 unverified, 118 trusted / 5448 security runtime SLoC (80.43% proven-extracted).

After: 4467 proven-extracted, 256 tested, 564 unverified, 161 trusted / 5448 security runtime SLoC (81.99% proven-extracted).

| Rust unit (source line) | Old → new | SLoC | Form and definition |
| --- | --- | ---: | --- |
| `bertie_tls::der::two_pow_256` (84) | tested → proven-extracted | 4 | A: `Bertie_tls.Der.two_pow_256_` |
| `bertie_tls::der::be_value_prefix_upper_128` (470) | unverified → verified | 10 | A: `Bertie_tls.Der.be_value_prefix_upper_128_` |
| `bertie_tls::der::be_value_prefix_lower_128` (482) | unverified → verified | 10 | A: `Bertie_tls.Der.be_value_prefix_lower_128_` |
| `bertie_tls::server::ServerDB::new` (19) | tested → proven-extracted | 13 | E: `Bertie_tls.Server.impl__new` |
| `bertie_tls::tls13cert::CloneforCertificateKey::clone` (38) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13cert.impl` |
| `bertie_tls::tls13formats::TryFrom<u8>forAlertLevel::try_from` (381) | unverified → trusted | 7 | D: `Bertie_tls.Tls13formats.impl_1` |
| `bertie_tls::tls13formats::TryFrom<u8>forAlertDescription::try_from` (395) | unverified → trusted | 32 | D: `Bertie_tls.Tls13formats.impl_3` |
| `bertie_tls::tls13keyscheduler::next_keys_c_2` (81) | unverified → proven-extracted | 18 | A: `Bertie_tls.Tls13keyscheduler.next_keys_c_2_` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::prnt_n` (91) | unverified → proven-extracted | 14 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::labels` (106) | unverified → proven-extracted | 21 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::get` (129) | unverified → proven-extracted | 3 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::set` (133) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::hash` (138) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13keyscheduler.Key_schedule.impl` |
| `bertie_tls::tls13utils::core::ops::Index<usize>forBytes::index` (378) | unverified → trusted | 4 | D: `Bertie_tls.Tls13utils.impl_15` |
| `bertie_tls::tls13utils::From<Bytes>forAppData::from` (1047) | tested → proven-extracted | 3 | C: `Bertie_tls.Tls13utils.impl_9` |

15 changed source units.

(b) Every changed unit explained by form A–E: PASS.
(c) No tier loss; unchanged fields and totals arithmetic: PASS.
(e) Inactive cfg twins unchanged: PASS.

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

(d) Exact JSON + Markdown equality: PASS.

## Scope

B stays unmapped: the receipt contains no explicit extraction feature/cfg set.
D records retain their trusted F* classification; linking them attributes trust to previously unverified Rust units.
Unsupported references, aliases, conditional blocks and generic binder identities remain unmapped.

Rerun: `python3 dev/v191/replay.py` recreates target/v191 from staged inputs.
