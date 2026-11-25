# README cleanup notes

## Scope and progress

- Docs-only reorganisation on `main`; only README.md, CHANGELOG.md and this file are in scope.
- Existing untracked logs are left untouched.
- Reviewed the original README, changelog, CLI definitions and evidence adapters.
- Example runs, link checks, release-date sources and deletion audit follow below.

## Build and quick-start verification

- `cargo build --release --locked` — exit 0.
- `./target/release/formal-coverage report --help` — exit 0.
- `./target/release/formal-coverage gate --help` — exit 0.
- `./target/release/formal-coverage report fixtures/inventory` — exit 0.
- `./target/release/formal-coverage gate fixtures/inventory --min-first-party-security-proven 80` — exit 1.
- `./target/release/formal-coverage report fixtures/classification --config fixtures/classification/formal-coverage.toml` — exit 0.

The inventory report and gate use the existing fixture exercised by
`tests/inventory.rs`; no standalone workspace exists under `tests/` or
`examples/`. The explicit-config example uses the existing classification
fixture exercised by `tests/classification.rs`. Its schema-2 entries produce
migration warnings, as documented. The 80% gate is expected to fail because
this fixture has no accepted proof evidence.

The two evidence examples are templates, not executed verifier runs:

```sh
./target/release/formal-coverage report path/to/workspace --verus-log path/to/verus.log
./target/release/formal-coverage report path/to/workspace --fstar-log path/to/fstar.log
```

They require a real workspace plus source-matching logs and `.evidence.json`
receipts. No real matching Verus or F* receipt is supplied for the quick-start
fixtures. Retained historical logs elsewhere do not establish evidence for
these fixtures. Both flags and all other quick-start options were checked with
`report --help`; the threshold was also checked with `gate --help`. These help
checks establish CLI syntax, not evidence acceptance. No proof credit or real
verifier execution was tested. The CI pointer refers to the retained consumer
workflow; no remote workflow was run.

## Relative-link and content checks

- Checked README.md and CHANGELOG.md with a Python standard-library link check: 35 relative link occurrences, 0 errors. Each target exists; each fragment matches a Markdown heading slug, including links into other documents. Remote URLs were excluded.
- Compared the original title, badges and intro against the new prefix: exact match.
- Compared the complete original Use in CI section against the retained section: exact match.
- `cargo test --locked --test readme` — exit 0; 1 test passed, including all TOML blocks and the shipped-scope schema.

## Source grounding

- `src/main.rs`: only Verus, Kani and F* evidence inputs reach the adapters;
  flags are repeatable vectors shared by `report` and `gate`. There is no
  evidence adapter for the other frameworks named in the README.
- `src/evidence.rs`: source-bound complete Verus receipts assign `proven` to
  eligible runtime units and `verified` to proof code; trust and escapes remain
  visible. Kani uses its closed command grammar and eligible harness scope.
- `src/kani.rs`: records unwind bounds and model assumptions; unsupported
  command/attribute forms do not establish scope.
- `src/fstar.rs`: validates source and extraction hashes, strict checking and
  unique mappings; hax extraction is trusted. Interfaces and patched models
  cannot earn Rust proof credit. Backend-specific `proverif`, `coq` and
  `ssprove` trust paths are reported separately.
- `src/hax.rs`: exact-path annotation whitelist and proof sizing, including
  exclusion of ProVerif/SSProve from F* proof cost.
- Corrected the old blanket statement that trait impl names are unsupported:
  named trait methods remain unsupported, while typed trait instance records
  have the guarded mapping described by the current adapter.

## Changelog dates and deduplication

Dates below are documentation/implementation commit dates from
`git log --date=short`, not inferred calendar dates. The 1.8.1 entry uses its
completed replay/documentation commit, rather than the later tag publication.

