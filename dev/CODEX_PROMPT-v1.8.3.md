You are making formal-coverage v1.8.3. `dev/PLAN-FSTAR-VPREFIX-v1.8.3.md` is authoritative; read it
fully first, then src/fstar.rs (`mapped()` and its tests, the classification loop), src/report.rs,
src/evidence.rs, tests/fstar_evidence.rs and dev/v182/ (replay.py, compare.py) as the pattern for
the replay.

Rules:
- Enforce every non-negotiable in the plan. Fail toward unmapped.
- Keep `dev/PROGRESS.md` (add a v1.8.3 section with M1-M2 checkboxes) and `dev/NOTES.md`
  (timestamped decisions, evidence, exact rerun commands) current. Commit after every
  milestone, on `main`, local only, with the trailer `Assisted-by: Codex (GPT-6.1)`, or the
  model you actually are if you are not Codex.
- Do M1-M2 in order, then write `V1.8.3-READY` in dev/PROGRESS.md and STOP.
- Resolve other ambiguity by choosing reasonably and logging it. Stop only for
  `HONEST-FINDING`: the v1.8.2 baseline replay doesn't match the CI artifact, a figure outside
  bertie-tls F* statuses changes, or the plan's rule can't be applied as written. Write it in
  NOTES and stop.
- Never modify `~/src/bertie-tls-flake-check` or any other checkout outside this repo; use copies
  under `target/` for replays.
- No push, no tag, no release bundle, no host installs, no email.
- Build/report/F* output goes to files under `logs/` or `target/`; read only `tail -n 30` or grep.
  Never stream full logs into your context.
- Run `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`
  before each commit.
- Report honestly: say what passed, what failed, what was skipped. Dudley checks "done" by
  rerunning acceptance.
