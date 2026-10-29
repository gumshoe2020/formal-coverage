# Scratch-server formal coverage

2026-10-28 v1.2. All thirteen pinned branch tips re-exported with `git archive`
under this repository's `target/v1.2-scratch/`; original scratch-server checkouts
and containers remain unchanged. Retained source-bound verifier receipts were
replayed, with every first-party unit's identity, size, classification and status
checked against the historical reports. No new verifier or application runs.

First-party security on **header-policy and spec-lemmas remains 644/1,363 SLoC
(47.25%)**. The report's literal first line now shows that figure. Spec-lemmas
third-party runtime security is **336,654 SLoC**, below the old combined
1,626,171 total; combined runtime security is 338,017 SLoC. Build-time code is
listed separately and can affect the binary: it is trusted, not measured by
runtime coverage. Full package source SLoC still includes cfg/test/platform code.

Closure `metadata` below means `cargo metadata --format-version 1 --frozen
--filter-platform aarch64-apple-darwin`, walked from all workspace members over
normal edges, separating proc macros and build dependencies and checking optional
activation. All runs record `cargo 1.99.1 (5f94df478 2026-08-27) (Homebrew)` in the
[results artifact](scratch-server/v1.2-results.json); the exact version there is
authoritative. `main` and `nix-scratch` fall back because `atomic-waker@1.1.2`
is not cached. No downloads were attempted beyond Cargo's frozen refusal.
Their full lockfile package sets remain, but source sizes are unconfirmed here;
the table flags unknown sizes and their security gates fail closed. A smaller
**measured** SLoC value on those rows is not a smaller dependency package set.
Historical source sizes from outside this milestone's allowed directories were
not reused as new source evidence.

The requested literal acceptance-C comparison **does not pass**. The exact
`cargo tree -e normal --target <host> --frozen --prefix none` package set includes
15 proc-macro/build packages excluded by the plan's separate build-time rule.
Metadata also enables `rand` defaults through dev-feature unification, retaining
`chacha20@0.10.2` and `getrandom@0.4.3` on normal paths while the normal-only tree
omits them. The conservative choice is to retain those two packages rather than
shrink metadata's enabled normal graph. Disabled weak optional edges were ruled
out from manifest definitions and resolved features, not guessed active.
Both complete sets, their exact difference, the first report line and commands
are recorded in the results artifact. C's 47.25% and lower-SLoC checks pass;
no exact-set equality is claimed. Satisfying both the literal equality and the
explicit proc-macro exclusion requires a plan correction.

Reproduce after an offline release build:
`CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" python3 scripts/target_deps.py`.
This writes all reports/logs locally and exits **1** when C differs, after
finishing the complete table. It never fetches, builds dependencies or runs
scratch-server scripts. For early branches only, an empty `[workspace]` is added
to the exported manifest so Cargo does not inherit this enclosing workspace;
dependency declarations and all Rust source bytes remain unchanged.

Both runtime blocks have first-party, third-party and combined figures in each
report. The following table gives combined measured runtime totals/statuses;
the unchanged first-party table below supplies the first-party view. All proof
code is excluded from runtime denominators.

