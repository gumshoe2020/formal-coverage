# PLAN-REALWORLD.md: v1.1-c, fair results on real-world verified projects

Approved by Andrew 2026-10-27 ("yes"). Authoritative for this milestone. It follows v1.1-b. Evidence
is in `<projects>/coverage-survey/FINDINGS.md`: v1.1-b reports
**0%** proven for Vest even though Vest's `vest_lib` verifies (2,935 verified, 0 errors with the
pinned Verus). The tool stays honest; it has to stop being blunt.

## Changes
1. **Evaluate cfg against the run's actual configuration.**
   - The receipt gains an optional `cfg` field: the `--cfg` values and enabled features of the
     verified build. When it is absent, infer it from the command where that's unambiguous
     (direct `verus` with no `--cfg` means no features).
   - Evaluate `#[cfg(...)]` and `cfg_attr` predicates (`all`, `any`, `not`, `feature = "..."`,
     `test` and plain names) against that set. Include the names Verus itself sets during
     verification only if you confirm them from Verus source or docs (for example
     `verus_keep_ghost`); otherwise mark them "unconfirmed" and treat them conservatively.
   - A `cfg_attr` that only adds an attribute which doesn't change which items exist (`no_std`,
     `feature(...)`, `allow`/`warn`/`deny`, `doc`) never blocks credit.
   - An unevaluable predicate stays conservative for **that item or module only**, never for the
     whole crate, and gets a warning naming it.
2. **Scope hazards to the verification run.** A hazard only affects attribution inside the
   receipt's verified scope (the root plus credited modules). Hazards in other crates, or in files
   the run never verified, are still reported and still fail `--no-new-escape-hatches`, but they
   don't block credit elsewhere.
3. **Trusted base, not zero.**
   - `#[verifier::external_body]` (and `assume_specification` if present; confirm the name in
     Verus) marks a **trusted** item. That item is `trusted` (not proven). Other units in scope
     can still be credited.
   - `assume`, `admit` and `#[verifier::external]` on verified paths are **unsound escapes**: the
     unit containing one is `hazard`, it's called out at the top, and it fails the gate. Other units
     in scope are credited but marked as relying on those escapes.
   - Every report states the headline as "proven, relying on N trusted items (S SLoC) and E
     escapes", with the full list in JSON and Markdown. Never print a proven percentage without
     that line next to it.
   - Write down in docs/evidence.md why this is still conservative: Verus checks callers against
     the trusted item's spec, so the trust is explicit and listed, never hidden.
4. **Count hazards once per site.** Report distinct (file, line, kind) sites. Affected units are a
   separate list, and duplicates must not inflate counts.
5. **Read Kani correctly.** `kani::assume` inside a `#[kani::proof]` harness constrains harness
   inputs and isn't an escape hatch. `#[kani::proof]` functions and `#[cfg(kani)]` modules are proof
   code, not runtime. Successful harnesses still only give `proven-bounded`, and never count toward
   the unbounded percentage.
6. **Release builds.** README and scripts use `cargo build --release` for real-world runs. Add a
   simple timing note.

## Acceptance
- Unit/fixture tests for every rule above:
  - feature-gated modules, credited or not depending on the receipt's cfg;
  - a harmless `cfg_attr` doesn't block credit;
  - an unevaluable predicate blocks only its own item;
  - an out-of-scope hazard doesn't block credit, but still fails the gate;
  - external_body shows as trusted and its callers are credited;
  - an assume in scope shows as a hazard and fails the gate;
  - file-scope `unsafe` is counted once;
  - `kani::assume` in a harness isn't a hazard;
  - a harness counts as proof code.
- **Scratch-server is unchanged:** stage 12 and stage 13 still report 644/1,363, with the same
  proof-code totals and 0 hazards.
- **Vest:** re-run Verus on `vest_lib` (use the scratch-server pinned Verus at
  `<scratch-server>/.tools/`, read-only, with its
  CARGO_HOME/RUSTUP_HOME), write the receipt with the actual cfg, and report the
  `vest_lib` runtime proven %, the trusted-item count and SLoC, and the escapes. Use the survey
  config at `coverage-survey/configs/vest.toml`. Expect nonzero; if it's still 0%, find out why and
  record it.
- **Firecracker** (inventory only, no Kani run): the hazard count drops to distinct sites, and the
  `kani::assume` entries are gone.
- **Anvil:** try Verus on Anvil's verified crate with the same pinned Verus. If it fails because
  Anvil tracks Verus main, record the error and report inventory only. Don't build Verus from
  source.
- Update coverage-survey results as a new table in `docs/survey.md` here (don't edit
  coverage-survey files). Re-measure the scratch-server branches and update docs/scratch-server.md
  only if numbers change, with the reason.
- `cargo test`, `cargo clippy -- -D warnings` and formatting pass.
