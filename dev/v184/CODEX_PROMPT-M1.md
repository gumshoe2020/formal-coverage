# formal-coverage v1.8.4 — M1 only: `[ship]` config and shipped-only first-party scope

Repo: this directory (~/src/formal-coverage), branch `main` (HEAD c944fd4; M0 done in d40ebf4).
`dev/v184/PLAN.md` is authoritative: read "Rule", "Non-negotiables" and "Milestones" first. Do ONLY M1.
No README edits, no version bump, no bertie-tls replay (those are M2).

## Grounding (verified 2026-11-24)
- Config parsing: `src/inputs.rs` (schemas 2 and 3; `ConfigV3`, normalized struct with `schema`). `[ship]` is a new OPTIONAL table in schema 3; no schema bump unless you find it necessary (log why in NOTES.md).
- First-party = every Cargo workspace member today (`src/dependencies.rs` keeps `workspace_members`); first-party files are read whole, so cfg-disabled modules count.
- Dependencies already use rustc dep-info for `compiled-files` size mode: `src/size.rs` (~line 200 derives the `.d` path from the `.rmeta` artifact). Reuse that machinery for first-party scope instead of writing a second dep-info reader.

## Build
1. Parse `[ship] packages`, `features`, `reason` (reason required and non-empty; a missing reason is a config error with a clear message). Unknown package name → config error.
2. With `[ship]`: first-party scope = files rustc reads for the listed packages (`cargo check --frozen -p <pkg> --lib --bins` with those features; `"default"` means default features) plus workspace members they reach via normal (non-dev, non-build) dependencies. Everything else first-party, including cfg-disabled modules, is "not shipped".
3. Not shipped is removed from every total (numerator and denominator; security and non-security; proven lines too). It stays visible: a "Not shipped" report section listing file + SLoC (+ tier it would have had), its own line in the markdown summary, and in JSON output.
4. Fail toward counting: no dep-info, or a package whose `cargo check` fails → its files stay in scope as today, with a warning naming the package and reason. A file is dropped only when dep-info proves it isn't compiled.
5. No `[ship]` → byte-identical reports to HEAD on the same input.

## Tests (fixture workspace, per PLAN M1)
Fixture: a lib package, an example bin member not depended on by the lib, a feature-gated module (`#[cfg(feature = "x")]`, not default), and a member the lib depends on normally. Assert: lib + reached member counted; bin member and gated module listed as not shipped and absent from all totals; enabling the feature in `[ship].features` brings the gated module back; missing dep-info → kept with warning; no `[ship]` → identical to the pre-change report; missing `reason` → error.

## Non-negotiables
- Only first-party scope changes. Dependency counts and Verus/Kani/F*/hax evidence rules unchanged; all existing tests pass unmodified (except purely additive fields in snapshot/JSON expectations, each justified in NOTES.md).
- No toolchain pin or `RUSTUP_TOOLCHAIN`; current stable (rustc 1.100.0). No `#[ignore]`. Never `--no-verify`. Commit on `main` in small commits; do not push.

## Acceptance (no RUSTUP_TOOLCHAIN set; paste exit codes into dev/v184/PROGRESS.md)
1. `cargo test --no-fail-fast` → 0
2. `cargo clippy --all-targets -- -D warnings` → 0
3. `cargo fmt --check` → 0
4. `<local-ci-gate>/check.sh --mode ci --repo-root "$PWD"` → 0
5. No-`[ship]` identity: run the CLI on `examples/scratch-server` config before (c944fd4, via `git worktree` in /tmp) and after; JSON and markdown outputs identical. Record the commands and `diff` result.

Update `dev/v184/PROGRESS.md` (M1 checklist) and `dev/v184/NOTES.md` (timestamped decisions). Resolve ambiguity by choosing reasonably and logging it; don't stop to ask.
