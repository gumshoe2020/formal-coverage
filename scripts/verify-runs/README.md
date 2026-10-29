# v1.4 run artifacts

[PLAN-VERIFY-RUNS.md](../../dev/PLAN-VERIFY-RUNS.md) is the historical v1.4 execution spec. These are first-party orchestration scripts;
all target/toolchain execution is inside Docker. Do not invoke `survey.py`'s old
host Verus execution mode for these runs.

The three `runs/v1.4/PROJECT/manifest.json` files retain exact archive commits,
archive hashes, container commands, scripts, immutable image IDs, tool versions,
lock hashes, timings, exit codes and log hashes. Raw logs, source-bound receipts,
full reports, Cargo caches and generated lockfiles are ignored. The checker needs
these local artifacts and the read-only survey clones. Small manifests, named
outcomes and summaries are committed; the large runtime/proc-macro source
inventories remain in the ignored reports.

## Execution

Build the first-party release binary using the repository-local Cargo environment
as specified by the plan. Host `formal-coverage report` uses Cargo metadata offline
and never builds target code. Its report target is explicitly ARM64 Linux.

Build images with the two Dockerfiles here, tagging only `fc-verify-*`, with output
redirected to the corresponding project's `image-build.log`. Each Dockerfile pins
the base by digest. Retained final image IDs pin the actual complete filesystem;
`packages.txt` records the system package closure. Apt mirrors in the recipe are
not snapshots: use the retained immutable image for an exact replay. Verus's
actual generated build locks are also retained and hashed, not substituted with
current dependency resolution.

Export using `python3 scripts/verify_runs.py export PROJECT`. This checks every
archived byte before reusing `<temporary-export>/PROJECT`. Docker gets that source
at the identical absolute path, read-only, plus exactly one writable mount:
`runs/v1.4/PROJECT` at `/out`. The `/tmp` alias on macOS denotes the same host
archive; receipt file hashes are relative and checked against the canonical root.
No other bind mounts or privileged flags are used. Temporary files and compiler
outputs go under `/out`. Each timeout explicitly stops its container.

Run a recorded step with the `container` action, its recorded immutable `--image`,
`--label`, `--timeout` and `--script`. Never reconstruct a receipt command from a
more convenient command: receipts retain the exact verifier invocation.

Firecracker's initial step is the documented parallel workspace CI command. Its
source-writing mock-kernel build script fails against the read-only mount.
`python3 scripts/verify-runs/kani.py` records/resumes the complete source-derived
harness inventory serially for VMM, retaining the same `-Z` flags and 40-minute
harness limit. It has a six-hour project budget and individual process guards.
A startup filter error and corrected pilot are retained separately from the
unique-harness counts. OOM diagnostics are retained without changing unwind
bounds or stubs. x86-only harnesses are explicitly not compiled.

Anvil lacks an archived root lock. Its manifest records unsuccessful resolution
attempts and the successful setup: writable copy of the archive under `/out`,
container-local bare remotes for the dated Verus/verus-tla commits, original
manifest URLs, then `cargo update -p vstd --precise` for the required Verus SHA.
For fresh resolution, create `/out/resolution` from the archive first. All Verus
git entries are checked before copying only the generated Cargo.lock into the
host export. `cargo fetch --locked` must then succeed against the unchanged
archived manifest. The generated lock's bytes and git pins are recorded.
`RUSTUP_TOOLCHAIN=1.98.1` selects Verus's required compiler without editing Anvil's
archived toolchain file. The full Cargo Verus run failed with SIGKILL during an
overlap with Kani; `python3 scripts/verify-runs/verus.py` performs the required
module fallback without overlapping Kani. Dependency proof summaries are never
reported as Anvil results.

For Vest and Firecracker, fetch their archived locks using `cargo fetch --locked`
inside the container. Fetch only; no target code is built by that step. Cache
mounts are project-local, not host Cargo/Rustup homes.

After execution, `python3 scripts/verify-runs/summarize.py` runs the three offline
host reports with every retained receipt, preserving rejection reasons and zero
credit from rejected runs. Vest reuses its original successful same-source
receipt. `scripts/check_survey.py` cross-checks the committed survey against the
retained source, logs, receipts, named outcomes, mount fences and dependencies.

A successful verifier result can be rejected by the existing deliberately narrow
receipt grammar. That is a recorded result, not permission to delete flags,
invent mappings/bounds, or expand the grammar. M2b later extended the closed Kani receipt grammar in `src/kani.rs` and
`src/evidence.rs`, explicitly authorized and regression-tested; proof gates were preserved.

M2 resume correction: the pinned CLI requires `focus` for selectors, and
`--verify-module` includes descendants (`--verify-only-module` excludes them).
Focus uses `/out/target/verus-partial`, so every fresh run first executes
`cargo clean -p verifiable-controllers --target-dir /out/target/verus-partial`.
Acceptance requires fresh root compilation, matching scope notes and a project
summary. Cache-only exits remain setup failures in previous_attempts. Four roots
not offered in the verifier's 221-module inventory are skipped with reasons.
Andrew's later M2b decision separately authorizes only the enumerated Kani command
grammar expansion; earlier no-expansion notes remain historical.

M2b artifacts are separate: `kani_credit.py` replays byte-identical M1 logs with
caller harness IDs matching the logged command. `credits.json` records all 40
harnesses: 34/37 raw passes credited with scopes, two OOMs, one architecture skip,
and three rejected raw passes (expected panic/two multi-harness logs). No runtime
mappings are inferred. M1 outcomes and inventory are retained historically; public metadata redacts machine paths.
`dependencies.py` records fresh locked fetches in separate m3-manifest.json files
and offline before/after inventories. `survey_snapshot.py` chooses the M2b
summary for current Firecracker rows and preserves the old historical survey.

## v1.4.1 release rerun

`rerun_iovec.py` repeats the original iovec harness command using the recorded
immutable Kani image with `--network none`. Do not overlap verifier runs. It writes
`release-rerun-manifest.json`, `release-iovec-rerun.log` and its receipt;
`credit_release_rerun.py` replays it with the retained receipts and writes separate
release summaries. The final idle-VM run on 2026-10-29 passed in 96.58 s. The earlier
200.84-second shared-VM pass is retained separately. The unchanged grammar credits 35/38
raw passes as bounded proof-code harnesses; runtime target credit stays 0.
Original M1/M2b outcomes remain historical. Offline reproduction uses the release
evidence bundle described in [evidence.md](../../docs/evidence.md).
