# PLAN-HAX-FSTAR-v1.7.1.md: real Bertie numbers (extract inside the shipped workspace)

Approved by Andrew 2026-11-02. Authoritative for v1.7.1. Extends dev/PLAN-HAX-FSTAR.md (v1.7). Every
v1.7 non-negotiable, design rule and receipt check still applies unchanged. This plan adds a
harness only. It does not change what earns credit.

**Guiding principle (Andrew): increased honesty and transparency.**

## Why
v1.7 produced zero proven-extracted credit for Bertie. Its lock audit stopped extraction because
resolving Bertie on its own drops one edge, `getrandom 0.4.3 -> rand_core 0.10.1`. That edge
exists in the shipped lock only because scratch-server's dev-dependency `rand_core = "0.10"`
turns on a getrandom feature through feature unification. The audit was right to stop. The fix
is to stop resolving Bertie on its own: run extraction **inside the scratch-server workspace**,
so the shipped `Cargo.lock` applies as-is.

## Grounded facts (probed 2026-11-02; re-check before relying on them)
- scratch-server `bertie-boundary` is at **e51db51** (same commit as the v1.7 manifest). Bertie
  is a git dependency, not a workspace member: `gumshoe2020/bertie` rev `fbbcc13`.
- v1.7 toolchain under `runs/v1.7/bertie/`: hax **0.4.2** (`5ccf89d`, tag cargo-hax-v0.4.2,
  matching Bertie's `hax-lib = 0.4.2`), Rust `nightly-2025-11-08`, prebuilt amd64 F*
  2025.03.25 and 2025.10.06 (bundled Z3 4.13.3) run under emulation. Reuse it read-only. Do not
  rebuild it.
- `hax-driver.py extract-fstar` runs
  `cargo hax -C -p bertie -p rand_core --no-default-features --features defensive,std ; into -i
  "-**::non_hax::** -bertie::stream::**" fstar --interfaces "+** +!bertie::tls13crypto::**
  +!bertie::tls13utils::** +!bertie::tls13cert::**"`.
  The proofs use features `defensive,std`. Bertie's defaults, which scratch-server ships, are
  `api,std,defensive`. `bertie::stream` is excluded from extraction.
- The shipped lock has three `rand_core` versions (0.6.4, 0.9.5, 0.10.1), so a bare
  `-p rand_core` is ambiguous inside scratch-server. It must be pinned to the version Bertie's
  own closure uses, and the choice recorded.
- Upstream `ef65f76` and shipped `fbbcc13` differ only in `src/stream/server.rs`,
  `src/tls13api.rs`, `src/tls13record.rs` and `tests/test_tls13api.rs`
  (`runs/v1.7/bertie/differences.json`).
- The committed `extraction-panic-free` snapshot uses the old `Core` namespace. hax 0.4.2's
  proof-libs use `Core_models`. Fresh 0.4.2 extraction should match its own proof-libs, but
  `patches.sh apply` may not apply cleanly to it.

## Harness
1. Build an extraction workspace under `runs/v1.7.1/<tree>/ws` from `git archive e51db51` of
   scratch-server and the Bertie tree (`git archive` of the pinned rev). Add Bertie as a path
   workspace member through `[patch."https://github.com/gumshoe2020/bertie"]`. The only allowed
   manifest edits are: removing Bertie's own `[workspace]` table, the `[patch]` entry and the
   member line. Every edit is recorded as a diff with before/after hashes. Rust sources are
   untouched and verified by hash against the archive.
2. **Lock audit v2 (strict, fail-closed).** Start from a copy of the shipped lock. Then:
   - every package entry in the shipped lock must be unchanged, with one exception: Bertie's
     `source` line, which becomes a path whose file tree must hash-match the pinned rev;
   - additions are allowed only when they are outside Bertie's normal-dependency closure under
     the extraction features (for example Bertie's dev-deps such as `dhat`). Each addition is
     listed;
   - any removal, version change, source change or new edge inside Bertie's closure blocks
     extraction. If the `getrandom -> rand_core` edge still drops, that is a real result. Stop
     and report it; do not work around it.
   - Bertie's closure is computed with the same feature set used for extraction.
3. **Feature disclosure.** Record `cargo tree -e features` for Bertie under the proof features
   (`defensive,std`) and under scratch-server's real build. Report every Bertie item compiled
   only under the shipped features (for example `#[cfg(feature = "api")]`) as **out of proof
   scope**. Do the same for everything under `bertie::stream` and `non_hax`, with SLoC. These
   items are never credited and never hidden.
4. **Upstream tree.** Use the same workspace with Bertie's path pointing at the `ef65f76`
   export. Lock audit v2 is the same; the hash check uses ef65f76. If ef65f76's `Cargo.toml`
   dependency list differs from fbbcc13's, report it, and the audit decides.

## Milestones
- **M1 Audit v2 + harness.** Implement the audit as code with unit tests: exact lock passes;
  version change, source change, removal, closure-internal addition, dropped edge and tree-hash
  mismatch are each rejected; a dev-only addition is accepted and listed. Build both workspaces
  and run `cargo metadata --offline` in Docker. Commit.
- **M2 Fresh extraction.** Run hax-driver's exact `cargo hax` arguments from the workspace root.
  The only change is pinning `rand_core` to `rand_core@<version>`; record the version and why.
  Use hax 0.4.2 for both trees. Hash and receipt every extracted file. Commit.
- **M3 Typecheck.** Use F* 2025.10.06, the version Bertie's CI pins alongside hax 0.4.2. If it
  fails, try 2025.03.25 and log both. Typecheck fresh `extraction`, then run `patches.sh apply`
  and typecheck `extraction-panic-free`. If any patch hunk fails, record it per hunk; that tier
  gets no fresh-Rust credit. All results go through the v1.7 `--fstar-log` receipts with no
  grammar changes and no lax or admit. Commit.
- **M4 Report and regression.** Report both trees in `compiled-files` mode: proven-extracted,
  trusted, escapes, contract kinds, out-of-proof-scope SLoC by reason, and libcrux. Update the
  Bertie section of `docs/survey.md`, including the `certificate_verify` / `ecdsa_signature`
  worked example. The v1.6 replay and all v1.7 fixtures must be byte-identical. Commit.

Budget: about 90 minutes of wall time on harness and toolchain issues before reporting what
blocks. No new toolchain builds. If hax 0.4.2 cannot extract inside the workspace, stop and
report; do not try other hax versions (see Decision 2).

## Non-negotiables (additions to v1.7's)
- Do not relax the v1.7 audit, receipts or grammar. Audit v2 is a separate, stricter-scoped
  check for the in-workspace harness. It is not an exception added to the old one.
- scratch-server, bertie-spike and coverage-survey are read-only; export with `git archive`.
  Third-party code runs only in Docker under the v1.7 mount rules. Network is only for
  lockfile crates. Outputs go to `runs/v1.7.1/` and `logs/v1.7.1/`.
- One commit per milestone. No push, tag, release or upstream contact.

## Acceptance (Dudley verifies independently)
- Audit v2 tests cover every rejection path, and the real lock diff is listed in full.
- Every credited definition traces to a receipt, a byte match against fresh extraction and a
  non-weakened F* command. hax is named as trusted on every headline.
- Out-of-proof-scope items (feature-gated, `stream`, `non_hax`) are listed with SLoC for both
  trees.
- The v1.6 replay and v1.7 fixtures are unchanged.
- No host installs, no out-of-repo writes, no push.

## Decisions (Andrew, 2026-11-02)
1. **Allowed lock difference:** only Bertie's own `source` line (git -> path, with content
   proven identical by hash) plus listed dev-only additions. **Decided: yes.**
2. **If patches don't apply to fresh hax 0.4.2 output:** report the panic-free tier as
   uncredited and stop, rather than hunting for the older hax the snapshot was made with.
   **Decided: stop.** Bertie pins hax-lib 0.4.2, so an older engine would prove something
   about a different toolchain than the code compiles against. That mismatch is itself a
   finding worth reporting upstream.
