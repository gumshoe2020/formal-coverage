Implement PLAN-PERF.md (v1.3) in this repository. That plan is authoritative.
- Read the plan and src/evidence.rs before changing anything; measure before and after.
- Respect the fences: this repository only, survey and scratch trees read-only, no network,
  project-local CARGO_HOME/TMPDIR, build output to files, read only `tail -n 30`.
- Output must stay byte-identical; prove it with the diff script (acceptance A).
- Resolve ambiguity conservatively and log it in NOTES.md; don't stop to ask. One commit per change,
  not pushed.
