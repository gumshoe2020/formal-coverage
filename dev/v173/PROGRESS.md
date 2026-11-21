# v1.7.3 progress

- 2026-11-21T15:48:05-05:00 Started M1 from 4031d85 on branch v1.7.3; implementing the supplied acceptance spec.
- 2026-11-21T15:48:44-05:00 M1 local acceptance passed: cargo test --frozen (88 passed, 0 failed, 1 ignored), cargo clippy --all-targets -- -D warnings, cargo fmt --check, git diff --check. Logs: target/v173/m1-*.log. Added five regressions (four integration tests plus ghost ownership test). Independent acceptance has not run.
- 2026-11-21T15:51:58-05:00 M2 passed locally: both frozen release builds and exact CI report invocations exited 0. Security 6,382 → 5,818 (drop 564); independent scanner also counted 564 unit-owned doc-only lines. 469 units, 223 changed sizes, zero tier changes or increases. server 73 → 20 (tested). See compare.md, compare.py and provenance.json. Bertie tracked tree remains clean and source/config/lock/log hashes unchanged.
- 2026-11-21T15:53:18-05:00 M3 local acceptance passed: cargo test --frozen (88 passed, 0 failed, 1 ignored), cargo clippy --all-targets -- -D warnings, cargo fmt --check, git diff --check; frozen release build and --version (1.7.3) passed. Version metadata and README/docs release notes updated. Logs: target/v173/m3-*.log. Independent acceptance has not run.

V1.7.3-READY — 2026-11-21T15:53:18-05:00: doc-only tokens excluded from source and inline ghost sizes; all local checks pass; Bertie security 6,382 → 5,818, independent doc count 564, server 73 → 20; no tier changes or size increases; independent acceptance pending.
