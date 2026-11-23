# v1.8.2 paired replay

Baseline: 7a60142 (v1.8.1). Bertie sources: 9212e97. All writes confined to this repository.
The pre-change baseline reproduced all 465 CI unit statuses exactly with an accepted CI receipt;
freshly extracted inputs passed the unmodified CI receipt hashes. Both release reports below use the same fresh verifier log.

## Controls

scratch-spec-lemmas separate: exact JSON and Markdown equality PASS
scratch-spec-lemmas runtime: exact JSON and Markdown equality PASS
vest separate: exact JSON and Markdown equality PASS
vest runtime: exact JSON and Markdown equality PASS
fixture-classification separate: exact JSON and Markdown equality PASS
fixture-classification runtime: exact JSON and Markdown equality PASS
fixture-doc_comments separate: exact JSON and Markdown equality PASS
fixture-doc_comments runtime: exact JSON and Markdown equality PASS
fixture-evidence separate: exact JSON and Markdown equality PASS
fixture-evidence runtime: exact JSON and Markdown equality PASS
fixture-ghost separate: exact JSON and Markdown equality PASS
fixture-ghost runtime: exact JSON and Markdown equality PASS
fixture-hax separate: exact JSON and Markdown equality PASS
fixture-hax runtime: exact JSON and Markdown equality PASS
fixture-inventory separate: exact JSON and Markdown equality PASS
fixture-inventory runtime: exact JSON and Markdown equality PASS

## Bertie, separate

Runtime proven-extracted units / SLoC: (50, 791) → (68, 1079).
Verus proven SLoC: 0 → 0.
75 unit statuses changed; no unit category or size changed.

| Unit | Old → new status | SLoC | Rule |
| --- | --- | ---: | --- |
| `bertie_tls::tls13api::Server::accept` | tested → proven-extracted | 25 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::read_handshake` | tested → proven-extracted | 14 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::write` | tested → proven-extracted | 9 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::read` | tested → proven-extracted | 9 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::close_notify` | tested → proven-extracted | 6 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::AeadKeyIV::new` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::AeadKey::new` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::HashAlgorithm::libcrux_algorithm` | unverified → trusted | 12 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::HashAlgorithm::hmac_algorithm` | unverified → trusted | 12 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::KemScheme::libcrux_kem_algorithm` | unverified → trusted | 11 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::new` | tested → trusted | 17 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::validate` | unverified → trusted | 41 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::hash` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::aead` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::signature` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::kem` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::psk_mode` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::zero_rtt` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::ciphersuite` | tested → trusted | 8 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::supported_group` | tested → trusted | 11 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::signature_algorithm` | unverified → trusted | 7 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::check` | unverified → trusted | 7 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::client_hello` | trusted → tested | 72 | R1 (backend path) |
| `bertie_tls::tls13formats::set_client_hello_binder` | trusted → unverified | 56 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_client_hello` | trusted → tested | 138 | R1 (backend path) |
| `bertie_tls::tls13formats::server_hello` | trusted → tested | 39 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_server_hello` | trusted → tested | 63 | R1 (backend path) |
| `bertie_tls::tls13formats::encrypted_extensions` | trusted → tested | 7 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_encrypted_extensions` | trusted → tested | 30 | R1 (backend path) |
| `bertie_tls::tls13formats::server_certificate` | trusted → tested | 20 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_server_certificate` | trusted → tested | 31 | R1 (backend path) |
| `bertie_tls::tls13formats::certificate_verify` | trusted → unverified | 26 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_certificate_verify` | trusted → tested | 44 | R1 (backend path) |
| `bertie_tls::tls13formats::finished` | trusted → unverified | 10 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_finished` | trusted → tested | 19 | R1 (backend path) |
| `bertie_tls::tls13formats::Transcript::add` | trusted → unverified | 24 | R1 (backend path), R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::to_two_inner` | trusted → proven-extracted | 24 | R1 (backend path) |
| `bertie_tls::tls13formats::handshake_data::to_four_inner` | trusted → proven-extracted | 41 | R1 (backend path) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::from_bytes` | unverified → proven-extracted | 6 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::len` | trusted → proven-extracted | 4 | R1 (backend path), R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_bytes` | unverified → proven-extracted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::concat` | tested → proven-extracted | 7 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::as_handshake_message` | unverified → proven-extracted | 15 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::next_handshake_message` | unverified → proven-extracted | 10 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_two` | unverified → proven-extracted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_four` | unverified → proven-extracted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::find_handshake_message` | unverified → proven-extracted | 20 | R4 (unique inherent method) |
| `bertie_tls::tls13handshake::proverif_extra::f` | trusted → tested | 4 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::derive_aead_key_iv` | trusted → proven-extracted | 51 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::derive_finished_key` | trusted → proven-extracted | 36 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::key_schedule::XPD` | trusted → unverified | 69 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::key_schedule::XTR` | trusted → unverified | 57 | R1 (backend path) |
| `bertie_tls::tls13record::DuplexCipherStateH::new` | tested → trusted | 13 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::declassify` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::into_raw` | unverified → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::as_raw` | tested → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::declassify_array` | unverified → trusted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::new` | tested → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::new_alloc` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::zeroes` | tested → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::len` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::push` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::extend_from_slice` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::append` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::from_slice` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::from_hex` | tested → trusted | 18 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::raw_slice` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::slice_range` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::slice` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::concat` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::concat_array` | unverified → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::update_slice` | unverified → trusted | 13 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::AppData::new` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::AppData::into_raw` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::AppData::as_raw` | tested → trusted | 3 | R4 (unique inherent method) |

