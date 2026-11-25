# Changelog

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
