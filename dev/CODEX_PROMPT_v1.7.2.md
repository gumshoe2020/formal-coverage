You are making formal-coverage v1.7.2. `dev/PLAN-HONEST-OPTIONS.md` is authoritative; read it fully
first, then the builder playbook's "Gotchas learned".

Rules:
- Enforce every non-negotiable in the plan. The option check stays a whitelist.
- Keep `dev/PROGRESS.md` (add a v1.7.2 section with M1-M4 checkboxes) and NOTES (timestamped
  decisions, evidence, exact rerun commands) current. Commit after every milestone, on `main`, local only.
- Do M1-M4 in order, then write `V1.7.2-READY` in dev/PROGRESS.md and STOP.
- Resolve other ambiguity by choosing reasonably and logging it; stop only for `HONEST-FINDING`.
- No push, no tag, no release bundle, no host installs, no email. Docker only as the plan allows.
- Build/report output goes to files under `logs/`; read only `tail -n 30` or grep. Never stream full
  logs into your context.
- Report honestly: say what passed, what failed, what was skipped. Dudley checks "done" by rerunning
  acceptance.
