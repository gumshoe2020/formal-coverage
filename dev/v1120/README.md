# v1.12.0 local acceptance

Inputs: Bertie origin/main `110802aae37cb2e4adce165a1410bdea4be01644`.
The verifier is copied read-only from `ci/fstar-cache` consumer commit
`c09ea5c46a945fa56228eed48334478e048a7311`:
SHA-256 `7227ed6256734dffeaf58d4f6e66f636078f596c56fc638b57a06aa0d649d0df`.
No consumer files/refs are edited. All generated inputs/logs/Git fixtures live
under this worktree's ignored `target/v1120/`.

Initialize a fresh scratch directory (do not overwrite retained evidence):

```sh
mkdir -p target/v1120/bertie target/v1120/logs
# BERTIE: a bertie-tls clone containing 110802a and the ci/fstar-cache commit c09ea5c
git -C "$BERTIE" archive 110802a | tar -x -C target/v1120/bertie
git -C "$BERTIE" show c09ea5c:scripts/verify-fstar.py > target/v1120/bertie/scripts/verify-fstar.py
git -C target/v1120/bertie init -q
git -C target/v1120/bertie add .
git -C target/v1120/bertie -c user.name=Dudley -c user.email=dudley@local commit -qm 'Scratch main inputs'
export BERTIE_DOCKER_TOOLS=/path/to/bertie-toolchain   # also read by acceptance.py
FSTAR_JOBS=1 \
  python3 target/v1120/bertie/scripts/verify-fstar.py > target/v1120/logs/fresh.log 2>&1
```

Record the full-run start timestamp in `target/v1120/fresh-start.json` as
`{"timestamp": UNIX_SECONDS}` before starting (the initial run used the actual
process start time from `ps`). Build the kit binary with `cargo build --locked`.
Then, sequentially, logging each phase to a file:

```sh
python3 dev/v1120/acceptance.py baseline > target/v1120/logs/a-baseline.log 2>&1
python3 dev/v1120/acceptance.py unchanged > target/v1120/logs/b-unchanged.log 2>&1
python3 dev/v1120/acceptance.py touched > target/v1120/logs/c-touched.log 2>&1
python3 dev/v1120/acceptance.py reverted > target/v1120/logs/c-reverted.log 2>&1
python3 dev/v1120/acceptance.py tamper > target/v1120/logs/tamper.log 2>&1
```

The adapter constructs a fresh full dependency context with actual executable
hash identities, prepares main's receipts with the real kit, and stores them
in an isolated local Git fixture. It fetches that local fixture into the scratch
workspace's data ref; no network or publication occurs. Reports use the real
compiled-files default and original `[ship]` config, same root and log paths.

For incremental phases, only the scratch verifier's module-list support is
changed: the adapter fills that list after unconditional fresh extraction and
libraries, immediately before the verifier's own strict-check loop. The loop,
commands, scheduling (`FSTAR_JOBS=1`), run receipt and evidence rules remain the
same. The combined log includes only selected fresh roots. Empty lists check
zero roots and still write current source-bound extraction evidence.

The touch adds one harmless public constant in `src/der.rs`. hax changes
`Der.fsti` while `Der.fst` stays byte-identical; its old graph has exactly two
roots depending on that interface: Der and Tls13formats. The adapter asserts that exact selection and
restores Rust bytes in `finally`, including on error. The reverted phase
extracts freshly again and must select zero roots. Byte comparisons require
`a-fresh.json == b-unchanged.json == c-reverted.json` without normalization or
field removal. The tamper phase changes a recorded standard-library hash and
requires the actual report CLI to fail naming the root and dependency field.
Timings, JSON/Markdown, contexts, main receipts and fresh slices are retained
under target/v1120; `a-inputs/` preserves the original full-run extraction/logs.

Hosted workflow execution and real data-branch publication are deliberately
not exercised here. An independent reviewer must rerun acceptance.

The original run also builds immutable `cac0f85` sources (`Cargo.toml`,
`Cargo.lock`, `src/`, `registry/`) into `target/v1120/v1110-build`; when that
binary is present, the baseline phase compares default JSON **and Markdown**
byte-for-byte with v1.11.0 before enabling reuse. No Git worktree/branch changes
are needed for that archive/build.