| Branch (commit) | Closure used | SECURITY-RELEVANT measured SLoC | proven / bounded / upstream / tested / unverified / hazard % | EVERYTHING ELSE SLoC | proven / bounded / upstream / tested / unverified / hazard % | Unknown runtime dependency sizes |
| --- | --- | ---: | --- | ---: | --- | ---: |
| main (`ce8baf1da131`) | lockfile (fallback) | 31 | 0.0000 / 0.0000 / 0.0000 / 0.0000 / 100.0000 / 0.0000 | 15 | 0.0000 / 0.0000 / 0.0000 / 0.0000 / 100.0000 / 0.0000 | 53 |
| nix-scratch (`3747f1e07822`) | lockfile (fallback) | 31 | 0.0000 / 0.0000 / 0.0000 / 0.0000 / 100.0000 / 0.0000 | 15 | 0.0000 / 0.0000 / 0.0000 / 0.0000 / 100.0000 / 0.0000 | 53 |
| verified-boundary (`bc2c73b03b6e`) | metadata | 51,988 | 0.2308 / 0.0000 / 0.0000 / 0.0731 / 99.6961 / 0.0000 | 291 | 0.0000 / 0.0000 / 0.0000 / 87.2852 / 12.7148 / 0.0000 | 0 |
| browser-whitelist (`3a3cf20dad01`) | metadata | 52,037 | 0.2402 / 0.0000 / 0.0000 / 0.1499 / 99.6099 / 0.0000 | 403 | 0.0000 / 0.0000 / 0.0000 / 90.8189 / 9.1811 / 0.0000 | 0 |
| https (`969540cf8dd5`) | metadata | 337,246 | 0.0371 / 0.0000 / 0.0000 / 0.0572 / 99.9057 / 0.0000 | 651 | 0.0000 / 0.0000 / 0.0000 / 88.9401 / 11.0599 / 0.0000 | 0 |
| https-headers (`131f109a5b5a`) | metadata | 337,252 | 0.0380 / 0.0000 / 0.0000 / 0.0581 / 99.9039 / 0.0000 | 681 | 0.0000 / 0.0000 / 0.0000 / 89.4273 / 10.5727 / 0.0000 | 0 |
| idle-connections (`a3952e45725d`) | metadata | 337,315 | 0.0379 / 0.0000 / 0.0000 / 0.0608 / 99.9013 / 0.0000 | 803 | 0.0000 / 0.0000 / 0.0000 / 91.4072 / 8.5928 / 0.0000 | 0 |
| proven-transport (`42b6565a2b69`) | metadata | 337,505 | 0.0933 / 0.0000 / 0.0000 / 0.0604 / 99.8462 / 0.0000 | 1,005 | 0.0000 / 0.0000 / 0.0000 / 91.5423 / 8.4577 / 0.0000 | 0 |
| firefox-headers (`39a1d2839722`) | metadata | 337,506 | 0.0936 / 0.0000 / 0.0000 / 0.0604 / 99.8459 / 0.0000 | 1,022 | 0.0000 / 0.0000 / 0.0000 / 91.6830 / 8.3170 / 0.0000 | 0 |
| chromium-headers (`df3f49ef9303`) | metadata | 337,542 | 0.1016 / 0.0000 / 0.0000 / 0.0631 / 99.8353 / 0.0000 | 1,195 | 0.0000 / 0.0000 / 0.0000 / 92.8870 / 7.1130 / 0.0000 | 0 |
| pragma-header (`6ee509803195`) | metadata | 337,544 | 0.1019 / 0.0000 / 0.0000 / 0.0634 / 99.8347 / 0.0000 | 1,218 | 0.0000 / 0.0000 / 0.0000 / 93.0213 / 6.9787 / 0.0000 | 0 |
| header-policy (`3fe444ddf1c4`) | metadata | 338,017 | 0.1905 / 0.0000 / 0.0000 / 0.0888 / 99.7207 / 0.0000 | 1,546 | 0.0000 / 0.0000 / 0.0000 / 93.2730 / 6.7270 / 0.0000 | 0 |
| spec-lemmas (`2fa5eb78a6f9`) | metadata | 338,017 | 0.1905 / 0.0000 / 0.0000 / 0.0888 / 99.7207 / 0.0000 | 1,642 | 0.0000 / 0.0000 / 0.0000 / 93.4227 / 6.5773 / 0.0000 | 0 |

## v1.2 runtime and build-time source inventory

Build-only unsafe counts are separate from runtime escape counts. `unknown` does
not mean safe or zero. Dev exclusions are unconfirmed on fallback. Full per-crate
source SLoC and unsafe counts are in the results artifact.

