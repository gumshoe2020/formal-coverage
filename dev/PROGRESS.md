# Progress

## v1.2a

2026-10-28: Require an explicit combined or first-party security threshold for
`gate` (either or both), preserving completeness/baseline/hazard checks. CLI
regressions cover neither flag (CLI error, exit 2, no report), each alone, and
both, including explicit zero and enforcement of each positive threshold.
README gate text updated.

Local verification: `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo test
--frozen` and `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo clippy
--frozen -- -D warnings` both exited 0. Output retained in
`logs/v1.2a-test.log` and `logs/v1.2a-clippy.log`; only `tail -n 30` read.
Formatting and `git diff --check` also passed. One local commit requested;
no push. Independent acceptance has not been performed.

- [x] M1 inventory — `CARGO_HOME="$PWD/.cargo-home" cargo test` passed (2 tests).
- [x] M2 security classification — `CARGO_HOME="$PWD/.cargo-home" cargo test` passed (3 tests, exact expected split).
- [x] M3 evidence and status — `CARGO_HOME="$PWD/.cargo-home" cargo test` passed (8 tests, including six escape variants and negative evidence cases).
- [x] M4 reports and gate — `CARGO_HOME="$PWD/.cargo-home" cargo test` (12 tests) and `CARGO_HOME="$PWD/.cargo-home" cargo clippy -- -D warnings` passed.
- [x] M5 scratch-server dogfood — `python3 scripts/dogfood.py --directory "$(cat logs/dogfood-export-path)"` ran all seven exported branches (five verifier runs, all 18/0); `python3 scripts/check_dogfood.py` passed for all seven artifacts and archive source identities.

Final revised-spec revalidation: `CARGO_HOME="$PWD/.cargo-home" cargo test` (20 tests), `CARGO_HOME="$PWD/.cargo-home" cargo clippy -- -D warnings`, `CARGO_HOME="$PWD/.cargo-home" cargo build`, `CARGO_HOME="$PWD/.cargo-home" cargo fmt --check`, `git diff --check`, and `python3 scripts/check_dogfood.py` passed. Detailed output: ignored `logs/m5-test.log`, `logs/m5-clippy.log`, `logs/m5-build.log`, `logs/fmt-check.log`, `logs/m5-acceptance.log`.

These are local agent-run acceptance checks. Independent human acceptance has not been performed. Kani/F* and scratch-server application/container tests were not run.

## v1.1-a

- [x] Syntax classifier, proof-code totals and global hazard regression — `CARGO_HOME="$PWD/.cargo-home" cargo test` passed (`logs/v1.1-a-test.log`).
- [x] Schema 2 baseline refusal and explicit schema-change acknowledgment — `CARGO_HOME="$PWD/.cargo-home" cargo test` passed, including CLI exit checks.
- [x] All 13 scratch-server branches remeasured — `python3 scripts/dogfood.py --directory "$(cat logs/dogfood-export-path)"` passed (`logs/v1.1-a-dogfood.log`); 11 fresh passing verifier scripts, plus separate parser runs for 6 multi-root stages.
- [x] Stage 12/13 invariant — `python3 scripts/check_dogfood.py` passed (`logs/v1.1-a-acceptance.log`): both first-party 453/1363 (33.24%); combined 453/1626171 (0.02785685%). Proof-code SLoC: 322 vs 4732.
- [x] `CARGO_HOME="$PWD/.cargo-home" cargo test` (23 tests), `cargo clippy -- -D warnings`, `cargo fmt --check`, `python3 scripts/check_dogfood.py`, `python3 -m py_compile scripts/dogfood.py scripts/check_dogfood.py` and `git diff --check` passed. Logs: `logs/v1.1-a-{test,clippy,fmt,acceptance}.log`. Pinned v1 build helper and same-commit export reuse check also passed.

Independent acceptance remains outstanding; ticks record only commands run locally.

## v1.1-b

- [x] Multi-receipt union, module resolution and conservative/hazard regressions — `CARGO_HOME="$PWD/.cargo-home" cargo test` passed (31 tests, `logs/v1.1-b-test.log`), including repeated Verus/Kani CLI options and module hazard gate exits.
- [x] All 13 scratch-server stage branches measured with all roots in one report call — `python3 scripts/dogfood.py --directory "$(cat logs/dogfood-export-path)"` passed (`logs/v1.1-b-dogfood.log`): 11 fresh verifier scripts and 23 separate root runs.
- [x] Stage 12/13 invariant, 644/1363, intent proof credit and archived receipt identities — `python3 scripts/check_dogfood.py` passed (`logs/v1.1-b-acceptance.log`). Both stages: 644/1363 first-party; 644/1626171 combined. Proof code: 322 SLoC, 39/39 verified vs 4732 SLoC, 200/200 verified. Intent module: 4410 SLoC, all 161 units verified.
- [x] `CARGO_HOME="$PWD/.cargo-home" cargo test` (31 tests), `cargo clippy -- -D warnings`, `cargo build`, `cargo fmt --check`, Python syntax and `git diff --check` passed. Logs: `logs/v1.1-b-{test,clippy,build,fmt-check,python,diff-check}.log`. Final-binary replay of all branch reports passed (`python3 logs/v1.1-b-refresh.py`, `logs/v1.1-b-refresh.log`); source-bound receipts are from this session’s fresh verifier runs. Runtime security unit identities, sizes and proven membership also match scratch-server’s retained external stage-13 union exactly (`logs/v1.1-b-external-union-check.log`).

Ticks record local command acceptance only; independent acceptance remains outstanding.

## v1.1-c

