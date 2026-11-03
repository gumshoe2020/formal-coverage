# PLAN-HAX-FSTAR.md: v1.7, hax/F* evidence (Bertie)

Approved by Andrew 2026-11-02. Authoritative for v1.7.

**Guiding principle (Andrew): increased honesty and transparency.** When a choice is ambiguous,
report more rather than less, label every heuristic and trust step, and never let a headline
imply more than the evidence shows.

## Why
We are contributing to Cryspen's Bertie (PR cryspen/bertie#164), and Bertie is the TLS stack
replacing rustls in scratch-server (stage 19). formal-coverage only credits Verus and Kani
evidence, so the 2026-11-02 baseline (`../coverage-survey/bertie/baseline-whole-crate.md`) reads
**0/7,150 first-party SLoC proven**. That is a tool limit, not a fact about Bertie.

Bertie's proofs run Rust -> hax extraction -> F* (runtime safety, parser round trips), plus
ProVerif and SSProve models. The goal is to credit **only F* proofs we re-check ourselves, against
the current Rust**, and to say exactly what each credited proof establishes.

A worked example of why "what it establishes" matters: `tls13formats.rs::certificate_verify`
carries a hax `ensures` that `parse_certificate_verify` recovers the signature. That holds, yet
`ecdsa_signature` (line 1130) emits non-minimal DER (fixed-width r/s with leading zeros) and
`parse_ecdsa_signature` rejects valid short r/s. The contract proves the encoder and parser agree
with each other, not that either follows DER. Strict peers reject about 1 in 128 handshakes
(scratch-server stage 19 blocker). The tool must make that kind of contract visible.

## Grounded facts (probed 2026-11-02; re-check before relying on them)
- Bertie checkout: `../bertie-spike/upstream`, pinned **ef65f76**. Its working tree has local
  spike edits (`src/tls13api.rs`, `src/tls13record.rs`): **always export with
  `git archive ef65f76`**, never read the working tree.
- Bertie ships no `Cargo.lock`. `hax-lib = 0.4.2`; libcrux crates are the 0.0.2/0.0.3-alpha line.
  Its `flake.nix` takes `cargoLock` as an argument, so versions are unpinned until a caller
  supplies a lockfile.
- What we ship: `../scratch-server/Cargo.lock` (branch `bertie-boundary`) pins Bertie from
  Andrew's fork `git+https://github.com/gumshoe2020/bertie?rev=fbbcc131de30d7e8f1599f66a83f16e694fe1997`
  and pins about 20 libcrux crates, some in two versions (`libcrux-p256`, `libcrux-hacl-rs`,
  `libcrux-macros`). `fbbcc13` is not in the local upstream clone; fetch it inside the container.
- `PROOFS.md`: F* **2025.03.25**; driver is `hax-driver.py` (`extract-fstar`, `typecheck`,
  `--lax`, `--admit`). Rocq 8.18 + SSProve; ProVerif 2.05.
- `proofs/fstar/`: `extraction` (generated), `extraction-lax` and `extraction-panic-free`
  (hand-edited snapshots, tracked as `.patch` files applied by `patches.sh apply`).
- CI (`.github/workflows/hax.yml`) runs only `extract-fstar`; it never typechecks. The
  panic-free snapshot is older than the current Rust and contains explicit `assume`s
  (see `../bertie-spike/REPORT.md` "Proofs" section for the inventory).
- `hax-driver.py` selects interfaces (not implementations) for crypto, utils and cert modules.
- `#[hax_lib::opaque]`, `proverif::replace`, `replace_body` exist in `src/`.

## Non-negotiables
1. **Never weaken validation or tune to raise a number.** A failed or partial F* run is the
   result.
2. **Third-party code runs only in Docker** (hax, F*, cargo for Bertie and its build scripts).
   Same mount rules as PLAN-VERIFY-RUNS.md: read-only source export at the same absolute path,
   one writable `runs/v1.7/bertie/`, optional cache volume; no `$HOME`, `~/.ssh`, `~/.cargo`,
   Docker socket, or `--privileged`. Network only to fetch pinned toolchains and crates.
3. **Pin and record** hax commit, F* version/commit, Z3 version, Rust toolchain, image digests,
   Bertie commit and the generated `Cargo.lock` hash in `runs/v1.7/bertie/manifest.json`.
4. **Credit only fresh extraction of the current Rust.** A definition earns credit only if the F*
   that was typechecked for it is byte-identical to what hax extracts from the receipt's Rust
   source. Definitions changed by the snapshot patches are hand-edited models: **trusted, not
   proven**, listed with file:line.
5. **Never credit weakened runs.** Reject receipts whose F* command includes `--lax`,
   `--admit_smt_queries true`, `--admit_except`, or hax-driver `--lax` / `--admit`. Closed
   grammar, as with Verus/Kani: anything unrecognised credits nothing.
