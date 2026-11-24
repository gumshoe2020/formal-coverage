# formal-coverage v1.8.4 — first-party code that does not ship is not counted (DRAFT)

Decided by Andrew 2026-11-24: "If they are not shipped as part of the binary, they should not count
towards coverage either way." Status: M0 done (d40ebf4, verified by Dudley 2026-11-24). M1 next.

## Background (bertie-tls main c66a986 coverage artifact)

The tool counts every Cargo workspace member as first-party, and reads first-party files whole, so
cfg-disabled modules also count. Dependencies already use `compiled-files` (rustc dep-info) and the
shipped build graph (v1.2/v1.5); first-party code does not.

In bertie-tls, 324 first-party security runtime lines don't ship in the library: `bogo_shim/`,
`record/`, `simple_https_client/`, `simple_https_server/`, `integration_tests/` (workspace members,
not dependencies of `bertie-tls`) and `src/test_utils.rs` (`#[cfg(feature = "test_utils")]`, not a
default feature). 303 unverified and 21 tested, 0 proven.

## Rule

New optional config:
```toml
[ship]
packages = ["bertie-tls"]     # the shipped artifact(s)
features = ["default"]        # as passed to cargo; "default" means default features
reason = "..."                # required
```
With `[ship]`, first-party scope = files rustc reads (dep-info from `cargo check --frozen -p <pkg>
--lib --bins` with those features) for the listed packages plus workspace members they reach through
normal (non-dev, non-build) dependencies. Other first-party files and cfg-disabled modules are
"not shipped": out of every total (numerator and denominator, security and non-security), listed in
their own report section with file and SLoC.

## Non-negotiables
1. No `[ship]` → behaviour unchanged.
2. Fail toward counting: no dep-info, or a package that fails `cargo check` → that package's
   files stay in scope as today, with a warning. A file is dropped only if dep-info shows it isn't compiled.
3. Not-shipped code stays visible (own section, own line in the markdown summary); never silently gone.
4. Applies "either way": dropped proven lines leave the numerator too.
5. Only first-party scope changes. Dependency counts, Verus/Kani/F* evidence rules unchanged.

## Milestones
- M0 (Andrew 2026-11-24: "keep Rust updated"): all gates run on current stable (1.100 now), no
  pinned older toolchain. `tests/size_modes.rs:141`
  (`missing_dep_info_falls_back_to_whole_crate_for_that_dependency_only`) hard-codes
  `target/formal-coverage/*/debug/deps/*.d`; cargo 1.100 lays out dep-info differently, so the test
  can't find the dir. The tool itself still finds dep-info on 1.100 (the test's first assertion,
  no fallbacks, passes). Fix the test to locate `.d` files the way the tool does. Fails identically on v1.8.2.
- M1: config + scope rule + tests (fixture workspace: lib, example bin member, feature-gated module,
  a member the lib depends on; missing dep-info → kept).
- M2: bertie-tls replay on c66a986 (+ `[ship]` in the copy only): before/after compare listing every
  dropped file and SLoC; confirm nothing else changed; README; bump to 1.8.4.
  README (Andrew 2026-11-24: "We'll want to update the readme too for 1.8.4"), following the existing
  per-version pattern: a `## v1.8.4: code that doesn't ship isn't counted` section at the top (above
  v1.8.1) explaining the rule, fail-toward-counting and the not-shipped report section; `[ship]`
  documented with an example in the `formal-coverage.toml` section; any statement that all workspace
  members count as first-party corrected; note that the gates run on current stable Rust. Acceptance:
  every config key and report section named in the README exists in the code, and every README example
  config parses (add a test if none covers it).

## Release gate (Dudley's acceptance)
v1.8.4 is not released unless `cargo test --no-fail-fast`, clippy `-D warnings` and fmt pass on
the current stable toolchain (no `RUSTUP_TOOLCHAIN` override), and the pre-push hook passes
without one. M0 is not optional and is not deferred to a later version.
After acceptance (Dudley, Andrew OK'd 2026-11-24): `rustup toolchain uninstall 1.97.1` so nothing falls back to it.

## Expected (guide, not target)
Security total 5,402 → ~5,078; proven share ~67.3% → ~71.6% before the v1.8.3 fix, ~74% after.

## Follow-up (bertie-tls, separate PR)
Add `[ship]` to bertie-tls `formal-coverage.toml`, re-measure, record in COVERAGE.md like earlier
tool changes; ratchet baseline regenerated from the proof run, not by hand.
