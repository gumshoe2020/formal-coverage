# PLAN-SECURITY-SCOPE.md: v1.5, honest security line counts with explicit inputs

Approved by Andrew 2026-10-29 ("Yes let's draft this plan and have it ready"). Authoritative for
this milestone. Starts after v1.4.2 is committed. Move this file to `dev/PLAN-SECURITY-SCOPE.md`
before launch.

## Problem (verified 2026-10-29, scratch-server spec-lemmas)

1. **Dependencies count every line of every crate.** `dependencies.rs` sizes each runtime
   dependency by walking all `.rs` files under its source directory: tests, benches, examples,
   other-OS modules and Verus ghost code included. The 38 runtime dependencies total 336,654
   SLoC. Three crates are 60% of it, and almost none of that is in the binary:
   - libc 118,894: a host `cargo build` compiles 48 of its files, about 12,000 lines.
   - vstd 49,663: the Verus library, almost entirely spec/proof code erased before codegen.
   - typenum 35,003: type-level code, about 17,900 lines in tests/benches; compiled files are
     about 13,700 lines.
2. **Dependency security is all-or-nothing.** `dependencies.rs` (~line 268) sets every dependency's
   `security` to "any first-party unit is security". There is no way to say otherwise.
3. **Tests are found by name.** Test exclusion relies on a `*::tests::*` override glob, so
   `scratch_server::policy_tests::*` (62 SLoC: `diagnostic_classes_and_privacy`,
   `unknown_log_is_bounded_in_every_sliding_minute`) counts as first-party security runtime code.
4. **Overrides carry no justification.** `security = [...]` / `not_security = [...]` are bare
   globs. The report lists what matched but not why, and nothing shows the number the default
   rules would have produced.

## Change

### 1. Dependency size modes (`--size-mode`)

- `whole-crate`: today's behaviour, kept for comparison. Output must stay byte-identical to
  v1.4.2 in this mode with an unchanged config.
- `compiled-files` (**new default**): count only the `.rs` files rustc read for that crate, taken
  from cargo dep-info (`target/<triple>/<profile>/deps/<crate>-<hash>.d`) after
  `cargo check --frozen --target <triple>` in a project-local target dir. This drops tests,
  benches, examples and cfg-disabled modules. Within those files, apply the existing
  spec/proof-code inventory (the first-party `Category::ProofCode` rule) so Verus ghost code in
  vstd and similar crates is excluded from runtime totals. Generated files under `OUT_DIR` count
  if dep-info lists them.
- `linked-lines` (opt-in, `--binary <path>`): read the binary's DWARF line table (`gimli` /
  `addr2line` / `object`, all already in the offline cargo cache; on macOS run `dsymutil` first)
  and count distinct `(file, line)` pairs per dependency source directory. That measures the code
  that is actually linked, after dead-code elimination, inlining and monomorphization. Report the
  unit as "linked lines", not SLoC, and say so in the Markdown.
- **Fail closed.** If dep-info or debug info is missing or unreadable for any runtime
  dependency, fall back to `whole-crate` for that dependency, mark it `size_mode: whole-crate
  (fallback)` with the reason, and list the fallbacks at the top of the report. Never shrink a
  count on missing evidence.
- Record the mode, the triple, the profile and (for linked-lines) the binary's sha256 in JSON and
  on the first line of the Markdown report.

### 2. Structural test detection

Classify a first-party function as test code if it carries `#[test]` (or a known test attribute
such as `#[tokio::test]`), or sits inside any item under `#[cfg(test)]`, at any nesting depth and
whatever the module is named. Test code joins `Category::Test`, which is excluded from runtime
totals like proof code and reported as its own line. The `*::tests::*` glob becomes unnecessary,
so drop it from the shipped example config.

### 3. Explicit inputs with reasons

New config schema, version 3 (`deny_unknown_fields` kept):

```toml
entry_points = ["scratch_server::main"]           # unchanged

[[security.include]]
pattern = "http_subset::parse"
reason  = "request parser: attacker-controlled bytes"

[[security.exclude]]
pattern = "scratch_server::healthcheck"
reason  = "container healthcheck, loopback only, never reads network input"

[dependencies.libc]
security = true                                    # default for runtime deps
reason   = "syscall surface"

[dependencies.typenum]
security = false
reason   = "type-level arithmetic only; no runtime code paths"

[dependencies."foo@1.2.3"]
sloc   = 1200
reason = "measured by hand: see docs/foo-size.md"
```

- `reason` is required and non-empty on every override. A pattern or dependency key that matches
  nothing is an error, not a warning.
- Dependency keys accept `name` or `name@version`; an ambiguous `name` (two versions in the
  closure) is an error.
- The old flat `security = [...]` / `not_security = [...]` arrays still parse (schema 2) and
  produce a warning per entry: "override has no reason". `formal-coverage migrate-config` rewrites
  a schema-2 file to schema 3 with `reason = "TODO"`, and a config containing a `TODO` reason fails
  `gate`.
