Implement dev/PLAN-HAX-FSTAR.md (v1.7) in this repository. That plan is authoritative, and its
guiding principle is increased honesty and transparency.
- Read the plan, README.md, docs/evidence.md, docs/survey.md, dev/PLAN-VERIFY-RUNS.md (Docker and
  mount rules), src/evidence.rs, src/kani.rs, src/report.rs and the v1.5/v1.6 regression/replay
  scripts before starting. Also read ../bertie-spike/REPORT.md (Proofs section) and
  ../coverage-survey/bertie/baseline-whole-crate.md. Re-probe the grounded facts you rely on.
- Work only in this repository. ../bertie-spike, ../scratch-server and ../coverage-survey are
  read-only; export Bertie with `git archive`, never read its working tree. Commit one commit per
  milestone (M1-M5), plus separate commits for any genuine tool-bug fix with a regression test.
  Use the repo's configured identity; do not override it. Don't push, tag or release, and don't
  contact upstream.
- Third-party code (hax, F*, Z3, cargo for Bertie and its build scripts) runs only in Docker under
  the plan's mount rules. No host installs of any kind.
- Redirect every build/test/verifier log to a file under logs/v1.7/ or runs/v1.7/ and read only
  `tail -n 30` or grep.
- Never weaken evidence or receipt validation, and never tune detection to raise a number. A failed
  or partial F* run is the result; record it.
- Keep dev/PROGRESS.md (a "v1.7" section with M1-M5 checkboxes and evidence) and dev/NOTES.md
  (timestamped decisions) current. Resolve ambiguity conservatively and in favour of disclosure,
  log it, don't stop to ask.
- Finish with a short summary: per-milestone result; for both Bertie trees (upstream ef65f76 and
  shipped fbbcc13): proven-extracted SLoC and %, trusted items, escapes, contract-kind counts,
  libcrux SLoC measured; how certificate_verify / ecdsa_signature were classified; whether the
  v1.6 replay is unchanged; and anything not done.
