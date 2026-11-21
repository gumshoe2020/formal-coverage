# Bertie v1.7.3 report-only comparison

Generated 2026-11-21T15:51:31-05:00.

Source: bertie-deps HEAD `f646099cee002c1d998b0e8c160886ef838f4f69`. Baseline: `4031d85`; candidate measurement code: `b3ed561` (M1). Both binaries built with `cargo build --release --frozen`. Package metadata still reads 1.5.0 at M2; M3 bumps it to 1.7.3.

Both runs exited 0 using the exact CI arguments from `.github/workflows/coverage.yml`:

```sh
formal-coverage report . --config formal-coverage.toml --fstar-log target/ci-logs/fstar/fstar.log --json <out>.json --markdown <out>.md
```

No log-layout adaptation or proof-ratio fallback was needed. No proofs, extraction, receipt, tier or threshold changes were made. Raw reports and build/run logs are retained locally under `target/v173/` (ignored). The same accepted F* receipt was used by both runs.

## Runtime security headlines

All statuses are reported separately; `proven-extracted` is the hax/F* tier and is not `proven`. Percentages use each row's security runtime denominator. First-party covers the entire Bertie workspace, as CI does.

| Scope / version | Security lines | Proven | Proven-extracted | Tested | Trusted | Unverified |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| first_party / 1.7.2 | 6,382 | 0 (0.00%) | 884 (13.85%) | 746 (11.69%) | 2,517 (39.44%) | 2,235 (35.02%) |
| first_party / 1.7.3 | 5,818 | 0 (0.00%) | 824 (14.16%) | 548 (9.42%) | 2,329 (40.03%) | 2,117 (36.39%) |
| third_party / 1.7.2 | 336,589 | 0 (0.00%) | 0 (0.00%) | 0 (0.00%) | 0 (0.00%) | 336,589 (100.00%) |
| third_party / 1.7.3 | 258,620 | 0 (0.00%) | 0 (0.00%) | 0 (0.00%) | 0 (0.00%) | 258,620 (100.00%) |
| all / 1.7.2 | 342,971 | 0 (0.00%) | 884 (0.26%) | 746 (0.22%) | 2,517 (0.73%) | 338,824 (98.79%) |
| all / 1.7.3 | 264,438 | 0 (0.00%) | 824 (0.31%) | 548 (0.21%) | 2,329 (0.88%) | 260,737 (98.60%) |

`proven-bounded`, `proven-upstream` and `hazard` are zero in all headline rows.

## Acceptance checks

- Unit identities: PASS (469 old / 469 new).
- No tier changes: PASS (0 changes).
- No size increases: PASS (0 increases).
- Independent doc count vs security drop: PASS; 6,382 → 5,818, drop **564**; independently counted **564** unit-owned doc-only lines (564 distinct source lines). Tolerance set to 5%; actual difference 0.
- Security source files contain 828 doc-comment lines in total; module/type docs outside runtime unit spans are excluded from the runtime denominator. The independent scanner skips strings and ordinary comments and handles nested block comments. No explicit doc attributes exist in these files.

`server` (`bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::server`, `src/stream/server.rs:111`): **73 → 20** lines; tier `tested` unchanged. The supplied rough estimate (76 lines / 57 doc lines) differs from this pinned source; these are the actual paired measurements.

## Every unit whose size changed (223)

Includes runtime, proof and test units across the complete report, not just the security headline.

