# Notes

2026-10-27: Read PLAN.md fully and scratch-server PROOFS.md, TLS-PROVIDER.md and verify.sh. Work is confined to this repository and exported archives in /tmp. Cargo cache and build output are local and ignored. Each milestone will be committed on main as requested.

2026-10-27: Inventory uses balanced Rust token trees with span locations, including Verus syntax that ordinary syn cannot parse. Call classification will deliberately overapproximate ambiguous methods, macros and unresolved calls. Unknown evidence remains trusted.

2026-10-27: M1 acceptance passed: exact four-function inventory including nested module and Verus contracts; nested comments/string comment markers checked. Cargo output: logs/m1-test.log.

2026-10-27: M2 acceptance passed (logs/m2-test.log). Lockfile closure intentionally overincludes third-party crates; configured workspace-local dependency sources supply measured SLoC, otherwise size is explicitly unknown. No inferred zero-size verification claim. Overrides apply after reachability, with not_security last and all matches listed.

2026-10-27: M3 acceptance passed (logs/m3-test.log). Read primary Verus summary emitter, Kani output guide/attributes, and nine published libcrux archives (README verification sections, manifests and selected API implementations). Exact release/target correspondence remains unconfirmed as in TLS-PROVIDER.md. Registry claims are partial primitive coverage, so whole dependency units remain trusted. Source-bound receipts are required and caller-supplied, not cryptographic execution attestations. Any first-party escape invalidates local proofs conservatively across the workspace. Kani promotes only the harness under its model/bounds, never callees.

2026-10-27: M4 acceptance passed: cargo test (12 tests) and clippy -D warnings; logs/m4-test.log and logs/m4-clippy.log. Both output formats retain the security/other split, measured SLoC statuses, party subtotals, escapes, overrides and unconfirmed upstream claims. Gates reject missing dependency sizes, threshold failures, baseline drops and new escapes. CLI threshold failure and success exit codes tested.

2026-10-27T16:06:16-04:00: PLAN.md and CODEX_PROMPT.md changed externally during M5. Re-read the full authoritative spec and adopted unverified/hazard/proven-bounded vocabulary. First-party hazards get independent SLoC and a top security alert. Unsafe-only Kani target coverage requires an explicit harness mapping, direct unambiguous call and numerical unwind bound; bounded coverage never contributes to unbounded proven. Third-party unsafe sites are measured separately and do not downgrade upstream status. Confirmed whole-crate registry support is tested with an explicitly synthetic entry; all real libcrux seed claims remain partial/unconfirmed.

2026-10-27T16:06:16-04:00: Dogfooding exposed and resolved Verus match-contract braces truncating function bodies. Added regressions for complete bodies, raw/character literals and empty/generic/macro unresolved calls. Missing/stale lock inventories and unknown dependency SLoC fail gates closed; configured dependency source manifests must match lockfile name/version. Source symlinks cannot silently hide Rust code. Conditional Verus files remain unverified because file summaries cannot establish which cfg definitions were checked.

2026-10-27T16:06:16-04:00: Exported all seven scratch-server branch commits to /tmp using git archive, omitting tracked symlinks such as the Nix result link. Downloaded registry sources with Cargo.lock SHA256 checks to measure Rust dependency SLoC and unsafe sites. Ran every available verify.sh locally in isolated exported workspaces: five runs passed, each 18 verified / 0 errors. main and nix-scratch have no verifier script. Report regeneration reuses only these same-session source-bound runs; no historical scratch-server logs were reused. Original scratch-server remains on idle-connections with a clean working tree. No containers or external services were changed.

2026-10-27T16:06:16-04:00: Local cargo test (20 tests), cargo clippy -- -D warnings and cargo build passed with output under logs/. Final seven-branch artifact acceptance is in progress. No actual Kani/F* run, scratch-server application tests, container checks or independent human acceptance were performed.

2026-10-27T16:08:45-04:00: Final M5 acceptance passed via python3 scripts/check_dogfood.py: seven branch artifacts, both classifications, independent weighted-total recomputation, exact source hashes from recorded git archives, five successful 18/0 verifier logs and no unknown dependency sizes/unsafe scans. Added aggregate third-party unsafe columns to the branch comparison. Final formatting and git diff whitespace checks passed. Externally supplied PLAN.md/CODEX_PROMPT.md revisions are recorded with the implementation without alteration.

## Final summary

M1–M5 implemented and locally accepted; five local milestone commits on main, no push. First-party security-proven share: 0% on main/nix-scratch, 40.54% verified-boundary, 35.71% browser-whitelist, 18.80% https, 18.99% https-headers, 17.37% idle-connections. Everything-else proven share remains 0% on all branches. The conservative combined dependency denominator is much larger; full split/status/unsafe-site numbers and run receipts are in docs/scratch-server.md and docs/scratch-server/. All seed upstream release/target claims remain unconfirmed; no whole-crate libcrux proof is inferred. Twenty tool tests, clippy, build, formatting and seven-branch artifact acceptance passed locally. No actual Kani/F* or application/container tests or independent human acceptance. No remaining build blocker.

2026-10-27T21:04:04-04:00: v1.1-a started after reading PLAN-PROOF-CODE.md and the required supporting documents, scratch-server PROOFS.md and verify.sh. Thirteen local stage branches exist. Proof category is based on the immediate spec/proof mode token preceding fn; runtime contracts and ghost blocks remain runtime. Evidence scope remains complete invoked-root files only; no module or multi-root union is inferred. Cross-schema acknowledgment will skip percentage comparison while retaining escape comparison, because denominators differ.

2026-10-27T21:08:07-04:00: Syntax/evidence/gate regression tests passed locally (cargo test, logs/v1.1-a-test.log). Schema 2 represents category separately from status; proof units use verified/unverified/hazard, with hazard units included in unverified counts and shown explicitly. Even a non-security proof hazard blocks gates. Baseline schema changes require acknowledgment and skip incomparable percentage comparisons while preserving current thresholds, completeness and escape comparison. M5's parser-only receipt scope is retained; multi-root verify.sh outputs are stored separately, followed by a complete parser-only invocation rather than a fabricated combined receipt. v1 comparison binary is built from formal-coverage commit 4685da4841e9f9bf2e74442d2e9d24e82317ead3 into ignored logs/v1-tool.

2026-10-27T21:09:08-04:00: Expanded regression acceptance passed: restricted visibility, impl/macro proof modes, runtime contracts/ghost blocks, string/comment controls, independent proof split, invoked-root scope despite hashing another module, all five proof-side hazard types with otherwise successful synthetic evidence, CLI escape-gate failure, and schema 1 baseline acknowledgment. Synthetic receipts test parser policy only, not fixture mathematical correctness.

2026-10-27T21:17:00-04:00: Full branch sweep has completed through pragma-header with fresh passing verifier scripts. Final artifact acceptance will recompute weighted runtime/proof totals and compare stage 12/13 security runtime units plus both first-party and combined headlines. Integration-test additions remain in everything else under the unchanged M5 overrides. No classifier adjustment has been made in response to measured differences.

2026-10-27T21:23:28-04:00: v1.1-a local artifact acceptance passed on all 13 branch tips. Eleven fresh verify.sh runs succeeded; six multi-root stages also received separate complete parser-root evidence runs. Stage 13 script summaries: 211/0 parser, 237/0 transport, 9/0 AEAD (overlapping obligations). The source-bound parser rerun passed 211/0. Both stage 12 and stage 13 runtime security headlines are 453/1363 = 33.23551% first-party, and 453/1626171 = 0.02785685% with conservative M5 dependencies. Security runtime units, numerator and denominator match without changing classification. Proof-code SLoC is 322 vs 4732 (+4410). Stage 13 adds 96 SLoC of integration-test functions in everything else; that is outside the security invariant.

