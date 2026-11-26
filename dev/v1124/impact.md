# v1.12.4 measured impact (compiled-files CLI reports, v1.12.3 vs v1.12.4)

Reports: `formal-coverage report <dir> --check-target-dir <scratch>` with the v1.12.3
release binary (built from cb3d236) and the v1.12.4 candidate. No prover logs, so
proven is 0 throughout. First-party security-relevant SLoC; unverified includes
hazard (shown separately). Test code is first-party test SLoC, outside runtime.

| Workspace | proven | tested | unverified (hazard) | total | test code |
| --- | ---: | ---: | ---: | ---: | ---: |
| fixtures/classification | 0 | 0 | 3 (0) | 3 | 0 |
| fixtures/doc_comments | 0 | 0 | 3 (0) | 3 | 0 |
| fixtures/evidence | 0 | 1 | 8 (0) | 9 | 2 |
| fixtures/ghost | 0 | 0 | 41 (5) | 41 | 0 |
| fixtures/hax | 0 | 0 | 3 (0) | 3 | 0 |
| fixtures/inventory | 0 | 0 | 11 (0) | 11 | 0 |
| fix-3 identity-attribute counterexample | 0 | 0 | 5 (0) | 5 | 0 |
| bertie-tls origin/main 1a46d03 | 0 | 1578 | 3454 (0) | 5032 | 506 |
| scratch-server 8e663e9 | 0 | 824 → 666 | 886 (83) → 700 (83) | 1710 → 1366 | 2633 → 2977 |

Escape hatches (hazards) and warnings are identical before and after in every workspace.
Unit ids and SLoC are unchanged; only category, status and test evidence move.

## bertie-tls origin/main 1a46d03: 25 units change

### test (builtin-attribute) → test (cfg-test): 25 units, 439 SLoC

- `bertie_tls::der::tests::known_answers_from_x690` (29, unverified)
- `bertie_tls::der::tests::spec_distinguishes_values` (9, unverified)
- `bertie_tls::server::psk_tests::lookup_db_ignores_client_ticket_when_psk_is_disabled` (17, unverified)
- `bertie_tls::server::psk_tests::lookup_db_refuses_psk_acceptance` (15, unverified)
- `bertie_tls::server::psk_tests::lookup_db_reports_mismatch_when_psk_mode_lacks_ticket_or_server_psk` (16, unverified)
- `bertie_tls::stream::client::tests::client` (19, unverified)
- `bertie_tls::tls13cert::unit_test::ecdsa_cert` (6, unverified)
- `bertie_tls::tls13cert::unit_test::long_length_bounds` (9, unverified)
- `bertie_tls::tls13cert::unit_test::malformed_certificates_do_not_panic` (36, unverified)
- `bertie_tls::tls13cert::unit_test::read_cert` (14, unverified)
- `bertie_tls::tls13cert::unit_test::rsa_cert` (5, unverified)
- `bertie_tls::tls13cert::unit_test::valid_certificates_give_the_expected_keys` (17, unverified)
- `bertie_tls::tls13crypto::maintained_kem_tests::supported_groups_keep_wire_sizes_and_shared_secrets` (48, unverified)
- `bertie_tls::tls13crypto::malformed_aead_tests::decrypt_rejects_ciphertext_shorter_than_tag` (10, unverified)
- `bertie_tls::tls13crypto::malformed_kem_input_tests::decap_rejects_empty_nist_ciphertext` (5, unverified)
- `bertie_tls::tls13crypto::malformed_kem_input_tests::encap_rejects_empty_nist_public_key` (5, unverified)
- `bertie_tls::tls13crypto::malformed_kem_input_tests::nist_shares_require_uncompressed_prefix` (23, unverified)
- `bertie_tls::tls13crypto::malformed_kem_input_tests::shared_secret_checks_bounds_and_unsupported_curves` (21, unverified)
- `bertie_tls::tls13formats::bounds_tests::binder_update_checks_source_length` (38, unverified)
- `bertie_tls::tls13formats::bounds_tests::truncated_transcript_preserves_valid_input` (20, unverified)
- `bertie_tls::tls13formats::handshake_data::capacity_tests::concatenation_preserves_valid_messages` (11, unverified)
- `bertie_tls::tls13formats::psk_tests::client_hello_refuses_psk` (13, unverified)
- `bertie_tls::tls13handshake::signature_input_tests::signature_input_checks_digest_capacity` (12, unverified)
- `bertie_tls::tls13keyscheduler::key_schedule::label_tests::hkdf_label_wire_length_bound` (30, unverified)
- `bertie_tls::tls13utils::tests::declassify_array_checks_length_and_preserves_values` (11, unverified)

