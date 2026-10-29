# PLAN-TARGET-DEPS.md: v1.2, target-accurate dependencies and a first-party headline

Approved by Andrew 2026-10-28 ("Yes please"). Authoritative for this milestone. Follows v1.1-c
(f7fa1f9).

## Problem (verified 2026-10-28)

1. **The dependency set is the whole lockfile.** `src/classify.rs` (~line 111) reads every
   `[[package]]` in Cargo.lock: build, dev/test and optional crates that never reach the shipped
   binary all count. On scratch-server's security side that is about 1.62M of 1.63M SLoC.
2. **The headline buries first-party work.** `report.rs` already computes `first_party` and
   `third_party` splits, but the two headline lines pool them. Scratch-server's first-party
   security code is 47.25% proven (header-policy and spec-lemmas, 1,363 SLoC), while the headline
   reads 0.04%.

## Change

1. **Target-accurate closure.** Run `cargo metadata --format-version 1 --frozen
   --filter-platform <triple>` (no `--no-deps`) and walk `resolve.nodes` from the workspace
   members, following only edges whose `dep_kinds` include a normal (`kind: null`) dependency
   for that target. This gives the **runtime** set. Then:
   - **Build dependencies** (build scripts, proc macros) form a separate **build-time** set:
     listed with SLoC and unsafe counts, never in the runtime denominators. Say plainly in the
     report that build-time code can still affect the binary and is trusted, not measured.
   - **Dev-dependencies** are excluded and counted in a one-line note.
   - **Optional dependencies** count only if the resolved features enable them (the resolve
     graph already reflects this).
   - `--target <triple>` on `report` and `gate`; default the host triple from `rustc -vV`.
     Record the triple and the cargo version in JSON and Markdown.
   - **Fail closed.** If `cargo metadata` fails (no network with `--frozen`, missing sources),
     fall back to today's lockfile closure, mark the report `closure: lockfile (fallback)` at the
     top with the reason, and keep gates failing on unknown security dependency sizes as now.
     Never silently shrink the set.
2. **First-party headline.** Each of the two blocks (SECURITY-RELEVANT, EVERYTHING ELSE) prints
   three lines: first-party, third-party (runtime), and combined. The first line of the report is
   the first-party security figure. JSON gets the same three figures per block. `gate` gains
   `--min-first-party-security-proven`; the existing `--min-security-proven` keeps its combined
   meaning.
3. **Docs.** README.md and docs/evidence.md: the closure rule, the build-time and dev exclusions,
   the fallback, the three-line headline. Rerun the scratch-server table in
   docs/scratch-server.md for the branches it already lists, with a column for the closure used.

## Acceptance (self-testable)

A. Fixture workspace with one normal, one build, one dev and one disabled-optional dependency:
   the runtime set holds only the normal one (and its normal transitive deps); build-time holds
   the build one; dev is noted; the optional one is absent. Same fixture with `--target` set to a
   triple that excludes a `[target.'cfg(windows)'.dependencies]` entry drops that entry.
B. Fallback: with `cargo metadata` made to fail, the report says `closure: lockfile (fallback)`
   with the reason and the dependency set equals today's.
C. Scratch-server branch spec-lemmas (`2fa5eb78a6f9`), host triple: the runtime dependency set
   equals `cargo tree -e normal --target <host> --frozen --prefix none` deduplicated by
   name@version (compare sets in a test script, report any difference); the first line reads
   first-party security 47.25% (or explain any difference from 1,363 SLoC); the runtime
   third-party security SLoC is lower than the v1.1-c 1,626,171 total.
D. Existing tests pass; `cargo test` green, `cargo clippy -- -D warnings` clean.

## Fences

- Work only in this repository, plus read-only use of `~/<agent-home>/workspace/projects/scratch-server`
  checkouts (worktrees under this repo's target/ are fine). No network: `--frozen` / `--offline`
  only; if a dependency's source is not already cached, say so in NOTES.md and use the fallback.
- Project-local `CARGO_HOME="$PWD/.cargo-home"` and `TMPDIR="$PWD/.tmp"` for this repo's builds.
  Build output to files; read only `tail -n 30` or grep for errors.
- Commit per change (closure, headline, docs/table) as the repo's configured identity. No push.
- Keep PROGRESS.md (## v1.2) and NOTES.md current, timestamped. Resolve ambiguity by choosing the
  more conservative option (larger set, lower percentage) and logging it.
