# formal-coverage: PLAN.md

Status: approved by Andrew 2026-10-27 ("begin building our formal coverage tool"). Authoritative
spec for the build.

## What it is
Test coverage tells you which lines ran. **Formal coverage** tells you which code is *proven*
correct, which is only *tested*, and which is merely *unverified*, and how much of each sits on the
path untrusted input travels. It is the automated version of the hand-written PROOFS.md and
TLS-PROVIDER.md in `gumshoe2020/scratch-server`.

**Andrew's requirement: security-relevant code and everything else are measured separately.**
A proof on a logging helper must never inflate the security number, and an unproven parser must
never be hidden by a well-proven utility module. Every report shows the two side by side.

## Concepts
(Vocabulary: the status is `unverified`, not "trusted": in security writing "trusted" means "relied on", which reads as "trustworthy". Andrew, 2026-10-27.)
- **Unit:** a function (first-party code) or a crate (third-party dependency). Size = SLoC
  (non-blank, non-comment lines), so percentages are weighted by code, not by count.
- **Security-relevant:** a unit that untrusted input can reach. Determined by:
  1. entry points declared in `formal-coverage.toml` (e.g. the function that receives bytes from
     a socket);
  2. reachability over a call graph from those entry points (v1 may be an approximate,
     syntactic, name-resolved call graph; it must say so in the report and err toward
     *including* code, never excluding it);
  3. explicit overrides in the config (`security = [paths/globs]`, `not_security = [...]`), each
     override listed in the report so nothing is silently reclassified.
  Third-party crates are security-relevant if a security-relevant first-party unit uses them.
- **Status** of each unit, strongest first:
  - `proven`: verified locally (Verus or Kani), *with run evidence* (a passing verifier result
    supplied to or run by the tool), and no escape hatch in the unit.
  - `proven-upstream`: a dependency whose verification is documented upstream, from a registry
    entry citing the primary source; each entry carries `confirmed` or `unconfirmed`.
  - `tested`: exercised by tests (v1 may use a simple signal: referenced from a test, or
    llvm-cov line data if present).
  - `unverified`: safe code with none of the above. Not proven, but still under the compiler's
    memory-safety guarantees.
  - `hazard` (lowest, below `unverified`; Andrew 2026-10-27): code that actively weakens
    guarantees other code relies on, so it is worse than merely unproven:
    - `unsafe` blocks/fns/impls: undefined behavior there can break memory safety anywhere in
      the process, including inside proven code;
    - verifier escape hatches `assume`, `admit`, `#[verifier::external_body]`,
      `#[verifier::external]`, `#[allow(...)]` on verification lints: they make a proof claim
      that was never checked.
    Exception: an `unsafe` unit with a passing Kani harness that covers it is reported as
    `proven-bounded` (state the bound), not `hazard` and not `proven`.
- **Hazards are reported, never folded in:** each listed by file:line, counted in their own
  column of both blocks, and a hazard in security-relevant first-party code is called out at the
  top of the report. Third-party crates: report whether they contain `unsafe` (count of sites,
  first-party scan of the downloaded source) as a separate column; v1 does not downgrade a
  `proven-upstream` crate for internal `unsafe` but must show it.

## Interface
```
formal-coverage report <workspace-dir> [--config formal-coverage.toml]
    [--verus-log FILE] [--kani-log FILE] [--json OUT] [--markdown OUT]
formal-coverage gate <workspace-dir> --min-security-proven <PCT>
    [--baseline previous.json] [--no-new-escape-hatches]
```
Headline output (two blocks, always both):
```
SECURITY-RELEVANT  1,240 SLoC   proven 18%  proven-upstream 41%  tested 30%  unverified 11%  hazard 0%
EVERYTHING ELSE      860 SLoC   proven  0%  proven-upstream  0%  tested 72%  unverified 27%  hazard 1%
```
plus first-party vs third-party sub-totals, the escape-hatch list, the overrides list, and the
upstream claims marked `unconfirmed`.

## Milestones (acceptance self-testable)
**M1 inventory.** Rust CLI. Reads a Cargo workspace (`cargo metadata`), parses first-party
sources (e.g. `syn`) and lists every function with crate, module path, file:line and SLoC,
including functions inside `verus! { }` blocks. JSON output. Tests: a small fixture workspace in
`fixtures/` with known function counts and SLoC; counts match exactly.

**M2 security classification.** Config file, entry points, approximate call graph, reachability,
overrides, third-party crate attribution. Tests: fixture with a known split (an entry point that
reaches a parser and a dependency, plus unrelated helpers); classification matches an expected
file exactly; overrides appear in output.

**M3 evidence and status.** Detect Verus-verified functions (have `requires`/`ensures` and the
file is covered by a passing verify log; parse `verification results:: N verified, 0 errors`;
a failing or missing log means not `proven`); detect Kani harnesses and their log results;
escape-hatch scan; upstream registry `registry/upstream.toml` (crate, version requirement,
operations, property, tool, source URL, confirmed|unconfirmed), seeded from scratch-server's
TLS-PROVIDER.md (libcrux crates, with the same confirmed/unconfirmed judgments). Tests: fixtures
covering each status and each escape hatch; an escape hatch downgrades `proven` to `hazard`.

**M4 reports and gate.** Markdown + JSON; the two-block headline; `gate` exits non-zero when the
security-proven percentage is below the threshold, when it drops against a baseline, or when a
new escape hatch appears. Tests for each gate condition.

**M5 dogfood on scratch-server.** Export (read-only, via `git -C <repo> archive <branch>` into a
temp dir; never check out or modify that repo) the branches `main`, `nix-scratch`,
`verified-boundary`, `browser-whitelist`, `https`, `https-headers`, `idle-connections` from
<scratch-server>, run its `./verify.sh` evidence where
the branch has one (or reuse its logs/ output if running is impractical, and say which), and write
`docs/scratch-server.md`: one row per branch with the security / everything-else numbers, plus a
short reading of how the security-proven share moved stage by stage. Commit a
`formal-coverage.toml` for scratch-server in `examples/scratch-server/` (not into that repo).

## Non-negotiables
- Honest by construction: when the tool can't tell, it says `unverified`/`unconfirmed`, never
  `proven`. Approximations are named in the report.
- The security/everything-else split appears in every output format.
- Rust, stable toolchain; `cargo test` and `cargo clippy -- -D warnings` pass.

## Out of scope (v1)
Non-Rust languages; precise MIR-level call graphs; running verifiers for arbitrary projects
(v1 consumes their logs, plus the scratch-server dogfood); a web UI.

## Open-source release gate (Andrew, 2026-10-27)
Both `scratch-server` and `formal-coverage` go public once the numbers are honest. Before
flipping either repo to public, all of these must hold, and Andrew confirms at that moment:
1. **Honest numbers:** formal-coverage v1.1 counts only crates linked into the shipped binary
   (target-accurate), reports build-time tooling separately, and measures itself; scratch-server
   EVOLUTION.md carries the v1.1 numbers for every stage.
2. **Clean history:** commits authored by Andrew's personal Gmail (Codex used the global git
   identity) are rewritten to the GitHub no-reply address, or the repo is published as a fresh
   export, as was done for context-compiler.
3. **Scans:** secrets scan (no keys, certificates or tokens in any commit), personal-data scan
   (home paths, LAN details judged acceptable or scrubbed), and a clean-clone build and test.
4. **License** chosen and added (context-compiler used MIT).