6. Work only in this repo; Bertie, scratch-server and survey checkouts are read-only. One commit
   per milestone; no push, tag, release, or contact with upstream.

## Design
- New status **proven-extracted** (hax/F*), reported as its own column alongside proven
  (Verus) and proven-bounded (Kani). Decided: it is **never** merged into "proven"; the
  extraction step is an extra trusted link, and readers should see it. Every proven-extracted
  headline names hax as a trusted item.
- New repeatable `--fstar-log run.log` with `run.log.evidence.json` receipt binding: Rust source
  file hashes, hax version/commit, extraction command, every extracted/checked `.fst`/`.fsti`
  hash, patch-file hashes, F* version, full F* command line, and per-module outcome.
- **Rust <-> F* mapping** via hax's naming (`Bertie.Tls13formats.parse_ecdsa_signature` <->
  `src/tls13formats.rs::parse_ecdsa_signature`). Unmapped definitions are reported, never guessed.
- **Trusted items, not proven:** interface-only `val`s (fsti without checked fst), `assume val`,
  hand-edited patched definitions, `opaque`, `proverif::replace`, `replace_body`.
  **Escapes (hazards):** `assume`, `admit`, `admitP`, `--admit_*` options inside files.
- **Contract kind per credited definition:**
  - `safety-only`: postcondition absent or `True` (panic/bounds freedom only);
  - `functional`: a real postcondition;
  - `self-consistency` (flag on functional): the postcondition mentions only same-crate
    functions, e.g. parse(serialize(x)) == x. Reported as "consistent with its own parser; not
    checked against an external spec". Decided: ships in v1.7. It is a heuristic and every
    report that uses it says so, with the postcondition text shown.
- ProVerif and SSProve results are listed under **Upstream claims, uncredited** with their
  files; they model the protocol, not this code line by line.

## Milestones
- **M1 Evidence model.** `--fstar-log` receipts, closed command grammar, new status and column,
  JSON/Markdown output, trusted/escape classification. Unit tests for each rejection path.
- **M2 Mapping and contract kinds.** hax name mapping, byte-identity check against fresh
  extraction, patch-altered -> trusted, contract-kind classification.
- **M3 Synthetic fixtures** (`fixtures/hax/`): a tiny hax crate with (a) a clean proof,
  (b) a `--lax` run, (c) an `admit`, (d) a patch-altered function, (e) an interface-only
  `val`, (f) a stale snapshot whose Rust changed after extraction, (g) a round-trip contract
  that should be flagged `self-consistency`. Each must classify as the plan says. These fixtures
  carry acceptance if the real Bertie run (M4) cannot complete.
- **M4 Real Bertie run in Docker.** Two source trees, both measured with the **shipped**
  dependency versions from `../scratch-server/Cargo.lock` (copy it in; never generate a fresh
  lock as the primary measurement):
  - **upstream** `ef65f76` (`git archive`), the numbers Cryspen would recognise;
  - **shipped** fork rev `fbbcc13`, the code scratch-server actually runs.
  For each: `hax-driver.py extract-fstar`; `patches.sh apply`; typecheck `extraction` and
  `extraction-panic-free` with pinned F* (no lax/admit). Budget: about 2 hours of toolchain
  setup, at most 3 nearby hax/F* versions if the pinned one fails to build; log every attempt.
  Feed whatever results exist through the receipts. Report both trees in `compiled-files` mode
  so libcrux is measured, and list every Rust and F* difference between them.
  Optional, clearly labelled secondary: a freshly generated lock for upstream, showing which
  libcrux versions differ from the shipped ones.
- **M5 Survey and regression.** Add a Bertie section to `docs/survey.md` with per-module
  proven-extracted, trusted, escapes and contract kinds, and the `certificate_verify` /
  `ecdsa_signature` case as a worked example. Rerun the v1.6 regression replay: Verus and Kani
  numbers for scratch-server and the survey must be unchanged.

## Acceptance (Dudley verifies independently; Codex's "done" is not acceptance)
- All fixtures classify as specified; every rejection path has a test.
- v1.6 replay output unchanged except for the new empty column.
- Bertie report: every credited definition traces to a receipt, a fresh-extraction byte match,
  and a non-weakened F* command; patched definitions appear as trusted with file:line.
- `certificate_verify` is labelled `self-consistency` (or the deviation is explained).
- No host installs, no out-of-repo writes, no push.

## Decisions (Andrew, 2026-11-02)
1. **proven-extracted** stays a separate column.
2. The `self-consistency` flag ships in v1.7, labelled as a heuristic.
3. Dependencies are measured at the shipped versions (scratch-server's `Cargo.lock`).
