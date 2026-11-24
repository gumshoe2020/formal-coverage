# v1.8.4 paired replay

Baseline: tag `v1.8.3`; candidate: HEAD (v1.8.4).
Bertie: archived `c66a986`, copied from `target/v183/bertie` into `target/v184/bertie`.
Original CI log/receipt and all receipt-bound source, extraction and library hashes verified.
Both binaries built and run on current stable (rustc 1.100.0 (0ef1bc386 2026-11-09)), with RUSTUP_TOOLCHAIN unset.
Bertie target: aarch64-apple-darwin (host); compiled-files, default features. Host dep-info exercises the shipped build; the original Linux CI first-party baseline is checked separately.
Controls retain v183 targets, evidence and whole-crate settings. All replay mutations stay under target/v184; no external checkout is accessed.

Shipped selection (temporarily appended to the archived config; restored after reporting):

```toml
[ship]
packages = ["bertie-tls"]
features = ["default"]
reason = "Ship the bertie-tls library with default features"
```

## Controls without [ship]

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

## Original CI and accepted v1.8.3 baseline

Original CI: 3635 proven-extracted, 452 tested, 1197 unverified, 118 trusted / 5402 security SLoC; 67.29% proven-extracted.
Rebuilt tag v1.8.3: 3776 proven-extracted, 442 tested, 1066 unverified, 118 trusted / 5402 security SLoC; 69.90% proven-extracted.
Original CI report predates the v1.8.3 R1 fix. The rebuilt tag matches the accepted v183 replay first-party totals and every unit tier: PASS. Original CI receipt and source hashes are preserved; no old CI totals are relabelled as v1.8.3.

## Bertie, separate

Without [ship]: exact JSON + Markdown equality PASS; accepted CI evidence PASS.
Before: 3776 proven-extracted, 442 tested, 1066 unverified, 118 trusted / 5402 security SLoC; 69.90% proven-extracted.
With [ship]: 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078 security SLoC; 74.36% proven-extracted.
Retained-units check PASS: all 448 entries identical in every JSON field (all fields, including tier, SLoC, proof and trust metadata).
Dropped would-be units check PASS; dependency inventories and third-party totals unchanged PASS; security arithmetic PASS.
Build/target/configuration metadata, extraction/proof inputs and upstream claims unchanged PASS.
Dependency size fallbacks: 2, identical before/after/ship.
Removed security runtime: {'tested': 21, 'unverified': 303} (324 SLoC).

| Dropped file | Whole-file SLoC | Security runtime SLoC by would-be tier | Other units: SLoC by category/tier |
| --- | ---: | --- | --- |
| `benches/client.rs` | 183 | 0 | test/unverified: 162 |
| `benches/client_stack.rs` | 144 | 0 | test/hazard: 94 |
| `benches/server.rs` | 135 | 0 | test/unverified: 117 |
| `bogo_shim/src/main.rs` | 221 | tested: 3, unverified: 132 | 0 |
| `integration_tests/src/lib.rs` | 0 | 0 | 0 |
| `integration_tests/tests/self_test.rs` | 63 | 0 | test/unverified: 54 |
| `record/src/debug.rs` | 43 | unverified: 37 | 0 |
| `record/src/lib.rs` | 23 | tested: 6 | 0 |
| `record/src/stream.rs` | 89 | tested: 6, unverified: 58 | 0 |
| `simple_https_client/src/tls13client.rs` | 51 | unverified: 34 | 0 |
| `simple_https_client/tests/test_adhoc.rs` | 52 | 0 | test/unverified: 38 |
| `simple_https_client/tests/test_tls13api.rs` | 188 | 0 | test/unverified: 107 |
| `simple_https_server/src/tls13server.rs` | 52 | unverified: 33 | 0 |
| `src/test_tls13traces_internal.rs` | 642 | 0 | test/unverified: 415 |
| `src/test_utils.rs` | 25 | tested: 6, unverified: 9 | 0 |
| `tests/test_tls13api.rs` | 396 | 0 | test/unverified: 305 |

Whole-file SLoC includes non-runtime source; unit SLoC is the coverage measure. Empty files remain visible. The ~324-SLoC estimate is a guide, not an assertion.

