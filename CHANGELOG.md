# Changelog

## 1.9.1 — 2026-11-25

- Map five more hax naming forms back to Rust, each only when exactly one Rust
  candidate fits and the F* module is strictly verified: trailing numeric escapes
  (`name_2_`), typed trait instance records with explicit method bodies, interface
  instance records (kept trusted, guarded by a canary showing F* checks the body),
  and empty-segment inherent impl names (`impl__new`).
- Cfg twins stay unmapped with a visible reason when the receipt lacks the
  extraction cfg set. Ambiguity, unsupported generics or a failed type guard
  leave a definition unmapped with the reason in JSON and Markdown.
- Existing mappings are unchanged. On bertie-tls: 79.30% to 80.96% proven.

## 1.9.0 — 2026-11-24

- Add a reusable, Python standard-library coverage CI kit extracted from bertie-tls:
  baseline/document checks, exact-rational git ratchet, configurable honest badge
  thresholds and informational proof-cost ranges.
- Add SHA-pinned composite coverage and main-only badge publication actions;
  the action pin also selects the measurement tool. Git state is the floor;
  missing base baselines fail closed. Only the publish job needs contents: write.
- Add kit tests, CI build/test workflow and documented consumer workflow.
- Measurement behavior and report format are unchanged from 1.8.5.

Earlier releases are described in README.md and the versioned docs/dev records.
