Implement dev/PLAN-PROOF-RATIO.md (v1.6) in this repository. That plan is authoritative.
- Read the plan, README.md, docs/evidence.md, docs/scratch-server.md, src/lib.rs, src/report.rs,
  src/inputs.rs and scripts/v15_regression.py / scripts/v15_replay.py before starting. Re-probe the
  grounded facts you rely on.
- Work only in this repository. scratch-server and the survey checkouts in ../coverage-survey are
  read-only. Commit one commit per milestone (M1-M5), plus separate commits for any genuine tool-bug
  fix with a regression test. Use the repo's configured identity; do not override it. Don't push,
  tag or release.
- Offline only: cargo --frozen with the existing cache. No Docker runs are needed for this
  milestone; if you believe one is, log why in NOTES.md and skip that step instead.
- Redirect every build/test/replay log to a file under logs/v1.6/ and read only `tail -n 30` or grep.
- Never weaken evidence or receipt validation, and never tune detection to raise a number. Mixed
  runtime/ghost lines stay runtime.
- Keep dev/PROGRESS.md (a "v1.6" section with M1-M5 checkboxes and evidence) and dev/NOTES.md
  (timestamped decisions) current. Resolve ambiguity conservatively, log it, don't stop to ask.
- Finish with a short summary: per-milestone result, the spec-lemmas numbers from M4 (standalone,
  inline, proven runtime in both modes, ratio, status), survey ratios, and anything not done.
