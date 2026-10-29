# PLAN-PROOF-CODE.md: v1.1-a, count proof code separately

Approved by Andrew 2026-10-27 ("Let's start with the honest fix first"). Authoritative for this
milestone. It is the first part of the v1.1 work in PLAN.md. Target-accurate dependency closure
and self-measurement remain separate, later milestones.

## Problem
Scratch-server stage 13 added about 4,400 SLoC of Verus intent lemmas (`proof fn` and `spec fn` in
the ghost module `crates/http-subset/src/intent.rs`). v1 counts that code as security-relevant
runtime code with status tested/unverified. The first-party security headline fell from
715/1,685 = 42.4% to 715/6,095 = 11.7% even though the runtime binary is byte-for-byte unchanged.
Proof code is erased at compile time and never runs, so it doesn't belong in the denominator of
"how much of the running code is proven". It's real, valuable work, so it should still be visible.

## Change
1. **New unit category `proof-code`.** A unit is proof code iff it is a Verus `spec fn`,
   `proof fn` or `broadcast proof fn`, with any visibility or `open`/`closed` qualifier. Classify
   by syntax only; `exec` functions (plain `fn`) are never proof code, including those carrying
   requires, ensures, invariant or ghost blocks, which stay in their runtime unit as today. Proof
   code is classified before the security/everything-else split and is removed from both
   denominators.
2. **Report proof code as its own section** in JSON and Markdown: `PROOF CODE`, with SLoC, units,
   and how many units are covered by source-bound verifier evidence (`verified`) or not
   (`unverified`), using the existing evidence rules. Split it by whether it lies in a configured
   security path or not, for information only.
3. **Escape hatches stay global.** `assume`, `admit`, `external_body`, `external` and `unsafe`
   inside proof code are still reported as `hazard` in the existing hazard section, are called out
   at the top, and still block gates. Removing proof code from the denominators must never hide a
   hatch, because a proof-side `assume(false)` can poison runtime proofs.
4. **Headline** = proven runtime security SLoC / runtime security SLoC. Put proof-code SLoC next to
   it, never folded in.
5. **Schema version 2.** `gate --baseline` refuses a v1 baseline with a clear message unless
   `--allow-schema-change` is given, so before/after percentages are never compared silently.
6. **Docs:** update README.md and docs/evidence.md with the definition, the reason (erased at
   compile time), and the hazard rule.

## Acceptance
- Unit tests for the classifier cover every qualifier form (`pub open spec fn`, `closed spec fn`,
  `pub proof fn`, `broadcast proof fn`, `proof fn` inside `impl` blocks and `verus!` macros); an
  exec `fn` with requires/ensures stays runtime; a `fn` named `spec_something` stays runtime; and a
  string or comment containing "proof fn" doesn't misclassify.
- Fixture test: an `assume(false)` in a proof fn shows as a security hazard and fails
  `gate --no-new-escape-hatches`.
- **Invariant check on scratch-server:** export (with `git -C ... archive`, never checkout) the tips
  of `header-policy` (stage 12) and `spec-lemmas` (stage 13). Their runtime security headline
  numerator and denominator must be **identical**, because stage 13 changed no runtime code; only
  proof-code SLoC differs. If they differ, find out why and record it before claiming acceptance.
- Re-measure every scratch-server stage branch (main, nix-scratch, verified-boundary,
  browser-whitelist, https, https-headers, idle-connections, proven-transport, firefox-headers,
  chromium-headers, pragma-header, header-policy, spec-lemmas, and any others present) with the
  same config/evidence approach as M5. Update docs/scratch-server.md with v1.1-a columns: runtime
  security proven %, proof-code SLoC, and the v1 number for comparison.
- `cargo test`, `cargo clippy -- -D warnings` pass.
