You are making formal-coverage v1.8. `dev/PLAN-FSTAR-RATIO.md` is authoritative; read it fully
first, then the repo README and src/ratio.rs, src/ghost.rs and src/fstar.rs.

Rules:
- Enforce every non-negotiable in the plan. hax annotation recognition is a whitelist.
- Keep `dev/PROGRESS.md` (add a v1.8 section with M1-M4 checkboxes) and `dev/NOTES.md`
  (timestamped decisions, evidence, exact rerun commands) current. Commit after every
  milestone, on `main`, local only, with the trailer `Assisted-by: Codex (GPT-6.1)`, or the
  model you actually are if you are not Codex.
- Do M1-M4 in order, then write `V1.8-READY` in dev/PROGRESS.md and STOP.
- Resolve other ambiguity by choosing reasonably and logging it. Stop only for
  `HONEST-FINDING`: a result that would mean a tier changes, a Verus figure changes, or the
  plan's counting rules can't be applied as written. Write it in NOTES and stop.
- Never modify `~/src/bertie-deps` or any other checkout outside this repo; use
  temp copies of configs under `target/` for replays.
- No push, no tag, no release bundle, no host installs, no email.
- Build/report output goes to files under `logs/` or `target/`; read only `tail -n 30` or grep.
  Never stream full logs into your context.
- Run `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`
  before each commit.
- Report honestly: say what passed, what failed, what was skipped. Dudley checks "done" by
  rerunning acceptance.
