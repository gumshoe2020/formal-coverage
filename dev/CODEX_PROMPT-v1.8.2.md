You are making formal-coverage v1.8.2. `dev/PLAN-FSTAR-TRUST-v1.8.2.md` is authoritative; read it
fully first, then src/fstar.rs (`sites()`, `mapped()`, the classification loop), src/lib.rs (config,
`scope_attributes`), src/report.rs, src/evidence.rs and tests/fstar_evidence.rs.

Rules:
- Enforce every non-negotiable in the plan. Fail toward trusted.
- Keep `dev/PROGRESS.md` (add a v1.8.2 section with M1-M3 checkboxes) and `dev/NOTES.md`
  (timestamped decisions, evidence, exact rerun commands) current. Commit after every
  milestone, on `main`, local only, with the trailer `Assisted-by: Codex (GPT-6.1)`, or the
  model you actually are if you are not Codex.
- Do M1-M3 in order, then write `V1.8.2-READY` in dev/PROGRESS.md and STOP.
- Resolve other ambiguity by choosing reasonably and logging it. Stop only for
  `HONEST-FINDING`: the v1.8.1 baseline replay doesn't match the CI artifact, a figure outside
  bertie-tls F* statuses changes, a gate trips on Bertie, or the plan's rules can't be applied as
  written. Write it in NOTES and stop.
- Never modify `~/src/bertie-der`, `~/src/bertie-hsdata-trusted` or any other checkout outside this
  repo; use copies under `target/` for replays.
- No push, no tag, no release bundle, no host installs, no email.
- Build/report/F* output goes to files under `logs/` or `target/`; read only `tail -n 30` or grep.
  Never stream full logs into your context.
- Run `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`
  before each commit.
- Report honestly: say what passed, what failed, what was skipped. Dudley checks "done" by
  rerunning acceptance.
