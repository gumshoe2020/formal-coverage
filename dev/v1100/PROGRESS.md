# v1.10.0 progress

Takeover: Claude Code (Sonnet 5.5) after Codex hit its usage limit; Codex had made no changes (clean tree).

- [x] M1 config + recorded floor in ratchet
- [x] M2 recording job/action
- [x] M3 docs, CHANGELOG, version, byte-identity replay

Reference for acceptance 4: `dev/v1100/ref-v191.txt` = `python3 dev/v1100/compare.py .` at v1.9.1 ci code (before any edit).
- 2026-11-25 17:16:44 M1: unittest discover exit 0 (59 tests); compare.py identical to ref-v191.txt
- 2026-11-25 17:17:59 M2: unittest discover exit 0 (71 tests); actionlint ok; compare.py identical
- 2026-11-25 17:21:34 M3 acceptance:
  1. python3 -m unittest discover -s ci/tests -v → 0 (71 tests)
  2. cargo test --no-fail-fast → 0; cargo clippy --all-targets -- -D warnings → 0; cargo fmt --check → 0
  3. quality/check.sh --mode ci (what .git/hooks/pre-push runs) → 0
  4. Default mode byte-identity: `python3 dev/v1100/compare.py <kit>` runs write-coverage + run.py (committed floor) on the kit test report and prints sha256 of COVERAGE.md, baseline.json and every output file (badges, summary, floor, diff, report). Output for the working tree is identical (diff exit 0) to the output from a worktree of v1.9.1 (99ccc33) and to dev/v1100/ref-v191.txt. The existing unittest suite (committed-mode tests unchanged) passes. The Rust fixtures (fixtures/, examples/) are untouched: no Rust source changed, only the version string; cargo test covers them. No bertie-tls/scratch-server checkouts were touched (outside this repo).
  5. target/debug/formal-coverage --version → "formal-coverage 1.10.0" (exit 0)
  extra: python3 ci/lint-actions.py (actionlint on workflows, README example, three composites) → 0

V1.10.0-READY