- Order: default rules, then includes, then excludes (exclude wins), as today.

### 4. Reporting

- A new **Inputs** section lists every override: pattern/key, direction, reason, units matched and
  the SLoC moved into or out of the security side.
- Each headline security figure is printed twice: **with inputs** and **default rules only** (the
  same analysis with no `[security.*]` or `[dependencies.*]` overrides). Overrides cannot quietly
  improve the headline.
- `gate` gates on the with-inputs figure. A new flag, `--max-override-shift <pct>`, fails when
  the with-inputs and default-only first-party security percentages differ by more than that
  amount (no default; off unless given).

### 5. Docs and numbers

README.md, docs/evidence.md: the three size modes and when to use each, structural test detection,
the schema-3 config and the two-figure headline. docs/scratch-server.md: one new table for
spec-lemmas in all three modes (first-party security SLoC and %, third-party runtime security size,
the top 5 dependencies by size in each mode). Ship `examples/scratch-server.formal-coverage.toml`
as a worked schema-3 config containing only overrides you can justify from the code. Anything
debatable (startup code, log formatting, trivial getters) gets a `NOTES.md` entry for Andrew
instead of an override.

## Acceptance (self-testable)

A. **compiled-files fixture:** a dependency crate with `#[cfg(windows)] mod win;`, a `tests/`
   file, a `benches/` file, an `examples/` file and a `verus!`-style spec function (reuse the
   proof-code fixture). On a non-Windows host triple only the compiled non-test files count and
   the spec function is excluded. Exact expected SLoC is asserted.
B. **linked-lines fixture:** a dependency with one function called by the binary and one never
   called. Built with `debug = true` in release, the called function's lines count and the
   uncalled one's do not. Also tested on a binary with no debug info: fallback marked, size equals
   whole-crate.
C. **Fallback:** remove one dependency's `.d` file; that dependency is marked
   `whole-crate (fallback)` with the reason, and the others still use compiled-files.
D. **Test detection:** a fixture with `#[cfg(test)] mod policy_tests`, a nested
   `#[cfg(test)] mod a { mod b { fn helper() } }` and a free `#[test] fn` puts all three in
   `Category::Test`. On scratch-server spec-lemmas with the `*::tests::*` override removed, the
   first-party security SLoC drops from 1,363 by exactly the `policy_tests` SLoC (62, or explain
   any difference) and the proven SLoC (644) is unchanged.
E. **Inputs:** an override with no reason fails in schema 3. A non-matching pattern fails. A
   non-matching dependency key fails. An ambiguous `name` fails. A `TODO` reason fails `gate`.
   `migrate-config` round-trips the existing scratch-server overrides. The Inputs section shows
   the SLoC moved per override, and the default-only figure equals a run with the overrides
   deleted.
F. **Regression:** `--size-mode whole-crate` with the v1.4.2 config reproduces the v1.4.2 JSON and
   Markdown for spec-lemmas and the three survey repos byte-for-byte (diff empty, apart from the
   new mode/schema fields, which are listed explicitly in the test).
G. **Real numbers:** spec-lemmas in compiled-files mode puts libc under 20,000 lines and the
   third-party runtime security total under 150,000. In linked-lines (host release build with
   debug info) it is lower still. Firecracker, Vest and Anvil run in all three modes without error;
   record the totals in docs/survey.md. If a bound fails, report the actual number and why. Do
   not loosen the bound.
H. `cargo test` green, `cargo clippy --all-targets -- -D warnings` clean, `cargo fmt --check`
   clean, `scripts/check-public.sh` 0 hits on the tree.

## Fences

- Work only in this repository, plus read-only use of `~/<agent-home>/workspace/projects/scratch-server` and
  `../coverage-survey` checkouts (worktrees or build dirs under this repo's `target/` or `.tmp/`
  are fine). Never write into those repos or their `target/` dirs.
- No network: `--frozen` / `--offline` only. New crates (`gimli`, `addr2line`, `object`) only at
  versions already in the cargo cache. If a needed source is missing, log it in NOTES.md and use
  the fallback.
- Project-local `CARGO_HOME="$PWD/.cargo-home"` and `TMPDIR="$PWD/.tmp"`. Send build output to
  files and read only `tail -n 30` or grep for errors.
- Commit per section (size modes, test detection, inputs, reporting, docs/numbers) as the repo's
  configured identity. Bump the version to 1.5.0. No push, no tag, no release bundle.
- Keep PROGRESS.md (`## v1.5`) and NOTES.md current, timestamped. Resolve ambiguity by choosing
  the more conservative option (larger security set, lower percentage) and logging it. Do not stop
  to ask.

## Out of scope

- Function-level reachability into dependencies (which dependency functions the security code
  calls). Linked-lines is the proxy for now.
- Changing the first-party reachability rule (identifier graph, unresolved calls pull in all).
  It over-includes, but that errs toward caution.
- Build-time dependency sizing (stays as v1.2 lists it).
