Size mode: `compiled-files`; target: `aarch64-unknown-linux-gnu`; profile: `dev (cargo check)`; binary sha256: n/a. Ghost lines: `separate`;

SECURITY-RELEVANT First-party (with inputs)  5832 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 14.09%  unverified 60.19%  hazard 0.00%  trusted 25.72%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction

# Formal coverage

closure: cargo metadata (runtime)

**SECURITY HAZARD: 2 first-party security-relevant unit(s) weaken guarantees.** See file:line details below.

**ESCAPES: 3054 distinct sites, including outside verified scope; all remain gate-visible.**

```text
SECURITY-RELEVANT First-party (with inputs)  5832 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 14.09%  unverified 60.19%  hazard 0.00%  trusted 25.72%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
SECURITY-RELEVANT First-party (default rules only)  5832 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 14.09%  unverified 60.19%  hazard 0.00%  trusted 25.72%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
SECURITY-RELEVANT Third-party (runtime) (with inputs)  294934 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 0.00%  unverified 100.00%  hazard 0.00%  trusted 0.00%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
SECURITY-RELEVANT Third-party (runtime) (default rules only)  294934 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 0.00%  unverified 100.00%  hazard 0.00%  trusted 0.00%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
SECURITY-RELEVANT Combined (with inputs)  300766 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 0.27%  unverified 99.23%  hazard 0.00%  trusted 0.50%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
SECURITY-RELEVANT Combined (default rules only)  300766 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 0.27%  unverified 99.23%  hazard 0.00%  trusted 0.50%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
EVERYTHING ELSE First-party  0 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 0.00%  unverified 0.00%  hazard 0.00%  trusted 0.00%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
EVERYTHING ELSE Third-party (runtime)  0 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 0.00%  unverified 0.00%  hazard 0.00%  trusted 0.00%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
EVERYTHING ELSE Combined  0 measured SLoC   proven 0.00%  proven-bounded 0.00%  proven-extracted 0.00%  proven-upstream 0.00%  tested 0.00%  unverified 0.00%  hazard 0.00%  trusted 0.00%; proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction
```

Percentages are SLoC-weighted, mutually exclusive statuses over measured runtime code. First-party, third-party runtime and combined figures are also in JSON summary.first_party, summary.third_party and summary.all for each block. Headline trust/escape counts conservatively list the whole runtime inventory, including outside verified scope; each receipt lists its cfg-active scope separately. Unknown sizes are excluded and prominently flagged; gates fail closed when security dependency sizes are unknown.

Approximate syntactic identifier graph: ambiguous names include all matching functions; unresolved calls and macros include all first-party functions. No entry points includes all. structural test code (test attributes, items under cfg(test) at any depth, integration-test and bench targets) inventoried separately as test-code, excluded from runtime totals; other cfg-disabled code included; spec/proof functions inventoried separately as proof-code, excluded from runtime totals. Overrides apply after reachability (not_security last). Dependency closure and target are recorded separately; runtime dependency sizes use compiled-files: only the .rs files rustc read for each package under cargo check (dep-info, including generated OUT_DIR files), minus spec/proof functions; inline cfg-disabled items in compiled files still count.

