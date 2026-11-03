# v1.7 Docker run

Primary pins and attempts are in `runs/v1.7/bertie/manifest.json`.
No upstream working tree is read. Export ef65f76 with `git archive` into
`.tmp/v1.7-sources/upstream`; fetch the full shipped fork revision **inside
Docker**, archive it and extract into `.tmp/v1.7-sources/shipped`. Copy the
shipped scratch-server lock into `runs/v1.7/bertie/shipped.Cargo.lock`.

`setup.sh` and `fstar-setup.sh` run in the pinned Rust image with one writable
`runs/v1.7/bertie` output mount. They install only into Docker/the output.
Keep HOME, CARGO_HOME, RUSTUP_HOME and opam state inside that output. Record
opam's resolved switch export as setup provenance. Setup must finish before
starting the next compiler build. No more than three toolchain candidates.

For proof runs mount `.tmp/v1.7-sources` read-only at its **host absolute path**
and `runs/v1.7/bertie` writable at its **host absolute path**. Create an in-container
`/out` symlink to the output path for installed tools that recorded that prefix.
Do not mount the repository, host home, credentials, Cargo cache or Docker socket.
Use at most six CPUs and five GiB; no privileged mode. Network is only for
pinned source/toolchains and lockfile crates. Redirect the entire invocation
into logs/v1.7/, inspecting only grep or `tail -n 30`.

`run.sh OUTPUT SOURCE_EXPORTS` copies immutable exports to the writable output
for generated proof files, preserving the original Rust. It starts with the
shipped lock, tries locked metadata, and extends that **existing** lock only for
workspace/dev requirements. `audit-lock.py` blocks extraction if any package
already named in the shipped lock changes version/source. It records additions
and all libcrux versions. This is an adapted shipped lock, never an independent
fresh resolution. Both original and adapted lock hashes must be retained.

The driver extraction, patch application, ordinary typecheck and panic-free
Makefile are each logged with exits. A failed extraction creates no fresh
artifact directory content. The checked snapshot can still be inventoried as
an upstream model; it cannot become fresh-Rust proof credit. Use strict receipts
for all results, including rejected/failed ones. Typecheck Makefile commands
outside the receipt grammar receive zero credit; do not add flags just to
accept them. A successful real run needs separately captured supported F*
commands with every F* input hashed and clean caches.

## F* has no Linux arm64 prebuilt; amd64 emulation works

Native (arm64) OCaml builds of F* 2025.03.25 failed (Sedlex/ppxlib vs. OCaml
5.4.1, then an OCaml-4.14.2-switch retry that also did not produce a working
`fstar.exe`). F*'s GitHub releases ship a Linux **x86_64** prebuilt for both
2025.03.25 and 2025.10.06; Docker Desktop's `--platform linux/amd64` QEMU
emulation runs that binary (plus its bundled z3) correctly on this arm64
host. `amd64-check.sh` uses the prebuilt binary this way; no native F* build
is required.

## `make clean` deletes exactly the files `make verify` needs

Bertie's `proofs/fstar/*/Makefile` `clean` target is `rm -rf .cache/*; rm
*.fst`. `hax-driver.py typecheck --clean` chains that straight into `make
verify`, which discovers zero `*.fst` ROOTS and reports success having
checked nothing. PROOFS.md's documented command is plain `./hax-driver.py
typecheck`, with no `--clean`. `amd64-check.sh` now clears only `.cache/`
directly and restores the archived/extracted `.fst` snapshot before
checking.

## F* version: CI's pin typechecks further than PROOFS.md's

PROOFS.md says F* 2025.03.25; Bertie's own `hax.yml` CI pins hax-lib-v0.4.2
with F* 2025.10.06 instead. Against the archived (not freshly hax-extracted
-- see below) `.fst`/`.fsti` snapshot, 2025.03.25 fails immediately
(`Namespace Core cannot be found` in the hax proof-libs). 2025.10.06
resolves that namespace and verifies 54 (extraction) / 47 (panic-free)
modules -- the hax proof-libs and several early Bertie modules -- before
failing on the same "Namespace Core cannot be found" at
`Bertie.Tls13utils.fsti`. Bertie's CI only runs `extract-fstar`, never
`typecheck`, so the committed snapshot can silently drift stale relative to
the hax-lib version CI pins; this is most likely that drift, not a defect
in Bertie's proofs. Both attempts are logged; the later, farther-reaching
one is reported as the primary diagnostic result. Neither credits any
proof: see the next section.

## Fresh extraction is blocked by lock drift, for an unrelated optional feature

`audit-lock.py` correctly refuses extraction for both trees: extending the
shipped lock to the full workspace changes `rand_core 0.10.1`'s presence in
bertie's own dependency closure. Traced cause: `bertie -> hax-lib 0.4.2 ->
hax-lib-macros-types -> uuid 1.29.0 -> getrandom 0.4.3`; in the shipped
(scratch-server) lock, `getrandom 0.4.3` also depends on `rand_core 0.10.1`
(an optional `rand_core`-compatibility feature), enabled by feature
unification from some other, unrelated scratch-server workspace member.
Resolving bertie standalone never turns that feature on, so the edge
disappears. This has nothing to do with bertie's own code paths, but the
audit is deliberately strict about any closure change, so it blocks
extraction for both trees, as designed. Not relaxed: loosening the audit to
special-case this would be tuning validation to get a number, which the
plan forbids. Because extraction is blocked, the strict receipt grammar
correctly rejects both trees' `--fstar-log` evidence with `unsupported or
failed extraction`, regardless of whether the archived snapshot above
happens to typecheck further with one F* version than another.
