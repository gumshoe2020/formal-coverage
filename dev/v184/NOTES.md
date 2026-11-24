# v1.8.4 M0 notes

## 2026-11-24T18:18:10Z — scope and implementation

Confirmed main, rustc 1.100.0, cargo 1.100.0, default stable-aarch64-apple-darwin; RUSTUP_TOOLCHAIN is unset. The existing untracked dev/v184/m0-run.log predates this work and will remain untouched and uncommitted.

Use the existing glob dependency to search the fixture target directory recursively for libother-*.rmeta, strip lib from the artifact stem, and derive the adjacent .d path exactly as src/size.rs does. Retain the assertion that exactly one dep-info file is selected and every fallback, dep isolation, SLoC, and banner assertion. This avoids assuming a Cargo target/profile layout and adds no dependency.

Interpret “test code only” together with the explicit requirement to keep PROGRESS.md and NOTES.md: implementation changes are confined to the named test; these two audit files are also included. No src, other tests, toolchain configuration, version, release docs, or M1/M2 changes. Memory search/diary tools are unavailable in this session; these files provide the local audit record.

## 2026-11-24T18:20:13Z — verification and handoff

All four requested acceptance commands returned exit 0 with RUSTUP_TOOLCHAIN unset on default stable 1.100.0. The CI gate ran fmt, all-targets/all-features clippy, all-targets build, and tests. The existing release performance test remains ignored by its pre-existing definition; no ignores were added. The focused regression passed separately. Every original assertion in the affected test remains intact. Independent Andrew/Dudley acceptance has not been run by Codex.

Commit only tests/size_modes.rs and the two M0 audit files on main using the normal pre-commit hook; do not push. No M1/M2 work or toolchain uninstall was performed.

## 2026-11-24T18:35:09Z — M1 scope and implementation decisions

Actual starting HEAD is 24e3752 (M1 prompt), on main; c944fd4 is the requested comparison baseline. Existing untracked m0-run.log and m1-run.log are untouched. No README/version/M2 or bertie replay changes. Memory search/diary tools are unavailable; this is the local audit record.

No config/report schema bump: `[ship]` is optional in config schema 3. New report `ship` and `not_shipped` fields are omitted when absent/empty, preserving existing serialized output. No existing test expectations need changes. Features are an explicit set: `default` keeps Cargo defaults, otherwise use `--no-default-features`; non-default entries pass to `--features`. Metadata uses package-qualified features for the normal graph. Dependency reporting and dependency sizing retain their original workspace roots and build options.

Binary-only roots omit `--lib` (Cargo rejects that flag without a library); all roots use `--bins`, and library roots use `--lib` too. Union files across targets/roots. Separate default target directory `target/formal-coverage-ship` avoids touching dependency check evidence; explicit check_target_dir still applies. Reuse the existing Make dep-info parser and factor the existing artifact-to-.d derivation into one shared helper.

Not-shipped inventory records each source file's whole-file SLoC and its units' would-be security/category/status tier. Scope is applied after dependency decisions/classification but before coverage sums and proof configuration/evidence. Removing units and source_files together excludes all runtime/proof/test totals and prevents file-scope escape scans from reintroducing dropped code. No Verus/Kani/F*/hax validator rules changed. Missing dep-info retains that reached package; a failed root check conservatively retains the entire first-party inventory because absence from the complete shipped build is unconfirmed. Graph failure also retains all packages with named warnings.

The example scratch-server directory contains only a config, not a workspace. The initial literal directory CLI attempt failed on its entry point (exit 1). For acceptance use the existing repository-local archived workspace target/v1.5-scratch/spec-lemmas with the unchanged examples/scratch-server/formal-coverage.toml via --config, identically before/after. Detached c944fd4 baseline worktree is /tmp/formal-coverage-v184-before, as explicitly requested. No other repository checkout is touched.

## 2026-11-24T18:35:09Z — M1 regression coverage and final scope audit

Four new fixture tests pass (exit 0). They cover a lib plus same-package bin, an unrelated binary-only member, normal/build/dev workspace edges, an optional workspace dependency, non-default and default feature modules, JSON/markdown visibility, required/empty reasons, unknown packages, missing dep-info for one target of a multi-target package, failed cargo check, exact security/non-security totals, unchanged external dependency/build inventories, and a synthetic Verus receipt proving excluded runtime/proof units. Synthetic evidence tests accounting policy, not mathematical validity. All existing tests remain unmodified; no ignored tests were added.