- [x] Per-item cfg, scoped attribution, explicit trust, distinct hazards and Kani regressions — `CARGO_HOME="$PWD/.cargo-home" cargo test` passed (40 tests; `logs/v1.1-c-test.log`), plus `cargo clippy -- -D warnings`.
- [x] All 13 scratch-server branches remeasured — `python3 scripts/remeasure.py "$(cat logs/dogfood-export-path)"` and `python3 scripts/check_survey.py` passed. Stages 12/13: 644/1363 each, 0 first-party hazards; proof code 322/4732 SLoC, 39/39 and 200/200 units verified. Dependency site-count reductions are explained in NOTES.md and docs/scratch-server.md; exact source-site sets are unchanged.
- [x] Fresh pinned Vest run (2935/0) and Anvil attempt (exit 1, missing dependencies) recorded; release survey replay and `python3 scripts/check_survey.py` passed (`logs/v1.1-c-{survey,survey-acceptance}.log`). Vest security/runtime credit: 2961/10555 and 2961/11916, relying on 31 cfg-active trusted items (234 SLoC) and 5 escapes. Firecracker: 412 distinct unsafe sites, no kani::assume escapes; inventory only.
- [x] Survey/evidence docs, retained receipts and timing notes complete. `CARGO_HOME="$PWD/.cargo-home" cargo test` (40 tests), `cargo clippy -- -D warnings`, `cargo fmt --check`, release build, `python3 scripts/check_survey.py`, `python3 scripts/check_dogfood.py`, Python syntax and `git diff --check` passed (`logs/v1.1-c-*`).

Ticks require passing local acceptance commands; independent acceptance remains outstanding.

## v1.2

2026-10-28: Read PLAN-TARGET-DEPS.md v1.2 and all required source/docs before editing. Implementing closure, headline, then docs/table as separate local commits. Acceptance A–D pending. All commands use offline/frozen Cargo, repository-local CARGO_HOME/TMPDIR; build output retained in logs.

2026-10-28 v1.2 closure checkpoint: A/B passed via `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo test --frozen` (logs/v1.2-closure-test.log): fixture asserts exact normal/transitive set, build/proc-macro/transitive set and counts, dev exclusion, disabled optional absence, Linux/Windows filtering, and forced invalid-target metadata fallback equal to the former lockfile set. Existing tests pass after correcting classification fixture to declare its external path dependency and exclude it from workspace membership. `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo clippy --frozen -- -D warnings` passed (logs/v1.2-closure-clippy.log). Initial failed fixture/build iterations retained in logs; final checks above passed.

2026-10-28 v1.2 headline checkpoint: `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo test --frozen` and `cargo clippy --frozen -- -D warnings` (same environment) passed; logs/v1.2-headline-{test,clippy}.log. Regression asserts first output line, all six headline rows, and independent first-party vs combined threshold behavior, including CLI --target and invalid threshold rejection. JSON retains the existing summary.first_party/third_party/all figures per block. C and final D pending release replay.

2026-10-28T11:26:05-04:00 v1.2 optional-edge correction: `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo test --frozen` and `cargo clippy --frozen -- -D warnings` passed (logs/v1.2-optional-{test,clippy}.log). A now additionally checks a renamed optional dependency referenced only by a weak feature, and its explicit activation. Frozen metadata retains some weak optional edges even when their feature is disabled; corrected graph traversal checks declarations plus resolved feature activation rather than assuming every emitted optional edge is enabled. Release replay updates follow in docs/table change.

2026-10-28T11:30:02-04:00 v1.2 final local acceptance record:

- [x] **A** — `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo test --frozen` passed. `tests/target_dependencies.rs` checks normal + transitive runtime packages, build + transitive/proc-macro packages with source SLoC/unsafe counts, a dev exclusion count, disabled optional absence, weak renamed optional absence and explicit activation, and Linux versus Windows target selection. CLI target handling is also covered in `tests/reports.rs`.
- [x] **B** — the same test forces full metadata failure with an invalid target, asserts nonempty reason and `closure: lockfile (fallback)` in Markdown, exact name@version equality with the former lockfile inventory, and unknown-security-size gate failure. The real main/nix-scratch replays independently confirm fallback sets equal their historical sets and gates still fail on unknown sizes.
- [ ] **C exact set equality is unmet** — `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" python3 scripts/target_deps.py` completed all 13 branches and exited **1**, explicitly reporting the literal set mismatch (logs/v1.2-scratch-acceptance.log). Its exact comparison is `cargo tree -e normal --target aarch64-apple-darwin --frozen --prefix none --manifest-path target/v1.2-scratch/spec-lemmas/Cargo.toml` (the script records its absolute path form). Fifteen tree-only packages are in the required separate build-time inventory; two metadata-runtime-only packages are retained because dev-feature unification enables them in metadata. Full sets/differences, command and per-branch results: docs/scratch-server/v1.2-results.json. C's other checks **pass**: pinned commit 2fa5eb78a6f9, first output line proven 47.25% = 644/1363, third-party security runtime SLoC 336654 < 1626171. No false equality or all-acceptance-pass claim.
- [x] **D** — final `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo test --frozen` and `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo clippy --frozen -- -D warnings` both exited **0** (logs/v1.2-final-test.log, logs/v1.2-final-clippy.log). `cargo fmt --check` with the same environment, `python3 -m py_compile scripts/target_deps.py`, and `git diff --check` passed (logs/v1.2-final-fmt.log, logs/v1.2-python.log, logs/v1.2-diff-check.log). Release build `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo build --release --frozen` passed (logs/v1.2-release.log).

README/evidence documentation and the current 13-branch table include the closure used, build/dev exclusions, explicit fallback and the three headline figures. All historical reports remain intact. Eleven branches use metadata, main/nix-scratch use fallback because atomic-waker@1.1.2 is not cached. All first-party units/statuses and proof totals match retained reports; all retained receipts are accepted. Source/runtime/build reports are under logs/v1.2-scratch-*; source archives stay under target/v1.2-scratch/.

