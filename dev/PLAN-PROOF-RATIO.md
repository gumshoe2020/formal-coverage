# PLAN-PROOF-RATIO.md: v1.6, proof-to-code ratio with a configurable acceptance range

Requested by Andrew 2026-10-31 ("we should be measuring lines of code over lines of proof in our
coverage tool too"; "make the default acceptance a configurable range"). Authoritative for this
milestone. Starts from main at 887b730 (v1.5 Linux).

## Why

A proof percentage says how much code is proven, not what the proof cost. The ratio of proof lines
to proven runtime lines tracks that cost per stage and over time (for scale, seL4 is roughly 20-25
proof lines per line of C). It is a cost signal, not a quality score: `ensures true` has a ratio
of zero. A very low ratio for proven code is worth a look (possibly a weak spec), and a very high
one is worth a look (possibly proof bloat). That is what the acceptance range expresses.

## Grounded facts (verified 2026-10-31)

1. Units are found by walking `proc_macro2` token streams (`src/lib.rs` ~460-560). A `fn` preceded
   by the ident `spec` or `proof` (or a Kani harness / `cfg(kani)`) becomes `Category::ProofCode`;
   everything else is `Category::Runtime`. Token spans carry line numbers.
2. Proof code is already inventoried separately and excluded from runtime denominators
   (`src/report.rs` ~117-122, "PROOF CODE" table, ~338 "proof-code: N SLoC alongside").
3. Inline Verus ghost code inside exec functions is counted as runtime SLoC today:
   `requires`/`ensures`/`recommends`/`decreases` clauses on signatures, `invariant`
   (and `invariant_except_break`, `ensures`, `decreases`) clauses on `while`/`loop`/`for`,
   `proof { ... }` blocks, `assert(...)` (including `assert(...) by { ... }` and
   `assert forall ...`), `assume(...)`, and `let ghost` / `let tracked` bindings.
4. Retained evidence exists for scratch-server spec-lemmas (`docs/scratch-server/spec-lemmas.*`,
   replayed by `scripts/v15_replay.py`). Current published figures (docs/scratch-server.md ~190):
   first-party security 644/1,363 proven, proof-code 4,732 SLoC.
5. Reports and gates already follow the v1.5 pattern: inputs need reasons, default-rules figures
   are shown next to with-inputs figures, `gate` fails on `TODO` reasons.

## Change

### 1. Inline ghost lines (`--ghost-lines separate|runtime`)

- `separate` (**new default**): inside each first-party runtime unit, mark the token ranges of the
  constructs in fact 3 as ghost. A source line is a **ghost line** only if every non-comment token
  on it is inside a ghost range; mixed lines stay runtime. Ghost lines leave the unit's runtime
  SLoC and are counted as inline proof SLoC for that unit. Record both numbers per unit in JSON
  (`runtime_sloc`, `inline_proof_sloc`).
- `runtime`: v1.5 behaviour. With an unchanged config, output must be byte-identical to v1.5
  (main at 887b730) for every case `scripts/v15_regression.py` covers, after removing only the
  listed new fields.
- Non-Verus crates are unaffected (no ghost constructs). Unparseable files keep every nonblank line
  as runtime (existing rule) and are listed.
- The report states the mode on its first line, and prints the runtime proven figures under
  both modes so the shift is visible.

### 2. Proof ratio

Per first-party crate, per module and overall, report:

- **proof SLoC** = standalone proof-code SLoC (fact 1) + inline proof SLoC (change 1);
- **proven runtime SLoC** = runtime SLoC with accepted evidence (status proven; proven-bounded
  reported as a separate column, not mixed in);
- **ratio** = proof SLoC / proven runtime SLoC, two decimals, `n/a` when proven runtime SLoC is 0.

Show the overall ratio for first-party security-relevant code (headline) and for all first-party
code. Also show standalone and inline proof SLoC separately so the split is visible. Proof code
that cannot be attributed to a module (e.g. crate-root lemmas) counts toward the crate and overall
rows and is listed as unattributed at module level.

### 3. Configurable acceptance range

- Default range: **min 1.0, max 10.0** proof lines per proven runtime line.
- Config (schema 3, existing file):
  ```toml
  [proof_ratio]
  min = 1.0
  max = 10.0
  reason = "why this range fits this project"
  ```
  A `[proof_ratio]` table without a non-empty `reason` is an error; a `TODO` reason fails `gate`,
  as for other inputs. Either bound may be omitted (open-ended), but not both.
- CLI `--proof-ratio-range MIN:MAX` (either side may be empty, e.g. `2:` or `:15`) overrides the
  config for that run and is recorded as such.
- The report prints the range, its source (`default`, `config` with reason, or `cli`) and a status:
  `within`, `below`, `above` or `n/a`.
- `gate`: fails when the overall first-party security-relevant ratio is `below` or `above` the
  range. `n/a` does not fail. Module rows outside the range are **warnings only**, listed in the
  report, never a gate failure. `--no-proof-ratio-gate` disables the ratio check for a run (recorded
  in the report).
- JSON carries `proof_ratio: {standalone_proof_sloc, inline_proof_sloc, proof_sloc,
  proven_runtime_sloc, ratio, range: {min, max, source, reason}, status, modules: [...]}`.

## Milestones and acceptance

- **M1 ghost lines.** Fixture crate with each construct in fact 3 (signature clauses, loop
  invariants on `while` and `loop`, `proof` block, `assert ... by`, `assert forall`, `assume`,
  `let ghost`, `let tracked`) plus mixed lines (`let x = f(y); assert(x > 0);` on one line) and
  multi-line clauses. Expected per-unit `runtime_sloc` / `inline_proof_sloc` are written by hand
  in the test and match. Mixed lines stay runtime.
- **M2 regression.** `--ghost-lines runtime` reproduces v1.5 output byte-for-byte for every case in
  `scripts/v15_regression.py` (after removing only the new fields). Log to `logs/v1.6/M2.log`.
- **M3 ratio and range.** Fixture tests: ratio arithmetic, `n/a` at zero proven, range from
  default/config/CLI with precedence CLI > config > default, open-ended bounds, missing reason
  is an error, `TODO` reason fails `gate`, `gate` passes within and fails below/above, module
  warnings don't fail `gate`, `--no-proof-ratio-gate` works.
- **M4 real numbers.** Replay scratch-server spec-lemmas with its retained evidence in both ghost
  modes. Report: standalone proof SLoC (expect 4,732 in `runtime` mode), inline proof SLoC,
  proven runtime SLoC in each mode, the ratio and its status under the default range, and the
  per-module rows. Sanity: in `separate` mode runtime SLoC drops by exactly the inline proof SLoC;
  proof SLoC total = standalone + inline. Also run the three survey repos (Vest, Anvil,
  Firecracker) as in v1.5 and record ratios (or `n/a`). Write the results into
  docs/scratch-server.md and docs/survey.md, labelled v1.6.
- **M5 docs and checks.** README section on the ratio, range and its limits (cost not quality;
  `ensures true` example), example config gains a `[proof_ratio]` table with a reason, evidence.md
  documents the ghost-line rule. `cargo test --frozen`, `cargo clippy --all-targets -D warnings`,
  `cargo fmt --check`, `git diff --check`, `scripts/check-public.sh` all clean.

## Non-negotiables

- Never raise a proof figure by reclassifying code without a stated rule; ghost-line detection is
  syntactic and conservative (mixed lines stay runtime).
- No change to evidence acceptance, receipts, or the security-relevance rules.
- Offline: `--frozen`, existing cargo cache. No new crates unless already in the offline cache;
  log any addition.
- Work only in this repository. scratch-server and survey checkouts are read-only. No push, tag or
  release. One commit per milestone, as Dudley (repo identity is set).
- Keep `dev/PROGRESS.md` (v1.6 section, M1-M5) and `dev/NOTES.md` (timestamped) current. Redirect
  long build/verifier output to files under `logs/v1.6/` and read only tails.

## Out of scope

- Proof lines in dependencies (vstd etc.); the ratio is first-party only.
- Kani harness ratios beyond counting them as standalone proof code (existing rule).
- Any judgement of proof quality.
