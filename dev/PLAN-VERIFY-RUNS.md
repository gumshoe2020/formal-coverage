# PLAN-VERIFY-RUNS.md: v1.4, real verifier runs for Anvil and Firecracker

Approved by Andrew 2026-10-28 ("yes, please"). Authoritative for this milestone.

## Why
The v1.1-c survey (`docs/survey.md`) has one real proof number. Vest is 28.05% proven from a
fresh Verus run. Anvil and Firecracker are inventory only:
- **Anvil** (`e83ea0f`, 2026-10-12): the Verus run exited 1 after 168 s because offline
  dependencies (`k8s-openapi` from git) were unavailable. Anvil tracks Verus `main` (unpinned).
- **Firecracker** (`84df490`): Kani was never run.

Every survey report also says "INCOMPLETE: N dependency sizes unknown", because closure fell
back to the lockfile offline (`--frozen` could not fetch git/crates.io sources).

The goal is real, reproducible verifier evidence for both projects, fed through the existing
`--verus-log` / `--kani-log` receipts, and honest numbers in the survey. Whatever the verifiers
produce is the result, including failures and timeouts. **Do not tune the tool to raise a number.**

## Grounded facts (probed 2026-10-28; re-check before relying on them)
- Docker is available: `linux/aarch64`, 12 CPUs, about 8 GB RAM in the Docker VM. There is no
  x86 host.
- Firecracker pins **Kani 0.67.0** in `tools/devctr/Dockerfile:248`
  (`cargo install --locked kani-verifier --version 0.67.0 && cargo kani setup`). Its CI command is
  in `tests/integration_tests/test_kani.py:38`:
  `cargo kani -Z unstable-options -Z stubbing -Z function-contracts -Z restrict-vtable -j --output-format terse --harness-timeout 40m --workspace`.
  Harness files include `src/vmm/src/{dumbo/pdu/ethernet.rs, vstate/memory_slice.rs,
  rate_limiter/mod.rs, devices/virtio/queue.rs, devices/virtio/iovec.rs, arch/aarch64/mod.rs,
  arch/x86_64/mod.rs}`. On an aarch64 host, x86_64-only harnesses won't compile in; report that
  as a scope limit, not a failure.
- Anvil builds Verus from source with `tools/setup-verus.sh` (clones verus-lang/verus `main`,
  `vargo build --release`, writes `rust-toolchain.toml`). Verification uses
  `cargo verus verify --lib -- --rlimit 100 [--verify-only-module <mod>]` (see `build.md`).
  CI verifies per controller in separate jobs (`.github/workflows/ci.yml`).
- `scripts/survey.py` exports survey clones with `git archive` into `/tmp` and runs only pinned
  Verus. Receipts are source-bound: they hash the exact files that were verified.

## Non-negotiables
1. **Third-party code runs only inside Docker.** That covers dependency build scripts, proc
   macros, the Verus build, Kani, and anything from the projects' own tooling. Nothing from
   Anvil, Firecracker, Verus or Kani runs on the Mac host. No host installs: no `cargo install`,
   `brew`, `pip`, or `rustup toolchain install` on the host.
2. **Container mounts:**
   - the `git archive` source export, **read-only**;
   - one writable output directory under this repo, `runs/v1.4/<project>/`, which is gitignored
     except for small summaries you choose to commit;
   - optionally a named Docker volume for caches.
   Never mount `$HOME`, `~/.ssh`, `~/.cargo`, the Docker socket, or any other workspace project.
   No `--privileged`. Network access inside the container is allowed, but only for fetching
   pinned toolchains and dependencies.
3. **Source must hash identically.** Mount the export at the *same absolute path* inside the
   container as on the host, so the paths in the verifier log match what the host-side
   `formal-coverage` receipt validator sees. If paths still differ, fix it in the run setup, not
   by relaxing receipt validation.