2026-10-27T21:23:28-04:00: Evidence scope explains the difference from the spec problem example: the same M5 parser-only receipt gives v1 502/1685 (29.79%) at stage 12 and 502/6095 (8.24%) at stage 13. The quoted 715 numerator was scratch-server's later explicit union of three root reports; multi-run union is not implemented here. Stage 13 proof totals are {"hazard_units": 0, "sloc": 4732, "units": 200, "unverified": 169, "verified": 31}. Imported intent functions remain unverified under existing invoked-root-only attribution despite the complete root verifier runs; no fabricated module/union evidence was introduced.

2026-10-27T21:23:28-04:00: Local cargo test (23 tests), clippy with warnings denied, formatting, archive/receipt artifact checks and Python syntax checks passed. All writes stayed within formal-coverage and /tmp exports; scratch-server source, branch checkout and containers were untouched. No push, Kani/F* reruns, scratch-server application/container tests, target-accurate dependency closure, self-measurement or independent human acceptance. Committing locally with the configured repository identity.

2026-10-27T21:59:29-04:00: v1.1-b started after reading the authoritative union/module spec and all requested supporting records, plus scratch-server PROOFS.md and verify.sh. Implementation will retain full-inventory hash requirements (missing module hashes reject the receipt), exact complete-command allowlisting, global hazards, and per-receipt provenance. Scratch-server stays read-only; only archive exports under /tmp will run verifiers.

2026-10-27T22:04:06-04:00: v1.1-b synthetic attribution acceptance passed (30 Rust tests) and clippy with warnings denied passed. Rejected receipts remain isolated with reasons; Kani mappings/bounds are receipt-local, and bounded evidence cannot demote an existing unbounded Verus result. Pinned Verus --help confirms --verify-root, --verify-module, --verify-only-module, --verify-function and --no-verify; all restricted/unknown commands are rejected, not interpreted. Help output is in logs/v1.1-b-verus-help.log.

2026-10-27T22:04:06-04:00: Extended the existing inventory to retain source filenames with no function units. Those files now participate in full-inventory hash requirements and global escape scans; otherwise module-level assumptions could disappear. An unattributed escape in a functionless file stays explicit, gets a top warning and blocks gates. Fixture receipts are synthetic policy tests, not verifier executions. Conditional files and descendant modules remain conservatively uncredited; cfg enablement is not inferred.

2026-10-27T22:06:58-04:00: Review added rejection of a flag supplied as the alleged Verus input filename, even if a caller hashes a file with that name. Final Rust checks pass with 31 tests, clippy with warnings denied and cargo build (logs/v1.1-b-{test,clippy,build}.log). This does not change supported scratch-server commands or tune measured coverage.

2026-10-27T22:19:01-04:00: Full v1.1-b branch sweep passed: 13 branches exported via git archive, 11 verify.sh runs, and 23 independent root invocations with individual source-bound receipts. Stages 8–13 have three roots. Stage 13 fresh script and individual results are 211/0 parser, 237/0 transport, 9/0 AEAD (overlapping queries). Both stage 12 and 13 single-call runtime headlines are 644/1363 = 47.24872%; proof-code SLoC is 322 (39/39 units verified) vs 4732 (200/200 verified). No number or classification was tuned to force a match. Full artifact acceptance and final binary replay are pending.

2026-10-27T22:26:42-04:00: v1.1-b final artifact acceptance passed for all 13 branch tips (python3 scripts/check_dogfood.py, logs/v1.1-b-acceptance.log). Every branch was also replayed through the final binary with all of its same-session receipts in one call (logs/v1.1-b-refresh.py and logs/v1.1-b-refresh.log). The replay adds no proof claims or post-processing union; the tool independently checks each receipt again. Both stages 12 and 13 are 644/1363 first-party runtime security SLoC (47.24872%), or 644/1626171 with conservative lockfile dependencies (0.03960225%). Security runtime units match in identity, size, classification and status. Source hashes match the recorded git archives. Stage 13 runtime security identities, SLoC and proven membership also match scratch-server logs/stage13-coverage-union.json exactly (logs/v1.1-b-external-union-check.log). No discrepancy required a classifier adjustment.

2026-10-27T22:26:42-04:00: Stage 12 proof code is 322 SLoC, 39/39 verified; stage 13 is 4732 SLoC, 200/200 verified (100% of units and SLoC). intent.rs contributes 4410 SLoC in 161 inventoried spec/proof units, all credited from the complete parser run. No proof module remains uncredited in the measured stages. Transport's #[path] mod http remains excluded from its receipt with a warning; the independent parser root credits that file and its plain intent module. IO/TLS/entry-point/test modules outside those Verus scopes remain tested/unverified, and uncontracted runtime functions receive no inferred proof credit. Docs retain parser-only pinned v1 comparison columns and explicitly distinguish their evidence scope from v1.1-b's internal union.

2026-10-27T22:26:42-04:00: Final local checks passed: 31 Rust tests, clippy -D warnings, build, fmt --check, Python syntax, archive/receipt acceptance and diff whitespace. Scratch-server remains clean on spec-lemmas; only archive exports under /tmp ran verifiers. No containers, scratch-server source/checkout, host configuration or external messaging were changed; no push. No actual Kani/F* runs, application/container tests, target-accurate dependency work, self-measurement or independent human acceptance were performed. Committing locally using the configured Dudley identity.

2026-10-27: v1.1-c started; read authoritative PLAN-REALWORLD.md and every requested supporting document. Source inspection confirms blanket cfg rejection, workspace-global hazard invalidation and per-unit hazard duplication. Survey clones and scratch-server stay read-only; third-party code will only be passed to pinned Verus on /tmp copies, never build/test scripts.

2026-10-27T23:07:04-04:00: v1.1-c checkpoint: synthetic policy tests now cover receipt-local cfg/features, harmless and unknown cfg_attr, nested/macro/module ownership, trusted external_body/assume_specification, in/out-of-scope escapes, distinct sites and Kani proof code/input assumptions. Local cargo test and clippy -D warnings pass (logs/v1.1-c-{test,clippy}.log). Fresh pinned Vest run passed 2935/0 in 15.99 s; Anvil direct run exited 1 after 168.04 s with missing dependencies, including verus_temporal_logic, kube and k8s_openapi. This does not establish a Verus-main incompatibility; dependency builds are prohibited, so inventory only is the honest Anvil outcome.

2026-10-27T23:07:04-04:00: First release replay of all 13 retained source-bound scratch-server branch runs preserved every first-party unit/status/size and runtime/proof totals, including 644/1363 at stages 12/13 and zero first-party hazards. Retained verifier runs are from v1.1-b earlier on 2026-10-27, not new verifier executions this milestone. Reports are remeasured with release builds. Final replay follows the completed code.

2026-10-27T23:07:04-04:00: Survey discrepancy analysis: Vest's old 73 external_body entries reduce to 29 distinct sites (same archived sources); four assume_specification directives are now also inventoried as trust. All 18 Firecracker kani::assume sites constrain harness inputs: four are in cfg(kani) Arbitrary generators/stubs rather than directly in proof functions. These proof-only helpers are now recognized syntactically and covered by a regression, while kani::assume outside proof context remains a hazard. Per-run cfg-active site counts are separate from the complete inventory and are being remeasured; no classifier has been tuned to an expected percentage.

