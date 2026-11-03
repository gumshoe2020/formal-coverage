Implement dev/PLAN-HAX-FSTAR-v1.7.1.md (v1.7.1) in this repository. That plan is authoritative and
extends dev/PLAN-HAX-FSTAR.md (v1.7), whose non-negotiables all still apply. Guiding principle:
increased honesty and transparency.
- Read both plans, README.md, docs/evidence.md, the Bertie section of docs/survey.md,
  scripts/hax/README.md, runs/v1.7/bertie/{manifest.json,summary.json,run.sh,audit-lock.py,
  amd64-check.sh}, and the v1.7 entries in dev/NOTES.md before starting. Re-probe the grounded
  facts you rely on.
- Work only in this repository. ../scratch-server, ../bertie-spike and ../coverage-survey are
  read-only; export with `git archive` (scratch-server e51db51, Bertie ef65f76 / fbbcc13), never
  read their working trees. Reuse the v1.7 toolchain under runs/v1.7/bertie read-only; do not
  rebuild it or modify runs/v1.7. Write outputs to runs/v1.7.1/ and logs/v1.7.1/.
- Commit one commit per milestone (M1-M4), plus separate commits for any genuine tool-bug fix
  with a regression test. Use the repo's configured identity. Don't push, tag or release, and
  don't contact upstream.
- Third-party code (hax, F*, Z3, cargo for Bertie/scratch-server and build scripts) runs only in
  Docker under the v1.7 mount rules. No host installs of any kind.
- Redirect every build/test/verifier log to a file under logs/v1.7.1/ or runs/v1.7.1/ and read
  only `tail -n 30` or grep.
- Never weaken the v1.7 audit, receipts or command grammar, and never tune detection to raise a
  number. Audit v2 is a new, separate check. If the lock audit v2 blocks, if hax 0.4.2 cannot
  extract, or if patches don't apply, that is the result: record it and stop that branch. Do not
  try other hax versions.
- Keep dev/PROGRESS.md (a "v1.7.1" section with M1-M4 checkboxes and evidence) and dev/NOTES.md
  (timestamped decisions) current. Resolve ambiguity conservatively and in favour of disclosure,
  log it, don't stop to ask.
- Finish with a short summary: per-milestone result; the full lock diff and audit v2 verdict for
  both trees; for both trees (upstream ef65f76, shipped fbbcc13): proven-extracted SLoC and %,
  trusted items, escapes, contract-kind counts, out-of-proof-scope SLoC by reason (feature-gated,
  stream, non_hax), libcrux SLoC; which F* version was used and whether patches applied; how
  certificate_verify / ecdsa_signature were classified; whether the v1.6 replay and v1.7
  fixtures are unchanged; and anything not done.
