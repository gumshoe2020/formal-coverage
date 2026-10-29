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