2026-10-27T23:11:33-04:00: Checkpoint correction: the last Kani string-negative fixture had a raw-string delimiter collision; the checkpoint's latest-test claim was premature (earlier tests had passed). Corrected the delimiter and reran cargo test/clippy successfully. Added cfg-active site-scope and nested-owner regressions. A follow-up review found a file-scope unsafe impl inside cfg-disabled tests still downgraded active siblings via conservative affected-unit fallback; corrected attribution to consult the run's active site list. The inventory and escape gate retain that site. This is a scope bug fix, not adjustment to a target result; final survey numbers are being regenerated.

2026-10-28T00:48:12-04:00: Resumed after Andrew's usage-window reset; re-read PROGRESS.md, NOTES.md, git history, CODEX_PROMPT.md and PLAN-REALWORLD.md. Final release replay confirms runtime/proof numbers and unit attribution unchanged across all 13 scratch branches. Dependency unsafe counts do change under the required distinct (file,line,kind) metric: futures-util 136→135, http 65→64, httparse 78→75, hashbrown 455→451, r-efi 5.3.0 15→14 / 6.0.0 332→331, prettyplease and verus_prettyplease 26→23 where present. Root cause is multiple unsafe tokens on one source line, e.g. httparse's unsafe fn with unsafe fn-pointer parameter on the same line in neon.rs:246, sse42.rs:126, avx2.rs:186. Asserted exact old/new dependency unsafe site-set equality, while recomputing per-crate distinct counts. Updated docs/scratch-server.md with a current correction table and retained old reports as historical. No classifier tuning or source change was made.

2026-10-28T00:49:53-04:00: v1.1-c final local acceptance passed. cargo test: 40 tests; clippy -D warnings; fmt --check; release build; scripts/check_survey.py; scripts/check_dogfood.py; Python syntax; git diff --check. Final release survey replays the fresh pinned verifier executions from this milestone, with exact archive/log-hash checks. Vest actual direct run had cfg=[] plus the three confirmed Verus driver names; 2935 verified / 0 errors in 15.99 s. Security runtime 2961/10555 = 28.05% proven, relying on 31 cfg-active trusted items (234 SLoC) and 5 unsafe escapes; all-library runtime 2961/11916 = 24.85% proven with the same reliance. Complete vest_lib inventory separately lists 33 trusted sites (246 SLoC) and 15 unsafe sites; whole workspace lists 39 trusted sites (520 SLoC) and 68 escapes. Final artifact site lists and receipt are under docs/survey/.

2026-10-28T00:49:53-04:00: Firecracker inventory has 412 distinct unsafe sites, preserving the exact original unsafe-site set; all 18 model-input kani::assume sites are excluded from escapes. Kani proof code is 69 functions / 906 SLoC; no Kani run or bounded proof is claimed. Anvil remains inventory-only: pinned Verus exited 1 due to missing compiled external dependencies, so compatibility with its Verus-main dependency is unconfirmed, not diagnosed as a version failure. All 13 scratch branches retain runtime/proof counts and classifications, with stages 12/13 644/1363, proof 322/4732 SLoC (39/39 and 200/200 verified), and zero first-party hazards. Only dependency unsafe occurrence-to-distinct-site counts changed, documented separately.

2026-10-28T00:49:53-04:00: Local acceptance only; independent acceptance remains outstanding. No Kani/F*, application/container tests, target-accurate dependency analysis or self-measurement. No push, branch creation, host configuration edits, third-party build/test/script execution, scratch-server/container modification, or external messaging. Writes remain inside formal-coverage and /tmp; no external diary write was performed under this turn's strict write boundary. Committing the final milestone with the configured repository identity.

2026-10-28 (v1.2): Conservative decisions: packages needed by both runtime and build contexts appear in both sets; build-only transitive normal dependencies stay build-time. Full package source SLoC still includes cfg/test code (no claim of linked-line precision). Metadata failure retains the prior name-based lockfile closure and completeness checks. No network or external diary writes under repository-only fence.

2026-10-28 (v1.2): Scratch-server cached registry files were copied read-only from scratch-server/.tools/cargo/registry into this repository’s local Cargo cache (missing files only). Full frozen host metadata for exported spec-lemmas succeeds, no network required. Proc-macro packages reached through normal edges are classified build-time; dual-use dependencies remain in both sets. Dev exclusion count includes their normal/build transitive packages, excluding packages already counted in runtime/build sets. Build unsafe sites are separate counts, not runtime escape sites. Codebase learning proposed: classification fixtures should declare intended external dependencies in Cargo.toml, rather than depending on stale lockfile entries.

2026-10-28 (v1.2 headline): To allow a first-party-only threshold, the CLI combined threshold now defaults to zero when omitted; supplied --min-security-proven values retain their combined meaning. Global completeness/unknown-size/hazard checks still run for either threshold. First-party headline is literally the first Markdown/CLI line, including its trust/escape statement; fallback warning stays immediately under the title near the top.

2026-10-28T11:26:05-04:00 (v1.2): Source-bound spec-lemmas replay revealed Cargo metadata can include disabled weak optional edges: spki→base64ct despite spki features=[alloc], and rustls-webpki→ring despite features=[alloc,std]. Checked manifest feature definitions and resolved features, omitting only optional edges whose activation can be ruled out; renamed optional and explicit-enablement regressions pass. Unknown declaration matches stay included. Proposed codebase learning: dep_kinds alone does not establish optional dependency activation, even with --filter-platform.

2026-10-28T11:30:02-04:00 (v1.2 final decisions/limitations): Literal acceptance C conflicts with separating proc macros: normal-only cargo tree includes 15 packages that the plan requires in the build-time set (all confirmed members of that set). Retained those explicit build-time exclusions. Metadata additionally unifies rand default features through dev dependencies, while normal-only cargo tree excludes chacha20@0.10.2 and getrandom@0.4.3. Retained these two enabled normal-edge packages rather than lowering the denominator; did not substitute a pruned cargo-tree closure for the prescribed metadata graph. The exact set check returns failure and both sets are retained. Proposed codebase learning: cargo metadata resolution is not a production-only feature resolution; future strict linkage measurement needs a corrected acceptance oracle and explicit feature/context semantics. No claim that all A–D pass.

2026-10-28T11:30:02-04:00 (v1.2 cache/fallback): Scratch-server read-only cache supplies the later branches. main/nix-scratch require uncached atomic-waker@1.1.2; frozen metadata refuses a download, so their full 53-package old lockfile closures remain with unknown sizes and failing gates. Did not access prior /tmp archives or global caches outside the allowed directories, fetch sources, or import historical size numbers as fresh evidence. Added an empty [workspace] only to the early branches' exported manifests to prevent inheritance from formal-coverage; original checkout and Rust source bytes are unchanged. Replayed retained source-bound receipts; no verifier/application scripts were executed. No external diary write because the repository-only fence forbids it.

2026-10-28 (v1.3 decisions): Read PLAN-PERF.md v1.3 and src/evidence.rs before changes. Acceptance replay uses existing <temporary-export> survey exports and target/v1.2-scratch exports read-only, with the report commands/options from survey.py and target_deps.py. Use retained Vest log/receipt and repository docs/survey/vest.toml (the retained survey config), avoiding sibling config reads. Do not run those driver scripts: they export/write trees or run verifiers. Baseline and candidate use exactly the same paths/options/environment; comparison excludes no bytes or fields. Existing Cargo metadata calls use --frozen or --no-deps --offline; CARGO_NET_OFFLINE=true also guards child processes. No network, external diary writes or memory queries under this fence. Repository started clean; no branches or pushes. Acceptance is local only until independently rerun.