| Branch | Runtime third-party security SLoC / unsafe sites | Build-time SLoC / unsafe sites | Runtime / build packages | Dev-only packages excluded |
| --- | ---: | ---: | ---: | ---: |
| main | unknown | unconfirmed | 53 / 0 | unconfirmed |
| nix-scratch | unknown | unconfirmed | 53 / 0 | unconfirmed |
| verified-boundary | 51,760 / 85 | 224,025 / 645 | 2 / 13 | 15 |
| browser-whitelist | 51,760 / 85 | 224,025 / 645 | 2 / 13 | 15 |
| https | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| https-headers | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| idle-connections | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| proven-transport | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| firefox-headers | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| chromium-headers | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| pragma-header | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| header-policy | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |
| spec-lemmas | 336,654 / 1,565 | 288,449 / 723 | 38 / 16 | 16 |

The following v1.1 sections are historical comparisons using the old lockfile
closure. Their source sizes, unsafe counts and evidence descriptions describe
those earlier runs; the v1.2 tables above supersede their current dependency
measurement. No historical report is overwritten.

## v1.1-b first-party runtime comparison

Same branch sources and config are measured by both tool versions. v1.1-b unions all root receipts in one CLI call and follows conservative module declarations; pinned v1 retains the M5 parser-only receipt scope and includes ghost code. These columns compare both evidence and category improvements, not just a denominator change. Dependencies still use the conservative M5 lockfile closure, so this milestone does not claim target-accurate runtime dependency measurement.

| Branch | Runtime security proven / SLoC | Runtime security proven % | Proof-code SLoC | Proof verified units / total (%) | Verus roots | v1 security proven / SLoC | v1 security proven % |
| --- | ---: | ---: | ---: | --- | ---: | ---: | ---: |
| main | 0/31 | 0.00% | 0 | 0/0 (0.00%) | 0 | 0/31 | 0.00% |
| nix-scratch | 0/31 | 0.00% | 0 | 0/0 (0.00%) | 0 | 0/31 | 0.00% |
| verified-boundary | 120/228 | 52.63% | 68 | 9/9 (100.00%) | 1 | 120/296 | 40.54% |
| browser-whitelist | 125/277 | 45.13% | 73 | 9/9 (100.00%) | 1 | 125/350 | 35.71% |
| https | 125/592 | 21.11% | 73 | 9/9 (100.00%) | 1 | 125/665 | 18.80% |
| https-headers | 128/598 | 21.40% | 76 | 9/9 (100.00%) | 1 | 128/674 | 18.99% |
| idle-connections | 128/661 | 19.36% | 76 | 9/9 (100.00%) | 1 | 128/737 | 17.37% |
| proven-transport | 315/851 | 37.02% | 130 | 17/17 (100.00%) | 3 | 128/981 | 13.05% |
| firefox-headers | 316/852 | 37.09% | 131 | 17/17 (100.00%) | 3 | 129/983 | 13.12% |
| chromium-headers | 343/888 | 38.63% | 144 | 19/19 (100.00%) | 3 | 156/1032 | 15.12% |
| pragma-header | 344/890 | 38.65% | 145 | 19/19 (100.00%) | 3 | 157/1035 | 15.17% |
| header-policy | 644/1363 | 47.25% | 322 | 39/39 (100.00%) | 3 | 502/1685 | 29.79% |
| spec-lemmas | 644/1363 | 47.25% | 4732 | 200/200 (100.00%) | 3 | 502/6095 | 8.24% |

Stage 12/13 runtime security invariant: **MATCH**. Acceptance also compares the combined dependency headline and individual security runtime units. Stage 13 adds 4,410 proof-code SLoC. Its integration-test additions increase everything else, outside this invariant.

## First-party view

| Branch | Security SLoC | Security proven % | Everything else SLoC | Everything else proven % |
| --- | ---: | ---: | ---: | ---: |
| main | 31 | 0.00 | 15 | 0.00 |
| nix-scratch | 31 | 0.00 | 15 | 0.00 |
| verified-boundary | 228 | 52.63 | 291 | 0.00 |
| browser-whitelist | 277 | 45.13 | 403 | 0.00 |
| https | 592 | 21.11 | 651 | 0.00 |
| https-headers | 598 | 21.40 | 681 | 0.00 |
| idle-connections | 661 | 19.36 | 803 | 0.00 |
| proven-transport | 851 | 37.02 | 1005 | 0.00 |
| firefox-headers | 852 | 37.09 | 1022 | 0.00 |
| chromium-headers | 888 | 38.63 | 1195 | 0.00 |
| pragma-header | 890 | 38.65 | 1218 | 0.00 |
| header-policy | 1363 | 47.25 | 1546 | 0.00 |
| spec-lemmas | 1363 | 47.25 | 1642 | 0.00 |