Local verification only; independent acceptance, new verifier runs and scratch-server application/container tests were not performed. Scratch-server `git status --short` remains empty. No network, push, external diary/message, AGENTS.md edits or scratch-server writes. Four local commits split closure, headline, weak-optional correction and docs/table; the plan remains unchanged and C requires reconciliation of its conflicting set expectation.

## v1.3

2026-10-28: Plan and evidence source read before edits. Release baseline build passed (`logs/v1.3/build-before.log`, only tail -n 30 read). Retained baseline executable: `logs/v1.3/formal-coverage-before`. Running all 16 reports through `scripts/check_perf.py capture before`; logs, raw JSON/Markdown and timings stay in logs/v1.3. Acceptance A–D pending. Survey/scratch exports remain read-only.

2026-10-28 v1.3 evidence-index checkpoint: indexed test identifiers, first-party escapes, trusted ownership and final proof-credit retention; contract identifiers computed once per Verus unit. All 32 JSON/Markdown files pass raw-byte comparison (`logs/v1.3/diff-indexes.log`). Vest 31.552→25.724 s, Anvil 2.131→1.226 s, Firecracker 30.378→2.070 s. `cargo test --frozen` and `cargo clippy --frozen -- -D warnings` passed (logs/v1.3/{test,clippy}.log). Required ignored release regression passed: 5,000 runtime units plus 2,000 test functions, full CLI report including metadata/serialization, 1.084 s (`logs/v1.3/performance-indexes.log`). Profiling remaining Vest cost before selecting a follow-up change.

2026-10-28 v1.3 reachability checkpoint: Whole-run sampling identified redundant reachability work at 68.27% of main-thread samples, above the plan's 20% threshold. Skip edge expansion once all units are reachable. Vest now 7.407 s, Anvil 0.652 s, Firecracker 0.744 s. All 32 output files remain byte-identical (`logs/v1.3/diff-reachability.log`). Frozen tests and clippy passed again (logs/v1.3/{test,clippy}-reachability.log). Vest still exceeds target; profile again before the next change.

2026-10-28 v1.3 final report measurements (release, wall seconds, one full CLI invocation per workspace; identical source/cache/options before and after):

| Workspace | Before | After |
| --- | ---: | ---: |
| vest | 31.552 | 2.865 |
| anvil | 2.131 | 0.582 |
| firecracker | 30.378 | 0.622 |
| scratch-main | 0.133 | 0.139 |
| scratch-nix-scratch | 0.126 | 0.137 |
| scratch-verified-boundary | 0.715 | 0.722 |
| scratch-browser-whitelist | 0.735 | 0.710 |
| scratch-https | 1.494 | 1.448 |
| scratch-https-headers | 1.496 | 1.440 |
| scratch-idle-connections | 1.463 | 1.447 |
| scratch-proven-transport | 1.529 | 1.542 |
| scratch-firefox-headers | 1.522 | 1.511 |
| scratch-chromium-headers | 1.541 | 1.552 |
| scratch-pragma-header | 1.579 | 1.503 |
| scratch-header-policy | 1.600 | 1.519 |
| scratch-spec-lemmas | 1.753 | 1.686 |

- **A passed locally:** `python3 scripts/check_perf.py diff` exited 0; all 32 JSON/Markdown files match raw bytes, no timing exclusions or normalization. `logs/v1.3/diff-final.log`; SHA-256 evidence in `logs/v1.3/identical-sha256.json`. Baseline/candidate reports and full command/timing manifests are in `logs/v1.3/{before,after}/`. Reproduction: capture `before` with the baseline executable, build candidate, capture `after`, then `diff`; capture refuses to overwrite a retained measurement directory. Intermediate index-only and reachability outputs/binaries are retained separately.
- **B passed locally:** before/after table above covers all 16 workspaces. All three survey reports are below 5 seconds, satisfying the plan's stopping condition. Baseline/final timings were collected without concurrent builds/tests; the index/reachability checkpoint measurements overlap some local checks and are diagnostic only.
- **D passed locally:** `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" CARGO_NET_OFFLINE=true cargo test --frozen` and `cargo clippy --frozen -- -D warnings` (same environment) exited 0. `cargo fmt --check`, Python syntax and `git diff --check` also passed. Build/check output retained in `logs/v1.3/final-*.log`; only tail -n 30 read. Final release build: `logs/v1.3/build-lines.log`.

Profile-based changes: evidence membership indexes; stop saturated reachability (68.27% of prior full-run samples); index source lines (at least 52.16% after the reachability fix). Profile files and selection rationale are recorded in NOTES.md. Stop after reaching the three-survey timing target; no additional hot spots changed.

- **C passed locally:** `CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" CARGO_NET_OFFLINE=true cargo test --release --frozen --test performance -- --ignored --nocapture` exited 0. Full report for 5,000 runtime units plus 2,000 test functions took **0.336 s**, below 2 s; also asserts all 7,000 units exist and the 4,000 called versus 1,000 uncalled runtime functions have the expected statuses. Log: `logs/v1.3/final-performance.log`.

All A–D checks above were run locally by Codex; independent acceptance has not been performed. No new verifier runs or survey/scratch application tests. No network, pushes, branch creation, survey/scratch source writes, external messages or diary writes. Four local commits separate the replay harness, evidence indexes/regression, reachability stop and source-line index. Repository-only artifacts retained; historical survey/scratch reports unchanged.

## v1.4 — PLAN-VERIFY-RUNS.md