Target: `aarch64-unknown-linux-gnu`; Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`.

Dev-dependencies excluded: 49 packages.

## Inputs

Config schema none (no config file). Every explicit input is listed with its reason. The "default rules only" headline figures drop all of them: the same analysis with no `[security.*]`, `[dependencies.*]` or schema-2 override entries. First-party security with inputs 0/5832 proven SLoC (0.00%); default rules only 0/5832 (0.00%); shift 0.00 percentage points. Runtime SLoC moved is credited to the input that decided each unit (the last match; excludes apply after includes). A dependency input moves that dependency's size.

None.

## Build-time dependencies

Build-time code can affect the binary and is trusted, not measured by runtime coverage. Build scripts, proc macros and their dependencies are excluded from runtime denominators; their source SLoC and unsafe counts are listed separately. A package used at runtime and build time appears in both inventories.

| Crate | SLoC | Unsafe sites |
| --- | ---: | ---: |
| clap_derive@4.6.7 | 3969 | 0 |
| hax-lib-macros@0.2.0 | 1862 | 0 |
| hax-lib-macros@0.4.2 | 2964 | 0 |
| heck@0.5.0 | 760 | 0 |
| libcrux-macros@0.0.2 | 93 | 0 |
| libcrux-macros@0.0.3 | 120 | 0 |
| proc-macro2@1.0.107 | 6662 | 6 |
| quote@1.0.47 | 2804 | 0 |
| syn@2.0.119 | 61443 | 69 |
| syn@3.0.9 | 63168 | 78 |
| thiserror-impl@1.0.69 | 1885 | 0 |
| tracing-attributes@0.1.31 | 3973 | 1 |
| unicode-ident@1.0.27 | 2046 | 2 |
| zerocopy-derive@0.8.63 | 41255 | 818 |
## Dependency sizes

Runtime dependencies measured in `compiled-files` mode. Size is in SLoC; whole-crate SLoC is the v1.4.2 measure, kept for comparison. Fallbacks keep the whole-crate size.

| Crate | Size mode | Size (SLoC) | Whole-crate SLoC | Note |
| --- | --- | ---: | ---: | --- |
| addr2line@0.27.1 | compiled-files | 2088 | 2865 |  |
| adler2@2.0.1 | compiled-files | 349 | 444 |  |
| anstream@1.0.0 | compiled-files | 2096 | 2441 |  |
| anstyle@1.0.14 | compiled-files | 1469 | 1589 |  |
| anstyle-parse@1.0.0 | compiled-files | 865 | 1117 |  |
| anstyle-query@1.1.5 | compiled-files | 201 | 226 |  |
| anyhow@1.0.104 | compiled-files | 3253 | 4866 |  |
| backtrace@0.3.77 | compiled-files | 3345 | 7305 |  |
| base64@0.20.0 | compiled-files | 2761 | 5238 |  |
| cfg-if@1.0.5 | compiled-files | 175 | 189 |  |
| clap@4.6.7 | compiled-files | 119 | 3901 |  |
| clap_builder@4.6.7 | compiled-files | 26201 | 26384 |  |
| clap_lex@1.1.1 | compiled-files | 721 | 721 |  |
| colorchoice@1.0.5 | compiled-files | 99 | 99 |  |
| getrandom@0.3.4 | compiled-files | 929 | 2318 |  |
| gimli@0.34.0 | compiled-files | 37073 | 49697 |  |
| hax-lib@0.2.0 | compiled-files | 355 | 687 |  |
| hax-lib@0.4.2 | compiled-files | 403 | 759 |  |
| hex@0.4.3 | compiled-files | 493 | 685 |  |
| is_terminal_polyfill@1.70.2 | compiled-files | 44 | 44 |  |
| lazy_static@1.5.1 | compiled-files | 200 | 435 |  |
| libc@0.2.191 | compiled-files | 16587 | 118894 |  |
| libcrux-chacha20poly1305@0.0.2 | compiled-files | 976 | 1136 |  |
| libcrux-curve25519@0.0.2 | compiled-files | 46 | 46 |  |
| libcrux-ecdh@0.0.2 | compiled-files | 549 | 755 |  |
| libcrux-ecdsa@0.0.3 | compiled-files | 325 | 515 |  |
| libcrux-ed25519@0.0.3 | compiled-files | 3884 | 3942 |  |
| libcrux-hacl-rs@0.0.2 | compiled-files | 9961 | 9961 |  |
| libcrux-hacl-rs@0.0.3 | compiled-files | 9961 | 9961 |  |
| libcrux-hkdf@0.0.3 | compiled-files | 614 | 614 |  |
| libcrux-hmac@0.0.3 | compiled-files | 918 | 918 |  |
| libcrux-intrinsics@0.0.2 | compiled-files | 330 | 1724 |  |
| libcrux-kem@0.0.2 | compiled-files | 913 | 913 |  |
| libcrux-ml-kem@0.0.2 | compiled-files | 12587 | 17426 |  |
| libcrux-p256@0.0.2 | compiled-files | 3442 | 3442 |  |
| libcrux-p256@0.0.3 | compiled-files | 3443 | 3443 |  |
| libcrux-platform@0.0.2 | compiled-files | 239 | 543 |  |
| libcrux-platform@0.0.4 | compiled-files | 214 | 498 |  |
| libcrux-poly1305@0.0.2 | compiled-files | 632 | 632 |  |
| libcrux-rsa@0.0.2 | compiled-files | 1366 | 1650 |  |
| libcrux-sha2@0.0.2 | compiled-files | 1380 | 1443 |  |
| libcrux-sha2@0.0.3 | compiled-files | 1383 | 1446 |  |
| libcrux-sha3@0.0.2 | compiled-files | 1783 | 2159 |  |
| libcrux-traits@0.0.2 | compiled-files | 58 | 58 |  |
| libcrux-traits@0.0.3 | compiled-files | 58 | 58 |  |
| log@0.4.34 | compiled-files | 2748 | 5461 |  |
| memchr@2.8.3 | compiled-files | 8385 | 13625 |  |
| miniz_oxide@0.9.1 | compiled-files | 2921 | 5703 |  |
| nu-ansi-term@0.50.3 | compiled-files | 2428 | 2684 |  |
| object@0.40.0 | compiled-files | 43244 | 62518 |  |
| once_cell@1.21.4 | compiled-files | 2045 | 3405 |  |
| pin-project-lite@0.2.17 | compiled-files | 1566 | 5078 |  |
| ppv-lite86@0.2.21 | compiled-files | 1600 | 3611 |  |
| rand@0.9.5 | compiled-files | 8448 | 8554 |  |
| rand_chacha@0.9.0 | compiled-files | 878 | 878 |  |
| rand_core@0.9.5 | compiled-files | 1441 | 1441 |  |
| rustc-demangle@0.1.28 | compiled-files | 2134 | 2134 |  |
| sharded-slab@0.1.7 | compiled-files | 4570 | 6240 |  |
| smallvec@1.16.4 | compiled-files | 2449 | 3541 |  |
| strsim@0.11.1 | compiled-files | 1084 | 1228 |  |
| thiserror@1.0.69 | compiled-files | 362 | 1841 |  |
| thread_local@1.1.10 | compiled-files | 893 | 893 |  |
| tracing@0.1.44 | compiled-files | 6685 | 70748 |  |
| tracing-core@0.1.36 | compiled-files | 5114 | 5654 |  |
| tracing-log@0.2.0 | compiled-files | 807 | 1436 |  |
| tracing-subscriber@0.3.23 | compiled-files | 15979 | 24715 |  |
| utf8parse@0.2.2 | compiled-files | 209 | 229 |  |
| zerocopy@0.8.63 | compiled-files | 24056 | 27029 |  |

Combined runtime security: 0/300766 proven SLoC (0.00%); proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction; proof-code: 0 SLoC alongside, excluded from both runtime denominators.

## PROOF CODE

| Classification | SLoC | Units | verified | unverified (including hazards, trusted and bounded) | Hazard units |
| --- | ---: | ---: | ---: | ---: | ---: |
| Total | 0 | 0 | 0 | 0 | 0 |
| SECURITY-RELEVANT | 0 | 0 | 0 | 0 | 0 |
| EVERYTHING ELSE | 0 | 0 | 0 | 0 | 0 |

TEST CODE: 1306 SLoC in 53 first-party units (1306 SLoC in 53 units reachable from security entry points), detected structurally and excluded from both runtime denominators.

| Origin | Security SLoC | Security proven | Everything else SLoC | Everything else proven | Trust / escapes |
| --- | ---: | ---: | ---: | ---: | --- |
| First-party | 5832 | 0.00% | 0 | 0.00% | proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction |
| Third-party | 294934 | 0.00% | 0 | 0.00% | proven, relying on 46 trusted items (10924 SLoC) and 3054 escapes; proven-extracted relies on trusted hax extraction |
## Proof ratio

Ghost lines: `separate`. Range 1.00:10.00; source `default`; ratio gate enabled. Cost signal, not proof quality. Module rows outside the range are warnings only.

| Scope | Standalone proof | Inline proof | Total proof | Proven runtime | Proven-bounded runtime | Ratio | Status |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| First-party security-relevant (headline) | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| All first-party | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Crate bertie | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Crate bogo_shim | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Crate integration_tests | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Crate record | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Crate simple_https_client | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Crate simple_https_server | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::benches::client | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::benches::client_stack | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::benches::server | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::server | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::server::ServerDB | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::bertie_stream | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::bertie_stream::From<TLSError>forBertieError | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::bertie_stream::From<std::io::Error>forBertieError | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::bertie_stream::TlsStream | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::client::<Stream:Read+Write>BertieStream<ClientState<Stream>> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::client::<Stream:Read+Write>ClientState<Stream> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::client::<T:Read+Write>TlsStream<T>forClientState<T> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::client::BertieStream<ClientState<TcpStream>> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::client::tests | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::server | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::server::<Stream:Read+Write>BertieStream<ServerState<Stream>> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::server::<Stream:Read+Write>ServerState<Stream> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::server::<Stream:Read+Write>TlsStream<Stream>forServerState<Stream> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::stream::server::BertieStream<ServerState<TcpStream>> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::test_tls13traces_internal | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::test_utils::RngCoreforTestRng | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::test_utils::TestRng | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tests::test_tls13api | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13api | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13api::Client | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13api::Server | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13cert | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13cert::unit_test | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::AeadAlgorithm | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::AeadKey | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::AeadKeyIV | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::Algorithms | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::DisplayforAlgorithms | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::HashAlgorithm | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::KemScheme | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13crypto::TryFrom<&str>forAlgorithms | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::ContentType | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::Extensions | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::Transcript | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::TryFrom<u8>forAlertDescription | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::TryFrom<u8>forAlertLevel | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::handshake_data | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::handshake_data::From<Bytes>forHandshakeData | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13formats::handshake_data::HandshakeData | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13handshake | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13handshake::proverif_extra | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13keyscheduler | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13keyscheduler::key_schedule | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13keyscheduler::key_schedule::KeySchedule | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13record | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13record::DuplexCipherStateH | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::<constC:usize>From<&[u8 ; C]>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::<constC:usize>From<[u8 ; C]>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::<constN:usize>From<&[u8 ; N]>forAppData | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::AppData | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::Bytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::Declassify | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::Declassify<u32>forU32 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::Declassify<u8>foru8 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<&[U8]>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<&[u8]>forAppData | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<&[u8]>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<&u8>forU8 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<Bytes>forAppData | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<Vec<U8>>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<Vec<u8>>forAppData | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<Vec<u8>>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<u16>forU16 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<u32>forU32 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::From<u8>forU8 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::U32 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::U8 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::core::ops::AddforU32 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::core::ops::BitXorforU8 | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::core::ops::Index<Range<usize>>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::core::ops::Index<usize>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::non_hax::core::ops::IndexMut<core::ops::Range<usize>>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bertie::tls13utils::non_hax::core::ops::IndexMut<usize>forBytes | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module bogo_shim::<unattributed> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module integration_tests::tests::self_test | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module record::From<TLSError>forAppError | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module record::From<std::io::Error>forAppError | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module record::debug | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module record::debug::DisplayforHex<'_> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module record::stream::<Stream>RecordStream<Stream>whereStream:Read+Write, | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module simple_https_client::tests::test_adhoc | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module simple_https_client::tests::test_adhoc::<'a>ReadforStream<'a> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module simple_https_client::tests::test_adhoc::<'a>Stream<'a> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module simple_https_client::tests::test_adhoc::<'a>WriteforStream<'a> | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module simple_https_client::tests::test_tls13api | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module simple_https_client::tls13client | 0 | 0 | 0 | 0 | 0 | n/a | n/a |
| Module simple_https_server::tls13server | 0 | 0 | 0 | 0 | 0 | n/a | n/a |

Runtime proven figures under both ghost modes (same accepted evidence):

| Scope | separate proven / runtime SLoC | runtime proven / runtime SLoC |
| --- | ---: | ---: |
| First-party security-relevant | 0 / 5832 | 0 / 5832 |
| All first-party | 0 / 5832 | 0 / 5832 |


## Units

| Function / crate | Location | SLoC | Classification | Status | Proof scope | Third-party unsafe sites |
| --- | --- | ---: | --- | --- | --- | ---: |
| bertie::benches::client::hs_per_second | benches/client.rs:31 | 4 | test-code | unverified | none | n/a |
| bertie::benches::client::mb_per_second | benches/client.rs:37 | 5 | test-code | unverified | none | n/a |
| bertie::benches::client::client_hello | benches/client.rs:64 | 25 | test-code | unverified | none | n/a |
| bertie::benches::client::parse_server_hello | benches/client.rs:92 | 23 | test-code | unverified | none | n/a |
| bertie::benches::client::parse_server_certificate | benches/client.rs:118 | 37 | test-code | unverified | none | n/a |
| bertie::benches::client::protocol | benches/client.rs:158 | 66 | test-code | unverified | none | n/a |
| bertie::benches::client::main | benches/client.rs:239 | 9 | test-code | unverified | none | n/a |
| bertie::benches::client_stack::protocol | benches/client_stack.rs:83 | 92 | test-code | hazard | none | n/a |
| bertie::benches::client_stack::main | benches/client_stack.rs:199 | 3 | test-code | hazard | none | n/a |
| bertie::benches::server::hs_per_second | benches/server.rs:27 | 4 | test-code | unverified | none | n/a |
| bertie::benches::server::mb_per_second | benches/server.rs:33 | 5 | test-code | unverified | none | n/a |
| bertie::benches::server::parse_client_hello | benches/server.rs:58 | 28 | test-code | unverified | none | n/a |
| bertie::benches::server::main | benches/server.rs:89 | 7 | test-code | unverified | none | n/a |
| bertie::benches::server::protocol | benches/server.rs:98 | 78 | test-code | unverified | none | n/a |
| bogo_shim::main | bogo_shim/src/main.rs:108 | 132 | SECURITY-RELEVANT | unverified | none | n/a |
| bogo_shim::skip | bogo_shim/src/main.rs:259 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bogo_shim::skip_currently | bogo_shim/src/main.rs:266 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bogo_shim::ignore | bogo_shim/src/main.rs:274 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| integration_tests::tests::self_test::test_sha256_chacha20_poly1305_rsa_pss_rsa_sha256_x25519 | integration_tests/tests/self_test.rs:15 | 4 | test-code | unverified | none | n/a |
| integration_tests::tests::self_test::test_sha256_chacha20_poly1305_ecdsa_secp256r1_sha256_x25519 | integration_tests/tests/self_test.rs:19 | 4 | test-code | unverified | none | n/a |
| integration_tests::tests::self_test::test_sha256_chacha20_poly1305_ecdsa_secp256r1_sha256_x25519kyber768 | integration_tests/tests/self_test.rs:23 | 4 | test-code | unverified | none | n/a |
| integration_tests::tests::self_test::test_sha256_chacha20_poly1305_ecdsa_secp256r1_sha256_x25519mlkem768 | integration_tests/tests/self_test.rs:27 | 4 | test-code | unverified | none | n/a |
| integration_tests::tests::self_test::test_sha256_chacha20_poly1305_ecdsa_secp256r1_sha256_p256 | integration_tests/tests/self_test.rs:31 | 4 | test-code | unverified | none | n/a |
| integration_tests::tests::self_test::test_sha256_chacha20_poly1305_rsa_pss_rsa_sha256_p256 | integration_tests/tests/self_test.rs:35 | 4 | test-code | unverified | none | n/a |
| integration_tests::tests::self_test::self_test_algorithm | integration_tests/tests/self_test.rs:88 | 42 | test-code | unverified | none | n/a |
| record::debug::info_record | record/src/debug.rs:6 | 35 | SECURITY-RELEVANT | unverified | none | n/a |
| record::debug::DisplayforHex<'_>::fmt | record/src/debug.rs:46 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| record::From<std::io::Error>forAppError::from | record/src/lib.rs:22 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| record::From<TLSError>forAppError::from | record/src/lib.rs:28 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| record::stream::<Stream>RecordStream<Stream>whereStream:Read+Write,::new | record/src/stream.rs:24 | 6 | SECURITY-RELEVANT | tested | none | n/a |
| record::stream::<Stream>RecordStream<Stream>whereStream:Read+Write,::read_record | record/src/stream.rs:36 | 49 | SECURITY-RELEVANT | unverified | none | n/a |
| record::stream::<Stream>RecordStream<Stream>whereStream:Read+Write,::write_record | record/src/stream.rs:115 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| simple_https_client::tls13client::main | simple_https_client/src/tls13client.rs:45 | 38 | SECURITY-RELEVANT | unverified | none | n/a |
| simple_https_client::tests::test_adhoc::<'a>Stream<'a>::new | simple_https_client/tests/test_adhoc.rs:19 | 5 | test-code | unverified | none | n/a |
| simple_https_client::tests::test_adhoc::<'a>ReadforStream<'a>::read | simple_https_client/tests/test_adhoc.rs:27 | 3 | test-code | unverified | none | n/a |
| simple_https_client::tests::test_adhoc::<'a>WriteforStream<'a>::write | simple_https_client/tests/test_adhoc.rs:33 | 3 | test-code | unverified | none | n/a |
| simple_https_client::tests::test_adhoc::<'a>WriteforStream<'a>::flush | simple_https_client/tests/test_adhoc.rs:38 | 3 | test-code | unverified | none | n/a |
| simple_https_client::tests::test_adhoc::test_adhoc | simple_https_client/tests/test_adhoc.rs:43 | 24 | test-code | unverified | none | n/a |
| simple_https_client::tests::test_tls13api::load_hex | simple_https_client/tests/test_tls13api.rs:16 | 4 | test-code | unverified | none | n/a |
| simple_https_client::tests::test_tls13api::test_full_round_trip | simple_https_client/tests/test_tls13api.rs:102 | 103 | test-code | unverified | none | n/a |
| simple_https_server::tls13server::main | simple_https_server/src/tls13server.rs:53 | 33 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::server::ServerDB::new | src/server.rs:24 | 16 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::server::lookup_db | src/server.rs:57 | 35 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::bertie_stream::From<std::io::Error>forBertieError::from | src/stream/bertie_stream.rs:19 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::bertie_stream::From<TLSError>forBertieError::from | src/stream/bertie_stream.rs:25 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::bertie_stream::TlsStream::write_tls | src/stream/bertie_stream.rs:31 | 2 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::bertie_stream::TlsStream::read_tls | src/stream/bertie_stream.rs:34 | 2 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::bertie_stream::TlsStream::stream_mut | src/stream/bertie_stream.rs:37 | 2 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::bertie_stream::read_record | src/stream/bertie_stream.rs:48 | 34 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::client::<Stream:Read+Write>ClientState<Stream>::new | src/stream/client.rs:30 | 8 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::client::<T:Read+Write>TlsStream<T>forClientState<T>::write_tls | src/stream/client.rs:41 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::client::<T:Read+Write>TlsStream<T>forClientState<T>::read_tls | src/stream/client.rs:56 | 17 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::client::<T:Read+Write>TlsStream<T>forClientState<T>::stream_mut | src/stream/client.rs:75 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::client::<Stream:Read+Write>BertieStream<ClientState<Stream>>::open_with_stream | src/stream/client.rs:81 | 12 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::client::BertieStream<ClientState<TcpStream>>::client | src/stream/client.rs:96 | 16 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::client::BertieStream<ClientState<TcpStream>>::open | src/stream/client.rs:113 | 10 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::client::BertieStream<ClientState<TcpStream>>::start | src/stream/client.rs:124 | 75 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::client::BertieStream<ClientState<TcpStream>>::write_all | src/stream/client.rs:227 | 6 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::client::BertieStream<ClientState<TcpStream>>::read | src/stream/client.rs:234 | 6 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::client::BertieStream<ClientState<TcpStream>>::write | src/stream/client.rs:241 | 7 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::client::tests::client | src/stream/client.rs:259 | 19 | test-code | unverified | none | n/a |
| bertie::stream::server::<Stream:Read+Write>ServerState<Stream>::new | src/stream/server.rs:40 | 9 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::server::<Stream:Read+Write>TlsStream<Stream>forServerState<Stream>::write_tls | src/stream/server.rs:52 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::<Stream:Read+Write>TlsStream<Stream>forServerState<Stream>::read_tls | src/stream/server.rs:67 | 17 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::<Stream:Read+Write>TlsStream<Stream>forServerState<Stream>::stream_mut | src/stream/server.rs:86 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::<Stream:Read+Write>BertieStream<ServerState<Stream>>::server_with_stream | src/stream/server.rs:92 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::BertieStream<ServerState<TcpStream>>::server | src/stream/server.rs:111 | 73 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::server::BertieStream<ServerState<TcpStream>>::connect | src/stream/server.rs:186 | 43 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::server::BertieStream<ServerState<TcpStream>>::close | src/stream/server.rs:237 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::BertieStream<ServerState<TcpStream>>::check_ccs_message | src/stream/server.rs:244 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::BertieStream<ServerState<TcpStream>>::write_all | src/stream/server.rs:259 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::BertieStream<ServerState<TcpStream>>::read | src/stream/server.rs:269 | 6 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::server::BertieStream<ServerState<TcpStream>>::write | src/stream/server.rs:276 | 7 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::stream::server::init_db | src/stream/server.rs:285 | 37 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::stream::server::read_file | src/stream/server.rs:329 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_client_hello | src/test_tls13traces_internal.rs:263 | 20 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_client_hello_record | src/test_tls13traces_internal.rs:285 | 27 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_client_hello_roundtrip | src/test_tls13traces_internal.rs:314 | 33 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_server_hello | src/test_tls13traces_internal.rs:349 | 17 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_server_hello_length_zero | src/test_tls13traces_internal.rs:368 | 6 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_server_hello_roundtrip | src/test_tls13traces_internal.rs:375 | 30 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_encrypted_extensions | src/test_tls13traces_internal.rs:407 | 15 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_server_certificate | src/test_tls13traces_internal.rs:423 | 15 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_server_certificate_verify | src/test_tls13traces_internal.rs:439 | 15 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_server_finished | src/test_tls13traces_internal.rs:455 | 15 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_parse_client_finished | src/test_tls13traces_internal.rs:471 | 15 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_key_schedule | src/test_tls13traces_internal.rs:487 | 161 | test-code | unverified | none | n/a |
| bertie::test_tls13traces_internal::test_finished | src/test_tls13traces_internal.rs:690 | 41 | test-code | unverified | none | n/a |
| bertie::test_utils::TestRng::new | src/test_utils.rs:9 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::test_utils::TestRng::raw | src/test_utils.rs:13 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::test_utils::RngCoreforTestRng::next_u32 | src/test_utils.rs:19 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::test_utils::RngCoreforTestRng::next_u64 | src/test_utils.rs:23 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::test_utils::RngCoreforTestRng::fill_bytes | src/test_utils.rs:27 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13api::in_psk_mode | src/tls13api.rs:39 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13api::get_server_info | src/tls13api.rs:50 | 16 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13api::Client::connect | src/tls13api.rs:68 | 32 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13api::Client::read_handshake | src/tls13api.rs:103 | 37 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13api::Client::read | src/tls13api.rs:141 | 28 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13api::Client::write | src/tls13api.rs:170 | 15 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13api::Server::accept | src/tls13api.rs:204 | 36 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13api::Server::read_handshake | src/tls13api.rs:241 | 22 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13api::Server::write | src/tls13api.rs:264 | 15 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13api::Server::read | src/tls13api.rs:280 | 22 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13cert::asn1_error | src/tls13cert.rs:42 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::long_length | src/tls13cert.rs:51 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::length_length | src/tls13cert.rs:67 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::short_length | src/tls13cert.rs:86 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::length | src/tls13cert.rs:101 | 23 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::read_sequence_header | src/tls13cert.rs:125 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::read_octet_header | src/tls13cert.rs:138 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::check_tag | src/tls13cert.rs:151 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::skip_sequence | src/tls13cert.rs:168 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::read_version_number | src/tls13cert.rs:181 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::skip_integer | src/tls13cert.rs:198 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::read_integer | src/tls13cert.rs:209 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::x962_ec_public_key_oid | src/tls13cert.rs:220 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::ecdsa_secp256r1_sha256_oid | src/tls13cert.rs:223 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::rsa_pkcs1_encryption_oid | src/tls13cert.rs:226 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::check_success | src/tls13cert.rs:230 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::read_spki | src/tls13cert.rs:238 | 64 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::verification_key_from_cert | src/tls13cert.rs:317 | 50 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13cert::ecdsa_public_key | src/tls13cert.rs:378 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::rsa_public_key | src/tls13cert.rs:393 | 23 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::debug_print | src/tls13cert.rs:425 | 10 | test-code | unverified | none | n/a |
| bertie::tls13cert::rsa_private_key | src/tls13cert.rs:436 | 25 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13cert::cert_public_key | src/tls13cert.rs:464 | 39 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13cert::unit_test::test | src/tls13cert.rs:511 | 15 | test-code | unverified | none | n/a |
| bertie::tls13cert::unit_test::ecdsa_cert | src/tls13cert.rs:527 | 6 | test-code | unverified | none | n/a |
| bertie::tls13cert::unit_test::rsa_cert | src/tls13cert.rs:534 | 5 | test-code | unverified | none | n/a |
| bertie::tls13cert::unit_test::read_cert | src/tls13cert.rs:540 | 14 | test-code | unverified | none | n/a |
| bertie::tls13crypto::AeadKeyIV::new | src/tls13crypto.rs:47 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::AeadKey::new | src/tls13crypto.rs:60 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::AeadKey::bytes | src/tls13crypto.rs:65 | 5 | test-code | unverified | none | n/a |
| bertie::tls13crypto::hash | src/tls13crypto.rs:122 | 10 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::HashAlgorithm::libcrux_algorithm | src/tls13crypto.rs:137 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::HashAlgorithm::hash_len | src/tls13crypto.rs:146 | 10 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::HashAlgorithm::hmac_algorithm | src/tls13crypto.rs:157 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::HashAlgorithm::hmac_tag_len | src/tls13crypto.rs:166 | 5 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::hmac_tag | src/tls13crypto.rs:173 | 13 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::hmac_verify | src/tls13crypto.rs:187 | 32 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::zero_key | src/tls13crypto.rs:220 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::hkdf_algorithm | src/tls13crypto.rs:225 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::hkdf_extract | src/tls13crypto.rs:234 | 13 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::hkdf_expand | src/tls13crypto.rs:248 | 20 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::AeadAlgorithm::key_len | src/tls13crypto.rs:278 | 9 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::AeadAlgorithm::iv_len | src/tls13crypto.rs:288 | 9 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::aead_encrypt | src/tls13crypto.rs:299 | 30 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::aead_decrypt | src/tls13crypto.rs:333 | 28 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::sign_rsa | src/tls13crypto.rs:374 | 36 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::sign | src/tls13crypto.rs:419 | 57 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::verify | src/tls13crypto.rs:479 | 101 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::supported_rsa_key_size | src/tls13crypto.rs:590 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::valid_rsa_exponent | src/tls13crypto.rs:600 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::KemScheme::libcrux_kem_algorithm | src/tls13crypto.rs:621 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::kem_keygen | src/tls13crypto.rs:633 | 28 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::encoding_prefix | src/tls13crypto.rs:667 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::into_raw | src/tls13crypto.rs:678 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::kem_encap | src/tls13crypto.rs:688 | 30 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::to_shared_secret | src/tls13crypto.rs:724 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::kem_decap | src/tls13crypto.rs:735 | 25 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::Algorithms::new | src/tls13crypto.rs:781 | 18 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::hash | src/tls13crypto.rs:800 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::aead | src/tls13crypto.rs:805 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::signature | src/tls13crypto.rs:810 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::kem | src/tls13crypto.rs:815 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::psk_mode | src/tls13crypto.rs:820 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::zero_rtt | src/tls13crypto.rs:825 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::ciphersuite | src/tls13crypto.rs:830 | 13 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13crypto::Algorithms::supported_group | src/tls13crypto.rs:844 | 17 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::Algorithms::signature_algorithm | src/tls13crypto.rs:862 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::Algorithms::check | src/tls13crypto.rs:875 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13crypto::TryFrom<&str>forAlgorithms::try_from | src/tls13crypto.rs:893 | 27 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13crypto::DisplayforAlgorithms::fmt | src/tls13crypto.rs:943 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::build_server_name | src/tls13formats.rs:58 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_server_name | src/tls13formats.rs:71 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::supported_versions | src/tls13formats.rs:86 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_supported_versions | src/tls13formats.rs:94 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::server_supported_version | src/tls13formats.rs:100 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_server_supported_version | src/tls13formats.rs:105 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::supported_groups | src/tls13formats.rs:109 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_supported_groups | src/tls13formats.rs:117 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::signature_algorithms | src/tls13formats.rs:122 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_signature_algorithms | src/tls13formats.rs:128 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::psk_key_exchange_modes | src/tls13formats.rs:133 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_psk_key_exchange_modes | src/tls13formats.rs:138 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::key_shares | src/tls13formats.rs:143 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::find_key_share | src/tls13formats.rs:149 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_key_shares | src/tls13formats.rs:162 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::server_key_shares | src/tls13formats.rs:167 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_server_key_share | src/tls13formats.rs:172 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::pre_shared_key | src/tls13formats.rs:183 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_psk_shared_key | src/tls13formats.rs:195 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::server_pre_shared_key | src/tls13formats.rs:213 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_server_psk_shared_key | src/tls13formats.rs:217 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::Extensions::merge | src/tls13formats.rs:230 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::merge_opts | src/tls13formats.rs:242 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_extension | src/tls13formats.rs:253 | 71 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_server_extension | src/tls13formats.rs:327 | 23 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_extensions_slice | src/tls13formats.rs:351 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_extensions | src/tls13formats.rs:363 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_server_extensions | src/tls13formats.rs:373 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::TryFrom<u8>forAlertLevel::try_from | src/tls13formats.rs:408 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::TryFrom<u8>forAlertDescription::try_from | src/tls13formats.rs:485 | 32 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::bench_client_hello | src/tls13formats.rs:519 | 16 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::get_psk_extensions | src/tls13formats.rs:536 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::client_hello | src/tls13formats.rs:551 | 86 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::set_client_hello_binder | src/tls13formats.rs:649 | 59 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::invalid_compression_list | src/tls13formats.rs:709 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::bench_parse_client_hello | src/tls13formats.rs:713 | 18 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::parse_client_hello | src/tls13formats.rs:732 | 142 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::server_hello | src/tls13formats.rs:875 | 50 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::unsupported_cipher_alert | src/tls13formats.rs:931 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::invalid_compression_method_alert | src/tls13formats.rs:935 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::bench_parse_server_hello | src/tls13formats.rs:939 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::parse_server_hello | src/tls13formats.rs:947 | 63 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::encrypted_extensions | src/tls13formats.rs:1016 | 7 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::parse_encrypted_extensions | src/tls13formats.rs:1024 | 30 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::server_certificate | src/tls13formats.rs:1058 | 28 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::bench_parse_server_certificate | src/tls13formats.rs:1090 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::parse_server_certificate | src/tls13formats.rs:1095 | 31 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::ecdsa_signature | src/tls13formats.rs:1130 | 24 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_r_len | src/tls13formats.rs:1155 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::parse_ecdsa_signature | src/tls13formats.rs:1163 | 20 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::certificate_verify | src/tls13formats.rs:1183 | 30 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::parse_certificate_verify | src/tls13formats.rs:1218 | 38 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::finished | src/tls13formats.rs:1260 | 18 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::parse_finished | src/tls13formats.rs:1282 | 19 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::ContentType::try_from_u8 | src/tls13formats.rs:1357 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_record | src/tls13formats.rs:1369 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::protocol_version_alert | src/tls13formats.rs:1378 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::application_data_instead_of_handshake | src/tls13formats.rs:1382 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::check_handshake_record | src/tls13formats.rs:1386 | 18 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13formats::get_handshake_record | src/tls13formats.rs:1405 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::Transcript::new | src/tls13formats.rs:1423 | 6 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13formats::Transcript::add | src/tls13formats.rs:1430 | 25 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::Transcript::transcript_hash | src/tls13formats.rs:1456 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::Transcript::transcript_hash_without_client_hello | src/tls13formats.rs:1462 | 18 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::get_hs_type | src/tls13formats/handshake_data.rs:43 | 16 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::to_two_inner | src/tls13formats/handshake_data.rs:63 | 24 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::handshake_data::to_four_inner | src/tls13formats/handshake_data.rs:88 | 41 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::handshake_data::from_bytes_inner | src/tls13formats/handshake_data.rs:132 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::to_bytes_inner | src/tls13formats/handshake_data.rs:145 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::from_bytes | src/tls13formats/handshake_data.rs:152 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::len | src/tls13formats/handshake_data.rs:163 | 6 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::to_bytes | src/tls13formats/handshake_data.rs:170 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::concat | src/tls13formats/handshake_data.rs:175 | 9 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::as_handshake_message | src/tls13formats/handshake_data.rs:186 | 24 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::next_handshake_message | src/tls13formats/handshake_data.rs:211 | 19 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::to_two | src/tls13formats/handshake_data.rs:231 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::to_four | src/tls13formats/handshake_data.rs:240 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::HandshakeData::find_handshake_message | src/tls13formats/handshake_data.rs:251 | 25 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13formats::handshake_data::From<Bytes>forHandshakeData::from | src/tls13formats/handshake_data.rs:280 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13handshake::algs_post_client_hello | src/tls13handshake.rs:74 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::algs_post_server_hello | src/tls13handshake.rs:77 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::algs_post_client_finished | src/tls13handshake.rs:80 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::server_info_post_client_hello | src/tls13handshake.rs:84 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::server_info_post_server_hello | src/tls13handshake.rs:87 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::server_info_post_client_finished | src/tls13handshake.rs:90 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::build_client_hello | src/tls13handshake.rs:135 | 46 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::compute_psk_binder_zero_rtt | src/tls13handshake.rs:182 | 48 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::put_server_hello | src/tls13handshake.rs:234 | 56 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::put_server_signature | src/tls13handshake.rs:299 | 53 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::put_psk_skip_server_signature | src/tls13handshake.rs:353 | 31 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::put_server_finished | src/tls13handshake.rs:385 | 50 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::get_client_finished | src/tls13handshake.rs:437 | 36 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::client_init | src/tls13handshake.rs:479 | 17 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::client_set_params | src/tls13handshake.rs:497 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::client_finish | src/tls13handshake.rs:506 | 37 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::put_client_hello | src/tls13handshake.rs:546 | 27 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::process_psk_binder_zero_rtt | src/tls13handshake.rs:574 | 35 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::get_server_hello | src/tls13handshake.rs:613 | 64 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::get_rsa_signature | src/tls13handshake.rs:685 | 21 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13handshake::get_server_signature_no_psk | src/tls13handshake.rs:710 | 47 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::get_server_signature | src/tls13handshake.rs:758 | 18 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::get_skip_server_signature_no_psk | src/tls13handshake.rs:777 | 20 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::get_skip_server_signature | src/tls13handshake.rs:798 | 19 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::get_server_finished | src/tls13handshake.rs:818 | 28 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::put_client_finished | src/tls13handshake.rs:847 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::server_init_no_psk | src/tls13handshake.rs:866 | 25 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::server_init_psk | src/tls13handshake.rs:893 | 25 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::server_init | src/tls13handshake.rs:921 | 23 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::server_finish | src/tls13handshake.rs:945 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13handshake::proverif_extra::f | src/tls13handshake.rs:960 | 4 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13keyscheduler::hash_empty | src/tls13keyscheduler.rs:10 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::derive_binder_key | src/tls13keyscheduler.rs:16 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::derive_aead_key_iv | src/tls13keyscheduler.rs:27 | 52 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13keyscheduler::next_keys_c_2 | src/tls13keyscheduler.rs:80 | 18 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::derive_0rtt_keys | src/tls13keyscheduler.rs:100 | 19 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::derive_finished_key | src/tls13keyscheduler.rs:120 | 36 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13keyscheduler::derive_hk_handles | src/tls13keyscheduler.rs:157 | 31 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::derive_hk_ms | src/tls13keyscheduler.rs:192 | 18 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::derive_app_handles | src/tls13keyscheduler.rs:212 | 16 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::derive_app_keys | src/tls13keyscheduler.rs:230 | 11 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::derive_rms | src/tls13keyscheduler.rs:243 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::hkdf_expand_label | src/tls13keyscheduler/key_schedule.rs:11 | 20 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::zero_salt | src/tls13keyscheduler/key_schedule.rs:32 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::no_psk | src/tls13keyscheduler/key_schedule.rs:47 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::zero_ikm | src/tls13keyscheduler/key_schedule.rs:59 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule::labels | src/tls13keyscheduler/key_schedule.rs:71 | 1 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule::prnt_n | src/tls13keyscheduler/key_schedule.rs:72 | 1 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule::get | src/tls13keyscheduler/key_schedule.rs:75 | 1 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule::set | src/tls13keyscheduler/key_schedule.rs:76 | 1 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule::hash | src/tls13keyscheduler/key_schedule.rs:78 | 1 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::prnt_n | src/tls13keyscheduler/key_schedule.rs:112 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::labels | src/tls13keyscheduler/key_schedule.rs:127 | 21 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::get | src/tls13keyscheduler/key_schedule.rs:150 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::set | src/tls13keyscheduler/key_schedule.rs:154 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::KeySchedule<TLSnames>forTLSkeyscheduler::hash | src/tls13keyscheduler/key_schedule.rs:159 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::key_schedule::convert_label | src/tls13keyscheduler/key_schedule.rs:184 | 32 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::label_to_bytes | src/tls13keyscheduler/key_schedule.rs:217 | 19 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::xtr_alg | src/tls13keyscheduler/key_schedule.rs:244 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::xpd_alg | src/tls13keyscheduler/key_schedule.rs:248 | 13 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::xtr | src/tls13keyscheduler/key_schedule.rs:262 | 8 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::xpd | src/tls13keyscheduler/key_schedule.rs:272 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::xpd_angle | src/tls13keyscheduler/key_schedule.rs:291 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::tagkey_from_handle | src/tls13keyscheduler/key_schedule.rs:304 | 8 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::key_schedule::set_by_handle | src/tls13keyscheduler/key_schedule.rs:313 | 9 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13keyscheduler::key_schedule::get_by_handle | src/tls13keyscheduler/key_schedule.rs:323 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::XPD | src/tls13keyscheduler/key_schedule.rs:333 | 69 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13keyscheduler::key_schedule::xtr_angle | src/tls13keyscheduler/key_schedule.rs:406 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13keyscheduler::key_schedule::XTR | src/tls13keyscheduler/key_schedule.rs:414 | 58 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13record::client_cipher_state0 | src/tls13record.rs:14 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::server_cipher_state0 | src/tls13record.rs:31 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::DuplexCipherStateH::new | src/tls13record.rs:53 | 14 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13record::duplex_cipher_state1 | src/tls13record.rs:71 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::derive_iv_ctr | src/tls13record.rs:83 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::encrypt_record_payload | src/tls13record.rs:99 | 24 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::padlen | src/tls13record.rs:124 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::decrypt_record_payload | src/tls13record.rs:135 | 25 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::encrypt_zerortt | src/tls13record.rs:165 | 19 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::decrypt_zerortt | src/tls13record.rs:185 | 19 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::encrypt_handshake | src/tls13record.rs:205 | 34 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13record::decrypt_handshake | src/tls13record.rs:242 | 48 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13record::encrypt_data | src/tls13record.rs:291 | 16 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::decrypt_data_or_hs | src/tls13record.rs:308 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13record::decrypt_data | src/tls13record.rs:317 | 13 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::parse_failed | src/tls13utils.rs:12 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::error_string | src/tls13utils.rs:48 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::tlserr | src/tls13utils.rs:53 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::tlserr | src/tls13utils.rs:59 | 5 | test-code | unverified | none | n/a |
| bertie::tls13utils::core::ops::BitXorforU8::bitxor | src/tls13utils.rs:71 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::From<u8>forU8::from | src/tls13utils.rs:77 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::U8::declassify | src/tls13utils.rs:84 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Declassify::declassify | src/tls13utils.rs:92 | 2 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Declassify<u8>foru8::declassify | src/tls13utils.rs:98 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::U8 | src/tls13utils.rs:106 | 5 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<&u8>forU8::from | src/tls13utils.rs:114 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::U16 | src/tls13utils.rs:126 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::From<u16>forU16::from | src/tls13utils.rs:134 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<u32>forU32::from | src/tls13utils.rs:145 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::core::ops::AddforU32::add | src/tls13utils.rs:153 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::U32 | src/tls13utils.rs:161 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Declassify<u32>forU32::declassify | src/tls13utils.rs:169 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<Vec<u8>>forBytes::from | src/tls13utils.rs:181 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<Vec<U8>>forBytes::from | src/tls13utils.rs:188 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<Vec<u8>>forBytes::from | src/tls13utils.rs:195 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::concat_inner | src/tls13utils.rs:200 | 9 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13utils::eq_inner | src/tls13utils.rs:210 | 13 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13utils::Bytes::prefix | src/tls13utils.rs:226 | 12 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13utils::Bytes::declassify | src/tls13utils.rs:241 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Bytes::into_raw | src/tls13utils.rs:246 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::as_raw | src/tls13utils.rs:252 | 6 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Bytes::declassify_array | src/tls13utils.rs:261 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::From<&[u8]>forBytes::from | src/tls13utils.rs:270 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<&[U8]>forBytes::from | src/tls13utils.rs:278 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::<constC:usize>From<[u8 ; C]>forBytes::from | src/tls13utils.rs:285 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::<constC:usize>From<&[u8 ; C]>forBytes::from | src/tls13utils.rs:293 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::U32::declassify | src/tls13utils.rs:301 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::u16_as_be_bytes | src/tls13utils.rs:305 | 14 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13utils::u32_as_be_bytes | src/tls13utils.rs:320 | 7 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::u32_from_be_bytes | src/tls13utils.rs:328 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::bytes | src/tls13utils.rs:342 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::bytes1 | src/tls13utils.rs:347 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::bytes2 | src/tls13utils.rs:352 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::core::ops::Index<usize>forBytes::index | src/tls13utils.rs:360 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::_update_at_usize_bytes_test | src/tls13utils.rs:366 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::non_hax::core::ops::IndexMut<usize>forBytes::index_mut | src/tls13utils.rs:385 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::non_hax::core::ops::IndexMut<core::ops::Range<usize>>forBytes::index_mut | src/tls13utils.rs:391 | 3 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::core::ops::Index<Range<usize>>forBytes::index | src/tls13utils.rs:400 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_eq_inner | src/tls13utils.rs:407 | 11 | SECURITY-RELEVANT | trusted | none | n/a |
| bertie::tls13utils::Bytes::new | src/tls13utils.rs:421 | 5 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Bytes::new_alloc | src/tls13utils.rs:427 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::zeroes | src/tls13utils.rs:433 | 6 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Bytes::len | src/tls13utils.rs:440 | 5 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Bytes::push | src/tls13utils.rs:446 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::extend_from_slice | src/tls13utils.rs:451 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::append | src/tls13utils.rs:456 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::from_slice | src/tls13utils.rs:462 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::from_hex | src/tls13utils.rs:467 | 19 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Bytes::raw_slice | src/tls13utils.rs:487 | 6 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::slice_range | src/tls13utils.rs:494 | 6 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::slice | src/tls13utils.rs:501 | 6 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::concat | src/tls13utils.rs:508 | 5 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::Bytes::concat_array | src/tls13utils.rs:514 | 5 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::update_slice | src/tls13utils.rs:520 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::Bytes::as_hex | src/tls13utils.rs:560 | 10 | test-code | unverified | none | n/a |
| bertie::tls13utils::check | src/tls13utils.rs:572 | 12 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::eq1 | src/tls13utils.rs:585 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_eq1 | src/tls13utils.rs:590 | 13 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::eq_slice | src/tls13utils.rs:605 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::eq | src/tls13utils.rs:622 | 5 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::check_eq_slice | src/tls13utils.rs:628 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_eq_with_slice | src/tls13utils.rs:640 | 23 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_eq | src/tls13utils.rs:664 | 9 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_eq_option | src/tls13utils.rs:674 | 13 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_mem | src/tls13utils.rs:689 | 21 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::encode_length_u8 | src/tls13utils.rs:711 | 20 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::encode_length_u16 | src/tls13utils.rs:732 | 21 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::encode_length_u24 | src/tls13utils.rs:754 | 22 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::length_u8_encoded | src/tls13utils.rs:777 | 22 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::length_u16_encoded_slice | src/tls13utils.rs:800 | 25 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::length_u16_encoded | src/tls13utils.rs:826 | 14 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::length_u24_encoded | src/tls13utils.rs:841 | 26 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_length_encoding_u8_slice | src/tls13utils.rs:868 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_length_encoding_u8 | src/tls13utils.rs:879 | 10 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_length_encoding_u16_slice | src/tls13utils.rs:890 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_length_encoding_u16 | src/tls13utils.rs:902 | 11 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::check_length_encoding_u24 | src/tls13utils.rs:914 | 15 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::AppData::new | src/tls13utils.rs:935 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::AppData::into_raw | src/tls13utils.rs:940 | 4 | SECURITY-RELEVANT | unverified | none | n/a |
| bertie::tls13utils::AppData::as_raw | src/tls13utils.rs:945 | 4 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<&[u8]>forAppData::from | src/tls13utils.rs:952 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::<constN:usize>From<&[u8 ; N]>forAppData::from | src/tls13utils.rs:958 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<Vec<u8>>forAppData::from | src/tls13utils.rs:964 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::From<Bytes>forAppData::from | src/tls13utils.rs:970 | 3 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tls13utils::random_bytes | src/tls13utils.rs:975 | 6 | SECURITY-RELEVANT | tested | none | n/a |
| bertie::tests::test_tls13api::load_hex | tests/test_tls13api.rs:18 | 4 | test-code | unverified | none | n/a |
| bertie::tests::test_tls13api::test_full_round_trip | tests/test_tls13api.rs:113 | 102 | test-code | unverified | none | n/a |
| bertie::tests::test_tls13api::test_full_round_trip_with_psk | tests/test_tls13api.rs:225 | 104 | test-code | unverified | none | n/a |
| addr2line 0.27.1 | third-party crate | 2088 | SECURITY-RELEVANT | unverified | none | 8 |
| adler2 2.0.1 | third-party crate | 349 | SECURITY-RELEVANT | unverified | none | 0 |
| anstream 1.0.0 | third-party crate | 2096 | SECURITY-RELEVANT | unverified | none | 3 |
| anstyle 1.0.14 | third-party crate | 1469 | SECURITY-RELEVANT | unverified | none | 1 |
| anstyle-parse 1.0.0 | third-party crate | 865 | SECURITY-RELEVANT | unverified | none | 3 |
| anstyle-query 1.1.5 | third-party crate | 201 | SECURITY-RELEVANT | unverified | none | 1 |
| anyhow 1.0.104 | third-party crate | 3253 | SECURITY-RELEVANT | unverified | none | 106 |
| backtrace 0.3.77 | third-party crate | 3345 | SECURITY-RELEVANT | unverified | none | 197 |
| base64 0.20.0 | third-party crate | 2761 | SECURITY-RELEVANT | unverified | none | 0 |
| cfg-if 1.0.5 | third-party crate | 175 | SECURITY-RELEVANT | unverified | none | 0 |
| clap 4.6.7 | third-party crate | 119 | SECURITY-RELEVANT | unverified | none | 0 |
| clap_builder 4.6.7 | third-party crate | 26201 | SECURITY-RELEVANT | unverified | none | 0 |
| clap_lex 1.1.1 | third-party crate | 721 | SECURITY-RELEVANT | unverified | none | 6 |
| colorchoice 1.0.5 | third-party crate | 99 | SECURITY-RELEVANT | unverified | none | 0 |
| getrandom 0.3.4 | third-party crate | 929 | SECURITY-RELEVANT | unverified | none | 97 |
| gimli 0.34.0 | third-party crate | 37073 | SECURITY-RELEVANT | unverified | none | 20 |
| hax-lib 0.2.0 | third-party crate | 355 | SECURITY-RELEVANT | unverified | none | 0 |
| hax-lib 0.4.2 | third-party crate | 403 | SECURITY-RELEVANT | unverified | none | 0 |
| hex 0.4.3 | third-party crate | 493 | SECURITY-RELEVANT | unverified | none | 0 |
| is_terminal_polyfill 1.70.2 | third-party crate | 44 | SECURITY-RELEVANT | unverified | none | 0 |
| lazy_static 1.5.1 | third-party crate | 200 | SECURITY-RELEVANT | unverified | none | 3 |
| libc 0.2.191 | third-party crate | 16587 | SECURITY-RELEVANT | unverified | none | 800 |
| libcrux-chacha20poly1305 0.0.2 | third-party crate | 976 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-curve25519 0.0.2 | third-party crate | 46 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-ecdh 0.0.2 | third-party crate | 549 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-ecdsa 0.0.3 | third-party crate | 325 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-ed25519 0.0.3 | third-party crate | 3884 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-hacl-rs 0.0.2 | third-party crate | 9961 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-hacl-rs 0.0.3 | third-party crate | 9961 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-hkdf 0.0.3 | third-party crate | 614 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-hmac 0.0.3 | third-party crate | 918 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-intrinsics 0.0.2 | third-party crate | 330 | SECURITY-RELEVANT | unverified | none | 163 |
| libcrux-kem 0.0.2 | third-party crate | 913 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-ml-kem 0.0.2 | third-party crate | 12587 | SECURITY-RELEVANT | unverified | none | 50 |
| libcrux-p256 0.0.2 | third-party crate | 3442 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-p256 0.0.3 | third-party crate | 3443 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-platform 0.0.2 | third-party crate | 239 | SECURITY-RELEVANT | unverified | none | 31 |
| libcrux-platform 0.0.4 | third-party crate | 214 | SECURITY-RELEVANT | unverified | none | 2 |
| libcrux-poly1305 0.0.2 | third-party crate | 632 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-rsa 0.0.2 | third-party crate | 1366 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-sha2 0.0.2 | third-party crate | 1380 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-sha2 0.0.3 | third-party crate | 1383 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-sha3 0.0.2 | third-party crate | 1783 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-traits 0.0.2 | third-party crate | 58 | SECURITY-RELEVANT | unverified | none | 0 |
| libcrux-traits 0.0.3 | third-party crate | 58 | SECURITY-RELEVANT | unverified | none | 0 |
| log 0.4.34 | third-party crate | 2748 | SECURITY-RELEVANT | unverified | none | 6 |
| memchr 2.8.3 | third-party crate | 8385 | SECURITY-RELEVANT | unverified | none | 333 |
| miniz_oxide 0.9.1 | third-party crate | 2921 | SECURITY-RELEVANT | unverified | none | 0 |
| nu-ansi-term 0.50.3 | third-party crate | 2428 | SECURITY-RELEVANT | unverified | none | 1 |
| object 0.40.0 | third-party crate | 43244 | SECURITY-RELEVANT | unverified | none | 20 |
| once_cell 1.21.4 | third-party crate | 2045 | SECURITY-RELEVANT | unverified | none | 49 |
| pin-project-lite 0.2.17 | third-party crate | 1566 | SECURITY-RELEVANT | unverified | none | 46 |
| ppv-lite86 0.2.21 | third-party crate | 1600 | SECURITY-RELEVANT | unverified | none | 168 |
| rand 0.9.5 | third-party crate | 8448 | SECURITY-RELEVANT | unverified | none | 19 |
| rand_chacha 0.9.0 | third-party crate | 878 | SECURITY-RELEVANT | unverified | none | 0 |
| rand_core 0.9.5 | third-party crate | 1441 | SECURITY-RELEVANT | unverified | none | 0 |
| rustc-demangle 0.1.28 | third-party crate | 2134 | SECURITY-RELEVANT | unverified | none | 0 |
| sharded-slab 0.1.7 | third-party crate | 4570 | SECURITY-RELEVANT | unverified | none | 61 |
| smallvec 1.16.4 | third-party crate | 2449 | SECURITY-RELEVANT | unverified | none | 80 |
| strsim 0.11.1 | third-party crate | 1084 | SECURITY-RELEVANT | unverified | none | 0 |
| thiserror 1.0.69 | third-party crate | 362 | SECURITY-RELEVANT | unverified | none | 0 |
| thread_local 1.1.10 | third-party crate | 893 | SECURITY-RELEVANT | unverified | none | 27 |
| tracing 0.1.44 | third-party crate | 6685 | SECURITY-RELEVANT | unverified | none | 7 |
| tracing-core 0.1.36 | third-party crate | 5114 | SECURITY-RELEVANT | unverified | none | 28 |
| tracing-log 0.2.0 | third-party crate | 807 | SECURITY-RELEVANT | unverified | none | 1 |
| tracing-subscriber 0.3.23 | third-party crate | 15979 | SECURITY-RELEVANT | unverified | none | 27 |
| utf8parse 0.2.2 | third-party crate | 209 | SECURITY-RELEVANT | unverified | none | 1 |
| zerocopy 0.8.63 | third-party crate | 24056 | SECURITY-RELEVANT | unverified | none | 601 |

## Trusted items

- src/tls13cert.rs:331 — hax replace (syntactic, includes inactive annotations) — 89 SLoC — affected units: bertie::tls13cert::verification_key_from_cert, bertie::tls13cert::cert_public_key
- src/tls13cert.rs:470 — hax replace (syntactic, includes inactive annotations) — 89 SLoC — affected units: bertie::tls13cert::verification_key_from_cert, bertie::tls13cert::cert_public_key
- src/tls13crypto.rs:74 — hax opaque (syntactic, includes inactive annotations) — 27 SLoC — affected units: bertie::tls13crypto::TryFrom<&str>forAlgorithms::try_from
- src/tls13crypto.rs:84 — hax replace (syntactic, includes inactive annotations) — 306 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::hmac_verify, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::verify, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap, bertie::tls13crypto::kem_decap
- src/tls13crypto.rs:148 — hax replace_body (syntactic, includes inactive annotations) — 148 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap
- src/tls13crypto.rs:167 — hax replace_body (syntactic, includes inactive annotations) — 148 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap
- src/tls13crypto.rs:192 — hax replace (syntactic, includes inactive annotations) — 306 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::hmac_verify, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::verify, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap, bertie::tls13crypto::kem_decap
- src/tls13crypto.rs:279 — hax replace_body (syntactic, includes inactive annotations) — 148 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap
- src/tls13crypto.rs:289 — hax replace_body (syntactic, includes inactive annotations) — 148 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap
- src/tls13crypto.rs:433 — hax replace_body (syntactic, includes inactive annotations) — 148 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap
- src/tls13crypto.rs:484 — hax replace (syntactic, includes inactive annotations) — 306 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::hmac_verify, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::verify, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap, bertie::tls13crypto::kem_decap
- src/tls13crypto.rs:640 — hax replace_body (syntactic, includes inactive annotations) — 148 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap
- src/tls13crypto.rs:695 — hax replace_body (syntactic, includes inactive annotations) — 148 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap
- src/tls13crypto.rs:738 — hax replace (syntactic, includes inactive annotations) — 306 SLoC — affected units: bertie::tls13crypto::HashAlgorithm::hash_len, bertie::tls13crypto::HashAlgorithm::hmac_tag_len, bertie::tls13crypto::hmac_verify, bertie::tls13crypto::AeadAlgorithm::key_len, bertie::tls13crypto::AeadAlgorithm::iv_len, bertie::tls13crypto::sign, bertie::tls13crypto::verify, bertie::tls13crypto::kem_keygen, bertie::tls13crypto::kem_encap, bertie::tls13crypto::kem_decap
- src/tls13crypto.rs:889 — hax opaque (syntactic, includes inactive annotations) — 27 SLoC — affected units: bertie::tls13crypto::TryFrom<&str>forAlgorithms::try_from
- src/tls13formats.rs:555 — hax replace_body (syntactic, includes inactive annotations) — 170 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:665 — hax replace_body (syntactic, includes inactive annotations) — 170 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:736 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:876 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:949 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1016 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1026 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1058 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1097 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1191 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1220 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1260 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1284 — hax replace (syntactic, includes inactive annotations) — 626 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::parse_client_hello, bertie::tls13formats::server_hello, bertie::tls13formats::parse_server_hello, bertie::tls13formats::encrypted_extensions, bertie::tls13formats::parse_encrypted_extensions, bertie::tls13formats::server_certificate, bertie::tls13formats::parse_server_certificate, bertie::tls13formats::certificate_verify, bertie::tls13formats::parse_certificate_verify, bertie::tls13formats::finished, bertie::tls13formats::parse_finished, bertie::tls13formats::Transcript::add
- src/tls13formats.rs:1433 — hax replace_body (syntactic, includes inactive annotations) — 170 SLoC — affected units: bertie::tls13formats::client_hello, bertie::tls13formats::set_client_hello_binder, bertie::tls13formats::Transcript::add
- src/tls13formats/handshake_data.rs:28 — hax opaque (syntactic, includes inactive annotations) — 0 SLoC — affected units: 
- src/tls13formats/handshake_data.rs:65 — hax replace (syntactic, includes inactive annotations) — 71 SLoC — affected units: bertie::tls13formats::handshake_data::to_two_inner, bertie::tls13formats::handshake_data::to_four_inner, bertie::tls13formats::handshake_data::HandshakeData::len
- src/tls13formats/handshake_data.rs:90 — hax replace (syntactic, includes inactive annotations) — 71 SLoC — affected units: bertie::tls13formats::handshake_data::to_two_inner, bertie::tls13formats::handshake_data::to_four_inner, bertie::tls13formats::handshake_data::HandshakeData::len
- src/tls13formats/handshake_data.rs:165 — hax replace_body (syntactic, includes inactive annotations) — 6 SLoC — affected units: bertie::tls13formats::handshake_data::HandshakeData::len
- src/tls13handshake.rs:687 — hax replace_body (syntactic, includes inactive annotations) — 21 SLoC — affected units: bertie::tls13handshake::get_rsa_signature
- src/tls13handshake.rs:960 — hax replace (syntactic, includes inactive annotations) — 25 SLoC — affected units: bertie::tls13handshake::get_rsa_signature, bertie::tls13handshake::proverif_extra::f
- src/tls13keyscheduler.rs:43 — hax replace_body (syntactic, includes inactive annotations) — 88 SLoC — affected units: bertie::tls13keyscheduler::derive_aead_key_iv, bertie::tls13keyscheduler::derive_finished_key
- src/tls13keyscheduler.rs:133 — hax replace_body (syntactic, includes inactive annotations) — 88 SLoC — affected units: bertie::tls13keyscheduler::derive_aead_key_iv, bertie::tls13keyscheduler::derive_finished_key
- src/tls13keyscheduler/key_schedule.rs:350 — hax replace_body (syntactic, includes inactive annotations) — 127 SLoC — affected units: bertie::tls13keyscheduler::key_schedule::XPD, bertie::tls13keyscheduler::key_schedule::XTR
- src/tls13keyscheduler/key_schedule.rs:429 — hax replace_body (syntactic, includes inactive annotations) — 127 SLoC — affected units: bertie::tls13keyscheduler::key_schedule::XPD, bertie::tls13keyscheduler::key_schedule::XTR
- src/tls13record.rs:211 — hax replace (syntactic, includes inactive annotations) — 82 SLoC — affected units: bertie::tls13record::encrypt_handshake, bertie::tls13record::decrypt_handshake
- src/tls13record.rs:244 — hax replace (syntactic, includes inactive annotations) — 82 SLoC — affected units: bertie::tls13record::encrypt_handshake, bertie::tls13record::decrypt_handshake
- src/tls13utils.rs:202 — hax replace (syntactic, includes inactive annotations) — 59 SLoC — affected units: bertie::tls13utils::concat_inner, bertie::tls13utils::eq_inner, bertie::tls13utils::Bytes::prefix, bertie::tls13utils::u16_as_be_bytes, bertie::tls13utils::check_eq_inner
- src/tls13utils.rs:212 — hax replace (syntactic, includes inactive annotations) — 59 SLoC — affected units: bertie::tls13utils::concat_inner, bertie::tls13utils::eq_inner, bertie::tls13utils::Bytes::prefix, bertie::tls13utils::u16_as_be_bytes, bertie::tls13utils::check_eq_inner
- src/tls13utils.rs:229 — hax replace_body (syntactic, includes inactive annotations) — 12 SLoC — affected units: bertie::tls13utils::Bytes::prefix
- src/tls13utils.rs:307 — hax replace (syntactic, includes inactive annotations) — 59 SLoC — affected units: bertie::tls13utils::concat_inner, bertie::tls13utils::eq_inner, bertie::tls13utils::Bytes::prefix, bertie::tls13utils::u16_as_be_bytes, bertie::tls13utils::check_eq_inner
- src/tls13utils.rs:409 — hax replace (syntactic, includes inactive annotations) — 59 SLoC — affected units: bertie::tls13utils::concat_inner, bertie::tls13utils::eq_inner, bertie::tls13utils::Bytes::prefix, bertie::tls13utils::u16_as_be_bytes, bertie::tls13utils::check_eq_inner

## Escape hatches

- /out/cargo-home/git/checkouts/libcrux-c3a2e0b268f26fe2/9d5290e/crates/sys/platform/src/linux_arm.rs:8 — unsafe — libcrux-platform@0.0.4 — SECURITY-RELEVANT
- /out/cargo-home/git/checkouts/libcrux-c3a2e0b268f26fe2/9d5290e/crates/sys/platform/src/x86.rs:239 — unsafe — libcrux-platform@0.0.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:76 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:88 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:159 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:173 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:193 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:238 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:360 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/addr2line-0.27.1/src/loader.rs:390 — unsafe — addr2line@0.27.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstream-1.0.0/src/adapter/strip.rs:136 — unsafe — anstream@1.0.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstream-1.0.0/src/adapter/strip.rs:147 — unsafe — anstream@1.0.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstream-1.0.0/src/adapter/strip.rs:148 — unsafe — anstream@1.0.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstyle-1.0.14/src/color.rs:615 — unsafe — anstyle@1.0.14 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstyle-parse-1.0.0/src/lib.rs:176 — unsafe — anstyle-parse@1.0.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstyle-parse-1.0.0/src/lib.rs:183 — unsafe — anstyle-parse@1.0.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstyle-parse-1.0.0/src/state/definitions.rs:116 — unsafe — anstyle-parse@1.0.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anstyle-query-1.1.5/src/windows.rs:13 — unsafe — anstyle-query@1.1.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/context.rs:157 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ensure.rs:49 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ensure.rs:69 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ensure.rs:246 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ensure.rs:247 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ensure.rs:413 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ensure.rs:417 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ensure.rs:418 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:163 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:188 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:213 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:243 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:269 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:278 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:383 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:415 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:442 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:478 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:541 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:555 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:606 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:649 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:658 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:690 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:697 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:703 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:709 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:715 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:723 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:724 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:726 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:728 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:729 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:730 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:732 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:736 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:740 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:744 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:750 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:754 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:760 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:767 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:773 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:778 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:784 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:788 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:797 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:815 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:822 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:823 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:826 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:827 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:835 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:844 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:847 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:852 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:858 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:859 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:862 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:864 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:869 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:878 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:881 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:885 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:887 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:894 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:899 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:900 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:918 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:920 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:941 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:944 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:948 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:951 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:959 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:963 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:968 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:970 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:976 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:977 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:980 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:984 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:985 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:994 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:999 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:1008 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/error.rs:1017 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/fmt.rs:7 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/fmt.rs:8 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/fmt.rs:11 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/fmt.rs:20 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/fmt.rs:21 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/fmt.rs:48 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:13 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:15 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:34 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:44 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:45 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:119 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:120 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:169 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:170 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:175 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/src/ptr.rs:176 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/tests/test_ensure.rs:332 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.104/tests/test_ensure.rs:604 — unsafe — anyhow@1.0.104 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:34 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:35 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:44 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:59 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:83 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:115 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:116 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:124 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:173 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:191 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:210 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:211 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:214 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:251 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:261 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:264 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:279 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:282 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/libunwind.rs:296 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:5 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:38 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:39 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:62 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:65 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:71 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:78 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:102 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:103 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/miri.rs:107 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/mod.rs:53 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/mod.rs:65 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/mod.rs:66 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/noop.rs:11 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:31 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:32 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:99 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:101 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:102 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:105 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:106 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:119 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:123 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:137 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:152 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:163 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:173 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:187 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win32.rs:215 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win64.rs:23 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win64.rs:24 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win64.rs:79 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win64.rs:84 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win64.rs:85 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win64.rs:97 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/backtrace/win64.rs:128 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/capture.rs:37 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/capture.rs:39 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:56 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:66 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:80 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:90 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:103 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:110 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:230 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:244 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:323 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:324 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:444 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:451 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:464 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/dbghelp.rs:472 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:6 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:56 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:107 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:144 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:146 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:351 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:353 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/print/fuchsia.rs:373 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:51 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:60 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:83 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:89 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:102 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:109 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:133 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:140 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:154 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:160 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:225 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:233 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:237 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:247 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:249 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:254 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:292 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:298 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:307 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:310 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:317 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:319 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:323 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/dbghelp.rs:325 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:104 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:195 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:346 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:347 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:363 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:376 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:435 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:441 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:448 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli.rs:452 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/elf.rs:75 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_aix.rs:14 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_aix.rs:20 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_aix.rs:28 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:26 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:55 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:61 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:62 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:63 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:64 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:66 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:74 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:86 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_dl_iterate_phdr.rs:106 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_haiku.rs:19 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:29 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:41 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:58 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:62 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:66 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:67 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:68 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:69 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:72 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_illumos.rs:97 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_libnx.rs:7 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_macos.rs:20 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_macos.rs:34 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_macos.rs:45 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_macos.rs:140 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:13 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:19 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:20 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:47 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:50 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:69 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:85 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:98 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:103 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/libs_windows.rs:117 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/lru.rs:26 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/lru.rs:34 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/lru.rs:44 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/lru.rs:57 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/lru.rs:73 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/macho.rs:275 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_fake.rs:11 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_unix.rs:18 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_unix.rs:19 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_unix.rs:40 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_unix.rs:46 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_windows.rs:19 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_windows.rs:20 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_windows.rs:56 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/mmap_windows.rs:62 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/parse_running_mmaps_unix.rs:60 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/parse_running_mmaps_unix.rs:65 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/parse_running_mmaps_unix.rs:105 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/parse_running_mmaps_unix.rs:117 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/parse_running_mmaps_unix.rs:119 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/stash.rs:29 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/gimli/stash.rs:45 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/miri.rs:8 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/miri.rs:56 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/mod.rs:63 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/mod.rs:105 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/mod.rs:158 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/mod.rs:162 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/mod.rs:174 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/mod.rs:178 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/mod.rs:428 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/noop.rs:8 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/symbolize/noop.rs:41 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/windows_sys.rs:264 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/windows_sys.rs:272 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/windows_sys.rs:500 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/windows_sys.rs:508 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/windows_sys.rs:510 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/windows_sys.rs:512 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/src/windows_sys.rs:522 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/tests/accuracy/main.rs:30 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/tests/sgx-image-base.rs:31 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/tests/sgx-image-base.rs:44 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/backtrace-0.3.77/tests/smoke.rs:18 — unsafe — backtrace@0.3.77 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-1.1.1/src/ext.rs:207 — unsafe — clap_lex@1.1.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-1.1.1/src/ext.rs:232 — unsafe — clap_lex@1.1.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-1.1.1/src/ext.rs:275 — unsafe — clap_lex@1.1.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-1.1.1/src/ext.rs:276 — unsafe — clap_lex@1.1.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-1.1.1/src/lib.rs:458 — unsafe — clap_lex@1.1.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-1.1.1/src/lib.rs:485 — unsafe — clap_lex@1.1.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/benches/buffer.rs:40 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/benches/buffer.rs:42 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/benches/buffer.rs:64 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/benches/buffer.rs:66 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/apple_other.rs:10 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/custom.rs:12 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/efi_rng.rs:35 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/efi_rng.rs:57 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/efi_rng.rs:71 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/efi_rng.rs:78 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/efi_rng.rs:105 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/esp_idf.rs:18 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/fuchsia.rs:14 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/getentropy.rs:21 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/getrandom.rs:28 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/getrandom.rs:33 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/hermit.rs:18 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/hermit.rs:20 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/hermit.rs:29 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/hermit.rs:31 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/hermit.rs:40 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:14 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:18 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:30 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:35 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:40 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:43 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:96 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_android_with_fallback.rs:97 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_raw.rs:10 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_raw.rs:126 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/linux_raw.rs:127 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/netbsd.rs:20 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/netbsd.rs:33 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/netbsd.rs:43 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/netbsd.rs:52 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/netbsd.rs:74 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/netbsd.rs:75 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:31 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:51 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:70 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:74 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:101 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:106 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:126 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:132 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:138 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:144 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:156 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:165 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rdrand.rs:174 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rndr.rs:24 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rndr.rs:47 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rndr.rs:91 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rndr.rs:117 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rndr.rs:127 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/rndr.rs:137 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/sanitizer.rs:15 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/sanitizer.rs:24 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/sanitizer.rs:50 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/sanitizer.rs:54 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/solaris.rs:29 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/solid.rs:13 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/use_file.rs:49 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/use_file.rs:63 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/use_file.rs:142 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/use_file.rs:161 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/use_file.rs:175 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/use_file.rs:217 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/use_file.rs:231 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/vxworks.rs:18 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/vxworks.rs:21 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/vxworks.rs:43 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/wasi_p1.rs:25 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/wasi_p2.rs:19 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/wasi_p2.rs:29 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/wasi_p2.rs:41 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/windows.rs:53 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/backends/windows_legacy.rs:37 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/error.rs:129 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/error.rs:136 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:70 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/lib.rs:109 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:10 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:13 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:18 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:19 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:26 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:35 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:38 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:56 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:63 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:71 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util.rs:78 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util_libc.rs:32 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/src/util_libc.rs:38 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/getrandom-0.3.4/tests/mod.rs:264 — unsafe — getrandom@0.3.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/endian_reader.rs:181 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/endian_reader.rs:183 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/endian_reader.rs:200 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/endian_reader.rs:217 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/endian_reader.rs:227 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:14 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:45 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:50 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:68 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:73 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:91 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:102 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:139 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:164 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:179 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:198 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:221 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/read/util.rs:230 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/write/unit.rs:3710 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/gimli-0.34.0/src/write/unit.rs:3722 — unsafe — gimli@0.34.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.2.0/src/dummy.rs:32 — assume — hax-lib@0.2.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.2.0/src/implementation.rs:94 — assume — hax-lib@0.2.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.2.0/src/implementation.rs:98 — assume — hax-lib@0.2.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.2.0/src/implementation.rs:100 — assume — hax-lib@0.2.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.2.0/src/implementation.rs:118 — assume — hax-lib@0.2.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.4.2/src/dummy.rs:32 — assume — hax-lib@0.4.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.4.2/src/implementation.rs:94 — assume — hax-lib@0.4.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.4.2/src/implementation.rs:98 — assume — hax-lib@0.4.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.4.2/src/implementation.rs:100 — assume — hax-lib@0.4.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/hax-lib-0.4.2/src/implementation.rs:118 — assume — hax-lib@0.4.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.5.1/src/inline_lazy.rs:36 — unsafe — lazy_static@1.5.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.5.1/src/inline_lazy.rs:40 — unsafe — lazy_static@1.5.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/lazy_static-1.5.1/tests/test.rs:27 — unsafe — lazy_static@1.5.1 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:133 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:134 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3130 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3139 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3148 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3157 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3161 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3165 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3171 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3178 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3188 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3192 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3196 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3206 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3214 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3218 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3222 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3289 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3294 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3298 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:3635 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:4147 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:4148 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:4149 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/fuchsia/mod.rs:4173 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/helenos.rs:54 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:421 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:520 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:997 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1005 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1013 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1019 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1093 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1276 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1277 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1282 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1283 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1288 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1289 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1435 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1442 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1474 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1488 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/macros.rs:1564 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/common/posix/pthread.rs:10 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/common/posix/pthread.rs:11 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/common/posix/pthread.rs:12 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/common/posix/pthread.rs:286 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/sigaction.rs:15 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:140 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:141 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:145 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:146 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:150 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:151 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:155 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:156 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:160 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:161 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:165 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:166 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:170 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:171 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:175 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:176 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:180 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:183 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:187 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:188 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:192 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:193 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:198 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:199 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:203 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:204 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:208 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:209 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:213 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:214 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:218 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:219 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:223 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:224 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:228 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:229 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:233 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:234 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:238 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:239 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:243 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/glibc/sysdeps/unix/linux/bits/types/siginfo_t.rs:244 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/helenos/fibril.rs:23 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/helenos/inet/tcp.rs:11 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/helenos/inet/tcp.rs:12 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/helenos/inet/tcp.rs:13 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/helenos/inet/tcp.rs:14 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/helenos/inet/tcp.rs:15 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/helenos/inet/tcp.rs:19 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/can.rs:112 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/if_packet.rs:125 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/netlink.rs:87 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/netlink.rs:93 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/netlink.rs:100 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/netlink.rs:106 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/netlink.rs:174 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/sctp.rs:65 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/sctp.rs:69 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/linux_uapi/linux/sctp.rs:73 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:105 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:106 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:110 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:111 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:115 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:116 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:120 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:121 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:125 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:126 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:130 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:131 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:135 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:136 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:140 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:141 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:145 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:146 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:150 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:151 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:155 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:156 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:160 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:161 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:165 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:166 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:170 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:171 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:175 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:176 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:180 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:181 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:185 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:188 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:192 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:193 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:197 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:198 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:202 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/musl/signal.rs:203 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:43 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:47 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:51 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:55 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:59 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:63 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:67 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:71 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/nto/net/bpf.rs:80 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/qurt/errno.rs:148 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/qurt/errno.rs:152 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:117 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:118 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:122 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:123 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:127 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:128 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:132 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:133 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:137 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:138 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:142 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:143 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:147 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:148 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:152 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:153 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:157 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:160 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:164 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:165 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:169 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:170 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:180 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:181 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:185 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:186 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:190 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:191 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:195 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:196 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:200 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:201 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:205 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/new/uclibc/sysdeps/linux/common/bits/siginfo.rs:206 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:18 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:26 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:29 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:31 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:34 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:36 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:45 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:52 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:55 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:65 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:68 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:70 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:71 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/psp.rs:73 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/solid/mod.rs:535 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/solid/mod.rs:546 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/solid/mod.rs:560 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/solid/mod.rs:598 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/solid/mod.rs:616 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/solid/mod.rs:622 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/teeos/mod.rs:1033 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/teeos/mod.rs:1334 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/types.rs:123 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:566 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:576 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2466 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2474 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2490 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2494 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2498 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2502 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2506 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2515 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2524 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2607 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2608 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2609 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2695 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:2785 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:3015 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/mod.rs:3033 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:272 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:276 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:280 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:284 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:288 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:310 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:319 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:322 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:327 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:387 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/aix/powerpc64.rs:397 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:16 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:18 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:20 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:27 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:28 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:30 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:36 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:40 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:48 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:58 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:65 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:67 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/b64/x86_64/mod.rs:69 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1849 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1855 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1875 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1885 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1904 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1908 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1924 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1928 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1932 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1949 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1955 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1974 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1975 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1976 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1977 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1986 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:1993 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:2002 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:2031 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:2056 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:2062 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:2063 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:2070 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:2093 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:4589 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:4603 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:4607 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:4611 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:4615 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:5446 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/apple/mod.rs:5452 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:480 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1244 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1248 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1252 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1264 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1268 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1272 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1277 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1282 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/dragonfly/mod.rs:1362 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/freebsd11/mod.rs:324 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/freebsd12/mod.rs:362 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/freebsd13/mod.rs:401 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/freebsd14/mod.rs:401 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/freebsd15/mod.rs:403 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1815 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1821 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1845 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1857 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1879 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1880 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1881 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1882 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1883 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1884 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1885 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1886 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1887 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1888 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1889 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1890 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1891 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1892 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1893 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1899 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1905 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1906 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1912 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1918 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1927 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:1933 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4431 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4435 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4439 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4452 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4456 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4460 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4464 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4468 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4473 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4477 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4481 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4485 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4491 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4497 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4503 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4513 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4518 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4522 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/mod.rs:4903 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/freebsd/x86_64/mod.rs:156 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/mod.rs:66 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/mod.rs:70 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/mod.rs:74 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/mod.rs:78 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/mod.rs:82 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/freebsdlike/mod.rs:1721 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:521 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:529 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:538 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:547 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:556 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:798 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:799 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/mod.rs:800 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/aarch64.rs:44 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/aarch64.rs:56 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:65 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:69 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:73 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:77 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:90 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:104 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:119 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:813 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:818 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1829 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1833 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1837 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1850 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1857 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1861 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1866 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:1870 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:2172 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:2285 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:2352 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/mod.rs:2353 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/riscv64.rs:28 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/netbsd/riscv64.rs:34 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:784 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:797 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:804 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:808 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:812 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:816 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:828 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:841 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:855 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:890 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:908 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:1864 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:1868 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:1872 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:1885 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:2023 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:2050 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/bsd/netbsdlike/openbsd/mod.rs:2057 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:527 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:540 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:553 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:557 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:561 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1687 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1696 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1705 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1714 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1718 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1724 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1733 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1737 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1745 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1753 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1763 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1767 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1771 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1775 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1779 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1787 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:1797 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:2034 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:2097 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:2098 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/cygwin/mod.rs:2099 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/bsd.rs:122 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:88 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:92 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:96 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:100 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1417 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1425 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1429 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1433 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1437 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1452 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1461 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1470 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1479 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1761 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1762 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1763 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1832 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/mod.rs:1839 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:484 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:503 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1290 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1295 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1300 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1309 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1314 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1323 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1339 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1344 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1349 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1354 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1359 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1364 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1369 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1379 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/haiku/native.rs:1384 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:1126 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:1130 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:1134 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:1138 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:1142 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3527 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3535 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3539 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3543 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3547 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3562 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3568 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3572 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3578 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3584 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3590 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3599 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3603 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3607 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3611 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3615 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3624 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3633 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3642 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3664 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:3668 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:4188 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:4189 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:4190 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:4589 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/hurd/mod.rs:4762 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b32/arm.rs:63 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b32/arm.rs:69 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b32/arm.rs:486 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b32/x86/mod.rs:64 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b32/x86/mod.rs:70 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b32/x86/mod.rs:554 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b64/mod.rs:192 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b64/x86_64/mod.rs:173 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/b64/x86_64/mod.rs:179 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3452 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3462 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3468 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3472 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3478 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3484 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3490 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3499 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3503 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3507 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3511 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3853 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3861 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3945 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3956 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3971 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3975 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3979 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3983 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3987 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/android/mod.rs:3991 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:7 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:12 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:17 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:22 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:31 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:40 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:45 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:50 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:60 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:65 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:70 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:75 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:80 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:85 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:90 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:95 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:118 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:128 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:133 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:143 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:153 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:163 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:173 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:178 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:187 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:192 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:197 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:202 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:207 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/lfs64.rs:212 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/mod.rs:1324 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/mod.rs:1337 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/mod.rs:1341 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/mod.rs:1347 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/mod.rs:1353 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/emscripten/mod.rs:1359 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:314 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:325 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:365 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:369 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:373 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:377 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:381 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/l4re/uclibc/mod.rs:385 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/gnu/mod.rs:456 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/gnu/mod.rs:468 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/gnu/mod.rs:920 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3353 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3357 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3361 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3365 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3369 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3373 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3377 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3381 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3385 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3394 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3399 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/mod.rs:3404 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:5 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:10 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:20 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:25 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:30 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:39 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:48 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:53 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:58 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:68 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:73 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:78 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:83 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:88 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:93 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:98 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:103 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:126 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:136 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:141 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:151 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:161 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:171 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:181 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:191 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:196 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:205 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:215 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:220 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:225 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:230 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:235 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux/musl/lfs64.rs:240 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1436 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1472 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1478 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1482 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1488 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1494 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1500 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1509 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1513 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1517 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1521 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1525 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1529 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1533 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1537 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1541 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1545 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1549 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1553 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1557 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/linux_l4re_shared.rs:1823 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1774 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1782 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1786 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1790 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1794 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1803 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1812 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1821 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1826 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/linux_like/mod.rs:1830 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/mod.rs:46 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/mod.rs:47 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/mod.rs:701 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/mod.rs:708 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/mod.rs:1399 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/newlib/mod.rs:895 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/newlib/mod.rs:904 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/newlib/mod.rs:913 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/newlib/mod.rs:922 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/newlib/mod.rs:1017 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/newlib/mod.rs:1018 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/newlib/mod.rs:1019 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/arch/x86_64.rs:101 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/arch/x86_64.rs:111 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:287 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:568 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2333 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2341 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2351 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2355 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2359 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2363 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2372 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2381 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2390 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2394 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2401 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2412 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2419 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2766 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2767 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2768 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:2928 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:3021 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:3030 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:3039 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:3048 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:3057 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/mod.rs:3066 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:146 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1081 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1088 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1123 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1129 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1140 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1141 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1158 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1162 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1167 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1185 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1194 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1203 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/nto/neutrino.rs:1212 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:390 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:394 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:398 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:402 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:406 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:410 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:414 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1235 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1238 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1241 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1246 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1255 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1264 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1273 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1404 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1405 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/redox/mod.rs:1406 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/compat.rs:12 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/compat.rs:37 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/compat.rs:46 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/compat.rs:59 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/compat.rs:133 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/compat.rs:184 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/compat.rs:204 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:628 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:637 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:645 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:654 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:707 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:710 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:714 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:718 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:722 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:726 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:730 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:734 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2221 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2225 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2229 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2237 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2251 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2255 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2264 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2273 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2282 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2287 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2291 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2760 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2761 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2762 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:2877 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:3024 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:3028 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/mod.rs:3032 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/x86_64.rs:104 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/unix/solarish/x86_64.rs:117 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:10 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:11 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:109 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:113 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:117 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:121 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:125 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:621 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:623 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:636 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:652 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:664 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:670 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1466 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1470 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1482 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1490 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1494 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1498 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1904 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1905 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:1906 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:2037 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:2443 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/vxworks/mod.rs:2450 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:59 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:60 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:70 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:71 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:554 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:556 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:676 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:685 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:697 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/wasi/mod.rs:1121 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/windows/mod.rs:313 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/src/windows/mod.rs:319 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.191/tests/const_fn.rs:2 — unsafe — libc@0.2.191 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:10 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:15 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:20 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:25 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:30 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:35 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:40 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:45 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:50 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:55 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:60 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:65 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:70 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:75 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:80 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:85 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:90 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:95 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:100 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:105 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:109 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:113 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:117 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:121 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:126 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:131 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:135 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:139 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:144 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:149 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:154 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:158 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:162 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:166 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:171 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:176 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:180 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:184 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:188 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:192 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:196 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:200 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:205 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:209 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:213 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:218 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:222 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:227 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:232 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:236 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:240 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:245 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:250 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:254 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:258 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:262 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:266 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:270 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:274 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:278 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:282 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:286 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:290 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:294 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:298 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:302 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:306 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:311 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:316 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:321 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:326 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:331 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:335 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:339 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:343 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:347 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:351 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/arm64.rs:355 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:13 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:20 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:27 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:35 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:42 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:50 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:58 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:64 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:69 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:74 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:79 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:83 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:105 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:162 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:173 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:194 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:204 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:209 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:214 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:227 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:236 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:240 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:244 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:248 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:253 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:258 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:263 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:267 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:272 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:277 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:282 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:287 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:291 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:295 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:300 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:305 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:310 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:315 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:320 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:325 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:330 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:335 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:340 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:345 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:350 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:355 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:373 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:379 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:384 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:390 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:395 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:401 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:406 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:412 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:418 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:423 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:427 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:432 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:438 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:443 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:448 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:453 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:458 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:462 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:467 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:472 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:476 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:482 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:488 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:494 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:500 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:507 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:518 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:523 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:528 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:532 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:537 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:541 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:546 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:552 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:557 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:562 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:566 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:571 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-intrinsics-0.0.2/src/avx2.rs:576 — unsafe — libcrux-intrinsics@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental.rs:37 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:84 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:141 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:201 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:254 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:335 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:390 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:438 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:456 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:486 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:542 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:595 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/incremental/multiplexing.rs:686 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:15 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:55 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:70 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:105 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:121 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:135 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:143 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:171 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:193 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:251 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:285 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:355 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:377 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:444 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:484 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:566 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:605 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:637 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:679 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:720 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:745 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:813 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:846 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/ind_cca/instantiations/avx2.rs:924 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:469 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:489 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:513 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:544 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:573 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:581 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:589 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:623 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:655 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:671 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:692 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:737 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-ml-kem-0.0.2/src/mlkem.rs:784 — unsafe — libcrux-ml-kem@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:7 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:9 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:11 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:13 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:15 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:26 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:32 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:38 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:44 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:52 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/linux_arm.rs:56 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:9 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:22 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:33 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:59 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:68 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:77 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:86 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:98 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:104 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:110 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:116 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:124 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/macos_arm.rs:133 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/x86.rs:50 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/x86.rs:51 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/x86.rs:116 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/x86.rs:122 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/x86.rs:127 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/x86.rs:135 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/libcrux-platform-0.0.2/src/x86.rs:139 — unsafe — libcrux-platform@0.0.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.34/src/lib.rs:452 — unsafe — log@0.4.34 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.34/src/lib.rs:1410 — unsafe — log@0.4.34 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.34/src/lib.rs:1438 — unsafe — log@0.4.34 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.34/src/lib.rs:1529 — unsafe — log@0.4.34 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.34/src/lib.rs:1564 — unsafe — log@0.4.34 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.34/src/lib.rs:1629 — unsafe — log@0.4.34 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/memchr.rs:41 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/memchr.rs:55 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/memchr.rs:69 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/memchr.rs:84 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/memchr.rs:99 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/memchr.rs:115 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/memchr.rs:131 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:43 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:63 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:97 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:113 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:125 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:156 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:200 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:241 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:269 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:289 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:309 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:353 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:361 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:377 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:403 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:423 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:457 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:473 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:504 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:548 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:580 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:600 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:644 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:659 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:685 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:705 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:743 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:759 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:790 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:836 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:870 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:890 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:934 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/memchr.rs:949 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/packedpair.rs:58 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/packedpair.rs:72 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/packedpair.rs:109 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/packedpair.rs:120 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/packedpair.rs:136 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/aarch64/neon/packedpair.rs:157 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:68 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:84 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:96 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:127 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:197 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:256 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:318 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:326 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:342 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:390 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:406 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:437 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:503 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:583 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:598 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:654 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:670 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:701 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:767 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:851 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/memchr.rs:866 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/mod.rs:72 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/mod.rs:108 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/rabinkarp.rs:112 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/rabinkarp.rs:150 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/rabinkarp.rs:210 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/rabinkarp.rs:248 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/rabinkarp.rs:300 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/rabinkarp.rs:315 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/all/rabinkarp.rs:362 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:111 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:143 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:254 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:349 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:416 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:450 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:493 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:592 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:670 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:710 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:765 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:874 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:962 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1017 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1022 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1044 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1083 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1125 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1154 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1180 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/memchr.rs:1205 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/packedpair.rs:54 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/packedpair.rs:80 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/packedpair.rs:170 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/packedpair.rs:229 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/packedpair.rs:269 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/generic/packedpair.rs:312 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/memchr.rs:28 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/memchr.rs:42 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/memchr.rs:56 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/memchr.rs:71 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/memchr.rs:86 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/memchr.rs:102 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/memchr.rs:118 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:43 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:58 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:92 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:108 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:120 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:151 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:195 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:240 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:268 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:288 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:308 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:352 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:360 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:376 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:402 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:417 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:451 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:467 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:498 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:542 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:574 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:594 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:638 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:653 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:679 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:694 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:732 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:748 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:779 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:825 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:859 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:879 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:923 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/memchr.rs:938 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/packedpair.rs:51 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/packedpair.rs:65 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/packedpair.rs:130 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/wasm32/simd128/packedpair.rs:150 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:49 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:69 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:120 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:136 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:148 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:179 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:245 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:299 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:336 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:356 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:376 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:396 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:416 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:436 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:480 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:488 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:504 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:536 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:556 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:607 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:623 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:654 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:720 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:764 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:784 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:804 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:824 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:868 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:883 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:915 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:935 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:990 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1006 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1037 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1105 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1151 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1171 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1191 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1211 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1255 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/memchr.rs:1270 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/packedpair.rs:54 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/packedpair.rs:68 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/packedpair.rs:120 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/packedpair.rs:134 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/packedpair.rs:150 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/avx2/packedpair.rs:175 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:78 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:90 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:100 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:109 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:151 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:174 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:183 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:197 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:206 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:220 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:230 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:245 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:255 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:270 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:281 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:297 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:308 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:324 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/memchr.rs:333 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:43 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:63 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:97 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:113 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:125 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:156 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:212 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:257 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:285 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:305 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:325 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:369 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:377 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:393 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:419 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:439 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:473 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:489 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:520 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:576 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:611 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:631 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:675 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:690 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:716 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:736 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:774 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:790 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:821 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:879 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:916 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:936 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:980 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/memchr.rs:995 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/packedpair.rs:51 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/packedpair.rs:65 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/packedpair.rs:102 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/packedpair.rs:116 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/packedpair.rs:132 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/arch/x86_64/sse2/packedpair.rs:153 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/ext.rs:8 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/ext.rs:21 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/ext.rs:32 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:30 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:62 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:95 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:128 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:166 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:204 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:316 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:331 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:347 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:394 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:411 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:469 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:488 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:504 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:541 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:574 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:609 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:646 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:688 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memchr.rs:730 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:222 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:273 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:286 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:301 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:318 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:339 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:362 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:383 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:404 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:425 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:718 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:775 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:783 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:797 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:816 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:835 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/memmem/searcher.rs:854 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:33 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:42 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:51 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:54 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:56 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:58 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:60 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:63 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:207 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:212 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:217 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:222 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:227 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:232 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:237 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:256 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:261 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:266 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:271 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:276 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:281 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:286 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:305 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:310 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:317 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:323 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:333 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:344 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:349 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:354 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:365 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:480 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:485 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:490 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:495 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:500 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:505 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/memchr-2.8.3/src/vector.rs:510 — unsafe — memchr@2.8.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/nu-ansi-term-0.50.3/src/windows.rs:22 — unsafe — nu-ansi-term@0.50.3 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/endian.rs:280 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/endian.rs:663 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/endian.rs:664 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/endian.rs:665 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/endian.rs:666 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/endian.rs:667 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/endian.rs:668 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:25 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:43 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:66 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:86 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:112 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:172 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:183 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:194 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:205 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:211 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/pod.rs:218 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/read/read_cache.rs:126 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/object-0.40.0/src/read/read_cache.rs:163 — unsafe — object@0.40.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_cs.rs:20 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_cs.rs:21 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_cs.rs:63 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:21 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:22 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:61 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:63 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:82 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:103 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:113 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:132 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_pl.rs:154 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:32 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:33 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:71 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:74 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:99 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:109 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:157 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:255 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:326 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/imp_std.rs:337 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:498 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:520 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:566 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:572 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:645 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:976 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:1010 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:1041 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:1092 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:1175 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/lib.rs:1280 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:68 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:95 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:99 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:156 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:227 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:238 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:261 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:314 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:382 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:404 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:420 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:472 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:475 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/src/race.rs:479 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/tests/it/race.rs:189 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.4/tests/it/sync_once_cell.rs:47 — unsafe — once_cell@1.21.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1057 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1093 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1141 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1187 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1371 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1734 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1742 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1757 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/src/lib.rs:1765 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/default/enum.expanded.rs:83 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/default/enum.expanded.rs:102 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/default/enum.expanded.rs:122 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/default/struct.expanded.rs:63 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/default/struct.expanded.rs:76 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/multifields/enum.expanded.rs:43 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/multifields/struct.expanded.rs:87 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/multifields/struct.expanded.rs:103 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/multifields/struct.expanded.rs:119 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/enum-all.expanded.rs:83 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/enum-all.expanded.rs:102 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/enum-all.expanded.rs:122 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/enum-mut.expanded.rs:44 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/enum-ref.expanded.rs:44 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-all.expanded.rs:79 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-all.expanded.rs:92 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-all.expanded.rs:106 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-mut.expanded.rs:63 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-mut.expanded.rs:76 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-none.expanded.rs:63 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-none.expanded.rs:76 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-ref.expanded.rs:63 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/naming/struct-ref.expanded.rs:76 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/not_unpin/enum.expanded.rs:67 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/not_unpin/enum.expanded.rs:86 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/not_unpin/struct.expanded.rs:63 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/not_unpin/struct.expanded.rs:76 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pinned_drop/enum.expanded.rs:67 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pinned_drop/enum.expanded.rs:86 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pinned_drop/enum.expanded.rs:121 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pinned_drop/struct.expanded.rs:63 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pinned_drop/struct.expanded.rs:76 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pinned_drop/struct.expanded.rs:105 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pub/enum.expanded.rs:67 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pub/enum.expanded.rs:86 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pub/struct.expanded.rs:63 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/pin-project-lite-0.2.17/tests/expand/pub/struct.expanded.rs:76 — unsafe — pin-project-lite@0.2.17 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:26 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:38 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:51 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:106 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:110 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:120 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:121 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:130 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:140 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:144 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:155 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:156 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:160 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:180 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:182 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:193 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:194 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:198 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:454 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:498 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:504 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:510 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:578 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:583 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:600 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:605 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:791 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:808 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:825 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:842 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/generic.rs:864 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:119 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:136 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:181 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:186 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:330 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:361 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:416 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/soft.rs:426 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:60 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:247 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:263 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:271 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:277 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:284 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:291 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/types.rs:295 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:64 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:92 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:122 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:129 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:148 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:180 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:187 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:215 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:222 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:231 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:259 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:267 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:274 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:278 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:282 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:285 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:304 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:305 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:342 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:346 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:349 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:362 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:363 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:400 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:404 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:407 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:420 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/mod.rs:421 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:20 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:56 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:83 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:88 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:95 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:101 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:110 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:118 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:136 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:175 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:183 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:232 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:240 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:287 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:295 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:328 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:336 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:346 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:354 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:366 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:375 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:385 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:394 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:508 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:528 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:549 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:596 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:600 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:604 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:615 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:624 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:639 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:651 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:665 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:673 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:683 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:695 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:705 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:724 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:732 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:752 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:761 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:768 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:783 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:795 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:812 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:819 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:826 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:830 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:848 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:858 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:862 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1078 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1084 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1094 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1110 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1147 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1148 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1170 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1171 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1194 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1195 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1232 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1233 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1268 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1269 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1270 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1299 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1300 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1301 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1328 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1344 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1359 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1371 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1410 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1416 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1421 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1426 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1439 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1448 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1454 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1464 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1479 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1489 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1552 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1567 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1584 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1591 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1595 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1599 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1610 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1665 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/ppv-lite86-0.2.21/src/x86_64/sse2.rs:1703 — unsafe — ppv-lite86@0.2.21 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/integer.rs:123 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/integer.rs:136 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/integer.rs:152 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/other.rs:122 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/other.rs:171 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/other.rs:188 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/slice.rs:99 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/uniform_other.rs:104 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/utils.rs:183 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/distr/utils.rs:184 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rng.rs:397 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rng.rs:419 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rng.rs:443 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rng.rs:466 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rng.rs:468 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rngs/thread.rs:106 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rngs/thread.rs:173 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rngs/thread.rs:181 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/rand-0.9.5/src/rngs/thread.rs:189 — unsafe — rand@0.9.5 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:780 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:781 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:793 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:812 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:875 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:885 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:935 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:961 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:1002 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/lib.rs:1009 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:102 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:107 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:174 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:188 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:217 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:234 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:250 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:278 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:304 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:321 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:341 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/mod.rs:354 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:91 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:96 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:101 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:283 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:342 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:355 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:604 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:616 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:628 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:785 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:800 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:812 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:824 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/slot.rs:829 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/stack.rs:90 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/stack.rs:99 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/page/stack.rs:113 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:836 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:842 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:885 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:915 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:998 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1008 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1035 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1050 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1095 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1125 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1166 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1173 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1228 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1244 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1273 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1291 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1330 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/pool.rs:1337 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/shard.rs:316 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/shard.rs:343 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/shard.rs:361 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/sharded-slab-0.1.7/src/shard.rs:386 — unsafe — sharded-slab@0.1.7 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:345 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:373 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:374 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:383 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:397 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:415 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:488 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:540 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:617 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:655 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:659 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:683 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:687 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:691 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:695 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:731 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:738 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:754 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:757 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:764 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:771 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:783 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:784 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:902 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:964 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:981 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:994 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1051 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1064 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1119 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1199 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1216 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1246 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1289 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1401 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1422 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1458 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1472 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1489 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1532 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1597 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1610 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1640 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1686 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1692 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1701 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1722 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1727 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1733 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1738 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1750 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1787 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1928 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1931 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:1973 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2008 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2059 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2078 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2088 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2209 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2323 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2360 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2362 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2376 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2453 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2498 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2519 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2534 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2540 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2548 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2579 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2662 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2673 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2685 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2771 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2778 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2814 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2821 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/lib.rs:2843 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/smallvec-1.16.4/src/tests.rs:1066 — unsafe — smallvec@1.16.4 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:115 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:119 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:132 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:140 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:161 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:208 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:238 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:241 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:266 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:282 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:291 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:300 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:311 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:430 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:438 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:462 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:474 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:552 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:586 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:590 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:616 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:624 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/lib.rs:628 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/thread_id.rs:103 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/thread_id.rs:114 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/thread_id.rs:120 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/thread_local-1.1.10/src/thread_id.rs:131 — unsafe — thread_local@1.1.10 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.44/src/instrument.rs:285 — unsafe — tracing@0.1.44 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.44/src/instrument.rs:297 — unsafe — tracing@0.1.44 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.44/src/instrument.rs:308 — unsafe — tracing@0.1.44 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.44/src/instrument.rs:370 — unsafe — tracing@0.1.44 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.44/src/instrument.rs:371 — unsafe — tracing@0.1.44 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.44/src/lib.rs:1100 — unsafe — tracing@0.1.44 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-0.1.44/src/span.rs:1604 — unsafe — tracing@0.1.44 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/callsite.rs:239 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/callsite.rs:469 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/dispatcher.rs:314 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/dispatcher.rs:323 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/dispatcher.rs:450 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/lazy.rs:37 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/metadata.rs:721 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/metadata.rs:1112 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/mutex.rs:26 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/mutex.rs:27 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/mutex.rs:61 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/mutex.rs:75 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/once.rs:28 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/once.rs:29 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/once.rs:53 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/once.rs:54 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/once.rs:94 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/once.rs:116 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/spin/once.rs:141 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:492 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:510 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:530 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:550 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:570 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:783 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:788 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:868 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-core-0.1.36/src/subscriber.rs:873 — unsafe — tracing-core@0.1.36 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-log-0.2.0/src/lib.rs:528 — unsafe — tracing-log@0.2.0 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/filter/layer_filters/mod.rs:873 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/filter/layer_filters/mod.rs:881 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/filter/layer_filters/mod.rs:1309 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/fmt/fmt_layer.rs:1070 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/fmt/mod.rs:445 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/fmt/mod.rs:449 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/layered.rs:78 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/layered.rs:214 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/layered.rs:240 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/layered.rs:344 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/layered.rs:376 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/layered.rs:380 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/layered.rs:381 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1245 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1529 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1547 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1665 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1672 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1755 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1756 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1868 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1883 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/layer/mod.rs:1891 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/reload.rs:188 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/src/reload.rs:200 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/tests/layer_filters/downcast_raw.rs:48 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/tracing-subscriber-0.3.23/tests/registry_span_instrument_drop.rs:77 — unsafe — tracing-subscriber@0.3.23 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/utf8parse-0.2.2/src/lib.rs:70 — unsafe — utf8parse@0.2.2 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/as_bytes_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/as_bytes_static_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/extend_vec_zeroed.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/insert_vec_zeroed.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/new_box_zeroed.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/new_box_zeroed_with_elems_dynamic_padding.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/new_box_zeroed_with_elems_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/new_vec_zeroed.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/new_zeroed.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/read_from_bytes.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/read_from_prefix.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/read_from_suffix.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_bytes_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_bytes_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_bytes_static_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_bytes_with_elems_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_bytes_with_elems_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_prefix_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_prefix_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_prefix_static_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_prefix_with_elems_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_prefix_with_elems_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_suffix_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_suffix_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_suffix_static_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_suffix_with_elems_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_from_suffix_with_elems_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_bytes_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_bytes_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_bytes_mut_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_bytes_mut_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_bytes_mut_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_bytes_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_deref_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_deref_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_deref_mut_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_deref_mut_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_deref_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_bytes_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_bytes_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_bytes_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_bytes_with_elems_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_bytes_with_elems_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_prefix_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_prefix_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_prefix_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_prefix_with_elems_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_prefix_with_elems_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_suffix_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_suffix_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_suffix_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_suffix_with_elems_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_from_suffix_with_elems_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_into_mut_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_into_mut_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_into_ref_dynamic_padding.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_into_ref_dynamic_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_into_ref_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_read_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/ref_type_write_static_size.rs:14 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_dynamic_padding.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_unchecked_dynamic_padding.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_unchecked_dynamic_padding.rs:7 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_unchecked_dynamic_padding.rs:11 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_unchecked_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_unchecked_dynamic_size.rs:7 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_at_unchecked_dynamic_size.rs:11 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_immutable_dynamic_padding.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_immutable_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_runtime_check_dynamic_padding.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_runtime_check_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_unchecked_dynamic_padding.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_unchecked_dynamic_padding.rs:7 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_unchecked_dynamic_padding.rs:10 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_unchecked_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_unchecked_dynamic_size.rs:7 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/split_via_unchecked_dynamic_size.rs:10 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/transmute.rs:13 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/transmute_ref_dynamic_size.rs:13 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/transmute_ref_static_size.rs:12 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_read_from_bytes.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_read_from_prefix.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_read_from_suffix.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_bytes_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_bytes_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_bytes_static_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_bytes_with_elems_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_bytes_with_elems_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_prefix_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_prefix_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_prefix_static_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_prefix_with_elems_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_prefix_with_elems_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_suffix_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_suffix_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_suffix_static_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_suffix_with_elems_dynamic_padding.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_ref_from_suffix_with_elems_dynamic_size.rs:4 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_transmute.rs:13 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_transmute_ref_dynamic_size.rs:13 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/try_transmute_ref_static_size.rs:12 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/write_to_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/write_to_prefix_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/write_to_prefix_static_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/write_to_static_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/write_to_suffix_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/write_to_suffix_static_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/zero_dynamic_padding.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/zero_dynamic_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/benches/zero_static_size.rs:6 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:52 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:73 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:84 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:101 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:120 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:141 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:159 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:184 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:200 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:204 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:208 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:212 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:214 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:218 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:225 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:238 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:243 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:245 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:254 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:263 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:294 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:301 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:313 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:326 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:332 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:334 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:338 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:346 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:352 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:354 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:358 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:370 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:379 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:380 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:388 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:394 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:395 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:403 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:419 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byte_slice.rs:430 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/byteorder.rs:540 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/error.rs:263 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/error.rs:365 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/error.rs:377 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/error.rs:658 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/error.rs:797 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:31 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:67 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:101 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:112 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:131 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:141 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:165 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:174 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:226 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:281 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:303 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:332 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:381 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:429 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:432 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:447 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:449 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:487 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:511 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:542 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:569 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:596 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:629 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:638 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:653 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:679 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:683 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:688 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:711 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:715 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:735 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:748 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:761 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:799 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:837 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:849 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:853 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:892 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:965 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:976 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:980 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:990 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1029 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1082 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1104 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1109 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1136 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1163 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1198 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1320 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1419 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1482 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1802 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/impls.rs:1813 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1112 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1813 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1829 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1833 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1851 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1871 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1886 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1900 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1909 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:1920 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:2171 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:2228 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:2269 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:3653 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:3659 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:3725 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:3774 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4151 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4175 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4187 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4216 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4244 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4245 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4246 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4294 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4309 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4310 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4320 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4361 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4363 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4381 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4416 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4428 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4457 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4489 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4493 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4496 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4497 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4523 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4526 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4540 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4543 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4545 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4558 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4561 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4563 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4576 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4619 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4622 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4623 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4624 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4643 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4663 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4667 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4674 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4679 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4689 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4697 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4703 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4708 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4714 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4733 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4734 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4738 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4767 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4777 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4780 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4783 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4864 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4871 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4915 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4916 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4919 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4963 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:4998 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5001 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5032 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5047 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5053 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5057 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5063 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5082 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5086 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5128 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5138 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5144 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5148 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/layout.rs:5154 — kani::assume — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:746 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1014 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1198 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1218 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1257 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1310 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1353 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1663 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:1899 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3440 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3564 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3584 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3603 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3620 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3702 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3782 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3822 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3877 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3882 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3888 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:3942 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:4052 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:4288 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:5452 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:5510 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:5562 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:5616 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:5621 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:5935 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:6024 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:6100 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:6181 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:6269 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:6375 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:6575 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:6741 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/lib.rs:7654 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:124 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:133 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:156 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:184 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:401 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:641 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1025 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1054 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1070 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1087 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1114 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1175 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1199 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1210 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1223 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/macros.rs:1245 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:98 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:152 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:178 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:201 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:213 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:229 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:242 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:272 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:298 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:309 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:314 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:351 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:356 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:377 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:390 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:403 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:408 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:415 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:431 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:475 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:483 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:497 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:517 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:533 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:588 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:600 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:623 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:640 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:662 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:664 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:673 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:683 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:720 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/inner.rs:733 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:100 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:183 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:217 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:228 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:237 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:252 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:255 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/invariant.rs:257 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:57 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:80 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:87 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:97 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:105 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:108 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:125 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:134 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:152 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:161 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:165 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:185 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:214 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:226 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:249 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:293 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:319 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:336 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:349 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:370 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:380 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:388 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:391 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:399 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/mod.rs:407 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:92 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:193 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:218 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:242 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:276 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:328 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:374 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:392 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:424 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:476 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:494 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:522 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:559 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:597 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:651 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:655 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:669 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:685 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:696 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:701 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:720 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:734 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:746 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:751 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:762 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:767 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:778 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:781 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:817 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:828 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:862 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:882 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:902 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:934 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:952 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:957 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:968 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1009 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1028 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1039 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1101 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1157 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1178 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1190 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1217 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1257 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1280 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1291 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1336 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1426 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1430 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1451 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1471 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/ptr.rs:1497 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:114 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:153 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:172 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:194 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:200 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:216 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:219 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:223 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:238 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:240 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:258 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:260 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:271 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:284 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:314 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:342 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:352 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:366 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:380 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:397 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:403 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:411 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:436 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:444 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:458 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:480 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/pointer/transmute.rs:489 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:87 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:114 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:155 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:181 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:207 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:233 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:291 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:321 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:348 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:448 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:534 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:564 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:675 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:775 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:915 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:965 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:972 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:977 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1033 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1040 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1052 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1105 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1146 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1171 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1177 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1201 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1208 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1252 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1259 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1264 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1316 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1323 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1335 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1423 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1440 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1452 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1864 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/ref.rs:1893 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:97 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:136 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:138 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:204 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:221 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:223 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:281 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:289 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:312 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:357 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:371 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:680 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:684 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:698 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:919 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:923 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:940 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:950 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:958 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:970 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:983 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1017 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1038 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1051 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1057 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1074 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1092 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1360 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1362 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1368 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/split_at.rs:1370 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:48 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:122 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:158 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:194 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:213 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:585 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:645 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:695 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:709 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:761 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:782 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:799 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:808 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:815 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:864 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:879 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:885 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:889 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:924 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macro_util.rs:957 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:30 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:128 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:197 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:317 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:329 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:446 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:502 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:522 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:719 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:726 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:734 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:755 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:757 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:760 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:763 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:766 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:769 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:831 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/macros.rs:845 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:36 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:39 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:137 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:283 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:295 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:306 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:334 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:340 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:350 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:357 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:367 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:387 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:389 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:416 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:426 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:444 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:457 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:468 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:543 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:553 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:632 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:646 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:693 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:712 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:719 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:726 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:738 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:744 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:754 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:760 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:770 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/util/mod.rs:776 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:149 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:190 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:239 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:249 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:262 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:268 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:351 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:366 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:370 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:387 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:528 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:556 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:576 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:586 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:592 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:659 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:667 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:679 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:692 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:700 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:702 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:704 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:706 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:719 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:723 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:754 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:765 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:843 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:889 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:917 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:1055 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:1057 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:1090 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:1177 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:1186 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- /out/cargo-home/registry/src/index.crates.io-1949cf8c6b5b557f/zerocopy-0.8.63/src/wrappers.rs:1196 — unsafe — zerocopy@0.8.63 — SECURITY-RELEVANT
- benches/client_stack.rs:35 — unsafe — bertie::benches::client_stack::protocol, bertie::benches::client_stack::main — SECURITY-RELEVANT
- benches/client_stack.rs:128 — unsafe — bertie::benches::client_stack::protocol — SECURITY-RELEVANT
- benches/client_stack.rs:168 — unsafe — bertie::benches::client_stack::protocol — SECURITY-RELEVANT

## Explicit overrides

None.

## Upstream claims

None matched.

## Upstream claims, uncredited

ProVerif and SSProve artifacts model the protocol/key schedule; this run gives them no Rust line credit.

- Upstream model (uncredited): proofs/proverif/handwritten_lib.pvl — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/proverif/extraction/lib.pvl — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/proverif/extraction/analysis.pv — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/BasePackages.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Utility.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/CoreTheorem.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/KeySchedulePackages.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Core.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/BertieResult.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Types.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Dependencies.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/ssp_helper.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/ExtraTypes.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/XTR_XPD.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/KeyPackages.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13crypto.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Fixes.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13utils.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13formats.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13keyscheduler_Key_schedule.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof

## Run receipts

- hax-fstar `/out/work/upstream/typecheck-extraction.log`: rejected. Credited units: . F* receipt rejected: unsupported or failed extraction
  Scope: 0 files; cfg: . Proven, relying on 0 trusted items (0 SLoC) and 0 escapes in this run. Trusted sites: . Escape sites: .
- hax-fstar `/out/work/upstream/typecheck-panic-free.log`: rejected. Credited units: . F* receipt rejected: unsupported or failed extraction
  Scope: 0 files; cfg: . Proven, relying on 0 trusted items (0 SLoC) and 0 escapes in this run. Trusted sites: . Escape sites: .

## Evidence and limitations

- No entry points: all first-party functions conservatively security-relevant.
- Upstream model (uncredited): proofs/proverif/handwritten_lib.pvl — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/proverif/extraction/lib.pvl — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/proverif/extraction/analysis.pv — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/BasePackages.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Utility.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/CoreTheorem.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/KeySchedulePackages.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Core.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/BertieResult.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Types.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/Dependencies.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/ssp_helper.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/ExtraTypes.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/XTR_XPD.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/handwritten/KeyPackages.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13crypto.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Fixes.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13utils.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13formats.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
- Upstream model (uncredited): proofs/ssprove/fextraction/Bertie_Tls13keyscheduler_Key_schedule.v — protocol/key-schedule model; not checked by this run and not line-by-line Rust proof
