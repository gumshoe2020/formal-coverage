# formal-coverage v1.9.1, run 2: finish M2 after the check (b) decision

Read, in order, all still authoritative: `dev/v191/PLAN.md` (including its new "Amendment
2026-11-25" section), `dev/v191/CODEX_PROMPT.md`, then `dev/v191/NOTES.md` and `dev/v191/PROGRESS.md`
from run 1. M0 and M1 are done and committed. Continue M2 only.

## Decision on run 1's HONEST-FINDING (Dudley, 2026-11-25)
Stopping was correct. The finding is a gap in check (b) as written, not in form A. The two
`be_value_prefix_{upper,lower}_128` units are `category: proof-code` (cfg(hax) extraction-only
helpers), outside the security runtime totals. The unchanged v1.8.1 classifier puts a strictly
checked proof-code unit in `verified`, which is the correct existing tier for it.

Change check (b) in `dev/v191/compare.py` so the allowed destinations are:
- `proven-extracted` (any form), or
- `trusted`, only when every crediting form is D, or
- `verified`, only when the unit's category is `proof-code` before and after, and its form is A–E.
Also assert that no `proof-code` unit's change moves any runtime total (the existing totals
arithmetic must still pass unchanged). Do not touch any other check, rule or assertion, and do not
change `src/`. If anything beyond this needs to change for the replay to pass, stop with
`HONEST-FINDING` as before.

Then finish M2 as PLAN.md describes: rerun `python3 dev/v191/replay.py` from an empty `target/v191`,
regenerate compare.md (with a short note at the top that check (b) was amended and why), confirm the
README `## v1.9.1` section matches what was actually built (B refused for lack of a cfg set in the
receipt; D credited as trusted), and rerun acceptance 1–6 on the final tree, recording exit codes
with timestamps in PROGRESS.md. Commit with prefix `v191 M2:` and the same trailer. No push, no
tag. Last line of PROGRESS.md `V1.9.1-READY` only if every check passes.
