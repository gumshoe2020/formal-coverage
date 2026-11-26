# v1.11.0 progress

- [x] M1 classifier (R1-R4 in src/contract.rs; old heuristic removed; unit + fixture tests)
- [x] M2 report (R6 in Rust report, summary.proof_depth, kit section, metrics, docs, CHANGELOG, version 1.11.0)
- [ ] M3 replay (dev/v1110/replay.py, compare.md)

See NOTES.md: HONEST-FINDING on two staged controls (found at M1 probe).
- [x] M3 replay written and run (dev/v1110/replay.py, compare.md). STOPPED: HONEST-FINDING, see NOTES.md.

## Acceptance (2026-11-26T00:52:29Z)

1. cargo test --no-fail-fast -> 0
2. cargo clippy --all-targets -- -D warnings -> 0
3. cargo fmt --check -> 0
4. .git/hooks/pre-push gate -> 0
5. python3 dev/v1110/replay.py -> **1** (checks a, b, d, e PASS in both modes; check c FAIL: 5 controls differ from PLAN.md's table)
6. formal-coverage --version -> formal-coverage 1.11.0

V1.11.0 NOT READY: awaiting a decision on two rule questions in NOTES.md (check_eq1 integer equality; R4 and runtime-category spec predicates). No V1.11.0-READY line written.

## M4 amendment A1 (2026-11-26T01:01:19Z)

- [x] M4 A1.1 data-integer operands are values (unit test: u8 equality functional; usize bound size-only), A1.2 rename to "functional, checked only against own code" / `functional_own_code_only`; replay control table updated, compare.md regenerated. NOTES.md HONEST-FINDING resolved by A1.

Acceptance (2026-11-26T01:01:19Z):
1. cargo test --no-fail-fast -> 0
2. cargo clippy --all-targets -- -D warnings -> 0
3. cargo fmt --check -> 0
4. .git/hooks/pre-push -> 0
5. python3 dev/v1110/replay.py (empty target/v1110) -> 0, all PASS
6. target/debug/formal-coverage --version -> formal-coverage 1.11.0

V1.11.0-READY