The initial fixture run exposed lib-prefixed rmeta filenames for binary checks too: keep the exact existing strip-lib derivation for both. An initial clippy type-complexity error was fixed with a named ShipPackage struct, without lint suppression. Missing/unreadable dep-info retains its error across root unions and warnings include the actual path/error. Dropped runtime units also leave input-movement SLoC totals. Unknown hax path inventory is recomputed from retained source files without changing path recognition or evidence rules.

For the requested would-be tier, Verus/Kani and F* entry points reuse their unchanged evidence application on a detached full inventory solely to update not_shipped.units. Its totals, receipts, dependencies, warnings and trust/escape inventories are discarded. Shipped evidence processing still sees only retained units/source files. The synthetic proof test verifies the excluded file is displayed as proven while runtime proven numerator and proof-cost totals remain zero. This adds work only when files are excluded; no-[ship] takes the original path.

Final explicit test/clippy/fmt commands each returned exit 0 on rustc/cargo 1.100.0 with RUSTUP_TOOLCHAIN unset. The full CI quality gate returned exit 0 (all checks passed, 65.394 seconds). These are Codex-run checks; independent acceptance by Andrew/Dudley remains outstanding.

## 2026-11-24T18:36:00Z — M1 handoff

Implementation and four additive regression tests committed on main as 332fc8a; its normal pre-commit hook ran fmt and all-targets/all-features clippy and passed (exit 0). Audit notes follow in a separate commit. No push, README/version edits, bertie replay, toolchain pin/override, hook bypass, or new ignores. The existing untracked m0-run.log and m1-run.log remain untouched; no other uncommitted changes are intended. The detached baseline worktree and raw comparison/check logs remain under /tmp for inspection. All requested Codex-run gates and both raw byte comparisons passed; independent Andrew/Dudley acceptance was not performed.

## 2026-11-24T18:48:16.725195+00:00 — M2 scope and replay decisions

M2 only, no src changes. Existing untracked m0/m1/m2-run.log files remain untouched. Memory tools are unavailable; local notes preserve the audit. Replay exports baseline tag v1.8.3 and copies existing v183 Bertie/CI inputs into target/v184 (excluding build artifacts/.git). Receipt-bound input hashes are checked before reporting. Control archives use v15_replay helpers with destinations redirected into target/v184; no previous replay trees are mutated.

Use host aarch64-apple-darwin for both Bertie binaries and ship/no-ship reports: only host std is installed, and actual successful cargo-check dep-info is required to exercise ship (a foreign target would conservatively retain everything). Verify the original CI first-party inventory/totals independently; dependency counts may differ from the old Linux-target replay, but must match within this paired replay and with/without ship. Both ghost modes are exercised. No toolchain overrides.

## 2026-11-24T18:55:18.173821+00:00 — original CI versus tag baseline

Inspection verifies original CI totals are 3635 proven-extracted / 5402, but target/v183/bertie-separate-after.json and dev/v183/compare.md record the accepted R1 result: 3776 proven-extracted, 442 tested, 1066 unverified, 118 trusted / 5402 (69.90%). The prompt labels the old CI figure as v1.8.3; its Expected section also anticipates the post-fix share around 74%. Honor the requested tag baseline, verify it against the accepted v183 replay, and preserve/report the old CI totals separately. This is an input-label discrepancy, not a replay behavior difference.

## 2026-11-24T18:55:18.173951+00:00 — M2 gates and diagnostic preview

Test, clippy, fmt and full CI commands returned 0 on stable 1.100.0; local target/debug binary prints 1.8.4 (exit 0). README's TOML examples all parse; ship examples also deserialize through the public Ship type. Named README keys/report headings were checked against src/inputs.rs, src/lib.rs and src/report.rs. No existing statement in README asserted that all workspace members always count; the new section explicitly distinguishes optional ship scope from the unchanged default.

