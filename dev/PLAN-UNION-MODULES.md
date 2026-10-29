# PLAN-UNION-MODULES.md: v1.1-b, multi-receipt union and module crediting

Approved by Andrew 2026-10-27 ("yes please"). Authoritative for this milestone. It follows v1.1-a
(PLAN-PROOF-CODE.md). Target-accurate dependency closure and self-measurement remain later
milestones.

## Problem
1. **One receipt per report.** Scratch-server verifies three Verus roots (`crates/http-subset/src/lib.rs`,
   `crates/transport-core/src/lib.rs`, `crates/transport-core/src/aead.rs`). The tool accepts one
   `--verus-log`, so a single report credits only one root: 453/1,363 = 33.2% for stage 13. The
   honest figure, 644/1,363 = 47.2%, currently comes from an external script
   (scratch-server `scripts/measure-coverage.py`) that unions three reports by unit id.
2. **Module files aren't credited.** A successful Verus run on a crate root verifies every module
   that root declares with `mod name;`. The tool credits only the invoked root file, so stage 13's
   `crates/http-subset/src/intent.rs` (4,410 proof-code SLoC, verified by `./verify.sh`) shows as
   unverified.

## Change
1. **Multiple receipts.** `report` and `gate` accept `--verus-log` (and `--kani-log`) more than once.
   Each log keeps its own `.evidence.json` receipt and is validated independently under the
   existing evidence rules (source hashes, exit code, tool version, command). A unit is credited if
   **any** accepted receipt covers it. A rejected receipt credits nothing and is listed with the
   reason. The report lists every receipt and which units each one credited. The union must happen
   inside the tool, never by post-processing.
2. **Module crediting.** From an invoked root, follow `mod name;` declarations by Rust's file rules
   (`name.rs` or `name/mod.rs`, relative to the declaring file) and credit units in those files.
   Be conservative:
   - Every followed file must appear, with a matching hash, in the receipt's `files`. Otherwise
     credit nothing in that file and warn.
   - Don't follow a `mod` with a `#[path]` attribute, or one gated by a `#[cfg(...)]` you can't show
     Verus enables. Leave it unverified, with a warning naming the module.
   - If the receipt's command restricts verification (`--verify-module`, `--verify-only-module`,
     `--verify-function`, `--no-verify` or similar; check Verus's own help or docs, and write
     "unconfirmed" for any flag you can't confirm), credit only what was actually verified. If
     unsure, credit only the root file.
   - `#[verifier::external]`/`external_body` items stay hazards and are never credited.
   - Inline `mod name { ... }` blocks in a credited file are already part of that file.
3. **Docs:** update README.md and docs/evidence.md with the union rule and the module rule,
   including the conservative exclusions.

## Acceptance
- Unit and fixture tests:
  - two receipts each covering different roots credit both;
  - a stale or mismatched receipt credits nothing while the others still count;
  - `mod a;` resolves to `a.rs` and to `a/mod.rs`;
  - a module file missing from the receipt's hashes isn't credited;
  - `#[path]` and unproven `#[cfg]` modules aren't credited, with a warning;
  - a restricted-verification command credits only the restricted scope (or only the root);
  - a module-level `assume`/`external_body` still shows as a hazard and fails the gate.
- **Scratch-server stage 13 (`spec-lemmas`)** with all three receipts in **one** CLI call: runtime
  security proven = **644/1,363**, matching the external union exactly. If it doesn't match,
  find the reason and record it; don't tune to match. `intent.rs` proof code is credited as verified.
- **Stage 12 (`header-policy`)**: same runtime numbers as stage 13 (the v1.1-a invariant still holds).
- Re-measure every scratch-server stage branch with one CLI call per branch using all of that
  branch's Verus roots. Update docs/scratch-server.md: runtime security proven %, proof-code SLoC
  and verified share, and the v1 number for comparison. Note which stages have several roots.
- `cargo test`, `cargo clippy -- -D warnings` and formatting pass.
