# v1.9.1 progress

2026-11-25T15:30:09.218228+00:00 — Started on main; RUSTUP_TOOLCHAIN unset. Existing untracked historical run logs left untouched.

- [x] M0: probe and canary
- [x] M1: rules and tests
- [x] M2: replay, documentation, version and acceptance (Codex-run; independent Dudley acceptance pending)

Run 1 stopped at check (b); run 2 below passes acceptance 1–6 under the approved amendment. Independent Dudley acceptance remains outstanding.

2026-11-25T15:31:28.299794+00:00 — Staged input validation (all receipt files/fresh/checked/patches/log SHA-256): exit 0. M0 patterns/provenance and definition inventory recorded in NOTES.md.

2026-11-25T15:31:29.329931+00:00 — M0 broken-body canary: exit 1; target/v191/canary/broken.log.

2026-11-25T15:33:13.394233+00:00 — M0 restored-body canary: exit 0; target/v191/canary/restored.log.

2026-11-25T15:33:23.840410+00:00 — M0 complete; D body checking confirmed; B unavailable under current receipt schema. Rules untouched.

2026-11-25T15:40:31.895382+00:00 — `cargo test --no-fail-fast`: exit 0; target/v191/m1-test.log.

2026-11-25T15:40:32.710041+00:00 — `cargo clippy --all-targets -- -D warnings`: exit 0; target/v191/m1-clippy.log.

2026-11-25T15:40:32.887153+00:00 — `cargo fmt --check`: exit 0; target/v191/m1-fmt-check.log.

2026-11-25T15:41:49.807513+00:00 — `cargo test v191 --no-fail-fast`: exit 0; `cargo clippy --all-targets -- -D warnings`: exit 0 (m1-extra-tests.log / m1-clippy-final.log under target/v191). M1 complete.

2026-11-25T15:48:15.800779+00:00 — `python3 dev/v191/replay.py` from empty target/v191: exit -15; logs/v191/preliminary-replay.log.

2026-11-25T15:49:29.135488+00:00 — sibling correction: `cargo test v191 --no-fail-fast` exit 0; `cargo clippy --all-targets -- -D warnings` exit 0; logs/v191/sibling-tests.log and sibling-clippy.log. Preliminary replay terminated before comparison (exit -15).

2026-11-25T15:50:26.148742+00:00 — `cargo test --no-fail-fast`: exit 0; logs/v191/acceptance-test.log.

2026-11-25T15:50:26.333650+00:00 — `cargo clippy --all-targets -- -D warnings`: exit 0; logs/v191/acceptance-clippy.log.

2026-11-25T15:50:26.528577+00:00 — `cargo fmt --check`: exit 0; logs/v191/acceptance-fmt.log.

2026-11-25T15:51:40.343257+00:00 — `local CI gate (--mode ci)`: exit 0; logs/v191/acceptance-local-ci.log.

2026-11-25T15:51:40.347518+00:00 — `target/debug/formal-coverage --version`: exit 0; logs/v191/acceptance-version.log.

2026-11-25T15:54:56.195012+00:00 — `python3 dev/v191/replay.py` from empty target/v191: exit 1; logs/v191/replay.log.

2026-11-25T15:55:38.532522+00:00 — Version stdout confirmed: `formal-coverage 1.9.1`. Release acceptance: 1–4 and 6 passed; 5 failed. M0/M1 committed; M2 stopped. See NOTES.md and compare.md.

HONEST-FINDING: STOPPED at replay check (b); cfg(hax) helper maps to verified under the unchanged classifier.

## 2026-11-25T15:57:30.248251+00:00 — M2 run 2

Run 2 started: continuing M2 under the approved check (b) amendment. Source rules and checks (a), (c), (d), (e) remain unchanged. Fresh replay and final acceptance pending.

2026-11-25T15:58:08.055636+00:00 — Run 2 acceptance 1: `cargo test --no-fail-fast`: exit 0; started 2026-11-25T15:57:43.999778+00:00; logs/v191/run2/test.log.

2026-11-25T15:58:08.269239+00:00 — Run 2 acceptance 2: `cargo clippy --all-targets -- -D warnings`: exit 0; started 2026-11-25T15:58:08.055859+00:00; logs/v191/run2/clippy.log.

2026-11-25T15:58:08.470746+00:00 — Run 2 acceptance 3: `cargo fmt --check`: exit 0; started 2026-11-25T15:58:08.269410+00:00; logs/v191/run2/fmt.log.

2026-11-25T15:59:15.819405+00:00 — Run 2 acceptance 4: `local CI gate (--mode ci)`: exit 0; started 2026-11-25T15:58:08.470942+00:00; logs/v191/run2/local-ci.log.

2026-11-25T16:04:02.022024+00:00 — Run 2 acceptance 5: `python3 dev/v191/replay.py`: exit 0; started 2026-11-25T15:59:16.943639+00:00; logs/v191/run2/replay.log. Fresh target/v191 absent before invocation.

2026-11-25T16:04:02.026382+00:00 — Run 2 acceptance 6: `target/debug/formal-coverage --version`: exit 0; started 2026-11-25T16:04:02.022219+00:00; logs/v191/run2/version.log.

2026-11-25T16:04:41.089760+00:00 — M2 complete: acceptance 1–6 all exit 0; replay (a)–(e) PASS; version stdout `formal-coverage 1.9.1`. README matches B refusal and D trusted behavior; compare.md regenerated. No source changes. Independent Dudley rerun/review pending.

V1.9.1-READY
