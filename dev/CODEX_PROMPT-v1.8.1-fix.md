Dudley's acceptance review of v1.8.1 (b70c00b) found one defect. Fix it, nothing else.

Defect: the four `bertie_tls::der::tests::*` units under `#[cfg(all(test, hax))]` moved from
`test` to `proof-code` (57 SLoC counted as proof). Test code is never proof. The existing rule
("items under cfg(test) at any depth inventoried as test-code") takes precedence over v1.8.1
rule 1. Andrew's plan intent: only extraction-only *proof* items become proof-code; anything
already classified as test stays test, with its v1.8.0 category, status and size unchanged.
The same precedence applies to rule 2: lines inside test units are never inline proof.

Do:
1. Add the precedence to the classification (one place) plus tests: `cfg(all(test, hax))` item
   stays test; item inside a `cfg(test)` mod that is also `cfg(hax)` stays test; a `cfg(hax)`
   statement inside a test fn is not inline proof.
2. Add one line to `dev/PLAN-FSTAR-RATIO-v1.8.1.md` rule 1: "Test classification takes
   precedence: anything the tool classifies as test today stays test."
3. Update `dev/v181/cfghax.py` (the independent scanner) to exclude test-scoped code, rerun the
   M3 replay/compare (`dev/v181/replay.py`, `dev/v181/compare.py`) and regenerate
   `dev/v181/compare.md`. Expected: 18 units / 197 standalone SLoC; the four tests back to
   test/unverified with their old sizes; scratch-server, Vest and fixtures still identical.
4. Update NOTES (timestamped) and docs/v1.8.md if it states the old figures. cargo test, clippy
   -D warnings, fmt --check. One local commit on main with the usual trailer. Write
   `V1.8.1-FIX-READY` in dev/PROGRESS.md and STOP.

All rules from dev/CODEX_PROMPT-v1.8.1.md still apply (no push/tag/release, no writes outside
this repo, logs to files, HONEST-FINDING stop conditions).
