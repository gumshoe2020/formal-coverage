You are building milestone v1.1-c of `formal-coverage` in this directory:
<checkout> (git branch `main`).

`PLAN-REALWORLD.md` is the authoritative spec: read it fully first, then
<projects>/coverage-survey/FINDINGS.md, PLAN-UNION-MODULES.md,
PLAN-PROOF-CODE.md, PLAN.md, README.md, docs/evidence.md, PROGRESS.md and NOTES.md.

Non-negotiables:
1. Write only inside this directory and temp dirs under /tmp. The scratch-server repository and
   <projects>/coverage-survey/ (vest, anvil and firecracker clones,
   configs and logs) are read-only to you. Run verifiers on copies under /tmp if they would write
   build output into the clones. Never modify scratch-server or its containers. Don't edit
   ~/.codex, ~/.cargo config, shell rc files, Docker or Nix config. Don't install Kani and don't
   build Verus from source.
2. Do not `git push`. Commit locally at checkpoints and when the milestone passes (or when blocked,
   with the blocker). Commits use this repo's configured identity (Dudley); do not override it.
3. Honest by construction: never report `proven` without verifier run evidence. Trusted items and
   escapes are always listed next to any proven percentage. Out-of-scope hazards still fail the gate.
4. If scratch-server's numbers change, or Vest is still 0%, don't tune the tool to get the expected
   number. Find the cause, record it in NOTES.md, and report it.
5. Treat the third-party clones as untrusted code: don't run their build scripts, tests or tools.
   Only run the pinned Verus binary on their sources.

Keep your context small: use release builds for real-world runs. Redirect cargo and verifier
output to ./logs/ and read only `tail -n 30`, or grep it for errors.

Observability: add a "v1.1-c" section to PROGRESS.md, and tick items only when their acceptance
command passed. Append timestamped NOTES.md entries. Finish with a short summary that includes:
the scratch-server stage 12/13 numbers; Vest's proven %, trusted items and escapes; Firecracker's
distinct hazard sites; and the Anvil outcome.