- [x] M1 Firecracker execution/accounting: pinned Kani 0.67.0 container; 37 pass, 2 CBMC OOM, 1 x86-only not compiled. All 42 actual receipts rejected with reasons; zero unbounded and zero bounded credited coverage (historical M1; bounded result superseded by M2b). Passing-target bounded-credit acceptance remains **unmet**, documented rather than bypassed.
- [x] M2 Anvil: 11 fresh subtree runs, raw verifier count 935 verified (formal-coverage credits 0 of 53 receipts: cargo verus focus is outside the Verus receipt grammar by design, so measured proven is 0.00%) / 0 errors; 4 nonselectable roots skipped with reasons. All logs replayed as repeated Verus receipts; rejections retained. Correct verus-partial cache cleaned before each run; source/pin unchanged.
- [x] M2b authorized closed Kani grammar: 34/37 raw passes credited as bounded harnesses with source-derived scopes; 2 multi-harness logs and 1 expected-panic test rejected. Runtime targets 0; unbounded credit 0. All original tests unchanged plus 7 new policy regressions pass; clippy, fmt and diff checks pass. M1 files remain byte-identical.
- [x] M3: fresh locked fetch-only containers exit 0 for Vest/Anvil/Firecracker. Offline host reports use cargo metadata (runtime), with zero unknown runtime/build sizes. Before/after figures and separate fetch manifests retained.
- [x] M4: survey/results updated with named outcomes, raw-vs-credited counts and model assumptions, ARM64/x86 limit, and dependency before/after figures. Extended artifact checker, cargo test --frozen, clippy -D warnings, fmt --check and git diff --check all exit 0 locally; runs/v1.4/checks.json records commands/results.

Only local verification will be claimed; independent acceptance remains outstanding.

M1 local checks: source-derived 40-harness inventory reconciled; accounting regression passed; offline report run with all 42 retained receipts; zero proof credit confirmed. Manifests include immutable image/base digests, actual Rust/Kani versions, system package versions, lock/log hashes and commands. Logs are under runs/v1.4/firecracker/. No independent acceptance.

Final v1.4: Anvil 11 fresh subtrees / raw verifier count 935 verified (formal-coverage credits 0 of 53 receipts: cargo verus focus is outside the Verus receipt grammar by design, so measured proven is 0.00%) / 0 errors, 4 roots skipped; partial Verus receipts rejected with reasons. Firecracker raw 37 pass / 2 OOM / 1 architecture skip; M2b credits 34 bounded harnesses under explicit source stubs/contracts/vtable assumptions, 3 raw passes rejected, no inferred runtime targets and no unbounded credit. M1 entry above is the retained pre-M2b historical outcome. All runtime/build dependency source sizes now known; Vest library proof numbers unchanged; headline reliance includes the dependency inventory, with receipt scope reported separately. Local checks only; independent acceptance not performed.

## v1.4.1 release pass

- [x] M1 honest prose and current timing: baseline tests passed; Vest replay 8.19 s, identical summary; quoted tool inventory reliance.
- [x] M2 output wording and diagnostic regression tests: 51 tests pass, 1 ignored; three report JSON replays differ only in rejection messages.
- [x] M3 pinned offline Firecracker rerun: final idle-VM rerun passed in 96.58 s; unchanged grammar accepts it; 35/38 harnesses credited, runtime targets 0. Earlier shared-VM pass retained separately.
- [x] M4 reproducible evidence bundle and portable checkers: all three local checkers pass, including 16 source/receipt replays; 409 MB bundle + SHA-256 written outside git; tracked tree 3.4 MB. Fresh clone check follows public cleanup in M5.
- [x] M5 public hygiene, MIT license and final checks: denylist/key scan 0; 51 tests pass/1 ignored; clippy/fmt/diff checks pass; package includes LICENSE; fresh temporary clone with staged candidate + SHA-verified bundle builds frozen and all three checkers pass. External independent acceptance not run.

Final v1.4.1 local acceptance (2026-10-29): final idle-VM Kani rerun 96.58 s, 38 raw passes / 1 OOM / 1 architecture skip; 35 bounded harnesses credited, runtime targets 0. Published proven percentages preserved. All 16 final clone replays preserve coverage and receipt decisions. Build/check logs retained under ignored logs/release-pass/fresh-clone-*.log; final bundle and SHA-256 are in ../release. Tracked tree about 3.5 MB. No push or history rewrite.

## v1.4.2 public evidence pass

- Baseline main 0bbf2b3 and original v1.4.1 bundle retained; no verifiers or Docker started.
- Probe found plain author prefixes in evidence and 11 pinned source archives; no JSON slash, Unicode slash/dot or URL-encoded variants found. Debug-quoted and Kani/Verus output use plain prefixes.
- **Round 1, superseded by round 2 authorization:** pinned source archives contain private paths in upstream documentation. Source-archive bytes/hashes must remain unchanged, while the bundle scan may exclude only Cargo registry sources. Preserving the explicit archive fence leaves public-scan failures; v1.4.2 cannot be declared public-safe.
- [x] Deterministic normalization and transitive re-hash; bundled old/new SHA-256 table. 305 complete JSON/report/summary outputs match v1.4.1 after only path and mapped-hash substitutions; the retained diff is empty.
- [x] Cargo package/lock bumped from 0.1.0 to 1.4.2; current docs, fetch URL and bundle generator use v1.4.2. Original v1.4.1 bundle checksum verified unchanged.
- [x] Fresh clone under TMPDIR with staged candidate and bundle: frozen release build, survey/dogfood checks and all 16 full replays pass. Before extraction all three checkers give one-line v1.4.2 fetch failures.
- [x] Local frozen tests: 51 passed, 1 ignored; clippy all targets with warnings denied, fmt and diff checks pass; tree public scan passes. 55,518 immutable archive/third-party/cache files (1,543,868,226 bytes) verified byte-identical.
- Round 1 bundle public scan: **failed**, 645 matching lines: 428 source archives, 99 third-party Git sources, 13 CI runner paths, 105 GitHub API URLs. Only registry sources excluded as requested. These findings cannot be resolved with only author-prefix substitution while preserving archives/third-party files and the specified exclusion. Candidate must not be published as public-safe.
- Acceptance logs retained under logs/release-v1.4.2/. All verification was run locally by Codex; independent external acceptance has not run. One combined commit because version/source-provenance changes are coupled to evidence hashes; no push or tag.

