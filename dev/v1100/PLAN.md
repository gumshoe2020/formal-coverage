# formal-coverage v1.10.0 — coverage numbers recorded on main, not in PRs

Status: APPROVED by Andrew 2026-11-25 ("yes to all", after Dudley's CI-bottleneck note).
Builder: Codex under CODEX_PLAYBOOK; Dudley reruns acceptance. No push/tag until Andrew OKs the
release.

## Why
Today every PR that changes coverage must commit the new numbers (`coverage/baseline.json` and the
measured section of `COVERAGE.md`), and the ratchet compares against the baseline committed at the
merge base. Two such PRs always conflict in those files; one waits, merges main, and re-runs the
full F* check. bertie-tls hit this on 2026-11-25 (PRs #87 and #89).

## New mode (opt-in; the existing committed-baseline mode stays the default and unchanged)
- **Floor from main's recorded measurement.** In the new mode, a PR's ratchet compares its fresh
  measurement against the measurement recorded for its merge-base commit on main, read from a data
  branch (default `coverage-data`), file `baselines/<full-sha>.json`, schema identical to today's
  baseline.json. If that file is missing (main commit not yet measured, or history before
  adoption): fail closed with a message naming the commit, except when an explicitly configured
  `bootstrap_baseline` path is given for the first adoption commit only.
- **Recording on main.** A job on `push` to main (extend `publish-badges` or add `publish-baseline`;
  pick one, log why) writes `baselines/<sha>.json` and an up-to-date rendered
  `COVERAGE-MEASURED.md` (today's measured-section content) to the data branch, with the same
  least-privilege model as badges today (`contents: write` only on that job, default
  `GITHUB_TOKEN`, never touching main). Writes are append-only per sha; an existing file for a sha
  is never overwritten.
- **No numbers in PRs.** In the new mode the action doesn't check `coverage/baseline.json` or the
  COVERAGE.md measured section against the measurement; projects may delete them and link to the
  data branch's `COVERAGE-MEASURED.md` instead. Prose in COVERAGE.md (tool changes, exceptions) is
  untouched.
- **Ratchet semantics unchanged:** same rules and exact-rational arithmetic, only the floor's
  source changes. The urgent-fix exception keeps working (document how it's expressed in the new
  mode).
- **Concurrency:** two PRs merged back to back each compared against their own merge base; after
  both merge, main's own push run measures the combined state and fails visibly if the combination
  breaks the ratchet against main's previous recorded sha. Document that projects should enable
  "require branches to be up to date" or a merge queue to avoid that; the kit doesn't change repo
  settings.

## Milestones
- M1: config key (`ci.json`: `floor: "committed" | "recorded"`, `data_branch`, `bootstrap_baseline`);
  ratchet reads the recorded floor; fail-closed paths; unit tests for missing sha, bootstrap,
  normal compare, append-only refusal.
- M2: recording job/action; tests with a local bare repo as remote (no network): first record,
  second record for a new sha, refusal to overwrite, PR mode reads the merge-base file.
- M3: docs (ci/README.md, README "Use in CI"), CHANGELOG 1.10.0, version bump; the existing mode's
  behaviour byte-identical (replay the v1.9.1 kit tests and the bertie/scratch-server kit fixtures).

## Non-negotiables
Default mode unchanged. Fail closed on any missing or unreadable floor. Never write to the default
branch. Python standard library only in the kit, as today. No push, no tag.

## After this (bertie-tls, separate PR, needs the release)
Pin v1.10.0, switch to `floor: "recorded"` with a bootstrap baseline equal to main's current
measurement, remove the measured section from COVERAGE.md (link to the data branch) and stop
committing baseline.json. scratch-server can follow later.
