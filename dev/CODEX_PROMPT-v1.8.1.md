You are making formal-coverage v1.8.1. `dev/PLAN-FSTAR-RATIO-v1.8.1.md` is authoritative; read it
fully first, then `dev/PLAN-FSTAR-RATIO.md` (v1.8, still in force), the v1.8 section of
`dev/NOTES.md`, `dev/v18/compare.md`, and src/lib.rs (classification, `cfg_implies_test`),
src/ratio.rs, src/hax.rs, src/report.rs and src/evidence.rs.

Rules:
- Enforce every non-negotiable in both plans. `cfg(hax)` detection is exact cfg algebra, not
  a text heuristic.
- Keep `dev/PROGRESS.md` (add a v1.8.1 section with M1-M3 checkboxes) and `dev/NOTES.md`
  (timestamped decisions, evidence, exact rerun commands) current. Commit after every
  milestone, on `main`, local only, with the trailer `Assisted-by: Codex (GPT-6.1)`, or the
  model you actually are if you are not Codex.
- Do M1-M3 in order, then write `V1.8.1-READY` in dev/PROGRESS.md and STOP.
- Resolve other ambiguity by choosing reasonably and logging it. Stop only for
  `HONEST-FINDING`: a result that would change a tier or size outside the plan's allowed
  changes, change any Verus/scratch-server/Vest figure, trip a gate on Bertie, or mean the
  plan's rules can't be applied as written. Write it in NOTES and stop.
- Never modify `~/src/bertie-deps` or any other checkout outside this repo; use
  copies under `target/` for replays.
- No push, no tag, no release bundle, no host installs, no email.
- Build/report output goes to files under `logs/` or `target/`; read only `tail -n 30` or grep.
  Never stream full logs into your context.
- Run `cargo test`, `cargo clippy --all-targets -- -D warnings` and `cargo fmt --check`
  before each commit.
- Report honestly: say what passed, what failed, what was skipped. Dudley checks "done" by
  rerunning acceptance.