## scratch-server 8e663e9: 81 units change

### test (builtin-attribute) → test (cfg-test): 29 units, 995 SLoC

- `scratch_server::dispatch_oracle::stage14_compatibility_header_spans_contain_bare_line_breaks` (12, unverified)
- `scratch_server::dispatch_oracle::stage14_compatibility_malformed_name_changes_rejection_diagnostic` (18, unverified)
- `scratch_server::dispatch_oracle::stage14_header_and_diagnostic_equivalence_100000_blocks` (101, unverified)
- `scratch_server::dispatch_oracle::stage14_limiter_equivalence_sliding_windows` (49, unverified)
- `scratch_server::dispatch_oracle::stage14_preserved_oracle_sanitizer_and_limiter` (24, unverified)
- `scratch_server::dispatch_oracle::stage14_sanitizer_equivalence_random_names` (15, unverified)
- `scratch_server::policy_tests::diagnostic_classes_and_privacy` (43, unverified)
- `scratch_server::policy_tests::unknown_log_is_bounded_in_every_sliding_minute` (17, unverified)
- `scratch_server::record_tests::stage15_aad_equivalence_80001` (11, unverified)
- `scratch_server::record_tests::stage15_inner_and_length_equivalence` (34, unverified)
- `scratch_server::record_tests::stage15_inner_plaintext_exact_capacity` (8, unverified)
- `scratch_server::record_tests::stage15_key_shape_and_offers_equivalence` (23, unverified)
- `scratch_server::record_tests::stage15_unpad_equivalence_100000` (17, unverified)
- `scratch_server::tls_test_provider::tests::aead_rejects_tampered_record_and_x25519_rejects_low_order` (17, unverified)
- `scratch_server::tls_test_provider::tests::bertie_alert_and_corrupt_record_never_dispatch` (46, unverified)
- `scratch_server::tls_test_provider::tests::bertie_healthcheck_absolute_timeout` (18, unverified)
- `scratch_server::tls_test_provider::tests::bertie_healthcheck_closed_port` (7, unverified)
- `scratch_server::tls_test_provider::tests::bertie_healthcheck_rejects_non_tls_alert_and_bad_framing` (27, unverified)
- `scratch_server::tls_test_provider::tests::bertie_healthcheck_serverhello` (19, unverified)
- `scratch_server::tls_test_provider::tests::bertie_signatures_are_canonical` (31, unverified)
- `scratch_server::tls_test_provider::tests::default_policy_browser_fixtures_and_sensitive_names` (88, unverified)
- `scratch_server::tls_test_provider::tests::hkdf_rfc5869_case1_and_hash_fork` (31, unverified)
- `scratch_server::tls_test_provider::tests::idle_and_partial_deadlines_tcp_and_tls` (90, unverified)
- `scratch_server::tls_test_provider::tests::idle_handshake_budget_is_absolute` (40, unverified)
- `scratch_server::tls_test_provider::tests::malformed_tls_never_dispatches` (60, unverified)
- `scratch_server::tls_test_provider::tests::tls_case_and_count_boundaries` (71, unverified)
- `scratch_server::tls_test_provider::tests::tls_routes_and_safari` (52, unverified)
- `transport_core::hello::tests::captured_clienthellos` (9, unverified)
- `transport_core::hello::tests::choices_and_rejections` (17, unverified)

### runtime (none) → test (cfg-test): 48 units, 344 SLoC