A concurrent Bertie diagnostic preview verified separate-mode exact no-ship JSON/Markdown equality and the rebuilt tag's 3776/5402 baseline. It was terminated after that pair (preview process exit 143) to avoid later overlapping writes to the same Bertie config/output paths; this is not an acceptance run or a behavior finding. The full from-scratch replay remains running sequentially. During setup, the candidate was rebuilt to 1.8.4 and replaced before its first report invocation; the replay script now builds that version directly.

## 2026-11-24T19:02:51.518563+00:00 — final clean replay

Initial sequential run verified all 16 control pairs. Stopped it during the no-ship Bertie stage (exit 143; no behavior finding), verified the archived config is unchanged, and restart the finalized script from an empty target/v184. This removes ambiguity from in-flight script/version edits. The finalized runner executes the independent controls and Bertie groups in two threads after preparing inputs/builds; each group owns distinct source trees, output files and Cargo target directories, with no shared config writes. Both modes and before/after within each group remain sequential.

Documentation/version/test commit bf0a590 passed normal pre-commit fmt and all-target/all-feature clippy (exit 0). No push or tag.

## 2026-11-24T19:12:45.206980+00:00 — STOP: retained tested tiers change with ship

M2 acceptance 1c fails. The separate-mode ship report drops the expected 324 security runtime SLoC (303 unverified, 21 tested, zero proven), but 7 retained runtime units (81 SLoC) change tested → unverified. All 448 retained entries keep their sizes and other fields. Observed totals: 3776 proven-extracted, 340 tested, 844 unverified, 118 trusted / 5078 (74.36%). Pure exclusion would instead yield 421 tested and 763 unverified. No ship fallback warnings; dependencies and dropped would-be tiers are unchanged. Both ghost modes pass Bertie no-ship byte equality.

Stopped all replay subprocesses immediately after finding the mismatch; final clean command exit 143 (intentional stop), direct retained-unit assertion exit 1. Shipped runtime output is incomplete. Initial sequential controls all passed, but final clean controls were interrupted during Vest. No final success compare was generated; dev/v184/compare.md is an explicit failure report with all dropped files, all 7 changed retained units and raw artifact hashes. Config in target/v184/bertie was restored to its original no-ship bytes after termination.

Read-only diagnosis: ship scope is applied in src/classify.rs:300 before evidence; src/ship.rs removes excluded source_files/units; src/evidence.rs:382 collects test identifiers from the reduced inventory and :425–429 sets tested tiers. Excluded tests can therefore stop conferring tested status to retained runtime code. This is a likely cause, not a repaired or independently accepted result. Per the explicit M2 stop rule, no src edits or further acceptance/release work. README/version/test changes are already committed as bf0a590. M2 remains incomplete and must not be released.

## 2026-11-24T19:15:40.539304+00:00 — M1b evidence scope repair

Starting HEAD ab986e5 on main (prompt recorded 8cce556). Existing untracked m0/m1/m1b/m2-run.log files are preserved. Full-inventory evidence is authoritative for retained and excluded units; project the result back to shipped coverage. Receipt-local proof scope remains complete; file-level trust/escape totals exclude not-shipped source. Reuse the existing full-inventory reconstruction, remove duplicate scoped evaluation, and apply this to both Verus/Kani/test and F* evidence. No-[ship] keeps the direct original path. README states the counted/evidence distinction. Memory search/diary tools are unavailable; audit stays in this repo.

## 2026-11-24T19:17:02.809709+00:00 — preserve prior M2 failure detail

The previous compare.md failure table is retained here before the clean replay regenerates the success report. These are the pre-M1b observations, not new results.

| Retained unit | Location | SLoC | Before → shipped |
| --- | --- | ---: | --- |
| `bertie_tls::stream::client::<Stream:Read+Write>BertieStream<ClientState<Stream>>::open_with_stream` | `src/stream/client.rs:81` | 11 | tested → unverified |
| `bertie_tls::stream::server::BertieStream<ServerState<TcpStream>>::connect` | `src/stream/server.rs:189` | 41 | tested → unverified |
| `bertie_tls::tls13crypto::Algorithms::aead` | `src/tls13crypto.rs:641` | 3 | tested → unverified |
| `bertie_tls::tls13crypto::Algorithms::signature` | `src/tls13crypto.rs:646` | 3 | tested → unverified |
| `bertie_tls::tls13crypto::Algorithms::zero_rtt` | `src/tls13crypto.rs:661` | 3 | tested → unverified |
| `bertie_tls::tls13crypto::Algorithms::ciphersuite` | `src/tls13crypto.rs:666` | 14 | tested → unverified |
| `bertie_tls::tls13utils::non_hax::random_bytes` | `src/tls13utils.rs:404` | 6 | tested → unverified |