2026-10-28 (v1.3 evidence indexes): Hash sets serve membership only and never determine output order. Escape owners are deduplicated per escape; vectors preserve original escape order and include legacy primary ownership even when absent from affected_units. Final credited-id membership preserves the old any-matching-unit behavior for duplicate ids. Existing evidence/cfg/union regressions pass. The synthetic acceptance fixture exceeds the requested scale: 5,000 runtime units plus 2,000 tests (7,000 total), rather than counting tests inside 5,000 total units. Initial post-index byte diff passed for all 16 workspaces; intermediate outputs will be retained separately before the final replay.

2026-10-28 (v1.3 profile selection): A late 1-second Vest sample after indexing showed receipt inventory/source-line traversal (693/711 samples in inventory), but that phase-local sample is not a wall-time percentage. Collected a full-process sample from launch (`logs/v1.3/profile-full-indexes.txt`, 20,534 main-thread samples): analyze_target's reachability edge checks/inserts account for 5,911 + 4,771 + 3,337 = 14,019 samples (68.27%). Selected the simpler fix: stop computing edges when the reachable set already contains every unit. This preserves the fixed point and all later overrides. Did not optimize line extraction based on the misleading short sample; its whole-run share is below the threshold at this point. Intermediate index-only outputs and executable retained in logs/v1.3/after-indexes and formal-coverage-indexes.

2026-10-28 (v1.3 second profile): With full-set reachability short-circuited, Vest takes 7.407 s. Full-process sample (`logs/v1.3/profile-full-reachability.txt`, 5,661 main-thread samples) has two disjoint source.lines().skip(...).take(...) collection subtrees with 1,435 and 1,518 samples, already 52.16% combined, exceeding 20% without counting smaller instances. Index source.lines() once per file and pass the borrowed line slice through recursive inventory walking. Keep the original per-unit line range, CRLF handling, join separator and SLoC calculation. Both initial analysis and receipt validation inventory use this path. Other hot spots remain untouched pending final timing; stop if all three survey reports are below 5 s as the plan directs.

2026-10-28 (v1.3 final): All 16 final report invocations match baseline JSON and Markdown byte-for-byte (32 files, no exclusions). Vest 31.552→2.865 s; Anvil 2.131→0.582 s; Firecracker 30.378→0.622 s. Stop under the plan's all-three-below-5-seconds rule, even if a residual function could exceed 20%. Required synthetic full-report regression passed in release at 0.336 s for 5,000 runtime functions plus 2,000 tests. Final frozen tests, clippy -D warnings, formatting and Python syntax pass; local acceptance only. No independent acceptance or fresh verifier execution claimed. No external diary write under repository-only/no-network fences. Suggested codebase learning: saturated conservative reachability needs no edge expansion; sample whole executions before assigning a hot spot a wall-time share.

2026-10-28T20:35:58.149853-04:00 (v1.4 start): Read authoritative v1.4 plan, survey.md, survey.py, README receipt section, evidence.md and receipt validator. Re-probed Docker linux/aarch64, 12 CPUs, 8319238144 bytes; exact three clone commits and project toolchain/CI files. Clones read-only; all third-party execution restricted to containers with same-path read-only archive plus one project output mount. No host installs, branches, pushes or external messages. Existing receipt grammar intentionally rejects cargo-verus and parallel/CI Kani commands: retain rejection evidence, never rewrite a command as a different successful run. Diary writes outside repository are excluded by the task fence.

2026-10-28T20:37:42.740809-04:00 (v1.4): M1 first image build failed: Kani 0.67.0 ARM64 release requires GLIBC_2.39, absent in pinned Bookworm base. No verifier executed. Retained image-build.log; use pinned Rust 1.98.0 Trixie base for compatible libc. This is run setup, not a coverage-tool bug.

2026-10-28T20:39:26.569358-04:00 (v1.4): vest/fetch: exit 0, 13.74 s; image sha256:620dbcd124499c59e2406d3741574b5c5838cf9eb9656f0c3a03948f79b02959; log runs/v1.4/vest/fetch.log.

2026-10-28T20:39:26.694436-04:00 (v1.4): firecracker/fetch: exit 0, 15.36 s; image sha256:620dbcd124499c59e2406d3741574b5c5838cf9eb9656f0c3a03948f79b02959; log runs/v1.4/firecracker/fetch.log.

2026-10-28T20:40:07.822023-04:00 (v1.4): firecracker/versions: exit 0, 0.69 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/versions.log.

2026-10-28T20:40:42.067152-04:00 (v1.4): Anvil has no archived root lock. Resolve a generated lock in a container-only writable copy with exact Verus and verus-tla revisions selected before Anvil date; normalize only git source query strings back to the original manifests tracking those URLs. Copy only the generated lock into the host export, never alter archived bytes. Require cargo fetch --locked on the unchanged export to validate this lock; retain both lockfiles and hashes. No proof/dependency credit from a failed resolution.

2026-10-28T20:41:04.439988-04:00 (v1.4): firecracker/kani: exit 1, 51.55 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani.log.

2026-10-28T20:41:06.165611-04:00 (v1.4): anvil/resolve-lock: exit 101, 23.95 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/anvil/resolve-lock.log.

2026-10-28T20:41:57.116076-04:00 (v1.4): M1 CI workspace compilation failed before harness verification: cpu-template-helper/build.rs writes src/utils/mock_kernel/kernel.bin and hits EROFS. Preserve readonly source fence. Narrow to per-harness VMM package; no code patches or generated-file insertion.

2026-10-28T20:41:58.251639-04:00 (v1.4): Anvil first resolution failed: explicit rev on direct vstd created a distinct Cargo source from verus-tla unpinned transitive vstd, causing duplicate links=prettyplease-verus02. No lock/verification credit. Retry using unmodified manifests and container-local git URL redirects to bare repositories whose main/HEAD are pinned; git-fetch-with-cli forces the redirects. Exact dependency identities remain the original URLs and selected commits.

2026-10-28T20:42:02.815792-04:00 (v1.4): anvil/resolve-lock-pinned: exit 0, 4.47 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/anvil/resolve-lock-pinned.log.

2026-10-28T20:42:15.947446-04:00 (v1.4): firecracker/kani-ethernet: exit 1, 18.55 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-ethernet.log.

2026-10-28T20:42:29.643278-04:00 (v1.4): First narrowed Kani filter mistakenly used module verification instead of archived kani_proofs. It compiled VMM successfully but matched no harness; no proof result. Correct the setup filter from inventoried unit ID; retain the failed-filter log.

2026-10-28T20:42:30.803252-04:00 (v1.4): Pinned redirect resolution retained a pre-existing cached Verus main (6048d5d9); inspection of generated lock caught it before verification. Force cargo update -p vstd --precise selected SHA and assert all Verus git packages use the date pin before installing generated lock.

2026-10-28T20:42:35.073937-04:00 (v1.4): anvil/pin-lock: exit 0, 4.19 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/anvil/pin-lock.log.

2026-10-28T20:42:42.347637-04:00 (v1.4): firecracker/kani-ethernet-corrected: exit 0, 12.47 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-ethernet-corrected.log.