| Version | Date | Git source |
| --- | --- | --- |
| 1.9.1 | 2026-11-25 | `99ccc33` (also `refs/tags/v1.9.1`) |
| 1.9.0 | 2026-11-24 | `13d7be3` (also `refs/tags/v1.9.0`) |
| 1.8.5 | 2026-11-24 | `a995961` (also `refs/tags/v1.8.5`) |
| 1.8.4 | 2026-11-24 | `bf0a590`; `refs/tags/v1.8.4` also resolves to that date |
| 1.8.1 | 2026-11-21 | `b70c00b` (completed replay and documentation) |
| 1.8 | 2026-11-21 | `e74b116` (completed paired replays and documentation) |
| 1.7.3 | 2026-11-21 | `1362183` (also `refs/tags/v1.7.3`) |
| 1.7 | 2026-11-02 | `ef4333a` (README F* release section); `8afd9df` (real replay) |
| 1.6 | 2026-10-31 | `5d028d1` (README proof-cost section); `e5d3021` (implementation) |

- Kept one heading per requested version, newest first.
- Merged 1.9.1 precedence, unique-unit, canary and replay details into its existing
  entry. Existing coverage figures and naming/refusal bullets remain.
- Merged the informational CI proof-cost statement and CI links into 1.9.0's
  existing measurement bullet; did not repeat its kit/action descriptions.
- Added the seven missing release entries and retained every README comparison
  and versioned-doc link in the appropriate changelog entry.
- Kept current extraction declarations and uppercase-name mapping in the README,
  removing their incidental 1.8.2/1.8.3 version labels.

## README paragraph deletion audit

No README paragraph was deleted outright without retaining its substantive
content in README.md or CHANGELOG.md. Release prose was moved or split as follows:

| Original block / paragraph opening | Destination |
| --- | --- |
| “Hax's trailing numeric escapes…” | hax/F* names; merged 1.9.1 changelog entry |
| “The CI kit…” | Use in CI and quick-start pointer; merged 1.9.0 changelog entry |
| “Numbered `impl_<N>__method`…” | hax/F* names; 1.8.5 changelog entry |
| “Optional `[ship]` configuration…” | Shipped scope; 1.8.4 changelog entry |
| “This fails toward counting…” | Shipped scope; 1.8.4 changelog entry |
| “In first-party hax crates…” | Extraction-only proof code; 1.8.1 changelog entry |
| “Proof ratios now include…” | Proof cost ratio and component fields; 1.8 changelog entry |
| “SLoC now excludes…” | Dependency sizing; 1.7.3 changelog entry |
| “Build with stable Rust…” and its shell block | Quick start; stable toolchain wording updated |
| “JSON report schema is 4…” | Proof cost ratio; historical additions also recorded under 1.8/1.8.1 |
| “cfg implication uses…” | Extraction-only proof code; historical unchanged-rules sentence in 1.8.1 changelog |
| “The identifier graph…” | Limitations; removed the historical M2b label, retained Kani grammar, regression tests and all replay caveats |
| “Licensed under MIT.” | Final License section |
| “v1.7 adds repeatable…” | hax/F* specifics; 1.7 changelog entry |

All other reference paragraphs and TOML blocks were retained under topic
headings. Removed only release-heading wrappers and transition wording such as
“now”, “adds” and “new”; those changes are described in the changelog. Removed
“(1.100.0 for v1.8.4)” because current stable Rust is the intended requirement.
Moved the misplaced whole-crate bullet back beside the other sizing modes.
The obsolete changelog sentence directing readers to README release history
was replaced by the dated entries.

## Verification limits and handoff

- The basic report records a conservative lockfile fallback because the
  inventory fixture has no Cargo.lock. The gate's reported failure is the
  documented 0% versus 80% first-party threshold.
- No code, tests, version metadata, branches or tags were changed. No push or
  remote workflow was requested or run. Existing untracked logs remain outside
  the commit.
- `scripts/check-public.sh` was not run, as requested.
- No independent reviewer has run the acceptance checks in this session.
- Final commit and the post-commit README-READY marker follow the requested
  sequence; that final marker will be a working-tree addition after the commit.

- Final structural check: nine unique dated release entries; no README version headings; five quick-start usage blocks plus build; License last with the requested three final links.
- `git diff --check` — exit 0.

Commit: `a702476` — `docs: reorganise README; move release notes to CHANGELOG`.
The commit hook exited 0; it skipped Rust checks because no Rust files were staged.
The release build and README test above were run separately; fmt/clippy and the full suite were not run.
This post-commit handoff and marker remain uncommitted by the requested ordering.

README-READY