### Non-F* trust annotations

| File:line | Category | SLoC | Affected units |
| --- | --- | ---: | --- |
| src/tls13cert.rs:331 | hax replace (non-F* trust: proverif) | 46 | bertie_tls::tls13cert::verification_key_from_cert |
| src/tls13cert.rs:470 | hax replace (non-F* trust: proverif) | 35 | bertie_tls::tls13cert::cert_public_key |
| src/tls13crypto.rs:76 | hax opaque (non-F* trust: inactive feature hax-pv) | 0 |  |
| src/tls13crypto.rs:86 | hax replace (non-F* trust: proverif) | 0 |  |
| src/tls13crypto.rs:155 | hax replace_body (non-F* trust: proverif) | 8 | bertie_tls::tls13crypto::HashAlgorithm::hash_len |
| src/tls13crypto.rs:179 | hax replace_body (non-F* trust: proverif) | 4 | bertie_tls::tls13crypto::HashAlgorithm::hmac_tag_len |
| src/tls13crypto.rs:204 | hax replace (non-F* trust: proverif) | 29 | bertie_tls::tls13crypto::hmac_verify |
| src/tls13crypto.rs:296 | hax replace_body (non-F* trust: proverif) | 8 | bertie_tls::tls13crypto::AeadAlgorithm::key_len |
| src/tls13crypto.rs:306 | hax replace_body (non-F* trust: proverif) | 8 | bertie_tls::tls13crypto::AeadAlgorithm::iv_len |
| src/tls13crypto.rs:453 | hax replace_body (non-F* trust: proverif) | 56 | bertie_tls::tls13crypto::sign |
| src/tls13crypto.rs:504 | hax replace (non-F* trust: proverif) | 101 | bertie_tls::tls13crypto::verify |
| src/tls13crypto.rs:702 | hax replace_body (non-F* trust: proverif) | 27 | bertie_tls::tls13crypto::kem_keygen |
| src/tls13crypto.rs:757 | hax replace_body (non-F* trust: proverif) | 30 | bertie_tls::tls13crypto::kem_encap |
| src/tls13crypto.rs:801 | hax replace (non-F* trust: proverif) | 24 | bertie_tls::tls13crypto::kem_decap |
| src/tls13formats.rs:557 | hax replace_body (non-F* trust: proverif) | 72 | bertie_tls::tls13formats::client_hello |
| src/tls13formats.rs:667 | hax replace_body (non-F* trust: proverif) | 56 | bertie_tls::tls13formats::set_client_hello_binder |
| src/tls13formats.rs:738 | hax replace (non-F* trust: proverif) | 138 | bertie_tls::tls13formats::parse_client_hello |
| src/tls13formats.rs:878 | hax replace (non-F* trust: proverif) | 39 | bertie_tls::tls13formats::server_hello |
| src/tls13formats.rs:951 | hax replace (non-F* trust: proverif) | 63 | bertie_tls::tls13formats::parse_server_hello |
| src/tls13formats.rs:1018 | hax replace (non-F* trust: proverif) | 7 | bertie_tls::tls13formats::encrypted_extensions |
| src/tls13formats.rs:1028 | hax replace (non-F* trust: proverif) | 30 | bertie_tls::tls13formats::parse_encrypted_extensions |
| src/tls13formats.rs:1060 | hax replace (non-F* trust: proverif) | 20 | bertie_tls::tls13formats::server_certificate |
| src/tls13formats.rs:1099 | hax replace (non-F* trust: proverif) | 31 | bertie_tls::tls13formats::parse_server_certificate |
| src/tls13formats.rs:1140 | hax replace (non-F* trust: proverif) | 26 | bertie_tls::tls13formats::certificate_verify |
| src/tls13formats.rs:1173 | hax replace (non-F* trust: proverif) | 44 | bertie_tls::tls13formats::parse_certificate_verify |
| src/tls13formats.rs:1219 | hax replace (non-F* trust: proverif) | 10 | bertie_tls::tls13formats::finished |
| src/tls13formats.rs:1243 | hax replace (non-F* trust: proverif) | 19 | bertie_tls::tls13formats::parse_finished |
| src/tls13formats.rs:1392 | hax replace_body (non-F* trust: proverif) | 24 | bertie_tls::tls13formats::Transcript::add |
| src/tls13formats/handshake_data.rs:34 | hax replace (non-F* trust: proverif) | 24 | bertie_tls::tls13formats::handshake_data::to_two_inner |
| src/tls13formats/handshake_data.rs:59 | hax replace (non-F* trust: proverif) | 41 | bertie_tls::tls13formats::handshake_data::to_four_inner |
| src/tls13formats/handshake_data.rs:134 | hax replace_body (non-F* trust: proverif) | 4 | bertie_tls::tls13formats::handshake_data::HandshakeData::len |
| src/tls13formats/handshake_type.rs:20 | hax opaque (non-F* trust: inactive feature hax-pv) | 0 |  |
| src/tls13handshake.rs:898 | hax replace (non-F* trust: proverif) | 4 | bertie_tls::tls13handshake::proverif_extra::f |
| src/tls13keyscheduler.rs:44 | hax replace_body (non-F* trust: proverif) | 51 | bertie_tls::tls13keyscheduler::derive_aead_key_iv |
| src/tls13keyscheduler.rs:134 | hax replace_body (non-F* trust: proverif) | 36 | bertie_tls::tls13keyscheduler::derive_finished_key |
| src/tls13keyscheduler/key_schedule.rs:312 | hax replace_body (non-F* trust: proverif) | 69 | bertie_tls::tls13keyscheduler::key_schedule::XPD |
| src/tls13keyscheduler/key_schedule.rs:391 | hax replace_body (non-F* trust: proverif) | 57 | bertie_tls::tls13keyscheduler::key_schedule::XTR |
| src/tls13record.rs:211 | hax replace (non-F* trust: proverif) | 30 | bertie_tls::tls13record::encrypt_handshake |
| src/tls13record.rs:244 | hax replace (non-F* trust: proverif) | 47 | bertie_tls::tls13record::decrypt_handshake |
| src/tls13utils.rs:202 | hax replace (non-F* trust: proverif) | 9 | bertie_tls::tls13utils::concat_inner |
| src/tls13utils.rs:212 | hax replace (non-F* trust: proverif) | 13 | bertie_tls::tls13utils::eq_inner |
| src/tls13utils.rs:229 | hax replace_body (non-F* trust: proverif) | 10 | bertie_tls::tls13utils::Bytes::prefix |
| src/tls13utils.rs:307 | hax replace (non-F* trust: proverif) | 14 | bertie_tls::tls13utils::u16_as_be_bytes |
| src/tls13utils.rs:409 | hax replace (non-F* trust: proverif) | 11 | bertie_tls::tls13utils::check_eq_inner |