## v1.4.2 round 2

- Archive preservation fence relaxed only for eleven affected scratch exports.
- Deterministic raw-record rebuild preserves tar metadata; substitutions use private runtime inputs. Audit lists every replacement position/rule and every changed member; protected source/Cargo bytes checked independently.
- Bundle scanner adds only the two specified substring masks and Git checkout path exclusion; tree mode retains its original behavior.
- Codex stopped on a usage limit before final integrity; round 2 left 33 bundle-scan hits (28 in the unchanged Firecracker export, 5 public PGP signature headers in pinned Anvil metadata).

## v1.4.2 round 3 (Claude Code takes over from Codex)

- [x] Codex's staged work reverified: archive redaction rerun from the SHA-verified v1.4.1 baseline reproduces all 13 archives, the 421-replacement log and the member proof byte-for-byte.
- [x] Firecracker exclusion justified first by scripts/check-upstream-archive.py: pin metadata, PAX commit and v1.4.1 SHA-256 agree; no author path, private LAN address or private term in the raw bytes (logs/release-v1.4.2/firecracker-provenance.log).
- [x] Exactly two new bundle-mode allowances, printed at scan start: Firecracker export exclusion; exact PGP signature/public key block armor masks. Regression cases: private-key armor still fails, exclusion is exact-path, tree mode unchanged.
- [x] Bundle rebuilt with .sha256; bundle scan 0 hits after six printed allowances (33 without the two new ones, matching round 2); tree scan 0.
- [x] No .rs/Cargo.* member changed (138 protected instances byte-identical); deterministic double rebuild of all 13 scratch archives matches the final archives; v1.4.1 bundle checksum unchanged.
- [x] Fresh TMPDIR clone: one-line v1.4.2 fetch failures without the bundle; frozen release build; survey, dogfood and perf checkers pass with 16 identical full replays. 305 v1.4.1 report/summary outputs compare with an empty diff.
- [x] cargo test --frozen 51 passed/1 ignored; clippy --all-targets -D warnings, fmt --check and git diff --check pass. Local acceptance only; no push, tag or publication.

## v1.5 — PLAN-SECURITY-SCOPE.md (Claude Code)

- [x] Baseline for F captured before code changes: the v1.4.2 binary reproduces the four bundle reports byte-for-byte; spec-lemmas host export 644/1,363 first-party, 336,654 third-party (logs/v1.5/baseline/).
- [x] Section 1, size modes: `--size-mode whole-crate|compiled-files|linked-lines` (default compiled-files), `--binary`, `--check-target-dir`; fail-closed fallbacks listed at the top of the report; mode/target/profile/binary sha256 in JSON and on Markdown line 1. A, B and C pass (tests/size_modes.rs): compiled 19 vs whole-crate 35 SLoC; linked-lines counts the called function and not the uncalled one; no-debug binary falls back to exactly 35; a deleted `.d` falls back for that dependency only. F after section 1: 5/5 cases identical (logs/v1.5/s1/F.log).
- [x] Section 2, structural test detection: `Category::Test` from test attributes, `cfg(test)` at any depth (including out-of-line module declarations) and integration-test/bench targets; reported as its own TEST CODE line. D fixture passes (tests/test_detection.rs). D on spec-lemmas without the glob: 1,363 → 1,301 first-party security SLoC (−62 = policy_tests exactly), proven 644 unchanged. `*::tests::*` dropped from the example config. F after section 2: 5/5 identical.
- [x] Section 3, explicit inputs: schema-3 config (`[[security.include]]`, `[[security.exclude]]`, `[dependencies."name[@version]"]` with `security`/`sloc`, required `reason`); schema 2 still parses and warns per entry; `migrate-config` (TODO reasons); a TODO reason fails `gate`. E (tests/inputs.rs): missing/empty reason, unmatched pattern, unmatched dependency key, ambiguous name and TODO gate all fail as required; migration round-trips the scratch-server overrides, and on spec-lemmas gives identical units, overrides and summary. F after section 3: 5/5 identical.
- [x] Section 4, reporting: an `## Inputs` section (pattern/key, direction, reason, matches, SLoC moved in/out, size change); every security headline printed with inputs and under default rules only; `summary_default_rules` in JSON; `gate --max-override-shift <pct>`. E: the default-rules figure equals a run with the inputs deleted, in the fixture (tests/inputs.rs) and on spec-lemmas (all five summary blocks equal; 644/1,348 default vs 644/1,301 with the v1.4.2 inputs). F after section 4: 5/5 identical.
- [x] Section 5, docs and numbers: README, docs/evidence.md, docs/scratch-server.md (three-mode spec-lemmas table), docs/survey.md (nine runs); worked schema-3 config examples/scratch-server.formal-coverage.toml with only code-justified inputs; debatable candidates are in NOTES.md for Andrew. Version 1.5.0.

Final v1.5 acceptance (local, 2026-10-29; logs under ignored logs/v1.5/):