## Bertie, runtime

Without [ship]: exact JSON + Markdown equality PASS; accepted CI evidence PASS.
Before: 4091 proven-extracted, 468 tested, 1095 unverified, 118 trusted / 5772 security SLoC; 70.88% proven-extracted.
With [ship]: 4091 proven-extracted, 447 tested, 792 unverified, 118 trusted / 5448 security SLoC; 75.09% proven-extracted.
Retained-units check PASS: all 448 entries identical in every JSON field (all fields, including tier, SLoC, proof and trust metadata).
Dropped would-be units check PASS; dependency inventories and third-party totals unchanged PASS; security arithmetic PASS.
Build/target/configuration metadata, extraction/proof inputs and upstream claims unchanged PASS.
Dependency size fallbacks: 2, identical before/after/ship.
Removed security runtime: {'tested': 21, 'unverified': 303} (324 SLoC).

| Dropped file | Whole-file SLoC | Security runtime SLoC by would-be tier | Other units: SLoC by category/tier |
| --- | ---: | --- | --- |
| `benches/client.rs` | 183 | 0 | test/unverified: 162 |
| `benches/client_stack.rs` | 144 | 0 | test/hazard: 94 |
| `benches/server.rs` | 135 | 0 | test/unverified: 117 |
| `bogo_shim/src/main.rs` | 221 | tested: 3, unverified: 132 | 0 |
| `integration_tests/src/lib.rs` | 0 | 0 | 0 |
| `integration_tests/tests/self_test.rs` | 63 | 0 | test/unverified: 54 |
| `record/src/debug.rs` | 43 | unverified: 37 | 0 |
| `record/src/lib.rs` | 23 | tested: 6 | 0 |
| `record/src/stream.rs` | 89 | tested: 6, unverified: 58 | 0 |
| `simple_https_client/src/tls13client.rs` | 51 | unverified: 34 | 0 |
| `simple_https_client/tests/test_adhoc.rs` | 52 | 0 | test/unverified: 38 |
| `simple_https_client/tests/test_tls13api.rs` | 188 | 0 | test/unverified: 107 |
| `simple_https_server/src/tls13server.rs` | 52 | unverified: 33 | 0 |
| `src/test_tls13traces_internal.rs` | 642 | 0 | test/unverified: 415 |
| `src/test_utils.rs` | 25 | tested: 6, unverified: 9 | 0 |
| `tests/test_tls13api.rs` | 396 | 0 | test/unverified: 305 |

Whole-file SLoC includes non-runtime source; unit SLoC is the coverage measure. Empty files remain visible. The ~324-SLoC estimate is a guide, not an assertion.

## Artifacts

- `target/v184/bertie-runtime-after.json` SHA-256 `283c30bbd2d8967d943e5e74690287728f0c3a9cfdefac6cc0df3412e7b608f7`
- `target/v184/bertie-runtime-before.json` SHA-256 `283c30bbd2d8967d943e5e74690287728f0c3a9cfdefac6cc0df3412e7b608f7`
- `target/v184/bertie-runtime-shipped.json` SHA-256 `4a6faf73efadc6d47fd4247998cd745c2a5d04d51e26a3bca6236dbaca42dddd`
- `target/v184/bertie-separate-after.json` SHA-256 `b670afc011099525a9877de5c26a7216cf22cefcea7ec187139878de978c399b`
- `target/v184/bertie-separate-before.json` SHA-256 `b670afc011099525a9877de5c26a7216cf22cefcea7ec187139878de978c399b`
- `target/v184/bertie-separate-shipped.json` SHA-256 `95372d5422b36dbd546d798a0b11eba45e5221eb613e71686fed76090ec9665b`

## Rerun

```sh
python3 dev/v184/replay.py
```

Recreates target/v184, rebuilds both binaries and regenerates this comparison. Requires the existing v183 archived Bertie/CI inputs, release evidence archives and populated offline Cargo caches. Original CI evidence is replayed; no new F*/Verus verification is claimed. Checks are Codex-run; independent acceptance remains Andrew/Dudley’s.
