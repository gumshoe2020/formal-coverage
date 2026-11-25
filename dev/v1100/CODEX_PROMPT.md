# formal-coverage v1.10.0 — recorded floor on main (M1–M3)

Repo: this directory (~/src/formal-coverage), branch `main`. `dev/v1100/PLAN.md` is authoritative;
read it fully first, then `ci/README.md`, `ci/run.py`, `ci/coverage_lib.py`, `ci/publish.py`,
`ci/config.py`, `ci/tests/`, and `.github/actions/{coverage,publish-badges}/action.yml`.

- M1, M2, M3 in order; commits prefixed `v1100 M1:` etc., trailer `Assisted-by: Codex (GPT-6.1)`
  or the model you actually are.
- Keep `dev/v1100/PROGRESS.md` (milestone checklist, timestamped exit codes) and
  `dev/v1100/NOTES.md` (decisions, blockers). Resolve ambiguity by choosing reasonably and logging
  it. Stop with `HONEST-FINDING` in NOTES.md only if the plan's rules can't be applied as written
  or the default mode's behaviour would change.
- No push, no tag, no network in tests (use a local bare repo as the remote). Never modify anything
  outside this repo. Logs to files under `logs/` or `target/`; read tails or grep.

## Acceptance (record each exit code in PROGRESS.md)
1. `python3 -m unittest discover -s ci/tests -v` → 0
2. `cargo test --no-fail-fast` → 0, `cargo clippy --all-targets -- -D warnings` → 0, `cargo fmt --check` → 0
3. the local CI gate that `.git/hooks/pre-push` runs → 0
4. the default (committed-floor) mode produces byte-identical outputs to v1.9.1 on the existing kit
   fixtures and test repos (write the comparison into PROGRESS.md)
5. `target/debug/formal-coverage --version` → `formal-coverage 1.10.0`

When done, write `V1.10.0-READY` as the last line of PROGRESS.md and stop. Dudley checks "done"
by rerunning acceptance.