2026-10-28T20:43:55.090836-04:00 (v1.4): Installed generated Anvil Cargo.lock (no archived lock existed), exact SHA256 1fe68c8ac987aff474a0d663c5c2543efd04c8a03474a1ca6c05f06cae59b3ba. Asserted all Verus git packages resolve to 781a460f; original Cargo.toml and every archived file remain byte-identical. Rust 1.98.1 is required by built Verus; set RUSTUP_TOOLCHAIN in the container instead of changing archived rust-toolchain.toml (1.98.0).

2026-10-28T20:43:55.540950-04:00 (v1.4): anvil/versions: exit 0, 0.37 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/versions.log.

2026-10-28T20:43:56.998205-04:00 (v1.4): firecracker/kani-harness-00: exit 0, 13.42 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-00.log.

2026-10-28T20:44:07.889088-04:00 (v1.4): anvil/fetch: exit 0, 12.26 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/fetch.log.

2026-10-28T20:44:22.195840-04:00 (v1.4): firecracker/kani-harness-02: exit 0, 25.00 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-02.log.

2026-10-28T20:45:49.659998-04:00 (v1.4): firecracker/kani-harness-03: exit 1, 87.19 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-03.log.

2026-10-28T20:46:48.857932-04:00 (v1.4): anvil/verus: exit 101, 152.38 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus.log.

2026-10-28T20:47:40.236108-04:00 (v1.4): Anvil full run: pinned Verus built successfully (vstd 2060/0); Anvil dependencies built and verus-tla verified 139/0. Project verifier was SIGKILLed without an Anvil summary while Kani was using ~6 GiB. Resource-constrained result, confounded by overlapping VM workloads; do not claim Anvil-alone OOM or attribute library counts to Anvil. Required module fallback will run after Kani finishes. No nearby Verus commit retries: pinned toolchain built; resource pressure is not a toolchain incompatibility.

2026-10-28T20:48:35.516667-04:00 (v1.4): firecracker/kani-harness-04: exit 0, 165.59 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-04.log.

2026-10-28T20:51:10.356580-04:00 (v1.4): firecracker/kani-harness-05: exit 0, 154.62 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-05.log.

2026-10-28T20:54:59.900975-04:00 (v1.4): firecracker/kani-harness-06: exit 0, 229.33 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-06.log.

2026-10-28T20:56:44.579121-04:00 (v1.4): firecracker/kani-harness-07: exit 0, 104.46 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-07.log.

2026-10-28T20:58:16.332504-04:00 (v1.4): firecracker/kani-harness-08: exit 0, 91.55 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-08.log.

2026-10-28T20:58:44.413288-04:00 (v1.4): firecracker/kani-harness-09: exit 0, 27.87 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-09.log.

2026-10-28T20:59:17.435772-04:00 (v1.4): firecracker/kani-harness-10: exit 0, 32.82 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-10.log.

2026-10-28T21:01:08.664242-04:00 (v1.4): firecracker/kani-harness-11: exit 0, 111.00 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-11.log.

2026-10-28T21:01:50.543107-04:00 (v1.4): firecracker/kani-harness-12: exit 0, 41.67 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-12.log.

2026-10-28T21:02:11.929851-04:00 (v1.4): firecracker/kani-harness-13: exit 0, 21.13 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-13.log.

2026-10-28T21:02:34.161839-04:00 (v1.4): firecracker/kani-harness-14: exit 0, 22.02 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-14.log.

2026-10-28T21:02:55.150108-04:00 (v1.4): firecracker/kani-harness-15: exit 0, 20.77 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-15.log.

2026-10-28T21:03:17.825723-04:00 (v1.4): firecracker/kani-harness-16: exit 0, 22.46 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-16.log.

2026-10-28T21:03:41.345700-04:00 (v1.4): firecracker/kani-harness-17: exit 0, 23.31 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-17.log.

2026-10-28T21:03:51.255526-04:00 (v1.4): Grounding discrepancy: archived Anvil .github/workflows/ci.yml has a full-verification job running cargo verus verify --lib -- --rlimit 100, not the plan's stated separate per-controller CI jobs. build.md explicitly documents --verify-only-module. Follow the authoritative plan's per-module resource fallback regardless; do not infer changed CI topology or run e2e jobs.

2026-10-28T21:04:48.371382-04:00 (v1.4): firecracker/kani-harness-18: exit 0, 66.81 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-18.log.

2026-10-28T21:07:44.339550-04:00 (v1.4): firecracker/kani-harness-19: exit 0, 175.75 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-19.log.

2026-10-28T21:09:35.843503-04:00 (v1.4): Conservative acceptance resolution: actual successful Kani pilot is rejected by the existing deliberately closed command grammar (requires exactly cargo kani --harness ID --output-format=regular --unwind N). CI/-Z commands and source-only unwind attributes are not that command. Likewise Cargo Verus/module invocations are deliberately unsupported, not a newly demonstrated parsing/path bug. Preserve exact actual commands and rejection reasons. Do not fabricate explicit bounds or covered-unit mappings, delete flags, or broaden policy. M1 passing-target proven-bounded acceptance remains unmet; raw successes are still real results. No coverage-tool bug-fix commit is justified.

2026-10-28T21:10:24.379792-04:00 (v1.4): firecracker/kani-harness-20: exit 0, 159.83 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-20.log.

2026-10-28T21:10:43.400840-04:00 (v1.4): firecracker/kani-harness-21: exit 0, 18.80 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-21.log.

2026-10-28T21:11:09.789042-04:00 (v1.4): firecracker/kani-harness-22: exit 1, 26.19 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-22.log.

2026-10-28T21:11:16.482896-04:00 (v1.4): firecracker/kani-harness-23: exit 0, 6.49 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-23.log.

2026-10-28T21:11:24.015840-04:00 (v1.4): firecracker/kani-harness-24: exit 0, 7.32 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-24.log.

2026-10-28T21:11:31.493267-04:00 (v1.4): firecracker/kani-harness-25: exit 0, 7.28 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-25.log.

2026-10-28T21:11:40.158723-04:00 (v1.4): firecracker/kani-harness-26: exit 0, 8.46 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-26.log.

2026-10-28T21:11:47.922120-04:00 (v1.4): firecracker/kani-harness-27: exit 0, 7.55 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-27.log.

2026-10-28T21:11:55.411939-04:00 (v1.4): firecracker/kani-harness-28: exit 0, 7.29 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-28.log.

2026-10-28T21:12:02.803456-04:00 (v1.4): firecracker/kani-harness-29: exit 0, 7.18 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-29.log.

2026-10-28T21:12:10.650252-04:00 (v1.4): firecracker/kani-harness-30: exit 0, 7.63 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-30.log.

2026-10-28T21:12:18.853523-04:00 (v1.4): firecracker/kani-harness-31: exit 0, 7.99 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-31.log.

2026-10-28T21:12:27.102161-04:00 (v1.4): firecracker/kani-harness-32: exit 0, 8.03 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-32.log.

2026-10-28T21:12:33.768279-04:00 (v1.4): firecracker/kani-harness-33: exit 0, 6.45 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-33.log.

2026-10-28T21:12:54.220174-04:00 (v1.4): firecracker/kani-harness-34: exit 0, 20.24 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-34.log.

2026-10-28T21:13:00.957350-04:00 (v1.4): firecracker/kani-harness-35: exit 0, 6.53 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-35.log.

2026-10-28T21:13:27.219300-04:00 (v1.4): firecracker/kani-harness-36: exit 0, 26.06 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-36.log.

2026-10-28T21:14:01.177905-04:00 (v1.4): firecracker/kani-harness-37: exit 0, 33.74 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-37.log.

2026-10-28T21:14:34.519706-04:00 (v1.4): firecracker/kani-harness-38: exit 0, 33.13 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-38.log.