4. **Pin everything and record it.** Record the Anvil and Firecracker commits, the Verus commit
   SHA, the Z3 version, the Kani version, the Rust toolchains, and the image digests, in
   `runs/v1.4/<project>/manifest.json`.
   - **Verus for Anvil:** use the last verus-lang/verus `main` commit on or before 2026-10-12
     (Anvil's commit date).
   - **If that commit fails to build or verify for toolchain reasons:** you may try at most 3
     nearby `main` commits. Log each attempt and its outcome in NOTES.md.
5. **Resource limits are results, not things to work around on the host.** Don't change Docker
   Desktop settings.
   - **Out of memory or too slow:** narrow the scope instead. Anvil can verify per
     `--verify-only-module`. Firecracker can run per harness, or the CI command without `-j`.
   - **Report what was skipped and why.**
   - **Time limit:** each project has about 6 hours of verifier wall time in total.
6. **Honest accounting.**
   - **Kani success** counts only as `proven-bounded`, never unbounded proven.
   - **Verus "verified"** counts only for the modules that run actually verified.
   - **Failed, timed-out or skipped modules or harnesses** are listed by name with their outcome.
   - **Rejected receipts:** keep the rejection reasons and report them.
7. **Tool changes are allowed only for genuine bugs.** The bar: a receipt from a genuinely
   successful run is rejected because of a parsing or path bug in `formal-coverage`.
   - Each fix is its own commit with a regression test.
   - Log it in NOTES.md.
   - Never weaken receipt validation, hash checks, escape handling or gate thresholds.
8. **Fences:**
   - Commit only in this repository. The survey clones in `../coverage-survey` stay read-only.
   - No pushes, no new branches, no external messages.
   - Use project-local `CARGO_HOME`/`TMPDIR` for host builds of `formal-coverage` itself.
   - Docker images you build are tagged with the `fc-verify-` prefix. Remove only those.
   - Redirect all build/verifier output to files and read only `tail -n 30` or grep for errors.
     Never stream full Verus, Kani or cargo logs into your context.
9. **Observability:** keep PROGRESS.md (M1–M4 checklist) and NOTES.md (timestamped decisions,
   attempts and blockers) current. Commit after each milestone.

## Milestones

### M1: Firecracker under Kani 0.67.0
- Build a `fc-verify-kani` image with Rust and Kani 0.67.0 (`cargo kani setup`).
- Run the CI command (fact above) on the export.
  - **If it can't finish in budget:** run harnesses individually with `--harness`, using the
    same `-Z` flags and `--harness-timeout`.
- **Acceptance:**
  - `runs/v1.4/firecracker/kani.log` exists, along with `harnesses.json`: per-harness
    pass/fail/timeout/not-compiled-on-aarch64, with counts.
  - `formal-coverage report <export> --kani-log <log>` accepts the receipt. If it rejects it,
    the reasons are documented.
  - Every passing harness's target units show `proven-bounded`.
  - The unbounded security-proven percentage stays 0.00%.
  - `manifest.json` is complete.

### M2: Anvil under pinned Verus
- Build a `fc-verify-verus` image. Build Verus at the pinned SHA, following `setup-verus.sh`
  (`get-z3.sh`, `vargo build --release`).
- Fetch Anvil's dependencies inside the container, then run `cargo verus verify --lib --
  --rlimit 100`.
  - **On out-of-memory or timeout:** fall back to one run per module. Cover the framework
    modules plus each controller in `src/controllers/` and `composition`, and produce one log
    per run.
- **Acceptance:**
  - One Verus log per run under `runs/v1.4/anvil/`, plus `modules.json` giving each module's
    verified/errors/timeout/OOM outcome with its verified and error counts.
  - All logs are passed as repeated `--verus-log` arguments to `formal-coverage report`. Each
    receipt is accepted, or its rejection reasons are documented.
  - Proven percentages appear only next to the trust/escape line, as the tool already enforces.

### M3: real dependency sizes
- Inside a container, fetch each project's locked dependency closure without building anything
  (for example `cargo fetch` / `cargo vendor` with `--locked`) into
  `runs/v1.4/<project>/cargo-home/`. Do this for Vest, Anvil and Firecracker.
- Run `formal-coverage` on the host, offline, with that `CARGO_HOME`. That step only reads
  manifests and sources and never executes them.
- **Acceptance:**
  - The "INCOMPLETE: N dependency sizes unknown" notes disappear for all three, or every
    remaining unknown is listed with the reason it couldn't be fetched.
  - Closure is no longer "lockfile (fallback)" where `cargo metadata` can now resolve.

### M4: survey update and checks
- Update `docs/survey.md` and `docs/survey/results.json` with the M1–M3 results.
  - Add a per-project "verifier outcome" row that names what passed, failed, timed out and was
    skipped.
  - Keep the Vest numbers unless M3 changes its dependency figures. If so, show before → after.
  - State the aarch64 scope limit for Firecracker.
- Extend `scripts/check_survey.py` so it checks the new rows against the run artifacts.
- **Acceptance:** run all of these with the project-local env from previous milestones; each
  must exit 0:
  - `python3 scripts/check_survey.py`
  - `cargo test --frozen`
  - `cargo clippy --frozen -- -D warnings`
  - `cargo fmt --check`
  - `git diff --check`

## Out of scope
- No pushes, publishing, or edits to the survey clones.
- No new projects beyond these three.
- No x86 emulation runs. Note them as possible future work only.
- No changes to tier semantics, thresholds or headline formatting.
- No Firecracker integration tests, VM boots, or anything needing `/dev/kvm`.