- A pass: compiled-files fixture dependency 19 SLoC against 35 whole-crate (the cfg(windows) module, tests/, benches/, examples/ and the 3-line spec function excluded), asserted exactly.
- B pass: linked-lines counts the called function's lines and none of the uncalled function's; a no-debug-info binary sends both dependencies to whole-crate fallback, sized exactly 35.
- C pass: deleting one dependency's `.d` file sends only that dependency to whole-crate fallback, reason "dep-info file missing"; the other stays compiled-files at 19.
- D pass: fixture puts `#[cfg(test)] mod policy_tests`, nested `#[cfg(test)] mod a { mod b { fn helper() } }` and a free `#[test] fn` (plus out-of-line and integration-test cases) in Category::Test. spec-lemmas without `*::tests::*`: 1,363 → 1,301 (−62 = policy_tests exactly), proven 644 unchanged.
- E pass: missing/empty reason, non-matching pattern, non-matching dependency key, ambiguous name and a TODO reason (gate) all fail; migrate-config round-trips the scratch-server overrides (spec-lemmas: identical units, overrides, summary); the Inputs section shows SLoC moved per input; the default-rules figure equals a run with the inputs deleted (fixture and spec-lemmas). CLI gate log: logs/v1.5/final/E-cli-gate.log.
- F pass: `--size-mode whole-crate` with the v1.4.2 config reproduces the v1.4.2 JSON and Markdown byte-for-byte for the bundled spec-lemmas case, the spec-lemmas host export, Vest, Anvil and Firecracker after removing only the listed new fields (scripts/v15_regression.py; logs/v1.5/final/F.log).
- G pass for spec-lemmas: compiled-files libc 11,645 (< 20,000), third-party runtime security 140,348 (< 150,000); linked-lines 8,721 linked lines (lower still). Survey: Vest, Anvil and Firecracker exit 0 in all three modes. The new modes fall back for every survey dependency on this host (no aarch64-linux standard library or binaries), so their totals stay 1,035,488 / 2,974,275 / 1,718,910; recorded in docs/survey.md.
- H pass: cargo test --frozen 63 passed, 0 failed, 1 ignored (the performance test, separately passing in 0.37 s); clippy --all-targets -D warnings clean; fmt --check clean; scripts/check-public.sh tree scan 0 lines.

All verification ran locally under Claude Code; no independent acceptance, push, tag, release bundle, Docker, Kani or Verus runs. scratch-server and the survey checkouts were only read (git archive / bundle archives); builds went to this repository's target/.

## v1.5 Linux (Claude Code)

