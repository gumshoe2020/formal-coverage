# v1.8.5 progress

- [x] Read authoritative PLAN.md, mapped/tests and v184 replay.
- [x] M1: inventory spans, numbered rule, unit and fixture tests; commit.
- [x] M2: staged replay, comparison, README/version; commit.
- [x] Codex acceptance commands: test, clippy, fmt, CI gate, empty-output replay, version.

Command timestamps below are UTC start times; exit codes are recorded at completion. Preliminary replays terminated with -15 were superseded by the final empty-output run.

All verification below is Codex-run; independent Andrew/Dudley acceptance remains outstanding.

- 2026-11-24T22:21:47.422689+00:00: `cargo clippy --all-targets -- -D warnings` → exit 101.

- 2026-11-24T22:21:54.196367+00:00: `cargo clippy --all-targets -- -D warnings` → exit 101.

- 2026-11-24T22:21:38.139529+00:00: `cargo test --no-fail-fast` → exit 0.

- 2026-11-24T22:22:05.283769+00:00: `cargo clippy --all-targets -- -D warnings` → exit 0.

- 2026-11-24T22:22:06.381416+00:00: `cargo fmt --check` → exit 0.

- 2026-11-24T22:22:18.721845+00:00: `cargo test --lib numbered --test fstar_evidence` → exit 0.

- 2026-11-24T22:24:10.969672+00:00: `cargo test --lib numbered --test fstar_evidence` → exit 0.

- 2026-11-24T22:23:40.609201+00:00: `cargo test --no-fail-fast` → exit 0.

- 2026-11-24T22:24:15.320112+00:00: `cargo clippy --all-targets -- -D warnings` → exit 0.

- 2026-11-24T22:24:17.049305+00:00: `cargo fmt --check` → exit 0.

- 2026-11-24T22:23:15.496847+00:00: `python3 dev/v185/replay.py` → exit -15.

- 2026-11-24T22:24:17.234120+00:00: `local CI gate (pre-push command, --mode ci --repo-root <repo>)` → exit 0.

- 2026-11-24T22:25:32.668230+00:00: `target/debug/formal-coverage --version` → exit 0.

- 2026-11-24T22:26:06.951473+00:00: `cargo test --lib numbered --test fstar_evidence` → exit 0.

- 2026-11-24T22:25:28.926745+00:00: `python3 dev/v185/replay.py` → exit -15.

- 2026-11-24T22:26:48.332014+00:00: `cargo test --no-fail-fast` → exit 0.

- 2026-11-24T22:27:23.975553+00:00: `cargo clippy --all-targets -- -D warnings` → exit 0.

- 2026-11-24T22:27:24.208659+00:00: `cargo fmt --check` → exit 0.

- 2026-11-24T22:27:24.446287+00:00: `local CI gate (pre-push command, --mode ci --repo-root <repo>)` → exit 0.

- 2026-11-24T22:28:31.827735+00:00: `target/debug/formal-coverage --version` → exit 0.

- 2026-11-24T22:28:51.691790+00:00 — version stdout: `formal-coverage 1.8.5`; RUSTUP_TOOLCHAIN unset. Final replay started by recreating target/v185.

- 2026-11-24T22:29:36.745800+00:00: `python3 dev/v185/controls.py` → exit 0.

- 2026-11-24T22:26:38.779757+00:00: `python3 dev/v185/replay.py` → exit 0.