## Third-party unsafe source inventory (v1.1-b historical counts)

| Branch | Security dependency SLoC | Security unsafe sites | Everything else dependency SLoC | Everything else unsafe sites |
| --- | ---: | ---: | ---: | ---: |
| main | 980,044 | 16,363 | 0 | 0 |
| nix-scratch | 980,044 | 16,363 | 0 | 0 |
| verified-boundary | 514,795 | 2,177 | 0 | 0 |
| browser-whitelist | 514,795 | 2,177 | 0 | 0 |
| https | 1,624,808 | 9,765 | 0 | 0 |
| https-headers | 1,624,808 | 9,765 | 0 | 0 |
| idle-connections | 1,624,808 | 9,765 | 0 | 0 |
| proven-transport | 1,624,808 | 9,765 | 0 | 0 |
| firefox-headers | 1,624,808 | 9,765 | 0 | 0 |
| chromium-headers | 1,624,808 | 9,765 | 0 | 0 |
| pragma-header | 1,624,808 | 9,765 | 0 | 0 |
| header-policy | 1,624,808 | 9,765 | 0 | 0 |
| spec-lemmas | 1,624,808 | 9,765 | 0 | 0 |

Site counts include all downloaded Rust sources in the conservative lockfile closure. Detailed reports list each crate separately; no internal-unsafe downgrade of upstream status is applied.

## Reading the progression

`main` and `nix-scratch` have no verifier script, so their locally proven share is
zero. `verified-boundary` introduces the contracted HTTP parser. Browser grammar
changes retain that evidence boundary. HTTPS adds substantial unverified provider,
transport and dependency code; upstream primitive claims do not prove those
adapters or whole crates. `https-headers` extends the grammar; `idle-connections`
adds unverified transport handling. Exact measured changes are in the two tables;
the broad dependency denominator can hide the parser improvement, so the
first-party view is also shown. No proof of end-to-end TLS is implied.

## Method, evidence and limitations

- Reproduce: `CARGO_HOME="$PWD/.cargo-home" cargo build` then
  `python3 scripts/dogfood.py`; acceptance: `python3 scripts/check_dogfood.py`.
  Exports, downloaded sources and isolated verifier toolchains live under /tmp.
  The runner builds v1 commit `4685da4841e9f9bf2e74442d2e9d24e82317ead3`
  under ignored logs/v1-tool for direct comparison; committed `.v1.json` files
  retain its summary, while complete v1 reports remain in ignored logs/.
  Cargo/verifier output lives in this repository's ignored logs/.
- Every available `./verify.sh` is run on its exported branch. Its output is
  retained as `.verify-script.log`. Each root named by that script is then run
  independently, with its own `.verus.log` and `.evidence.json`; all root receipts
  go into **one** v1.1-b CLI call per branch. Stages 8–13 have three roots;
  stages 3–7 have one; main/nix-scratch have none. No historical scratch-server
  verifier logs are reused. JSON records every receipt and its credited units.
- Dependency archives are downloaded from static.crates.io and checked against
  each branch's Cargo.lock SHA256. All registry source SLoC is measured locally,
  including optional/build/test and platform-specific code. This deliberately
  overstates the runtime dependency boundary. Acceptance rejects unknown sizes
  for these branch measurements.
- Syntactic identifier reachability is approximate. Unknown calls/macros include
  all first-party functions. The example config explicitly excludes port parsing,
  standalone healthcheck and test modules; all matches are recorded as overrides.
  Production TLS, transport, dispatch and parser remain security-relevant.