Non-F* trust: 44 sites; 1375 summed affected SLoC.
All other unit fields, categories, source/dependency sizes, escape sites, proof cost, contract text and control figures are unchanged. F* mappings/classifications, trust attribution and credited-unit lists change as the rules require.

## Bertie, runtime

Runtime proven-extracted units / SLoC: (50, 892) → (68, 1192).
Verus proven SLoC: 0 → 0.
75 unit statuses changed; no unit category or size changed.

| Unit | Old → new status | SLoC | Rule |
| --- | --- | ---: | --- |
| `bertie_tls::tls13api::Server::accept` | tested → proven-extracted | 25 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::read_handshake` | tested → proven-extracted | 14 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::write` | tested → proven-extracted | 9 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::read` | tested → proven-extracted | 9 | R4 (unique inherent method) |
| `bertie_tls::tls13api::Server::close_notify` | tested → proven-extracted | 6 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::AeadKeyIV::new` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::AeadKey::new` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::HashAlgorithm::libcrux_algorithm` | unverified → trusted | 12 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::HashAlgorithm::hmac_algorithm` | unverified → trusted | 12 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::KemScheme::libcrux_kem_algorithm` | unverified → trusted | 11 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::new` | tested → trusted | 17 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::validate` | unverified → trusted | 41 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::hash` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::aead` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::signature` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::kem` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::psk_mode` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::zero_rtt` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::ciphersuite` | tested → trusted | 11 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::supported_group` | tested → trusted | 14 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::signature_algorithm` | unverified → trusted | 10 | R4 (unique inherent method) |
| `bertie_tls::tls13crypto::Algorithms::check` | unverified → trusted | 11 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::client_hello` | trusted → tested | 85 | R1 (backend path) |
| `bertie_tls::tls13formats::set_client_hello_binder` | trusted → unverified | 59 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_client_hello` | trusted → tested | 141 | R1 (backend path) |
| `bertie_tls::tls13formats::server_hello` | trusted → tested | 49 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_server_hello` | trusted → tested | 63 | R1 (backend path) |
| `bertie_tls::tls13formats::encrypted_extensions` | trusted → tested | 7 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_encrypted_extensions` | trusted → tested | 30 | R1 (backend path) |
| `bertie_tls::tls13formats::server_certificate` | trusted → tested | 28 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_server_certificate` | trusted → tested | 31 | R1 (backend path) |
| `bertie_tls::tls13formats::certificate_verify` | trusted → unverified | 34 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_certificate_verify` | trusted → tested | 44 | R1 (backend path) |
| `bertie_tls::tls13formats::finished` | trusted → unverified | 18 | R1 (backend path) |
| `bertie_tls::tls13formats::parse_finished` | trusted → tested | 19 | R1 (backend path) |
| `bertie_tls::tls13formats::Transcript::add` | trusted → unverified | 24 | R1 (backend path), R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::to_two_inner` | trusted → proven-extracted | 24 | R1 (backend path) |
| `bertie_tls::tls13formats::handshake_data::to_four_inner` | trusted → proven-extracted | 41 | R1 (backend path) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::from_bytes` | unverified → proven-extracted | 9 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::len` | trusted → proven-extracted | 5 | R1 (backend path), R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_bytes` | unverified → proven-extracted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::concat` | tested → proven-extracted | 7 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::as_handshake_message` | unverified → proven-extracted | 18 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::next_handshake_message` | unverified → proven-extracted | 13 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_two` | unverified → proven-extracted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_four` | unverified → proven-extracted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::find_handshake_message` | unverified → proven-extracted | 22 | R4 (unique inherent method) |
| `bertie_tls::tls13handshake::proverif_extra::f` | trusted → tested | 4 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::derive_aead_key_iv` | trusted → proven-extracted | 51 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::derive_finished_key` | trusted → proven-extracted | 36 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::key_schedule::XPD` | trusted → unverified | 69 | R1 (backend path) |
| `bertie_tls::tls13keyscheduler::key_schedule::XTR` | trusted → unverified | 57 | R1 (backend path) |
| `bertie_tls::tls13record::DuplexCipherStateH::new` | tested → trusted | 13 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::declassify` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::into_raw` | unverified → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::as_raw` | tested → trusted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::declassify_array` | unverified → trusted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::new` | tested → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::new_alloc` | unverified → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::zeroes` | tested → trusted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::len` | tested → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::push` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::extend_from_slice` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::append` | unverified → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::from_slice` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::from_hex` | tested → trusted | 18 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::raw_slice` | unverified → trusted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::slice_range` | unverified → trusted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::slice` | unverified → trusted | 5 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::concat` | tested → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::concat_array` | unverified → trusted | 4 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::Bytes::update_slice` | unverified → trusted | 13 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::AppData::new` | tested → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::AppData::into_raw` | unverified → trusted | 3 | R4 (unique inherent method) |
| `bertie_tls::tls13utils::AppData::as_raw` | tested → trusted | 3 | R4 (unique inherent method) |

### Non-F* trust annotations

| File:line | Category | SLoC | Affected units |
| --- | --- | ---: | --- |
| src/tls13cert.rs:331 | hax replace (non-F* trust: proverif) | 46 | bertie_tls::tls13cert::verification_key_from_cert |
| src/tls13cert.rs:470 | hax replace (non-F* trust: proverif) | 35 | bertie_tls::tls13cert::cert_public_key |
| src/tls13crypto.rs:76 | hax opaque (non-F* trust: inactive feature hax-pv) | 0 |  |
| src/tls13crypto.rs:86 | hax replace (non-F* trust: proverif) | 0 |  |
| src/tls13crypto.rs:155 | hax replace_body (non-F* trust: proverif) | 9 | bertie_tls::tls13crypto::HashAlgorithm::hash_len |
| src/tls13crypto.rs:179 | hax replace_body (non-F* trust: proverif) | 4 | bertie_tls::tls13crypto::HashAlgorithm::hmac_tag_len |
| src/tls13crypto.rs:204 | hax replace (non-F* trust: proverif) | 29 | bertie_tls::tls13crypto::hmac_verify |
| src/tls13crypto.rs:296 | hax replace_body (non-F* trust: proverif) | 8 | bertie_tls::tls13crypto::AeadAlgorithm::key_len |
| src/tls13crypto.rs:306 | hax replace_body (non-F* trust: proverif) | 8 | bertie_tls::tls13crypto::AeadAlgorithm::iv_len |
| src/tls13crypto.rs:453 | hax replace_body (non-F* trust: proverif) | 56 | bertie_tls::tls13crypto::sign |
| src/tls13crypto.rs:504 | hax replace (non-F* trust: proverif) | 101 | bertie_tls::tls13crypto::verify |
| src/tls13crypto.rs:702 | hax replace_body (non-F* trust: proverif) | 27 | bertie_tls::tls13crypto::kem_keygen |
| src/tls13crypto.rs:757 | hax replace_body (non-F* trust: proverif) | 30 | bertie_tls::tls13crypto::kem_encap |
| src/tls13crypto.rs:801 | hax replace (non-F* trust: proverif) | 24 | bertie_tls::tls13crypto::kem_decap |
| src/tls13formats.rs:557 | hax replace_body (non-F* trust: proverif) | 85 | bertie_tls::tls13formats::client_hello |
| src/tls13formats.rs:667 | hax replace_body (non-F* trust: proverif) | 59 | bertie_tls::tls13formats::set_client_hello_binder |
| src/tls13formats.rs:738 | hax replace (non-F* trust: proverif) | 141 | bertie_tls::tls13formats::parse_client_hello |
| src/tls13formats.rs:878 | hax replace (non-F* trust: proverif) | 49 | bertie_tls::tls13formats::server_hello |
| src/tls13formats.rs:951 | hax replace (non-F* trust: proverif) | 63 | bertie_tls::tls13formats::parse_server_hello |
| src/tls13formats.rs:1018 | hax replace (non-F* trust: proverif) | 7 | bertie_tls::tls13formats::encrypted_extensions |
| src/tls13formats.rs:1028 | hax replace (non-F* trust: proverif) | 30 | bertie_tls::tls13formats::parse_encrypted_extensions |
| src/tls13formats.rs:1060 | hax replace (non-F* trust: proverif) | 28 | bertie_tls::tls13formats::server_certificate |
| src/tls13formats.rs:1099 | hax replace (non-F* trust: proverif) | 31 | bertie_tls::tls13formats::parse_server_certificate |
| src/tls13formats.rs:1140 | hax replace (non-F* trust: proverif) | 34 | bertie_tls::tls13formats::certificate_verify |
| src/tls13formats.rs:1173 | hax replace (non-F* trust: proverif) | 44 | bertie_tls::tls13formats::parse_certificate_verify |
| src/tls13formats.rs:1219 | hax replace (non-F* trust: proverif) | 18 | bertie_tls::tls13formats::finished |
| src/tls13formats.rs:1243 | hax replace (non-F* trust: proverif) | 19 | bertie_tls::tls13formats::parse_finished |
| src/tls13formats.rs:1392 | hax replace_body (non-F* trust: proverif) | 24 | bertie_tls::tls13formats::Transcript::add |
| src/tls13formats/handshake_data.rs:34 | hax replace (non-F* trust: proverif) | 24 | bertie_tls::tls13formats::handshake_data::to_two_inner |
| src/tls13formats/handshake_data.rs:59 | hax replace (non-F* trust: proverif) | 41 | bertie_tls::tls13formats::handshake_data::to_four_inner |
| src/tls13formats/handshake_data.rs:134 | hax replace_body (non-F* trust: proverif) | 5 | bertie_tls::tls13formats::handshake_data::HandshakeData::len |
| src/tls13formats/handshake_type.rs:20 | hax opaque (non-F* trust: inactive feature hax-pv) | 0 |  |
| src/tls13handshake.rs:898 | hax replace (non-F* trust: proverif) | 4 | bertie_tls::tls13handshake::proverif_extra::f |
| src/tls13keyscheduler.rs:44 | hax replace_body (non-F* trust: proverif) | 51 | bertie_tls::tls13keyscheduler::derive_aead_key_iv |
| src/tls13keyscheduler.rs:134 | hax replace_body (non-F* trust: proverif) | 36 | bertie_tls::tls13keyscheduler::derive_finished_key |
| src/tls13keyscheduler/key_schedule.rs:312 | hax replace_body (non-F* trust: proverif) | 69 | bertie_tls::tls13keyscheduler::key_schedule::XPD |
| src/tls13keyscheduler/key_schedule.rs:391 | hax replace_body (non-F* trust: proverif) | 57 | bertie_tls::tls13keyscheduler::key_schedule::XTR |
| src/tls13record.rs:211 | hax replace (non-F* trust: proverif) | 30 | bertie_tls::tls13record::encrypt_handshake |
| src/tls13record.rs:244 | hax replace (non-F* trust: proverif) | 47 | bertie_tls::tls13record::decrypt_handshake |
| src/tls13utils.rs:202 | hax replace (non-F* trust: proverif) | 9 | bertie_tls::tls13utils::concat_inner |
| src/tls13utils.rs:212 | hax replace (non-F* trust: proverif) | 13 | bertie_tls::tls13utils::eq_inner |
| src/tls13utils.rs:229 | hax replace_body (non-F* trust: proverif) | 11 | bertie_tls::tls13utils::Bytes::prefix |
| src/tls13utils.rs:307 | hax replace (non-F* trust: proverif) | 14 | bertie_tls::tls13utils::u16_as_be_bytes |
| src/tls13utils.rs:409 | hax replace (non-F* trust: proverif) | 11 | bertie_tls::tls13utils::check_eq_inner |

Non-F* trust: 44 sites; 1431 summed affected SLoC.
All other unit fields, categories, source/dependency sizes, escape sites, proof cost, contract text and control figures are unchanged. F* mappings/classifications, trust attribution and credited-unit lists change as the rules require.

## Artifacts

- `target/v182/bertie-runtime-after.json` SHA-256 `5572e427bc1e7d8b83e1d6029729c98ec782a90e62cd2cfd071eb4bbf74b304d`
- `target/v182/bertie-runtime-before.json` SHA-256 `a3e359032ba5b8ce9c2a7aef4d345194e532cdce0944397c6bceac02c549f3e1`
- `target/v182/bertie-separate-after.json` SHA-256 `a1c71648947e3aaea0c6276911f4631e84e212c09ddbab2ac835733e13b720fe`
- `target/v182/bertie-separate-before.json` SHA-256 `8451b3dc82a0a0b2af330cb6379147b80bc709094ea7219f0718f74c53d68ebb`

## Rerun

```sh
python3 dev/v182/baseline.py > logs/v1.8.2/baseline-comparison.log 2>&1
python3 dev/v182/replay.py bertie > logs/v1.8.2/m3-bertie.log 2>&1
python3 dev/v182/replay.py fixtures > logs/v1.8.2/m3-fixtures.log 2>&1
python3 dev/v182/replay.py verus > logs/v1.8.2/m3-verus.log 2>&1
python3 dev/v182/compare.py > logs/v1.8.2/m3-compare.log 2>&1
```

Local verification only; independent acceptance remains Dudley’s. Existing ignored performance acceptance and fresh Verus execution were not run.