Observed totals: 3776 proven-extracted, 340 tested, 844 unverified, 118 trusted / 5078 security SLoC (74.36% proven-extracted).
Dropping only the excluded runtime units would give 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078. The observed extra 81 tested → unverified SLoC is the failure; the plausible proven percentage does not make this acceptable.

## 2026-11-24T19:19:32.771563+00:00 — regression verification

Seven ship tests pass, including member integration tests, cfg(test) files, full retained-unit JSON identity in both ghost modes, and a synthetic Kani receipt whose excluded harness proves a retained runtime function. The member fixture declares its app dev-dependency; its actual workspace cargo test --workspace --offline also returns 0. The Kani test checks receipt policy, not a new verifier execution.

Exported pre-fix HEAD into target/v184-regression-before, copied only the additive tests, and ran the two not_shipped_ regressions there: expected cargo exit 101, both tests fail. The validation wrapper returns 0 after checking those exact failures. Log: target/v184-m1b-regression-before.log. No existing test assertion was edited or weakened. All mutations are inside this repo.

## 2026-11-24T19:38:04.337185+00:00 — M1b committed; M2 replay continuing

Repair, three additive regressions, and README sentence committed on main as 12f83c3. Normal pre-commit fmt and all-target/all-feature clippy passed (exit 0). Final full tests, requested clippy/fmt, CI gate, and version commands all return 0; target/debug prints formal-coverage 1.8.4. No toolchain pin/override, new ignore, hook bypass, push or tag.

Clean replay started by replay.py recreating target/v184; it builds the same repaired source as 12f83c3. Test-only additions after the candidate build do not change the replay binary. Separate Bertie no-ship JSON and Markdown match v1.8.3 byte-for-byte. Both independent groups are still running; no replay success is claimed yet. A read-only process sample (target/v184-m1b-runtime-sample.txt) shows baseline F* receipt validation active, not a discovered retained-unit failure.

## 2026-11-24T19:57:55.794239+00:00 — repaired Bertie separate result

All shipped separate retained units equal the no-ship report in every JSON field; the seven prior tested-to-unverified changes are gone. Actual first-party security totals: 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078 (74.36%). Both Bertie no-ship modes are byte-identical in JSON and Markdown to v1.8.3. Full replay is still pending. Apparent long elapsed pauses with little CPU time suggest host sleep; caffeinate -i -w <replay-pid> now holds a temporary idle-sleep assertion until this replay ends, without changing files or permanent settings.

## 2026-11-24T20:02:01.767558+00:00 — M1b repair and M2 replay complete

Clean python3 dev/v184/replay.py returns exit 0. All 16 controls pass byte-exact JSON/Markdown equality; both Bertie no-ship modes match v1.8.3, and both shipped modes preserve every field of all 448 retained units. Dropped would-be units, build/config/proof metadata, dependency inventories, third-party totals and coverage arithmetic all pass the unchanged strict comparator. No tier normalization or comparator edits.

Verified separate shipped totals: 3776 proven-extracted, 421 tested, 763 unverified, 118 trusted / 5078 (74.36%). Runtime shipped: 4091 proven-extracted, 447 tested, 792 unverified, 118 trusted / 5448 (75.09%). Each mode removes exactly 324 runtime SLoC (303 unverified, 21 tested); 16 excluded files remain visible. Two pre-existing dependency sizing fallbacks are identical before/after/ship; there are no ship fallback warnings. The archived Bertie config is restored byte-for-byte.

compare.md is regenerated as a success report with every dropped file and artifact hashes; the previous seven-unit failure and table remain above in this file. PROGRESS.md records all six requested acceptance command exits as 0 on current default stable 1.100.0, RUSTUP_TOOLCHAIN unset, version 1.8.4. Source has not changed since repair commit 12f83c3. Final audit/report commit uses the normal hook on main; no push or tag. Existing untracked run logs remain outside commits. Independent acceptance remains Andrew/Dudley’s; existing verifier receipts were replayed, not newly executed.
