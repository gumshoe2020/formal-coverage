Implement PLAN-RELEASE.md in this repository. That plan is authoritative.
- Read the plan, README.md, docs/survey.md, docs/evidence.md and the scripts it names before
  starting. Re-probe the grounded facts you rely on.
- Hard fence: no measurement rule, grammar or threshold changes; no published proven % may move.
  Third-party code runs only in Docker as in v1.4 (pinned images, --network none for verifiers,
  never mount $HOME, ~/.ssh, ~/.cargo or the Docker socket). No host installs.
- scratch-server is read-only and is being edited by another agent right now: measure it only via
  `git archive` of the pinned commits listed in the plan, never its working tree.
- Commit only in this repository, one commit per milestone (M1-M5). Don't push, don't rewrite
  history. The repo's git identity is already set; don't change it.
- Redirect every build/verifier log to a file and read only `tail -n 30` or grep.
- Keep PROGRESS.md (add a "v1.4.1 release pass" section with M1-M5) and NOTES.md (timestamped)
  current. Resolve ambiguity conservatively, log it, don't stop to ask.