- [x] Container: local `rust:1.98.0-bookworm` arm64 (sha256:82150a52ec20…), `fcsurvey-main`, `--cpus 6 --memory 4g`, repository at `/fc`. Firecracker's pinned 1.97.0 (`rustup toolchain install` from its rust-toolchain.toml) and the musl target for 1.98.0 installed; rustup self-updated 1.29.0 → 1.29.2 while doing so. Linux formal-coverage built in the container (`--frozen`, `.cargo-home`).
- [x] Driver: `scripts/v15_replay.py --container NAME [--target TRIPLE]` runs the Linux binary through `docker exec`; `scripts/v15_size_check.py` checks per-dependency ordering and reference equality; tests in `scripts/test_v15_linux.py`.
- [x] Firecracker system build dependencies from its own devctr (libclang, musl-tools, static musl libseccomp v2.6.1) installed in the container; crates still only from pinned cargo homes with --frozen.
- [x] Survey in the container (gnu): whole-crate equals macOS v1.5 (totals and every dependency size); compiled-files third-party Vest 669,825 (0 fallbacks), Anvil 1,298,101 (0), Firecracker 465,089 (2: errno, aho-corasick, built by no cargo check).
- [x] linked-lines: Firecracker musl binary (release.sh's default target, sha256 3e31ea7e…) 4,571 linked lines, 1 fallback (wyz); Anvil plain-cargo glibc controllers, largest vstatefulset_controller 123,754 (8 fallbacks), seven binaries 119,266–123,754; Vest is a library and stays on compiled-files. aws-lc's ~108k C/asm linked lines and static musl are outside all modes (.rs only), reported separately.
- [x] spec-lemmas aarch64-unknown-linux-musl: whole-crate 336,654 (= macOS); compiled-files libc 15,868 (< 20,000), third-party 144,872 (< 150,000); linked-lines 8,873 (static ELF sha256 4b3940ed…). macOS 11,645 / 140,348 / 8,721 kept as comparison rows.
- [x] Sanity: compiled-files ≤ whole-crate everywhere; linked-lines ≤ compiled-files for every measured dependency, 9 exceptions all whole-crate fallbacks (wyz; Anvil's 8); first-party unchanged macOS vs Linux in every mode (logs/v1.5-linux/sanity.log).
- [x] Tool fix: cargo_version read in the project root (reports the rustup-selected cargo).
- [x] Docs: survey.md (Linux nine-run table, provenance, fallback list), scratch-server.md (Linux musl headline, macOS comparison rows), evidence.md, README.
- [x] Acceptance (logs/v1.5-linux/acceptance/): all runs exit 0; F PASS 5/5 identical; cargo test --frozen 63 passed/1 ignored; clippy --all-targets -D warnings, fmt --check, git diff --check clean; 11 driver tests pass; public tree scan 0 lines.

## v1.6 — PLAN-PROOF-RATIO.md

- [x] M1 ghost lines — default separate / compatibility runtime, per-unit counts, mixed-line ownership; hand-counted eight-unit fixture (including the later literal regression) covers all required constructs. logs/v1.6/M1.log and ghost-literal-after.log pass (both modes).
- [x] M2 v1.5 regression — all five v15 cases byte-identical in runtime mode after only enumerated additions removed; logs/v1.6/M2.log. Baseline built before edits; measurement sources identical to 887b730. Normalizer negative tests pass.
- [x] M3 ratio and range — first-party security/all, crate/module rows, both-mode figures, unbounded/bounded split, configurable range and gate. Three fixture tests pass: logs/v1.6/M3.log; full suite and clippy pass in M3-initial.log / M3-clippy.log.
- [x] M4 retained evidence and survey measurements — five paired final release replays; strict invariants PASS (logs/v1.6/M4.log), final M2 still 5/5 identical. Spec: 4,732 standalone; runtime 0 inline, 644/1,363 proven/runtime, ratio 7.35 within; separate 198 inline, 446/1,165, ratio 11.05 above. Survey security ratios: Vest 8.05 → 14.19, Anvil/Firecracker n/a. All crate/module rows and portable provenance in docs/scratch-server/v1.6-results.json; docs tables updated.
- [ ] M5 docs and checks — README/range limits/example config/evidence rule complete. Final frozen test suite: 69 pass, 1 existing release-only performance test ignored; clippy all targets with -D warnings, fmt, diff checks and 2 normalizer tests pass (logs/v1.6/M5-*). Structural public scan: 0 hits. Full private-denylist scan remains unverified: no denylist exists inside the permitted repository. Independent acceptance not run.

## v1.7 — hax/F* evidence

- [x] M1: fail-closed receipts, separate status, reporting and rejection tests.
- [x] M2: conservative mapping, extraction identity, contracts and model trust.
- [x] M3: seven synthetic acceptance cases.
- [x] M4: two Docker Bertie attempts, shipped lock and dependency measurement.
- [x] M5: survey, worked example and unchanged v1.6 replay.

Evidence: logs/v1.7/ and runs/v1.7/bertie/. Independent acceptance is Dudley's.

M1 evidence: logs/v1.7/m1-test.log (existing suite and closed-command rejection unit test pass). Receipt acceptance infrastructure does not yet promote units; mapping arrives in M2.

M2 evidence: logs/v1.7/m2-unit-final.log (mapping-parser/contract/comment policy tests); full synthetic receipt tests in M3. Added explicit per-definition JSON/Markdown, module/interface byte identity and patch trust, Rust hax trust inventory. Unknown contract forms receive no credit.

M3 evidence: logs/v1.7/m3-final.log — four integration tests pass, covering seven required synthetic variants, malformed/missing receipts, command/identity/hash/path/inventory/outcome rejection boundaries, and failed/partial checks. These are synthetic policy tests, not verifier execution.

M4 evidence (resumed by Claude Code after Codex's usage-limit stop mid-M4; Codex's staged uncredited-model-reporting diff in src/fstar.rs/src/report.rs/README.md was verified — cargo test 7/7 fstar_evidence + full suite, clippy, fmt all clean — and committed as-is): runs/v1.7/bertie/manifest.json, differences.json, summary.json, work/{upstream,shipped}/{report.json,report.md,typecheck-extraction.log(.evidence.json),typecheck-panic-free.log(.evidence.json),extract.log,extract.exit,patches.log,patches.exit,lock-audit.json}. Codex's own in-progress attempt (hax built natively via OCaml, F* native build failing) is in those same work/ directories as `typecheck-native-*.log`. F* has no Linux arm64 prebuilt; Docker Desktop's `--platform linux/amd64` QEMU emulation runs F*'s Linux x86_64 prebuilt (both 2025.03.25 and 2025.10.06) correctly, unblocking the run without a native OCaml build. Found and fixed (in scripts/hax/amd64-check.sh, not src/, so no cargo regression needed) a destructive-but-vacuous bug: `hax-driver.py typecheck --clean` chains `make clean` (`rm *.fst`) straight into `make verify`, leaving zero ROOTS and reporting trivial success. Fresh hax extraction is blocked for both trees by the conservative lock-drift audit (rand_core 0.10.1 presence changes via an optional getrandom feature pulled in by an unrelated scratch-server workspace member, traced exactly; not relaxed — see scripts/hax/README.md). `--fstar-log` therefore correctly rejects both trees with `unsupported or failed extraction`: zero proven-extracted credit. A diagnostic-only typecheck of the archived/committed `.fst`/`.fsti` snapshot (not fresh extraction, so no credit either way) was run with both F* 2025.03.25 (fails immediately, "Namespace Core cannot be found") and 2025.10.06, Bertie's own CI pin (verifies 54/47 modules before failing on the same error in `Bertie.Tls13utils.fsti`, identically for both trees — most likely stale-snapshot drift, since Bertie's CI only runs `extract-fstar`, never `typecheck`). Both trees measured in `compiled-files` mode: upstream 5,832/294,934/300,766 first-party/third-party/combined security SLoC, shipped 5,863/294,934/300,797; both 0.00% proven-extracted, 2 hazards, 3,054 escapes, 46 trusted items (10,924 SLoC). 4 Rust files differ between trees; all .fst/.fsti identical. Secondary fresh-resolution libcrux diff recorded (25 packages for bertie alone vs. 39 in the shipped lock).

M5 evidence: docs/survey.md "Bertie run (v1.7)" section (table, worked `certificate_verify`/`ecdsa_signature` example with real file:line from the archive) and docs/scratch-server.md/README/evidence.md touches. v1.6 replay unchanged: `scripts/v17_regression.py logs/v1.6/ratio-runtime logs/v1.7/replay-m4-check` (the true pre-v1.7 v1.6 baseline, not `logs/v1.7/replay/runtime`, which already carries the v1.7 `proven-extracted` field) — all five retained cases (`scratch-spec-lemmas`, `spec-lemmas-host`, `vest`, `anvil`, `firecracker`) byte-identical except the new empty `proven-extracted` column, PASS, against a binary rebuilt from the post-M4-commit source (logs/v1.7/m5-v16-replay-final.log).

## v1.7.1 — real Bertie numbers (PLAN-HAX-FSTAR-v1.7.1.md)

Codex hit its usage limit before writing anything for v1.7.1; Claude Code took over at a clean tree (2026-11-02).

- [x] M1 Audit v2 + harness — `scripts/hax/lock_audit_v2.py` (21 unit tests in `scripts/hax/test_v171.py`: exact lock passes; version change, source change, checksum change, removal, closure-internal addition, dropped edge (inside and outside the closure), closure-internal edge addition, non-dev Bertie edge, non-path Bertie, unresolvable reference and tree-hash mismatch are each rejected; dev-only package and edge additions accepted and listed), `scripts/hax/v171_workspace.py` (workspace builder with recorded manifest edits and tree hash check), Docker driver `scripts/hax/v171-docker.sh`. Both workspaces built under `runs/v1.7.1/{upstream,shipped}/ws`, `cargo metadata --offline` ran in Docker; audit v2 passes for both trees (75-package closure, 45 added packages + 8 Bertie dev edges, all listed in `lock-audit-v2.json`; the full lock diff is `lock.diff`). Evidence: logs/v1.7.1/m1-*.log, runs/v1.7.1/*/.
- [x] M2 Fresh extraction — `scripts/hax/v171-extract.sh` (Docker, offline) ran the hax-driver arguments plus the `rand_core@0.9.5` pin from the workspace root for both trees: exit 0, 19 fresh files each, all different from the committed snapshot, Rust sources/manifests byte-identical before and after (`extract/pre.sha256`, `post.sha256`). `scripts/hax/v171_inventory.py` hashes every file against the snapshot (`extraction-inventory.json`). Receipt label caveat in dev/NOTES.md. Evidence: logs/v1.7.1/m2-*.log, runs/v1.7.1/*/{extract,fresh}/.
- [x] M3 Typecheck and patches — F* 2025.10.06 (QEMU amd64, Z3 4.13.3) typechecks all 8 `.fst` modules of the fresh extraction in both trees (73–98 s each) through one grammar-compliant `fstar.exe --include ... FILE` command per module, logged in `typecheck-extraction-2025.10.06.log` with v1.7 receipts (`*.log.evidence.json`, accepted by the validator); 2025.03.25 not needed. `patches.sh apply` fails in both trees: 30 of 31 hunks fail (per-hunk record `patches/hunks.json`), the same on the committed snapshot (`patch-control/`), so patch failure is independent of the fresh extraction; the extraction-panic-free tier was not reached and receives no fresh-Rust credit. Report root = the bertie package alone with the audited lock pruned by cargo (`root-manifest.diff`, `root-lock-check.json`: 101 packages pruned, none added, no identity or version change, one dropped optional edge); 4 rootlock unit tests added (25 tests total pass). Evidence: logs/v1.7.1/m3-*.log, runs/v1.7.1/*/{typecheck-*,patches,patch-control,root-lock*}.
- [x] M4 Report and regression — fixed `unsafe_options()` (commit 56188b3, regression test `implicit_argument_lines_are_not_file_options_but_real_pragmas_still_are`; cargo test, clippy -D warnings, fmt --check pass), rebuilt this repository's tool from a `git archive` export in Docker (offline, pinned 1.98.0; `runs/v1.7.1/tool-build.*`), reported both trees in `compiled-files` mode, receipts accepted. upstream: 1,068/5,507 first-party security SLoC proven-extracted (19.39%, 50 units, all safety-only); shipped: 1,081/5,538 (19.52%, 51 units). `scripts/hax/v171_numbers.py` (3 unit tests; 28 total pass) derives trusted/escape/contract-kind/out-of-scope/libcrux figures into `numbers.json`; docs/survey.md Bertie section rewritten. Regression: `scripts/v17_regression.py logs/v1.6/ratio-runtime logs/v1.7/replay-m4-check` still PASS (logs/v1.7.1/m4-v16-retained-regression.log); fixtures/ and tests/ byte-identical to 8afd9df; **no new v1.6 replay was run** (see NOTES: it needs host cargo running third-party build scripts, which this task forbids). Evidence: logs/v1.7.1/m4-*.log, runs/v1.7.1/*/{report.json,report.md,numbers.json}.

## v1.7.2 — honest F* options and let-rec names (PLAN-HONEST-OPTIONS.md)

Codex hit its usage limit before writing anything for v1.7.2; Claude Code took over at a clean tree (2026-11-03).

- [x] M1 Option whitelist — `--split_queries {always,on_failure,no}` added to the F* option whitelist (F* help text and source quoted in NOTES, no soundness concern found); a rejected option is named in the escape kind (`unsupported F* option: <text>`). Unit tests plus one end-to-end test (`assume`, `--admit_smt_queries true`, `--lax` still block inside a split_queries region). cargo test --frozen 81 passed, clippy --all-targets -D warnings and fmt clean. Logs (untracked, gitignored): logs/v1.7.2/m1-*.log.
- [x] M2 `let rec` names — `let rec f` yields `f` (was `rec`), column-zero `and g` continues the group; two unit tests with literal `Bertie.Der.fst` snippets. cargo test --frozen 83 passed, clippy -D warnings clean: logs/v1.7.2/m2-*.log.
- [x] M3 Regression on existing runs (report-only) — new tool (sha256 01311d9e…) vs the v1.7.1 binary on identical copies of both trees; control reproduces v1.7.1 reports exactly (0 changed leaves), new vs control 72 changed leaves per tree, all explained in NOTES: upstream proven-extracted 1,068 -> 1,078 (`padlen` credited), 3 hazard-region functions trusted -> unverified, 15 `rec` names fixed, 6 escape kinds now name the option. Receipts do not bind the tool binary; none edited. Rerun: `bash dev/v172/run-m3.sh`. Logs: logs/v1.7.2/m3-*.log, dev/v172/m3-diff-*.txt.
- [x] M4 bertie-der re-measure (report only) — `fixed` (f694480) with the v1.7.2 tool: proven-extracted 1,068 -> 1,557 of 5,973 SLoC (17.88% -> 26.07%), credited units 50 -> 80 (20 functional); control reproduces v1.7.1 exactly, 211 changed leaves all explained. Expectation holds for `der` (29 of 32 runtime units credited, 19 functional; 3 stay unmapped because hax appends `_`), with caveats in SUMMARY.md. Rerun: `bash dev/v172/run-m3.sh && bash dev/v172/run-m4.sh`. Evidence: `projects/bertie-der/evidence/m5-v1.7.2/`, logs/v1.7.2/m4-*.log.

V1.7.2-READY