2026-10-28T21:14:41.407968-04:00 (v1.4): firecracker/kani-harness-39: exit 0, 6.60 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/kani-harness-39.log.

2026-10-28T21:15:28.183515-04:00 (v1.4): M1 batch complete. Corrected the new, not-yet-committed run adapter's raw accounting for two successful substring filters (verify_from_bytes and verify_src_mac each also verifies its longer-named sibling). All checked names retained; unique source harnesses counted once. Regression script test_accounting.py passes and rejects missing target, mismatched totals, mixed failure and nonzero exits. This is orchestration accounting, not a formal-coverage validator change. Actual totals: {'pass': 37, 'not-compiled-on-aarch64': 1, 'fail': 2}. Both real failed harnesses have CBMC OOM diagnostics. All original logs/receipts preserved.

2026-10-28T21:15:28.792708-04:00 (v1.4): firecracker/versions-nightly: exit 0, 0.32 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/versions-nightly.log.

2026-10-28T21:15:59.349480-04:00 (v1.4): M1 local accounting/report checks passed: 37 unique passes, 2 CBMC OOMs, 1 x86-only not compiled; all 42 receipts passed to report and rejected with reasons, no bounded/unbounded credit. Manifest complete with actual nightly Rust version and package closure. Commit M1 now; later milestones retain all coverage policy unchanged.

2026-10-28T21:16:07.493532-04:00 (v1.4): anvil/verus-crds: exit 1, 0.22 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-crds.log.

2026-10-28T21:16:07.780636-04:00 (v1.4): anvil/verus-external_shim_layer: exit 1, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-external_shim_layer.log.

2026-10-28T21:16:08.060214-04:00 (v1.4): anvil/verus-kubernetes_api_objects: exit 1, 0.19 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-kubernetes_api_objects.log.

2026-10-28T21:16:08.350143-04:00 (v1.4): anvil/verus-kubernetes_cluster: exit 1, 0.20 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-kubernetes_cluster.log.

2026-10-28T21:16:08.611007-04:00 (v1.4): anvil/verus-reconciler: exit 1, 0.17 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-reconciler.log.

2026-10-28T21:16:08.878160-04:00 (v1.4): anvil/verus-shim_layer: exit 1, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-shim_layer.log.

2026-10-28T21:16:09.151878-04:00 (v1.4): anvil/verus-state_machine: exit 1, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-state_machine.log.

2026-10-28T21:16:09.447342-04:00 (v1.4): anvil/verus-tla_demo: exit 1, 0.20 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-tla_demo.log.

2026-10-28T21:16:09.744260-04:00 (v1.4): anvil/verus-unit_tests: exit 1, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-unit_tests.log.

2026-10-28T21:16:10.024364-04:00 (v1.4): anvil/verus-vstd_ext: exit 1, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-vstd_ext.log.

2026-10-28T21:16:10.312812-04:00 (v1.4): anvil/verus-vdeployment_controller: exit 1, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-vdeployment_controller.log.

2026-10-28T21:16:10.605218-04:00 (v1.4): anvil/verus-vreplicaset_controller: exit 1, 0.19 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-vreplicaset_controller.log.

2026-10-28T21:16:10.880353-04:00 (v1.4): anvil/verus-vstatefulset_controller: exit 1, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-vstatefulset_controller.log.

2026-10-28T21:16:11.164787-04:00 (v1.4): anvil/verus-rabbitmq_controller: exit 1, 0.19 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-rabbitmq_controller.log.

2026-10-28T21:16:11.448721-04:00 (v1.4): anvil/verus-composition: exit 1, 0.19 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-composition.log.

2026-10-28T21:16:33.062837-04:00 (v1.4): anvil/focus-help: exit 0, 0.19 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/focus-help.log.

2026-10-28T21:16:37.735723-04:00 (v1.4): anvil/focus-source: exit 0, 0.18 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/focus-source.log.

2026-10-28T21:17:16.746615-04:00 (v1.4): M2 documented per-module verify invocations all fail before source verification: pinned cargo-verus says "Partial verification must use cargo verus focus". Retain all 15 failures and null counts. Re-probed cargo verus focus --help inside the pinned container: "Verify only crate contents, without dependencies"; args after -- forwarded to Verus and focus defaults forwarding to roots. Conservative setup correction: same pinned binary, same --lib, --rlimit 100 and --verify-only-module scopes, using documented focus verb. No tool/validator/threshold/escape changes, no version override or nearby commit. Original attempts remain in previous_attempts and every receipt is reported. Apportion remaining six-hour budget across pending modules (maximum 30 minutes each) so every requested scope is attempted.

2026-10-28T21:18:23.094326-04:00 (v1.4): anvil/verus-focus-crds: exit 101, 66.20 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-crds.log.

2026-10-28T21:19:16.322103-04:00 (v1.4): anvil/verus-focus-external_shim_layer: exit 101, 53.10 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-external_shim_layer.log.

2026-10-28T21:19:48.552723-04:00 (v1.4): anvil/verus-focus-kubernetes_api_objects: exit 0, 32.09 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-kubernetes_api_objects.log.

2026-10-28T21:19:50.002118-04:00 (v1.4): anvil/verus-focus-kubernetes_cluster: exit 0, 1.32 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-kubernetes_cluster.log.

2026-10-28T21:19:51.446238-04:00 (v1.4): anvil/verus-focus-reconciler: exit 0, 1.32 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-reconciler.log.

2026-10-28T21:19:52.864979-04:00 (v1.4): anvil/verus-focus-shim_layer: exit 0, 1.30 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-shim_layer.log.

2026-10-28T21:19:54.283101-04:00 (v1.4): anvil/verus-focus-state_machine: exit 0, 1.29 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-state_machine.log.

2026-10-28T21:19:55.706721-04:00 (v1.4): anvil/verus-focus-tla_demo: exit 0, 1.30 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-tla_demo.log.

2026-10-28T21:19:57.092462-04:00 (v1.4): anvil/verus-focus-unit_tests: exit 0, 1.26 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-unit_tests.log.

2026-10-28T21:19:58.522618-04:00 (v1.4): anvil/verus-focus-vstd_ext: exit 0, 1.31 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-vstd_ext.log.

2026-10-28T21:19:59.936530-04:00 (v1.4): anvil/verus-focus-vdeployment_controller: exit 0, 1.31 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-vdeployment_controller.log.

2026-10-28T21:20:01.361550-04:00 (v1.4): anvil/verus-focus-vreplicaset_controller: exit 0, 1.30 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-vreplicaset_controller.log.

2026-10-28T21:20:02.769196-04:00 (v1.4): anvil/verus-focus-vstatefulset_controller: exit 0, 1.29 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-vstatefulset_controller.log.

2026-10-28T21:20:04.216090-04:00 (v1.4): anvil/verus-focus-rabbitmq_controller: exit 0, 1.33 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-rabbitmq_controller.log.

2026-10-28T21:20:05.636594-04:00 (v1.4): anvil/verus-focus-composition: exit 0, 1.29 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-focus-composition.log.

2026-10-28T21:20:50.112160-04:00 (v1.4): anvil/selector-help: exit 0, 0.20 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/selector-help.log.

2026-10-28T21:21:02.156726-04:00 (v1.4): anvil/selector-semantics: exit 0, 0.19 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/selector-semantics.log.

2026-10-28T21:21:11.061569-04:00 (v1.4): anvil/selector-recursion: exit 0, 0.19 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/selector-recursion.log.