| Unit | File:line | Old | New | Tier |
| --- | --- | ---: | ---: | --- |
| `bertie_tls::benches::client::client_hello` | `benches/client.rs:40` | 25 | 24 | unverified |
| `bertie_tls::benches::client::parse_server_certificate` | `benches/client.rs:91` | 37 | 36 | unverified |
| `bertie_tls::benches::client::parse_server_hello` | `benches/client.rs:68` | 20 | 19 | unverified |
| `bertie_tls::benches::client::protocol` | `benches/client.rs:131` | 66 | 65 | unverified |
| `bertie_tls::benches::client_stack::protocol` | `benches/client_stack.rs:59` | 92 | 91 | hazard |
| `bertie_tls::benches::server::parse_client_hello` | `benches/server.rs:36` | 25 | 24 | unverified |
| `bertie_tls::benches::server::protocol` | `benches/server.rs:73` | 78 | 77 | unverified |
| `bertie_tls::der::agree` | `src/der.rs:464` | 5 | 4 | proven-extracted |
| `bertie_tls::der::agree_elim` | `src/der.rs:549` | 6 | 5 | proven-extracted |
| `bertie_tls::der::be_value` | `src/der.rs:101` | 4 | 3 | proven-extracted |
| `bertie_tls::der::be_value_prefix` | `src/der.rs:90` | 10 | 9 | proven-extracted |
| `bertie_tls::der::be_value_prefix_bound` | `src/der.rs:266` | 11 | 10 | proven-extracted |
| `bertie_tls::der::be_value_prefix_injective` | `src/der.rs:527` | 11 | 10 | proven-extracted |
| `bertie_tls::der::be_value_prefix_lower_128` | `src/der.rs:482` | 11 | 10 | unverified |
| `bertie_tls::der::be_value_prefix_lower_bound` | `src/der.rs:278` | 11 | 10 | proven-extracted |
| `bertie_tls::der::be_value_prefix_upper_128` | `src/der.rs:470` | 11 | 10 | unverified |
| `bertie_tls::der::be_value_shift` | `src/der.rs:290` | 22 | 19 | proven-extracted |
| `bertie_tls::der::decode_ecdsa_signature` | `src/der.rs:419` | 44 | 43 | proven-extracted |
| `bertie_tls::der::der_roundtrip` | `src/der.rs:594` | 37 | 34 | proven-extracted |
| `bertie_tls::der::der_sig_unique` | `src/der.rs:556` | 37 | 31 | proven-extracted |
| `bertie_tls::der::der_sig_valid` | `src/der.rs:126` | 34 | 28 | proven-extracted |
| `bertie_tls::der::der_uint_valid` | `src/der.rs:106` | 19 | 11 | proven-extracted |
| `bertie_tls::der::encode_ecdsa_signature` | `src/der.rs:357` | 26 | 25 | proven-extracted |
| `bertie_tls::der::encode_uint` | `src/der.rs:336` | 20 | 17 | proven-extracted |
| `bertie_tls::der::encoder_correct` | `src/der.rs:161` | 8 | 7 | proven-extracted |
| `bertie_tls::der::parser_complete` | `src/der.rs:181` | 19 | 17 | proven-extracted |
| `bertie_tls::der::parser_sound` | `src/der.rs:170` | 10 | 9 | proven-extracted |
| `bertie_tls::der::pow256` | `src/der.rs:205` | 11 | 10 | proven-extracted |
| `bertie_tls::der::pow256_32_is_bound` | `src/der.rs:241` | 7 | 6 | proven-extracted |
| `bertie_tls::der::pow256_is_pow2` | `src/der.rs:229` | 11 | 10 | proven-extracted |
| `bertie_tls::der::pow256_mono` | `src/der.rs:217` | 11 | 10 | proven-extracted |
| `bertie_tls::der::shifted_eq` | `src/der.rs:255` | 10 | 9 | proven-extracted |
| `bertie_tls::der::skip_zeros` | `src/der.rs:313` | 18 | 17 | proven-extracted |
| `bertie_tls::der::tests::der_sig_valid_for_some_values` | `src/der.rs:646` | 15 | 13 | unverified |
| `bertie_tls::der::two_pow_256` | `src/der.rs:84` | 5 | 4 | tested |
| `bertie_tls::der::uint_len_unique` | `src/der.rs:512` | 14 | 13 | proven-extracted |
| `bertie_tls::der::uint_range` | `src/der.rs:494` | 17 | 15 | proven-extracted |
| `bertie_tls::der::uint_to_32_bytes` | `src/der.rs:388` | 30 | 28 | proven-extracted |
| `bertie_tls::der::uint_unique` | `src/der.rs:539` | 9 | 8 | proven-extracted |
| `bertie_tls::der::zero_prefix` | `src/der.rs:249` | 5 | 4 | proven-extracted |
| `bertie_tls::server::ServerDB::new` | `src/server.rs:24` | 16 | 13 | tested |
| `bertie_tls::server::lookup_db` | `src/server.rs:57` | 35 | 32 | trusted |
| `bertie_tls::stream::bertie_stream::TlsStream::read_tls` | `src/stream/bertie_stream.rs:34` | 2 | 1 | unverified |
| `bertie_tls::stream::bertie_stream::TlsStream::stream_mut` | `src/stream/bertie_stream.rs:37` | 2 | 1 | unverified |
| `bertie_tls::stream::bertie_stream::TlsStream::write_tls` | `src/stream/bertie_stream.rs:31` | 2 | 1 | unverified |
| `bertie_tls::stream::bertie_stream::read_record` | `src/stream/bertie_stream.rs:48` | 34 | 33 | unverified |
| `bertie_tls::stream::client::<Stream:Read+Write>BertieStream<ClientState<Stream>>::open_with_stream` | `src/stream/client.rs:81` | 12 | 11 | tested |
| `bertie_tls::stream::client::<Stream:Read+Write>ClientState<Stream>::new` | `src/stream/client.rs:30` | 8 | 7 | tested |
| `bertie_tls::stream::client::BertieStream<ClientState<TcpStream>>::client` | `src/stream/client.rs:96` | 16 | 11 | tested |
| `bertie_tls::stream::client::BertieStream<ClientState<TcpStream>>::open` | `src/stream/client.rs:113` | 10 | 9 | tested |
| `bertie_tls::stream::client::BertieStream<ClientState<TcpStream>>::read` | `src/stream/client.rs:234` | 6 | 3 | tested |
| `bertie_tls::stream::client::BertieStream<ClientState<TcpStream>>::start` | `src/stream/client.rs:124` | 75 | 72 | unverified |
| `bertie_tls::stream::client::BertieStream<ClientState<TcpStream>>::write` | `src/stream/client.rs:241` | 7 | 4 | tested |
| `bertie_tls::stream::server::<Stream:Read+Write>BertieStream<ServerState<Stream>>::server_with_stream` | `src/stream/server.rs:92` | 15 | 14 | unverified |
| `bertie_tls::stream::server::<Stream:Read+Write>ServerState<Stream>::new` | `src/stream/server.rs:40` | 9 | 8 | tested |
| `bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::close` | `src/stream/server.rs:237` | 11 | 10 | unverified |
| `bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::connect` | `src/stream/server.rs:186` | 43 | 41 | tested |
| `bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::read` | `src/stream/server.rs:275` | 6 | 3 | tested |
| `bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::server` | `src/stream/server.rs:111` | 73 | 20 | tested |
| `bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::write` | `src/stream/server.rs:282` | 7 | 4 | tested |
| `bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::write_all` | `src/stream/server.rs:265` | 9 | 6 | unverified |
| `bertie_tls::stream::server::init_db` | `src/stream/server.rs:291` | 37 | 36 | unverified |
| `bertie_tls::stream::server::read_file` | `src/stream/server.rs:335` | 8 | 7 | unverified |
| `bertie_tls::tls13api::Client::connect` | `src/tls13api.rs:72` | 33 | 18 | tested |
| `bertie_tls::tls13api::Client::read` | `src/tls13api.rs:146` | 28 | 15 | tested |
| `bertie_tls::tls13api::Client::read_handshake` | `src/tls13api.rs:108` | 37 | 26 | tested |
| `bertie_tls::tls13api::Client::write` | `src/tls13api.rs:175` | 15 | 9 | tested |
| `bertie_tls::tls13api::Server::accept` | `src/tls13api.rs:209` | 40 | 25 | tested |
| `bertie_tls::tls13api::Server::close_notify` | `src/tls13api.rs:312` | 12 | 6 | tested |
| `bertie_tls::tls13api::Server::read` | `src/tls13api.rs:289` | 22 | 9 | tested |
| `bertie_tls::tls13api::Server::read_handshake` | `src/tls13api.rs:250` | 22 | 14 | tested |
| `bertie_tls::tls13api::Server::write` | `src/tls13api.rs:273` | 15 | 9 | tested |
| `bertie_tls::tls13api::get_server_info` | `src/tls13api.rs:52` | 17 | 8 | unverified |
| `bertie_tls::tls13api::in_psk_mode` | `src/tls13api.rs:40` | 11 | 8 | unverified |
| `bertie_tls::tls13cert::cert_public_key` | `src/tls13cert.rs:464` | 39 | 35 | trusted |
| `bertie_tls::tls13cert::check_tag` | `src/tls13cert.rs:151` | 15 | 14 | trusted |
| `bertie_tls::tls13cert::debug_print` | `src/tls13cert.rs:425` | 10 | 9 | unverified |
| `bertie_tls::tls13cert::ecdsa_public_key` | `src/tls13cert.rs:378` | 11 | 10 | trusted |
| `bertie_tls::tls13cert::length` | `src/tls13cert.rs:101` | 23 | 18 | trusted |
| `bertie_tls::tls13cert::read_integer` | `src/tls13cert.rs:209` | 7 | 6 | trusted |
| `bertie_tls::tls13cert::read_octet_header` | `src/tls13cert.rs:138` | 10 | 7 | trusted |
| `bertie_tls::tls13cert::read_sequence_header` | `src/tls13cert.rs:125` | 10 | 7 | trusted |
| `bertie_tls::tls13cert::read_spki` | `src/tls13cert.rs:238` | 64 | 61 | trusted |
| `bertie_tls::tls13cert::read_version_number` | `src/tls13cert.rs:181` | 15 | 10 | trusted |
| `bertie_tls::tls13cert::rsa_private_key` | `src/tls13cert.rs:436` | 25 | 11 | trusted |
| `bertie_tls::tls13cert::skip_integer` | `src/tls13cert.rs:198` | 9 | 6 | trusted |
| `bertie_tls::tls13cert::skip_sequence` | `src/tls13cert.rs:168` | 10 | 6 | trusted |
| `bertie_tls::tls13cert::verification_key_from_cert` | `src/tls13cert.rs:317` | 50 | 46 | trusted |
| `bertie_tls::tls13crypto::AeadAlgorithm::iv_len` | `src/tls13crypto.rs:305` | 9 | 8 | trusted |
| `bertie_tls::tls13crypto::AeadAlgorithm::key_len` | `src/tls13crypto.rs:295` | 9 | 8 | trusted |
| `bertie_tls::tls13crypto::AeadKey::bytes` | `src/tls13crypto.rs:67` | 5 | 4 | unverified |
| `bertie_tls::tls13crypto::AeadKey::new` | `src/tls13crypto.rs:62` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::AeadKeyIV::new` | `src/tls13crypto.rs:49` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::Algorithms::aead` | `src/tls13crypto.rs:910` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::Algorithms::check` | `src/tls13crypto.rs:979` | 12 | 11 | unverified |
| `bertie_tls::tls13crypto::Algorithms::ciphersuite` | `src/tls13crypto.rs:935` | 13 | 11 | tested |
| `bertie_tls::tls13crypto::Algorithms::hash` | `src/tls13crypto.rs:905` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::Algorithms::kem` | `src/tls13crypto.rs:920` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::Algorithms::new` | `src/tls13crypto.rs:844` | 18 | 17 | tested |
| `bertie_tls::tls13crypto::Algorithms::psk_mode` | `src/tls13crypto.rs:925` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::Algorithms::signature` | `src/tls13crypto.rs:915` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::Algorithms::signature_algorithm` | `src/tls13crypto.rs:966` | 12 | 10 | unverified |
| `bertie_tls::tls13crypto::Algorithms::supported_group` | `src/tls13crypto.rs:949` | 16 | 14 | tested |
| `bertie_tls::tls13crypto::Algorithms::zero_rtt` | `src/tls13crypto.rs:930` | 4 | 3 | tested |
| `bertie_tls::tls13crypto::HashAlgorithm::hash_len` | `src/tls13crypto.rs:153` | 10 | 9 | trusted |
| `bertie_tls::tls13crypto::HashAlgorithm::hmac_algorithm` | `src/tls13crypto.rs:164` | 13 | 12 | unverified |
| `bertie_tls::tls13crypto::HashAlgorithm::hmac_tag_len` | `src/tls13crypto.rs:178` | 5 | 4 | trusted |
| `bertie_tls::tls13crypto::HashAlgorithm::libcrux_algorithm` | `src/tls13crypto.rs:139` | 13 | 12 | unverified |
| `bertie_tls::tls13crypto::KemScheme::libcrux_kem_algorithm` | `src/tls13crypto.rs:645` | 12 | 11 | unverified |
| `bertie_tls::tls13crypto::TryFrom<&str>forAlgorithms::try_from` | `src/tls13crypto.rs:997` | 26 | 25 | trusted |
| `bertie_tls::tls13crypto::aead_decrypt` | `src/tls13crypto.rs:351` | 29 | 28 | unverified |
| `bertie_tls::tls13crypto::aead_encrypt` | `src/tls13crypto.rs:316` | 31 | 30 | unverified |
| `bertie_tls::tls13crypto::encoding_prefix` | `src/tls13crypto.rs:729` | 10 | 7 | trusted |
| `bertie_tls::tls13crypto::hash` | `src/tls13crypto.rs:124` | 10 | 7 | trusted |
| `bertie_tls::tls13crypto::hkdf_algorithm` | `src/tls13crypto.rs:237` | 13 | 12 | trusted |
| `bertie_tls::tls13crypto::hkdf_expand` | `src/tls13crypto.rs:265` | 20 | 17 | trusted |
| `bertie_tls::tls13crypto::hkdf_extract` | `src/tls13crypto.rs:251` | 13 | 10 | trusted |
| `bertie_tls::tls13crypto::hmac_tag` | `src/tls13crypto.rs:185` | 13 | 10 | trusted |
| `bertie_tls::tls13crypto::hmac_verify` | `src/tls13crypto.rs:199` | 32 | 29 | trusted |
| `bertie_tls::tls13crypto::into_raw` | `src/tls13crypto.rs:740` | 9 | 7 | trusted |
| `bertie_tls::tls13crypto::kem_decap` | `src/tls13crypto.rs:798` | 25 | 24 | trusted |
| `bertie_tls::tls13crypto::kem_encap` | `src/tls13crypto.rs:750` | 31 | 30 | trusted |
| `bertie_tls::tls13crypto::kem_keygen` | `src/tls13crypto.rs:695` | 28 | 27 | trusted |
| `bertie_tls::tls13crypto::sign` | `src/tls13crypto.rs:439` | 57 | 56 | trusted |
| `bertie_tls::tls13crypto::sign_rsa` | `src/tls13crypto.rs:393` | 37 | 36 | unverified |
| `bertie_tls::tls13crypto::supported_rsa_key_size` | `src/tls13crypto.rs:613` | 9 | 7 | unverified |
| `bertie_tls::tls13crypto::to_shared_secret` | `src/tls13crypto.rs:787` | 10 | 9 | trusted |
| `bertie_tls::tls13crypto::valid_rsa_exponent` | `src/tls13crypto.rs:624` | 6 | 4 | unverified |
| `bertie_tls::tls13crypto::verify` | `src/tls13crypto.rs:499` | 104 | 101 | trusted |
| `bertie_tls::tls13crypto::zero_key` | `src/tls13crypto.rs:232` | 4 | 3 | trusted |
| `bertie_tls::tls13formats::ContentType::try_from_u8` | `src/tls13formats.rs:1315` | 10 | 9 | unverified |
| `bertie_tls::tls13formats::Extensions::merge` | `src/tls13formats.rs:231` | 10 | 8 | unverified |
| `bertie_tls::tls13formats::Transcript::add` | `src/tls13formats.rs:1388` | 25 | 24 | trusted |
| `bertie_tls::tls13formats::Transcript::transcript_hash` | `src/tls13formats.rs:1414` | 5 | 4 | unverified |
| `bertie_tls::tls13formats::Transcript::transcript_hash_without_client_hello` | `src/tls13formats.rs:1420` | 18 | 17 | unverified |
| `bertie_tls::tls13formats::build_server_name` | `src/tls13formats.rs:59` | 12 | 11 | unverified |
| `bertie_tls::tls13formats::check_extension` | `src/tls13formats.rs:254` | 71 | 70 | unverified |
| `bertie_tls::tls13formats::check_server_name` | `src/tls13formats.rs:72` | 14 | 10 | unverified |
| `bertie_tls::tls13formats::check_supported_versions` | `src/tls13formats.rs:95` | 5 | 4 | unverified |
| `bertie_tls::tls13formats::client_hello` | `src/tls13formats.rs:552` | 86 | 85 | trusted |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::as_handshake_message` | `src/tls13formats/handshake_data.rs:186` | 24 | 18 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::concat` | `src/tls13formats/handshake_data.rs:175` | 9 | 7 | tested |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::find_handshake_message` | `src/tls13formats/handshake_data.rs:251` | 25 | 22 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::from_bytes` | `src/tls13formats/handshake_data.rs:152` | 10 | 9 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::len` | `src/tls13formats/handshake_data.rs:163` | 6 | 5 | trusted |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::next_handshake_message` | `src/tls13formats/handshake_data.rs:211` | 19 | 13 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_bytes` | `src/tls13formats/handshake_data.rs:170` | 4 | 3 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_four` | `src/tls13formats/handshake_data.rs:240` | 10 | 5 | unverified |
| `bertie_tls::tls13formats::handshake_data::HandshakeData::to_two` | `src/tls13formats/handshake_data.rs:231` | 8 | 3 | unverified |
| `bertie_tls::tls13formats::merge_opts` | `src/tls13formats.rs:243` | 10 | 8 | unverified |
| `bertie_tls::tls13formats::parse_client_hello` | `src/tls13formats.rs:733` | 142 | 141 | trusted |
| `bertie_tls::tls13formats::server_hello` | `src/tls13formats.rs:876` | 50 | 49 | trusted |
| `bertie_tls::tls13formats::supported_versions` | `src/tls13formats.rs:87` | 7 | 6 | unverified |
| `bertie_tls::tls13handshake::client_set_params` | `src/tls13handshake.rs:511` | 9 | 8 | unverified |
| `bertie_tls::tls13handshake::process_psk_binder_zero_rtt` | `src/tls13handshake.rs:590` | 35 | 34 | proven-extracted |
| `bertie_tls::tls13keyscheduler::derive_0rtt_keys` | `src/tls13keyscheduler.rs:100` | 19 | 18 | proven-extracted |
| `bertie_tls::tls13keyscheduler::derive_aead_key_iv` | `src/tls13keyscheduler.rs:27` | 52 | 51 | trusted |
| `bertie_tls::tls13keyscheduler::derive_app_handles` | `src/tls13keyscheduler.rs:212` | 16 | 15 | proven-extracted |
| `bertie_tls::tls13keyscheduler::derive_hk_handles` | `src/tls13keyscheduler.rs:157` | 31 | 30 | proven-extracted |
| `bertie_tls::tls13keyscheduler::hash_empty` | `src/tls13keyscheduler.rs:10` | 5 | 4 | proven-extracted |
| `bertie_tls::tls13keyscheduler::key_schedule::hkdf_expand_label` | `src/tls13keyscheduler/key_schedule.rs:11` | 20 | 19 | unverified |
| `bertie_tls::tls13keyscheduler::key_schedule::no_psk` | `src/tls13keyscheduler/key_schedule.rs:47` | 11 | 10 | unverified |
| `bertie_tls::tls13keyscheduler::key_schedule::zero_ikm` | `src/tls13keyscheduler/key_schedule.rs:59` | 10 | 9 | unverified |
| `bertie_tls::tls13keyscheduler::key_schedule::zero_salt` | `src/tls13keyscheduler/key_schedule.rs:32` | 14 | 13 | unverified |
| `bertie_tls::tls13record::DuplexCipherStateH::new` | `src/tls13record.rs:53` | 14 | 13 | tested |
| `bertie_tls::tls13record::client_cipher_state0` | `src/tls13record.rs:14` | 9 | 8 | trusted |
| `bertie_tls::tls13record::decrypt_handshake` | `src/tls13record.rs:242` | 48 | 47 | trusted |
| `bertie_tls::tls13record::decrypt_record_payload` | `src/tls13record.rs:135` | 25 | 24 | trusted |
| `bertie_tls::tls13record::decrypt_zerortt` | `src/tls13record.rs:185` | 19 | 17 | trusted |
| `bertie_tls::tls13record::derive_iv_ctr` | `src/tls13record.rs:83` | 15 | 14 | trusted |
| `bertie_tls::tls13record::duplex_cipher_state1` | `src/tls13record.rs:71` | 11 | 10 | trusted |
| `bertie_tls::tls13record::encrypt_close_notify` | `src/tls13record.rs:308` | 13 | 8 | trusted |
| `bertie_tls::tls13record::encrypt_handshake` | `src/tls13record.rs:205` | 34 | 30 | trusted |
| `bertie_tls::tls13record::encrypt_record_payload` | `src/tls13record.rs:99` | 24 | 23 | trusted |
| `bertie_tls::tls13record::encrypt_zerortt` | `src/tls13record.rs:165` | 19 | 17 | trusted |
| `bertie_tls::tls13record::server_cipher_state0` | `src/tls13record.rs:31` | 12 | 11 | trusted |
| `bertie_tls::tls13utils::AppData::as_raw` | `src/tls13utils.rs:946` | 4 | 3 | tested |
| `bertie_tls::tls13utils::AppData::into_raw` | `src/tls13utils.rs:941` | 4 | 3 | unverified |
| `bertie_tls::tls13utils::AppData::new` | `src/tls13utils.rs:936` | 4 | 3 | tested |
| `bertie_tls::tls13utils::Bytes::append` | `src/tls13utils.rs:456` | 5 | 4 | unverified |
| `bertie_tls::tls13utils::Bytes::as_hex` | `src/tls13utils.rs:560` | 10 | 9 | unverified |
| `bertie_tls::tls13utils::Bytes::as_raw` | `src/tls13utils.rs:252` | 6 | 5 | tested |
| `bertie_tls::tls13utils::Bytes::concat` | `src/tls13utils.rs:508` | 5 | 4 | tested |
| `bertie_tls::tls13utils::Bytes::concat_array` | `src/tls13utils.rs:514` | 5 | 4 | unverified |
| `bertie_tls::tls13utils::Bytes::declassify` | `src/tls13utils.rs:241` | 4 | 3 | tested |
| `bertie_tls::tls13utils::Bytes::extend_from_slice` | `src/tls13utils.rs:451` | 4 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::from_hex` | `src/tls13utils.rs:467` | 19 | 18 | tested |
| `bertie_tls::tls13utils::Bytes::from_slice` | `src/tls13utils.rs:462` | 4 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::into_raw` | `src/tls13utils.rs:246` | 5 | 4 | unverified |
| `bertie_tls::tls13utils::Bytes::len` | `src/tls13utils.rs:440` | 5 | 4 | tested |
| `bertie_tls::tls13utils::Bytes::new` | `src/tls13utils.rs:421` | 5 | 4 | tested |
| `bertie_tls::tls13utils::Bytes::new_alloc` | `src/tls13utils.rs:427` | 5 | 4 | unverified |
| `bertie_tls::tls13utils::Bytes::prefix` | `src/tls13utils.rs:226` | 12 | 11 | trusted |
| `bertie_tls::tls13utils::Bytes::push` | `src/tls13utils.rs:446` | 4 | 3 | unverified |
| `bertie_tls::tls13utils::Bytes::raw_slice` | `src/tls13utils.rs:487` | 6 | 5 | unverified |
| `bertie_tls::tls13utils::Bytes::slice` | `src/tls13utils.rs:501` | 6 | 5 | unverified |
| `bertie_tls::tls13utils::Bytes::slice_range` | `src/tls13utils.rs:494` | 6 | 5 | unverified |
| `bertie_tls::tls13utils::Bytes::update_slice` | `src/tls13utils.rs:520` | 15 | 13 | unverified |
| `bertie_tls::tls13utils::Bytes::zeroes` | `src/tls13utils.rs:433` | 6 | 5 | tested |
| `bertie_tls::tls13utils::_update_at_usize_bytes_test` | `src/tls13utils.rs:366` | 15 | 14 | unverified |
| `bertie_tls::tls13utils::check` | `src/tls13utils.rs:572` | 12 | 11 | trusted |
| `bertie_tls::tls13utils::check_eq` | `src/tls13utils.rs:664` | 9 | 7 | trusted |
| `bertie_tls::tls13utils::check_eq1` | `src/tls13utils.rs:590` | 13 | 11 | trusted |
| `bertie_tls::tls13utils::check_eq_option` | `src/tls13utils.rs:674` | 13 | 11 | trusted |
| `bertie_tls::tls13utils::check_eq_slice` | `src/tls13utils.rs:628` | 11 | 9 | trusted |
| `bertie_tls::tls13utils::check_eq_with_slice` | `src/tls13utils.rs:640` | 23 | 21 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u16` | `src/tls13utils.rs:903` | 11 | 6 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u24` | `src/tls13utils.rs:915` | 15 | 10 | trusted |
| `bertie_tls::tls13utils::check_length_encoding_u8` | `src/tls13utils.rs:879` | 11 | 7 | trusted |
| `bertie_tls::tls13utils::check_mem` | `src/tls13utils.rs:689` | 21 | 18 | trusted |
| `bertie_tls::tls13utils::encode_length_u16` | `src/tls13utils.rs:732` | 21 | 16 | trusted |
| `bertie_tls::tls13utils::encode_length_u24` | `src/tls13utils.rs:754` | 22 | 17 | trusted |
| `bertie_tls::tls13utils::encode_length_u8` | `src/tls13utils.rs:711` | 20 | 15 | trusted |
| `bertie_tls::tls13utils::eq` | `src/tls13utils.rs:622` | 5 | 3 | trusted |
| `bertie_tls::tls13utils::eq1` | `src/tls13utils.rs:585` | 4 | 3 | trusted |
| `bertie_tls::tls13utils::eq_slice` | `src/tls13utils.rs:605` | 15 | 13 | trusted |
| `bertie_tls::tls13utils::length_u16_encoded` | `src/tls13utils.rs:826` | 14 | 8 | trusted |
| `bertie_tls::tls13utils::length_u16_encoded_slice` | `src/tls13utils.rs:800` | 25 | 19 | trusted |
| `bertie_tls::tls13utils::length_u24_encoded` | `src/tls13utils.rs:841` | 26 | 20 | trusted |
| `bertie_tls::tls13utils::length_u8_encoded` | `src/tls13utils.rs:777` | 22 | 16 | trusted |
| `bogo_shim::main` | `bogo_shim/src/main.rs:108` | 132 | 121 | unverified |
| `record::debug::info_record` | `record/src/debug.rs:6` | 35 | 34 | unverified |
| `simple_https_client::tls13client::main` | `simple_https_client/src/tls13client.rs:34` | 38 | 34 | unverified |

## Independent source count by file

| Security file | All doc lines | Distinct unit-owned doc-only lines | Unit-weighted lines |
| --- | ---: | ---: | ---: |
| `bogo_shim/src/main.rs` | 16 | 11 | 11 |
| `record/src/debug.rs` | 1 | 1 | 1 |
| `record/src/lib.rs` | 3 | 0 | 0 |
| `record/src/stream.rs` | 0 | 0 | 0 |
| `simple_https_client/src/tls13client.rs` | 16 | 4 | 4 |
| `simple_https_server/src/tls13server.rs` | 14 | 0 | 0 |
| `src/der.rs` | 131 | 58 | 58 |
| `src/server.rs` | 15 | 6 | 6 |
| `src/stream/bertie_stream.rs` | 4 | 4 | 4 |
| `src/stream/client.rs` | 20 | 17 | 17 |
| `src/stream/server.rs` | 73 | 69 | 69 |
| `src/test_utils.rs` | 0 | 0 | 0 |
| `src/tls13api.rs` | 121 | 105 | 105 |
| `src/tls13cert.rs` | 70 | 51 | 51 |
| `src/tls13crypto.rs` | 88 | 61 | 61 |
| `src/tls13formats.rs` | 71 | 19 | 19 |
| `src/tls13formats/handshake_data.rs` | 47 | 30 | 30 |
| `src/tls13handshake.rs` | 7 | 2 | 2 |
| `src/tls13keyscheduler.rs` | 5 | 5 | 5 |
| `src/tls13keyscheduler/key_schedule.rs` | 4 | 4 | 4 |
| `src/tls13record.rs` | 23 | 21 | 21 |
| `src/tls13utils.rs` | 99 | 96 | 96 |

## Report hashes

- `v172.json` SHA-256: `5a7f02bf937ed5cc4928434273609de2f2cf7cb223a7508d5faccd6c8dbfd891`.
- `v173.json` SHA-256: `735e772a98dea6384f0c8ca79da232da3af20e2ca1a837a905515fb31e6c1c82`.