- Verus `proven` covers contracted functions in complete invoked roots and
  conservatively followed plain module files. Spec/proof functions can be
  verified without explicit contracts. `intent.rs` is reached by the parser's
  plain `mod intent;`. Transport's `#[path] mod http;` remains uncredited by
  that receipt, with a warning; the separate parser receipt credits the parser
  and intent module. Non-Verus IO, TLS, entry points and tests remain outside
  these run scopes. Source-span SLoC includes contracts and proof statements. The verifier's query count is not the function
  count. Tests are only a reference signal; no application tests/containers were run.
- Registry entries preserve TLS-PROVIDER.md's **unconfirmed** exact release/target
  correspondence. Partial primitive coverage never promotes whole dependency
  units. See [evidence rules and primary sources](evidence.md).
- The pinned v1 comparison retains the single parser-root receipt used by M5
  and v1.1-a. At stage 12 that gives 502/1685 (29.79%); at stage 13,
  502/6095 (8.24%). The earlier external union of three v1 reports was
  715/1685 and 715/6095. v1.1-b uses all receipts internally and excludes erased
  proof code, so the table explicitly compares different evidence scopes.
  No post-processing union supplies v1.1-b headline numbers.

## v1.1-c distinct dependency unsafe sites

Remeasured all 13 archived branch tips with the release tool and retained,
source-bound verifier receipts. Runtime/proof SLoC, every first-party unit's
attribution, and combined status totals are unchanged. Stages 12/13 remain
644/1,363 runtime security proven SLoC, relying on 0 trusted items (0 SLoC) and
0 first-party escapes; proof code remains 322/4,732 SLoC, all units verified.

Dependency unsafe counts **do change**, because v1.1-b counted token occurrences
and v1.1-c counts distinct `(file, line, kind)` sites. The exact unsafe site set
is unchanged: multiple unsafe tokens on one line now count once. For example,
httparse's `byte_is_allowed` declarations contain both `unsafe fn` and an
`unsafe fn` pointer parameter on the same line in its neon/sse42/avx2 files;
its count therefore changes from 78 occurrences to 75 distinct sites.

These current counts supersede the historical unsafe table above; no coverage
classification or threshold was adjusted to preserve old counts. Full branch
summaries and per-dependency before/after counts are retained in
[v1.1-c-results.json](scratch-server/v1.1-c-results.json). The original v1.1-b
JSON reports remain historical artifacts with their own receipts.

| Branch | Security dependency unsafe occurrences → distinct sites | Everything else dependency occurrences → distinct sites |
| --- | ---: | ---: |
| main | 16,363 → 16,358 | 0 → 0 |
| nix-scratch | 16,363 → 16,358 | 0 → 0 |
| verified-boundary | 2,177 → 2,166 | 0 → 0 |
| browser-whitelist | 2,177 → 2,166 | 0 → 0 |
| https | 9,765 → 9,749 | 0 → 0 |
| https-headers | 9,765 → 9,749 | 0 → 0 |
| idle-connections | 9,765 → 9,749 | 0 → 0 |
| proven-transport | 9,765 → 9,749 | 0 → 0 |
| firefox-headers | 9,765 → 9,749 | 0 → 0 |
| chromium-headers | 9,765 → 9,749 | 0 → 0 |
| pragma-header | 9,765 → 9,749 | 0 → 0 |
| header-policy | 9,765 → 9,749 | 0 → 0 |
| spec-lemmas | 9,765 → 9,749 | 0 → 0 |

## v1.4.1 release evidence

Full per-branch JSON reports are in the release evidence bundle, restored to
`docs/scratch-server/` when extracted. This keeps the tracked build small. The
release pass regenerated those reports from the thirteen commits in `runs.json`,
using their retained vendor/lockfile inputs and receipts. Every status SLoC total
and proof-code total matched the earlier full reports. Their output schema and
messages reflect the current tool; the separate v1.2 runtime-closure table above
is retained as published. `check_dogfood.py` independently reconciles the full
vendor reports, source archives, receipt/log hashes and first-party comparison
table. `check_perf.py` replays them without running scratch-server code.
See [reproduction instructions](evidence.md#reproduce-the-numbers-v141).