2026-10-28T21:23:07.207477-04:00 (v1.4): M2 focus retry exposed Cargo cache reuse: after one fresh kubernetes_api_objects root-only run (0 verified/0 errors), later selectors replayed that root's note and emitted no fresh project summary. Recorded unconfirmed/errors, never borrowed counts. Re-probed pinned config.rs and user_filter.rs inside Docker: --verify-only-module excludes descendants, --verify-module matches module prefix and descendants. Setup correction for intended controller/framework subtree coverage: cargo clean -p verifiable-controllers in /out target before every focus run, then same --rlimit 100 with --verify-module. Preserve both original attempts per root. Parsed 221 selectable module paths from the pinned verifier's diagnostic; four Rust-only/cfg-excluded roots (crds, external_shim_layer, shim_layer, unit_tests) have no selectable Verus module and are explicitly skipped with prior errors retained. No source, verifier, validator, bounds, or host-resource changes. This removes stale setup output, not proof failures.

2026-10-28T21:23:08.927712-04:00 (v1.4): anvil/verus-subtree-kubernetes_api_objects: exit 0, 1.58 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-kubernetes_api_objects.log.

2026-10-28T21:23:10.558550-04:00 (v1.4): anvil/verus-subtree-kubernetes_cluster: exit 0, 1.52 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-kubernetes_cluster.log.

2026-10-28T21:23:12.188587-04:00 (v1.4): anvil/verus-subtree-reconciler: exit 0, 1.50 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-reconciler.log.

2026-10-28T21:23:13.806056-04:00 (v1.4): anvil/verus-subtree-state_machine: exit 0, 1.49 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-state_machine.log.

2026-10-28T21:23:15.427144-04:00 (v1.4): anvil/verus-subtree-tla_demo: exit 0, 1.50 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-tla_demo.log.

2026-10-28T21:23:17.049026-04:00 (v1.4): anvil/verus-subtree-vstd_ext: exit 0, 1.51 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-vstd_ext.log.

2026-10-28T21:23:18.673459-04:00 (v1.4): anvil/verus-subtree-vdeployment_controller: exit 0, 1.51 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-vdeployment_controller.log.

2026-10-28T21:23:20.296456-04:00 (v1.4): anvil/verus-subtree-vreplicaset_controller: exit 0, 1.50 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-vreplicaset_controller.log.

2026-10-28T21:23:21.917710-04:00 (v1.4): anvil/verus-subtree-vstatefulset_controller: exit 0, 1.51 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-vstatefulset_controller.log.

2026-10-28T21:23:23.543565-04:00 (v1.4): anvil/verus-subtree-rabbitmq_controller: exit 0, 1.50 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-rabbitmq_controller.log.

2026-10-28T21:23:25.154556-04:00 (v1.4): anvil/verus-subtree-composition: exit 0, 1.49 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-subtree-composition.log.

2026-10-28T23:50:58.009845-04:00 (v1.4): Resume per Andrew/Dudley review: previous subtree cleans targeted /out/target/debug while focus uses /out/target/verus-partial; stale exits are setup failures, never verification evidence. Correct clean target before every focus command. Preserve all previous attempts; require fresh Checking verifiable-controllers, own selected module notes, and actual project summary for verified outcome. M1 commit stays intact. Andrew explicitly authorizes new M2b closed Kani feature grammar after M2, superseding only the earlier no-expansion fence. No other fences change.

2026-10-28T23:51:21.270942-04:00 (v1.4): anvil/verus-fresh-kubernetes_api_objects: exit 0, 23.08 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-kubernetes_api_objects.log.

2026-10-28T23:53:04.101064-04:00 (v1.4): firecracker/m2b-help: exit 0, 0.23 s; image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0; log runs/v1.4/firecracker/m2b-help.log.

2026-10-28T23:53:39.139404-04:00 (v1.4): anvil/verus-fresh-kubernetes_cluster: exit 0, 137.75 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-kubernetes_cluster.log.

2026-10-28T23:54:01.665347-04:00 (v1.4): anvil/verus-fresh-reconciler: exit 0, 22.40 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-reconciler.log.

2026-10-28T23:54:24.304339-04:00 (v1.4): anvil/verus-fresh-state_machine: exit 0, 22.52 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-state_machine.log.

2026-10-28T23:54:46.683926-04:00 (v1.4): anvil/verus-fresh-tla_demo: exit 0, 22.24 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-tla_demo.log.

2026-10-28T23:55:09.292871-04:00 (v1.4): anvil/verus-fresh-vstd_ext: exit 0, 22.48 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-vstd_ext.log.

2026-10-28T23:56:27.983456-04:00 (v1.4): anvil/verus-fresh-vdeployment_controller: exit 0, 78.57 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-vdeployment_controller.log.

2026-10-28T23:58:32.712153-04:00 (v1.4): anvil/verus-fresh-vreplicaset_controller: exit 0, 124.60 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-vreplicaset_controller.log.

2026-10-29T00:00:17.450813-04:00 (v1.4): anvil/verus-fresh-vstatefulset_controller: exit 0, 104.61 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-vstatefulset_controller.log.

2026-10-29T00:01:59.282503-04:00 (v1.4): anvil/verus-fresh-rabbitmq_controller: exit 0, 101.70 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-rabbitmq_controller.log.

2026-10-29T00:02:22.926139-04:00 (v1.4): anvil/verus-fresh-composition: exit 0, 23.52 s; image sha256:4a2a61e3b1e54ccf6227adba84935fef774ac3bb7e7302b4ab2555b7c86cdae2; log runs/v1.4/anvil/verus-fresh-composition.log.

2026-10-29T00:03:28.125099-04:00 (v1.4): M2 acceptance locally checked: 11 fresh own-scope project summaries total raw verifier count 935 verified (formal-coverage credits 0 of 53 receipts: cargo verus focus is outside the Verus receipt grammar by design, so measured proven is 0.00%)/0 errors; four roots absent from pinned selectable-module inventory skipped. All 53 retained run receipts passed to report and rejected with reasons under unchanged Verus policy. Historical survey regression and fresh-scope regression pass. M1 history left intact; proceeding to authorized M2b.

2026-10-29T00:08:36.658372-04:00 (v1.4): M2b implementation: closed feature allowlist, positive CLI bounds/source unwind/default checked model, source-derived stubs/contracts/vtable scopes and unique pretty-name resolution. Original tests pass unchanged; new feature/negative tests pass. M1 files retained byte-for-byte; normalized caller receipts and reports go in m2b/. No Kani rerun: two substring filters need unallowlisted --exact, so remain rejected; expected-panic source/log also remains rejected. Scope-omission receipt claims fail deny_unknown_fields. Expected eligible credit is 34/37 raw passes, not an invented 37/37; runtime targets need independent explicit mappings and are not inferred.

2026-10-29T00:25:10.409805-04:00 (v1.4): M2b acceptance re-verified after external wrapper interruption: build, 11 test suites (0 failed), clippy clean; cargo fmt --check and git diff --check exit 0. Replay artifacts reconcile 34 credited/37 raw passes, scopes for all 40 source harnesses (null with reasons for uncredited), runtime targets 0/unbounded credit 0. Original M1 manifest/harnesses/summary byte-identical to 1bb0948. Two ambiguous logs and expected-panic remain rejected; no verifier rerun or extra flag exception. Commit authorized M2b now.

2026-10-29T00:25:25.915885-04:00 (v1.4): vest/m3-fetch: exit 0, 0.60 s; image sha256:620dbcd124499c59e2406d3741574b5c5838cf9eb9656f0c3a03948f79b02959; log runs/v1.4/vest/m3-fetch.log.

