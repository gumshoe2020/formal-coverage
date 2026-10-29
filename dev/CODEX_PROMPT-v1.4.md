Implement PLAN-VERIFY-RUNS.md (v1.4) in this repository. That plan is authoritative.
- Read the plan, docs/survey.md, scripts/survey.py and the README's receipt section before starting.
  Re-probe the grounded facts you rely on.
- Hard fence: third-party code (dependency builds, Verus, Kani, project tooling) runs ONLY inside
  Docker containers with the mounts the plan allows. Never mount $HOME, ~/.ssh, ~/.cargo or the
  Docker socket. No host installs, no Docker Desktop setting changes.
- Survey clones in ../coverage-survey are read-only. Commit only in this repository, one commit per
  milestone (plus separate commits for any genuine tool-bug fix, with a regression test). Don't push.
- Redirect every build/verifier log to a file and read only `tail -n 30` or grep. Never stream full
  Verus, Kani or cargo output.
- Failures, timeouts and OOMs are results: narrow scope as the plan says, record them, keep going.
  Never weaken receipt validation or tune the tool to raise a number.
- Keep PROGRESS.md (M1-M4) and NOTES.md (timestamped) current. Resolve ambiguity conservatively,
  log it, don't stop to ask.
