Implement PLAN-TARGET-DEPS.md (v1.2) in this repository. That plan is authoritative.
- Read the plan, then src/classify.rs, src/lib.rs, src/report.rs, README.md and docs/evidence.md
  before changing anything.
- Respect every fence in the plan: this repository only (scratch-server read-only), no network,
  project-local CARGO_HOME and TMPDIR, build output to files and read only `tail -n 30`.
- Meet acceptance A-D yourself and record the commands and results in PROGRESS.md (## v1.2).
- Resolve ambiguity by choosing the more conservative option and logging it in NOTES.md; don't stop
  to ask. Propose any codebase learnings in NOTES.md; don't edit AGENTS.md.
- One commit per change, not pushed.
