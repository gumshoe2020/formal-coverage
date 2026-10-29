# PLAN-PERF.md: v1.3, report speed

Requested by Andrew 2026-10-28 ("It ran slow on our random repos"). Authoritative for this milestone.

## Finding (Dudley, macOS `sample` of a release run on Firecracker, 2026-10-28)

Firecracker takes 33 s wall; Vest 32 s. Nearly all samples (6,715 of 6,718) sit in
`evidence::apply_evidence_many`, under a `slice::Iter<String>::any` closure: string `memcmp`,
stable sorts and BTreeSet appends. Source (src/evidence.rs, around line 357):

    for u in &mut report.units {
        if u.category == Runtime && tests.iter().any(|t| identifiers(t).contains(&u.name.as_str()))

`identifiers()` builds a fresh `BTreeSet<&str>` from a test's token text, for every unit times
every test: O(units x tests x tokens log tokens). Firecracker has no verifier receipts, so this
is pure inventory cost.

## Change

1. Build the identifier set of all test functions once (a `HashSet<&str>` or `BTreeSet<&str>`)
   and test each unit's name against it. Same for any other per-unit rescans in this function
   (`identifiers(&u.tokens)` computed twice per unit; escape hatches filtered per unit by
   `affected_units.contains` - index escapes by unit id once).
2. Then profile again (`sample` on macOS, or `cargo build --release` + timing) and fix the next
   hot spot only if it is above 20% of wall time. Stop when the three survey repos each run in
   under 5 s or no single hot spot exceeds 20%; record the profile summary in NOTES.md.
3. No change to output. This is a pure performance change.

## Acceptance

A. Byte-identical JSON and Markdown before and after (except timing fields, if any) for the
   three survey repos (Vest with its config and receipt, Anvil, Firecracker; see
   scripts/survey.py for the exact commands; reuse the extracted trees under
   <temporary-export> read-only, or re-export per the script) and all 13
   scratch-server branches (scripts/target_deps.py). Write a script that diffs old vs new.
B. Wall times before/after for each, in PROGRESS.md. Target: Vest and Firecracker under 5 s.
C. A regression test that a synthetic workspace with 5,000 units and 2,000 test functions
   reports in under 2 s in release (mark it `#[ignore]` if it is too slow in debug, and run it
   in release in the acceptance record).
D. `cargo test --frozen` green, `cargo clippy --frozen -- -D warnings` clean.

## Fences

This repository only; survey/scratch trees read-only. CARGO_HOME="$PWD/.cargo-home",
TMPDIR="$PWD/.tmp", no network, build output to files and read only `tail -n 30`.
One commit per change, not pushed. PROGRESS.md (## v1.3) and NOTES.md current.
