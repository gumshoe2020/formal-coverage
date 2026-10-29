One small fix in this repository (formal-coverage), follow-up to v1.2 (f330600).

Problem: v1.2 changed `gate --min-security-proven` from required to `default_value_t = 0.`, and the new
`--min-first-party-security-proven` also defaults to 0. A `gate` call with neither flag now passes the
threshold check silently. Gates must fail closed.

Fix: `gate` must require at least one of `--min-security-proven` / `--min-first-party-security-proven`
to be given explicitly (either or both). An omitted one imposes no threshold. With neither given,
the CLI errors with a clear message and non-zero exit. Update README.md's gate text.

Acceptance: tests for neither (error), each one alone, and both; `cargo test --frozen` green and
`cargo clippy --frozen -- -D warnings` clean, with CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp",
output to logs/ and read only `tail -n 30`. One commit, not pushed. Note it in PROGRESS.md (## v1.2a).
Work only in this repository.