- `scratch_server::record_tests::compare_unpad` (13, tested → unverified)
- `scratch_server::record_tests::random` (6, tested → unverified)
- `scratch_server::tls_test_provider::HkdfExpanderforExpander::expand_block` (6, unverified)
- `scratch_server::tls_test_provider::HkdfExpanderforExpander::expand_slice` (9, tested → unverified)
- `scratch_server::tls_test_provider::HkdfExpanderforExpander::hash_len` (3, unverified)
- `scratch_server::tls_test_provider::MessageDecrypterforRecordCipher::decrypt` (26, tested → unverified)
- `scratch_server::tls_test_provider::MessageEncrypterforRecordCipher::encrypt` (34, tested → unverified)
- `scratch_server::tls_test_provider::MessageEncrypterforRecordCipher::encrypted_payload_len` (3, tested → unverified)
- `scratch_server::tls_test_provider::SignatureVerificationAlgorithmforP256Verify::public_key_alg_id` (3, unverified)
- `scratch_server::tls_test_provider::SignatureVerificationAlgorithmforP256Verify::signature_alg_id` (3, unverified)
- `scratch_server::tls_test_provider::SignatureVerificationAlgorithmforP256Verify::verify_signature` (24, unverified)
- `scratch_server::tls_test_provider::SignerforP256Signer::scheme` (3, tested → unverified)
- `scratch_server::tls_test_provider::SignerforP256Signer::sign` (13, unverified)
- `scratch_server::tls_test_provider::SigningKeyforP256SigningKey::algorithm` (3, unverified)
- `scratch_server::tls_test_provider::SigningKeyforP256SigningKey::choose_scheme` (5, unverified)
- `scratch_server::tls_test_provider::SigningKeyforP256SigningKey::public_key` (3, unverified)
- `scratch_server::tls_test_provider::Tls13AeadAlgorithmforChaCha::decrypter` (3, unverified)
- `scratch_server::tls_test_provider::Tls13AeadAlgorithmforChaCha::encrypter` (3, unverified)
- `scratch_server::tls_test_provider::Tls13AeadAlgorithmforChaCha::extract_keys` (7, unverified)
- `scratch_server::tls_test_provider::Tls13AeadAlgorithmforChaCha::key_len` (3, unverified)
- `scratch_server::tls_test_provider::crypto::ActiveKeyExchangeforExchange::complete` (7, tested → unverified)
- `scratch_server::tls_test_provider::crypto::ActiveKeyExchangeforExchange::group` (3, tested → unverified)
- `scratch_server::tls_test_provider::crypto::ActiveKeyExchangeforExchange::pub_key` (3, unverified)
- `scratch_server::tls_test_provider::crypto::KeyProviderforP256KeyProvider::load_private_key` (31, unverified)
- `scratch_server::tls_test_provider::crypto::SecureRandomforOsRandom::fill` (3, unverified)
- `scratch_server::tls_test_provider::crypto::SupportedKxGroupforX25519::name` (3, tested → unverified)
- `scratch_server::tls_test_provider::crypto::SupportedKxGroupforX25519::start` (8, tested → unverified)
- `scratch_server::tls_test_provider::crypto::tls13::HkdfforHkdf::expander_for_okm` (3, unverified)
- `scratch_server::tls_test_provider::crypto::tls13::HkdfforHkdf::extract_from_secret` (11, tested → unverified)
- `scratch_server::tls_test_provider::crypto::tls13::HkdfforHkdf::extract_from_zero_ikm` (3, unverified)
- `scratch_server::tls_test_provider::crypto::tls13::HkdfforHkdf::hmac_sign` (5, unverified)
- `scratch_server::tls_test_provider::fmt::DebugforP256Signer::fmt` (3, unverified)
- `scratch_server::tls_test_provider::fmt::DebugforP256SigningKey::fmt` (3, unverified)
- `scratch_server::tls_test_provider::hash::ContextforHashContext::finish` (3, tested → unverified)
- `scratch_server::tls_test_provider::hash::ContextforHashContext::fork` (3, tested → unverified)
- `scratch_server::tls_test_provider::hash::ContextforHashContext::fork_finish` (5, tested → unverified)
- `scratch_server::tls_test_provider::hash::ContextforHashContext::update` (3, tested → unverified)
- `scratch_server::tls_test_provider::hash::HashforSha256::algorithm` (3, unverified)
- `scratch_server::tls_test_provider::hash::HashforSha256::hash` (3, tested → unverified)
- `scratch_server::tls_test_provider::hash::HashforSha256::output_len` (3, unverified)
- `scratch_server::tls_test_provider::hash::HashforSha256::start` (3, tested → unverified)
- `scratch_server::tls_test_provider::load_config` (8, unverified)
- `scratch_server::tls_test_provider::provider` (12, tested → unverified)
- `scratch_server::tls_test_provider::rand_core::TryRngforOsRandom::try_fill_bytes` (3, unverified)
- `scratch_server::tls_test_provider::rand_core::TryRngforOsRandom::try_next_u32` (5, unverified)
- `scratch_server::tls_test_provider::rand_core::TryRngforOsRandom::try_next_u64` (5, unverified)
- `scratch_server::tls_test_provider::rustls_server_config` (12, unverified)
- `scratch_server::tls_test_provider::unpad_message` (14, unverified)

### test (builtin-attribute) → test (test-target): 4 units, 53 SLoC

- `scratch_server::tests::differential::duplicates_reject_for_every_policy` (11, unverified)
- `scratch_server::tests::differential::fifty_thousand_inside_subset` (5, unverified)
- `scratch_server::tests::differential::fifty_thousand_mutated_or_arbitrary` (8, unverified)
- `scratch_server::tests::differential::grammar_boundaries_and_regressions` (29, unverified)