2026-10-29T00:25:33.741672-04:00 (v1.4): anvil/m3-fetch: exit 0, 0.71 s; image sha256:620dbcd124499c59e2406d3741574b5c5838cf9eb9656f0c3a03948f79b02959; log runs/v1.4/anvil/m3-fetch.log.

2026-10-29T00:25:43.890968-04:00 (v1.4): firecracker/m3-fetch: exit 0, 0.39 s; image sha256:620dbcd124499c59e2406d3741574b5c5838cf9eb9656f0c3a03948f79b02959; log runs/v1.4/firecracker/m3-fetch.log.

2026-10-29T00:26:43.468987-04:00 (v1.4): M3 complete locally: three fresh cargo +1.98.0 fetch --locked container steps exit 0, no builds. Prior manifests restored unchanged; m3-manifest.json retains each actual command/log hash/lock hash. Offline inventories: Vest 121 runtime packages / 1035488 SLoC; Anvil 209 / 2974275; Firecracker 78 / 1718910. All build/runtime source sizes known. Before/after table in docs/survey/v1.4-dependencies.json; Vest proof numerator/denominator unchanged. No host third-party execution, source edits or verifier reruns.

2026-10-29T00:30:47.385701-04:00 (v1.4): M4 acceptance complete locally: extended survey checker verifies archive bytes, pinned manifests and permitted mounts, all log/receipt hashes, fresh matching Anvil scope summaries, Kani raw-versus-credited outcomes with source-derived stub/contract/vtable scopes, dependency rows and historical Vest/scratch invariants. cargo test --frozen, clippy --frozen -- -D warnings, fmt --check, git diff --check all exit 0 (runs/v1.4/final-*.log; checks.json). Docs retain the 34/37 limitation and zero runtime mapping credit, not a bare bounded count. Independent acceptance not run. Existing untracked DUDLEY-STATUS.md was not authored, edited or staged. No push, host installs, Docker setting changes, external messages or survey-clone edits.

2026-10-29T13:02:45.562875-04:00 (v1.4.1 M1): Baseline 260bb02 includes the release plan/prompt beyond a07ac51. Baseline cargo test --frozen passed. Vest current release replay took 8.19 s with identical summary. Tool headlines show Vest 39 trusted/520 SLoC/5110 escapes; Anvil 401/2732/21636; Firecracker 0/0/27142. Receipt scope and 412 first-party unsafe sites are separate. Scratch headline uses its retained whole inventory. No rule or percentage changed. Historical container helper lacks network isolation; release verifier will explicitly use --network none.

2026-10-29T13:05:05.458605-04:00 (v1.4.1 M2): 51 tests pass, 1 ignored. Three offline report JSON replays compared recursively against pre-change artifacts; differences only replace the combined rejection message. Markdown metadata parenthetical now appears only on fallback. Decisions, summaries and percentages identical. M3 initial attempt stopped after docker stats revealed concurrently started release-negative verifier containers owned by another agent; do not treat resource-contended attempt as the required idle rerun.

2026-10-29T13:13:45.372090-04:00 (v1.4.1 M3): Pinned image sha256:07252577f6970a9acd106f521ea8f1ad0534d7220826e5e3bd96363ecf84cad0, --network none, original flags/source/bounds, exact read-only source + project-only output mounts. First attempt stopped when another agent started verifiers. Second started without competing verifier and passed in 200.84 s; later Nix builds overlapped, so idle-VM timing criterion is not claimed. Receipt accepted unchanged grammar: 35/38 bounded harnesses, 38 pass/1 OOM/1 architecture skip, runtime targets 0, all unbounded percentages preserved. Original M1/M2b artifacts retained. Earlier iovec OOM is environmental: identical source/model now passes. Corrected scratch prose to quote the current v1.2 headline (2472 inventory escapes), distinct from old vendor-closure reports.

2026-10-29T13:20:02.587086-04:00 (v1.4.1 M4): All 13 scratch archives match the supplied pin list; git archive only, never the concurrent working tree. Replayed historical vendor closure to preserve every published status SLoC total, and kept the separate v1.2 runtime-closure table. New reports use relative artifact paths. Anvil metadata exports require empty [workspace] isolation inside this repository, leaving the original archive and Rust sources intact; exact runtime/dependency totals preserved. Anvil headers crate has known source size but unconfirmed unsafe count; checker preserves that distinction. Bundle includes raw logs/source archives/offline source caches; tracked full reports removed, tracked tree about 3.4 MB. All three missing-bundle invocations give one clear fetch/extract line. Raw verifier whitespace retained; .gitattributes excludes *.log whitespace checks. Survey and dogfood audits passed; performance replay and fresh clone acceptance pending.

2026-10-29T13:22:42.379225-04:00 (v1.4.1 M4): Completed all three local checkers. Sixteen full replays preserve unit identity/category/security/status/SLoC, summary totals, credited IDs and receipt decisions/reasons. Bundle 409470978 bytes, 56030 relative paths, SHA-256 sidecar in ../release; no upload. All newly generated reports/case paths have no local-user path prefixes. Fresh-clone reproduction will use the final M5 tree and final bundle after public path redaction.

2026-10-29T13:26:17.368603-04:00 (v1.4.1 M5): Moved build records to dev/ and fixed Markdown file links. Public README shortened; MIT license and Cargo metadata added. Known private paths/names redacted from tracked records. Receipt executable paths normalized to verus without changing flags, source hashes or raw logs; original receipt/manifest metadata retained in the ignored evidence bundle for audit. Refreshed affected presentation hashes and source provenance (Cargo license metadata only). Public scanner exits 0 and its key-marker/case-insensitive smoke checks pass. cargo test --frozen: 51 passed/1 ignored; clippy -D warnings and fmt --check pass; cargo package list includes LICENSE; git diff --check passes. Survey and dogfood checks pass after redaction. Final bundle/fresh clone acceptance pending.

2026-10-29T13:32:39.729251-04:00 (v1.4.1 final M3 acceptance): After other-agent builds finished, repeated iovec with the identical pinned image/flags and --network none on the idle VM. Passed in 96.58 s; no competing builds/verifiers observed at start, during, or after. Start snapshot retained in release-idle-start.txt. Earlier shared-VM pass retained separately. Existing grammar accepts final receipt; canonical replay preserves every unit status/SLoC and summary, still 35/38 harnesses, runtime targets 0. This final idle run satisfies the earlier outstanding timing condition.

2026-10-29T13:37:41.670282-04:00 (v1.4.1 final local acceptance): SHA-256 verified final bundle 409756240 bytes. Fresh local clone under .tmp, staged final candidate applied (no author target/ or sibling clones), bundle extracted, cargo build --release --frozen passed in 8.09 s. All three checkers pass, including 16 full replays with identical coverage/unit statuses/receipt decisions and all three one-line missing-bundle failures before extraction. Logs copied to logs/release-pass/fresh-clone-*.log. Separate current v1.2 spec-lemmas replay verified every Rust source byte against pinned archive, matched the retained full summary and 2472-escape headline. Final public scan, package LICENSE inclusion, Python compilation, Cargo tests (51/1 ignored), clippy, fmt and diff checks pass; tracked tree about 3.5 MB. Exact raw logs retained; no grammar/threshold/rule changes or published proven percentage movements. Final source changes since clone verification are these acceptance notes only. Independent external acceptance remains for the reviewer; no push, branch creation, identity change, history rewrite, host installs or private-source working-tree reads.
