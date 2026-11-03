# Real-world survey

<!-- v1.6 measurements -->
## v1.6 proof cost replay

2026-10-31. Pinned source archives, configs, aarch64 Linux target triples and all
retained verifier receipts were replayed offline, using **whole-crate** sizing
as in the v1.5 byte-regression cases. These are first-party ratios, independent
of dependency sizing; the v1.5 Linux compiled-files/linked-lines measurements
below are historical, not new container runs. No Docker or verifier execution.

Default range **1.0:10.0**, source `default`. The table's counts and status are
for the **first-party security-relevant headline**; all-first-party ratios are
shown separately. Every bounded runtime column is zero: Firecracker's accepted
Kani harness evidence establishes no runtime target mappings.

| Project | Ghost mode | Standalone proof | Inline proof | Total proof | Proven runtime | Bounded runtime | Ratio | Status | All-first-party ratio |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | ---: |
| Vest | runtime | 23,838 | 0 | 23,838 | 2,961 | 0 | 8.05 | within | 34.99 |
| Vest | separate | 23,838 | 2,405 | 26,243 | 1,850 | 0 | 14.19 | above | 59.74 |
| Anvil | runtime | 53,101 | 0 | 53,101 | 0 | 0 | n/a | n/a | n/a |
| Anvil | separate | 53,101 | 1,557 | 54,658 | 0 | 0 | n/a | n/a | n/a |
| Firecracker | runtime | 906 | 0 | 906 | 0 | 0 | n/a | n/a | n/a |
| Firecracker | separate | 906 | 0 | 906 | 0 | 0 | n/a | n/a | n/a |

Vest's separate-mode security ratio is above the range. Anvil and Firecracker
are `n/a` because neither has accepted unbounded runtime proof credit. Those
`n/a` results do not pass judgement on proof quality or erase rejected runs.
Receipt decisions, credited unit identities, statuses, trust footprints and
escapes remain identical between modes: Vest 1 accepted / 0 rejected, Anvil
0 / 53, Firecracker 35 / 45. The audit also checks every unit's line movement,
proof sums and exact ratio arithmetic. No range or detection rule was tuned to
produce these values.

[v1.6 results and provenance](scratch-server/v1.6-results.json) retain both-mode
headlines and crate rows; complete per-module JSON/Markdown reports and logs are
under `logs/v1.6/M4/{runtime,separate}/`. Reproduction uses the same five-case
commands in [scratch-server v1.6](scratch-server.md#v16-proof-cost-and-ghost-lines).


<!-- /v1.6 measurements -->

Real Docker verifier attempts on the unchanged archived sources, measured
2026-10-28–29. Exact pins, commands, timings, exits and hashes are in the
[results snapshot](survey/results.json) and committed
[run manifests](../runs/v1.4). Full logs, source-bound receipts, dependency
caches and JSON/Markdown reports remain locally under `runs/v1.4/PROJECT/`.
These are first-party source inventories and conservative dependency package
source sizes, not linked binary sizes. Target: **aarch64-unknown-linux-gnu**.

Andrew authorized M2b's closed Kani grammar extension. Hashes, result validation,
hazard handling and unbounded gates remain enforced. **35 of 38 raw Kani passes
receive bounded harness credit**, assuming the hashed source's stubs, verified
contracts and type-compatible vtable targets. Three raw passes remain uncredited:
one expected-panic test and two multi-harness logs. `--exact`, needed to separate
those two filters, is outside the authorized grammar. The 2026-10-29 release rerun passed with identical flags; no extra flag
exception was used. [Per-harness credit and scope](../runs/v1.4/firecracker/release-credits.json)
retains every assumption and rejection reason. **Runtime target credit is zero**;
no runtime coverage mapping is inferred from a harness self-check.

Cargo Verus/module commands remain outside the supported Verus receipt grammar.
Anvil's real partial successes are recorded, with rejection reasons and zero
unbounded receipt credit. Original M1 log bytes and source hashes are preserved; public metadata redacts
machine paths. Normalized M2b caller metadata and reports live separately in `m2b/`.

Vest keeps its retained source-bound successful Verus evidence. Its security
runtime result is **2,961/10,555 = 28.05% proven, relying on 39 trusted items (520 SLoC) and 5110 escapes**. All-runtime `vest_lib` is **2,961/11,916 =
24.85% proven, relying on 39 trusted items (520 SLoC) and 5110 escapes**.
The cfg-active Vest receipt scope separately uses 31 trusted items (234 SLoC)
and 5 first-party escapes. Anvil receives 0 of 53 receipt credits: **0.00%
proven, relying on 401 trusted items (2732 SLoC) and 21636 escapes**.
Firecracker's unbounded security result is **0.00% proven, relying on
0 trusted items (0 SLoC) and 27142 escapes**. Accepted Kani evidence yields only bounded harness credit, never unbounded `proven`.

| Project (commit) | Runtime / proof-code SLoC | Coverage evidence |
| --- | ---: | --- |
| Vest (`ce6705f`) | 88,267 / 103,612 | Retained Vest evidence; same first-party proof numbers |
| Anvil (`e83ea0f`) | 13,893 / 53,101 | All actual run receipts and rejection reasons retained |
| Firecracker (`84df490`) | 77,638 / 906 | 35/38 bounded harnesses under explicit stubs/contracts/vtable assumptions; runtime targets 0 |

| Audit row | Actual verifier outcome |
| --- | --- |
| Vest verifier outcome | Retained vest_lib Verus: 2,935 verified, 0 errors; fresh execution skipped; receipts accepted=1, rejected=0 |
| Anvil verifier outcome | Modules: verified=11, errors=0, timeout=0, OOM=0, skipped=4; named [scopes](../runs/v1.4/anvil/modules.json); receipts accepted=0, rejected=53 |
| Firecracker verifier outcome | Harnesses: pass=38, fail=1, timeout=0, not-compiled-on-aarch64=1, skipped=0; named [harnesses](../runs/v1.4/firecracker/release-credits.json); bounded harness credit=35/38, assuming source stubs/contracts and type-compatible vtable targets; runtime targets=0; receipts accepted=35, rejected=45 |

Named outcomes, with per-run verified/error counts where emitted:
[Anvil modules](../runs/v1.4/anvil/modules.json),
[Firecracker harnesses](../runs/v1.4/firecracker/release-credits.json).
Each project's summary retains every receipt's accepted flag and reasons.
Missing project summaries are null, never inferred from successful dependency
verification or from a killed process.

## Firecracker run

Kani **0.67.0**, Rust driver **1.97.0**, Kani nightly **2025-11-21**,
native ARM64 Linux. The Bookworm setup attempt failed because the release
binary requires glibc 2.39; the pinned Trixie image resolved that setup issue.
The documented parallel workspace command failed before harness verification:
`cpu-template-helper/build.rs` writes a mock kernel into its source directory,
and the required read-only archive mount correctly prevented it. No source or
build-script patch bypassed this fence.

VMM harnesses then ran serially with the same `-Z` options and 40-minute harness
timeout. One pilot used an incorrect module filter, matched no harness, and was
corrected from the archived source; both logs are retained. Unique-harness
counts exclude duplicate pilots. Unsupported foreign functions/concurrency
warnings remain in the logs; proof claims are only under Kani's actual model.
The **x86_64 memory-region harness is not compiled on aarch64**; no x86 emulation
or KVM run was attempted. x86 verification remains possible future work.

- `vmm::devices::virtio::queue::verification::verify_checked_new`: **fail** — CBMC out of memory.

The initial iovec-write OOM on 2026-10-28 overlapped Anvil's full attempt.
On 2026-10-29, the identical source and flags passed in **96.58 seconds** in the
pinned Kani image with `--network none`. This establishes that the earlier OOM
was environmental resource contention, rather than failure of the safety property.
The final rerun ran on the idle Docker VM, with no competing build or verifier
containers observed. The earlier 200.84-second pass overlapped later Nix builds
and is retained separately as historical evidence.
No source, bounds, stubs or host resources changed. The unchanged receipt grammar
accepts this harness: **35/38 raw passes credited**, runtime targets still 0.
The unique outcomes are **38 pass / 1 OOM / 1 architecture skip**. Original M1
and M2b artifacts are historical; release artifacts retain the rerun separately.

## Anvil run

Verus main **781a460f35066e4bbc914b7fe3feb219ca9c850f**, the last main commit
on/before 2026-10-12; **Z3 4.16.0**, **Rust 1.98.1**. Verus built successfully
from source following `get-z3.sh` and `vargo build --release`; no alternate
Verus commit was necessary. The build's 2,060/0 vstd result and the dependency's
139/0 temporal-logic result are **not Anvil proof counts**.

Anvil has no archived top-level lockfile. A generated lock pins Verus and
verus-tla (`e7eda3f24b60fbbf4396e623a06da9ec32512c0e`) through unchanged manifest
URLs. The generated lock is recorded and locked fetching was validated against
the unchanged archive. The manifest records the failed first resolution,
source-identity conflict, cached-main correction and final exact pins. Every
archived file, including Cargo.toml and rust-toolchain.toml, remains unchanged;
the required compiler is selected by container environment instead.

The full `cargo verus verify --lib -- --rlimit 100` attempt exited 101 after
its verifier was SIGKILLed while Kani was active. This is a resource-constrained
result with a recorded overlap, not evidence of Verus version incompatibility.
The fresh module fallback ran without Kani overlap: **11 subtrees, raw verifier count 935 verified,
0 errors**. formal-coverage credits 0 of 53 receipts because `cargo verus focus`
is outside the Verus receipt grammar by design; the measured result is **0.00%
proven, relying on 401 trusted items (2732 SLoC) and 21636 escapes**. The runs cover every selectable framework/controller subtree and composition.
Four roots (`crds`, `external_shim_layer`, `shim_layer`, `unit_tests`) are absent
from the pinned verifier's selectable-module inventory and explicitly skipped.

The pinned CLI requires `cargo verus focus` for partial verification and
`--verify-module` to include descendants. Before every final run, the root crate
was cleaned in the actual `/out/target/verus-partial` directory. Fresh root
compilation, matching scope notes and a real project summary were required.
Earlier rejected selectors and cache-only exits remain setup failures in the
retained previous attempts. Per-module counts are exact project summaries;
dependency summaries and cache output are never borrowed as Anvil results.

## Bertie run (v1.7.1)

Two Bertie trees were measured: **upstream** `ef65f76b4f86` and the **shipped** fork
`fbbcc131de30` (the code scratch-server runs). Exactly 4 Rust files differ
(`src/stream/server.rs`, `src/tls13api.rs`, `src/tls13record.rs`, `tests/test_tls13api.rs`;
an encrypted `close_notify` alert). hax **0.4.2** (`5ccf89dd`) extracted both trees fresh,
inside a scratch-server workspace with Bertie as a path member, in Docker with no network;
F* **2025.10.06** (prebuilt x86_64 under QEMU, bundled Z3 4.13.3) typechecked the result.
Everything below is "proven-extracted" in the v1.7 sense: F* accepted hax's output, and **hax
itself is trusted**. Receipts: `runs/v1.7.1/{upstream,shipped}/`. This replaces the v1.7
run, which was blocked (see "v1.7 history" at the end of this section).

**Lock audit v2** (a new, separate check; the v1.7 audit is unchanged): the lock cargo wrote
for the workspace is compared with the shipped scratch-server lock by resolved dependency
target. It **passes for both trees** and the two resolved locks are byte-identical. No
package was removed or changed version, source or checksum; no edge was dropped (the
`getrandom 0.4.3 -> rand_core 0.10.1` edge that blocked v1.7 is retained). Added: 45
packages and 8 Bertie dev-only edges, all outside Bertie's 75-package normal+build closure and
all listed in `lock-audit-v2.json`; the full textual diff is `lock.diff` (648 lines: those
additions, bare references respelled with a version because a second version now exists, and
Bertie's `source` line changing from git to path).

**Patches do not apply.** `patches.sh apply` fails 30 of 31 hunks in both trees (per hunk:
`patches/hunks.json`), and fails identically on the committed v1.7 snapshot
(`patch-control/`). The `extraction-lax` and `extraction-panic-free` tiers are therefore
not typechecked and earn no fresh-Rust credit. Only the fresh `extraction` tier is credited.

Typecheck: all 8 `.fst` modules of the fresh `extraction` verify in both trees (73-98 s
each, one grammar-compliant `fstar.exe --include ... FILE` command per module, no lax or
admit option); 2025.03.25 was not needed. The hax/core/rust_primitives/libcrux-models F*
libraries are hashed inside the receipt, so their `assume`s and options show up as
library-origin counts below, not as Bertie's.

Report root: the `bertie` package alone with the audited lock pruned offline by cargo (101
packages pruned, none added, no identity or version change; one optional edge,
`getrandom 0.4.3 -> rand_core 0.10.1`, is absent from the pruned lock; `root-lock-check.json`,
`root-manifest.diff`). Bertie's other workspace members are not in the audited closure; one of
them pulls `libcrux-platform` from a git source.

`compiled-files` mode, first-party security SLoC (Bertie only):

| | upstream | shipped |
| --- | ---: | ---: |
| First-party security SLoC | 5,507 | 5,538 |
| **proven-extracted** | **1,068 (19.39%)** | **1,081 (19.52%)** |
| trusted (unit-level) | 2,407 | 2,407 |
| tested | 681 | 693 |
| unverified | 1,351 | 1,357 |
| Credited units / contract kind | 50, all `safety-only` | 51, all `safety-only` |
| Third-party runtime security SLoC | 225,162 | 225,162 |
| of which libcrux (22 package entries) | 54,848 | 54,848 |
| First-party hazard units | 2 (`benches/client_stack.rs`) | 2 |

Trusted items (report headline, 2,541 items / 13,954 SLoC; item SLoC overlap and can exceed
unit SLoC): 357 in Bertie's extraction, 2,137 in the hax/model F* libraries, 46 first-party
Rust opaque/replacement items (10,924 SLoC), 1 hax record. F* escapes by origin: Bertie
extraction 29 `assume` + 2 option hazards; libraries 163 `assume` + 4 option hazards; Rust
`unsafe`: 3 first-party, 2,637 third-party; plus 10 third-party `assume` and 75 `kani::assume`.
The F* extraction definitions classify as 50 `.fst` proven-extracted (all `safety-only`),
113 hazard, 49 trusted, 11 unmapped (the 155 `.fsti` entries that read "proven-extracted"
are interface signatures; a signature earns no credit when its body file is a hazard or
trusted). Every contract in the credited set is `safety-only`: the module proves hax's
output typechecks (no panics/overflows as encoded), not that it implements TLS 1.3.

Out-of-proof-scope first-party security SLoC (never credited, never hidden; exclusive
reasons in the order non_hax, stream, feature-gated):

| Reason | upstream | shipped |
| --- | ---: | ---: |
| `bertie::stream` (hax-excluded; also behind the `api` feature) | 474 | 480 |
| `non_hax` (hax-excluded) | 6 | 6 |
| feature-gated, compiled only under shipped features, other than `stream` | 0 | 0 |
| feature-gated, inactive in both builds (`hax-pv`, `secret_integers`, `test_utils`, ...) | 52 | 52 |
| in proof scope | 4,975 | 5,000 |

The scope split is a syntactic cfg evaluation (`scripts/hax/v171_numbers.py`, unit-tested),
not the compiler's. The only feature that differs between the proof build (`defensive,std`)
and scratch-server's real build is `api`, which gates the whole `stream` module.

### Worked example: `certificate_verify` / `ecdsa_signature`

Both are in `Bertie.Tls13formats.fst`, a file that contains 12 genuine `assume`s and a
`#push-options "--admit_smt_queries true"` region, so the tool treats its definitions as
hazard or trusted, and nothing in it is credited.

| Rust unit | Location (upstream) | SLoC | Status |
| --- | --- | ---: | --- |
| `ecdsa_signature` | `src/tls13formats.rs:1130` | 24 | unverified (F* body: hazard, `safety-only`) |
| `parse_ecdsa_signature` | `src/tls13formats.rs:1163` | 20 | unverified (F* body: hazard, `safety-only`) |
| `certificate_verify` | `src/tls13formats.rs:1183` | 30 | trusted (F* body: trusted, `functional`) |
| `parse_certificate_verify` | `src/tls13formats.rs:1218` | 38 | trusted (F* body: trusted, `safety-only`) |

`certificate_verify` carries a hax `ensures` about `parse_certificate_verify`, a real
functional postcondition, but the module is not credited, so it is not claimed. If it were
credited it would be a self-consistency statement (encoder and parser agree with each
other), not conformance with RFC 8446's DER encoding: `ecdsa_signature` emits fixed-width
r/s and `parse_ecdsa_signature` rejects valid short r/s, and no contract in the module would
notice. The interface (`.fsti`) entries for `ecdsa_signature` read `proven-extracted`
`safety-only`; that is the signature only.

### Tool bug found on the way

`unsafe_options()` classified F* implicit-argument lines (`#u8`, `#t_Bytes`,
`#FStar.Tactics.Typeclasses.solve`) as weakened file options, so every hax-extracted
`.fst` body was a hazard and the first fresh report credited 0 units. Fixed in its own
commit with a regression test; real option pragmas, `assume` and `--admit_smt_queries`
remain hazards. Before the fix: 0 credited units, 174/176 hazard definitions; after: 50/51
credited, 113 hazard (the genuine ones).

### v1.7 history (superseded)

The v1.7 run could not extract: its lock audit blocked both trees because extending the
shipped lock to the full workspace changed `rand_core 0.10.1`'s presence in Bertie's
closure (`getrandom 0.4.3` carries an optional `rand_core` edge switched on by another
scratch-server member), so `--fstar-log` correctly rejected the evidence and nothing was
credited. It typechecked only the archived snapshot, which fails (`Namespace Core cannot be
found` with 2025.03.25, and after 54/47 modules with 2025.10.06). It reported 5,832 / 5,863
first-party security SLoC at 0.00% proven-extracted; the denominators differ from the
5,507 / 5,538 above by 325 SLoC because the v1.7 report root was Bertie's whole workspace
(its `record`, `bogo_shim`, `simple_https_client` and `simple_https_server` members are
now out of the root).

## Dependency sizes: before → after

Fetch-only containers populated separate project Cargo homes; host reports read
those caches offline without executing target code. Vest and Firecracker use
archived lockfiles; Anvil uses the explicitly recorded generated lock. All
three now use **Cargo metadata (runtime)**, rather than lockfile fallback, with
**zero unknown runtime or build-time source sizes**. The prior lockfile inventory
and the new target-filtered runtime inventory have different package sets.

| Project | Prior lockfile packages / unknown sizes | Runtime packages / SLoC after | Build-time packages / SLoC after |
| --- | ---: | ---: | ---: |
| Vest | 207 / 207 | 121 / 1,035,488 | 40 / 544,353 |
| Anvil | 0 / 0 (inventory incomplete) | 209 / 2,974,275 | 65 / 757,393 |
| Firecracker | 181 / 181 | 78 / 1,718,910 | 69 / 741,286 |

Vest dependency runtime SLoC changes **0 known (207 unknown packages) →
1,035,488 measured (121 resolved runtime packages)**. Its first-party proof
numerator and denominator are preserved. Headline reliance includes the newly measured
dependency inventory; receipt-scope reliance is reported separately above. Build-time
packages remain a separate trusted supply-chain inventory; dev-only packages
are excluded where Cargo confirms the graph. Features can still be unified by
Cargo, and package inventories include cfg/test/platform-specific source.

## Dependency size modes (v1.5, aarch64 Linux)

2026-10-29. The same bundle archives, pinned cargo homes, configs and receipts,
replayed by `scripts/v15_replay.py --container` with a Linux build of this tool
inside an aarch64 Linux container. `cargo metadata`, `cargo check` and every
build script ran on Linux. All runs exit 0. Target `aarch64-unknown-linux-gnu`,
except where Firecracker's shipped musl binary needs a matching closure.

| Project | Mode | First-party security proven / SLoC: with inputs; default rules only | Third-party runtime security | Fallbacks | Test code (units / SLoC) |
| --- | --- | --- | ---: | ---: | ---: |
| Vest | whole-crate | 2,961 / 10,555 (28.05%); 2,961 / 88,267 (3.35%) | 1,035,488 SLoC | 0 | not detected |
| Vest | compiled-files | 2,961 / 10,546 (28.08%); 2,961 / 80,922 (3.66%) | 669,825 SLoC | 0 of 121 | 411 / 7,345 |
| Vest | linked-lines | n/a: Vest is a library with no shipped binary; compiled-files applies | — | — | — |
| Anvil | whole-crate | 0 / 13,893 (0.00%); same | 2,974,275 SLoC | 0 | not detected |
| Anvil | compiled-files | 0 / 9,121 (0.00%); same | 1,298,101 SLoC | 0 of 209 | 351 / 4,772 |
| Anvil | linked-lines | 0 / 9,121 (0.00%); same | 123,754 linked lines | 8 of 209 | 351 / 4,772 |
| Firecracker | whole-crate | 0 / 77,638 (0.00%); same | 1,718,910 SLoC | 0 | not detected |
| Firecracker | compiled-files | 0 / 40,765 (0.00%); same | 465,089 SLoC | 2 of 78 | 1,774 / 36,873 |
| Firecracker | linked-lines (musl) | 0 / 40,765 (0.00%); same | 4,571 linked lines | 1 of 78 | 1,774 / 36,873 |

**Container provenance.** Local image `rust:1.98.0-bookworm` (arm64, digest
`sha256:82150a52ec202c1b14d7817e14516c392bb7f5cfebd88f1ed531cb37ebd39922`),
run natively on the arm64 host with `--cpus 6 --memory 4g`, with the repository
mounted at a fixed container path. Each project's pinned toolchain ran: Anvil's
`rust-toolchain.toml` pins rustc 1.98.0 (the image's); Firecracker's pins 1.97.0,
installed with `rustup toolchain install`; Vest pins none and uses 1.98.0. Reports
record the cargo that ran (`cargo_version` is now read in the project root, so a
rustup override is reported). Crate sources came only from the pinned cargo
homes with `--frozen` and `CARGO_NET_OFFLINE`; no crate was missing. Firecracker's
build scripts need system libraries, not crates. Its own build container
(`tools/devctr/Dockerfile`) supplies them, and the same set was installed here:
libclang for `userfaultfd-sys`'s bindgen, musl-tools, and libseccomp v2.6.1 built
statically with `musl-gcc` (Debian's 2.5.4 lacks `seccomp_export_bpf_mem`).

**Binaries for linked-lines.** Each is a release build inside the container with
only `CARGO_PROFILE_RELEASE_DEBUG=true` and `CARGO_PROFILE_RELEASE_STRIP=false`
overridden; every other setting is the project's own release profile.
- **Firecracker:** `tools/release.sh`'s default and shipped target is
  **aarch64-unknown-linux-musl**, built as that script does (`--workspace --bins
  --examples`; panic abort, LTO, one codegen unit). The `firecracker` binary is
  static, sha256 `3e31ea7e48f5621a9d0b7239042bb83091f8f64eca2c8dd287b75585b8a56a27`.
  Its linked-lines run uses the musl target so the closure matches the binary. On
  musl the closure is the same 78 packages, whole-crate 1,718,910, compiled-files
  464,396 (2 fallbacks, as on gnu). With debug info on, Firecracker's build
  script embeds its empty default seccomp policy: that changes data, not linked code.
- **Anvil:** its controllers ship as glibc binaries (`docker/controller/Dockerfile`,
  Ubuntu). Upstream builds them with `cargo verus build`. Verus is out of scope
  here, so they were built with plain `cargo build --release --bins --target
  aarch64-unknown-linux-gnu`, which compiles (as `cargo check` does) with ghost
  code erased; the manifest sets no release profile. The table uses the largest
  of the seven runnable binaries, `vstatefulset_controller` (sha256
  `8884b3971ed3247489cb68e16b9a84f0b8c5bc4c340d7c7016e8100b6de1e910`). The others
  range from 119,266 linked lines (admission controllers, 3 fallbacks) to 123,728
  (controllers, 8 fallbacks). `esr_composition` is an empty `fn main() {}`
  verification target and was not measured.
- **Vest** is a library; it stays on compiled-files.

**Fallbacks (every one, with its reason):**
- Firecracker compiled-files (gnu and musl): `errno@0.3.14` and
  `aho-corasick@1.1.5`, "no library artifact from cargo check". `cargo metadata`
  keeps both in the runtime closure, but `cargo check` (exit 0) never builds them:
  rustix uses errno only off the Linux raw backend, and regex enables aho-corasick
  only through dev-dependency feature unification (criterion, env_logger).
- Firecracker linked-lines: `wyz@0.5.1`, "symbols from this package are linked
  but have no line rows".
- Anvil linked-lines (`vstatefulset_controller`): `either@1.19.0`,
  `getrandom@0.4.3`, `http@0.2.12`, `http-body@0.4.6`, `hyper@0.14.32`,
  `rand@0.10.3`, `rand_core@0.10.1`, `socket2@0.5.10`, all "symbols from this
  package are linked but have no line rows". Each shares its crate name with
  another version in the closure, so its symbols cannot be told apart by version.
  The admission controllers instead fall back on `getrandom@0.3.4`,
  `getrandom@0.4.3` and `hashbrown@0.14.5`.

**Checks** (`scripts/v15_size_check.py`):
- whole-crate in the container reproduces the macOS v1.5 totals and every
  per-dependency size; only the recorded cargo version differs.
- First-party figures equal the macOS runs in every mode. Test detection and
  inventory are syntactic, so they do not depend on the target.
- compiled-files ≤ whole-crate holds for every dependency.
- linked-lines ≤ compiled-files holds for every measured dependency. There are 9
  exceptions, all fallbacks to whole-crate, which is larger than compiled-files
  by construction: wyz (1,733 > 1,330) and Anvil's eight above (for example
  hyper 22,470 > 19,413).

Reproduce (Firecracker additionally needs the devctr system packages above, and
linked-lines a `--binary` built as described):

```sh
docker run -d --name fcsurvey-main --cpus 6 --memory 4g -v "$PWD":/fc -w /fc rust:1.98.0-bookworm sleep infinity
docker exec -e CARGO_HOME=/fc/.cargo-home fcsurvey-main cargo build --release --frozen --target-dir /fc/target/linux-tool
python3 scripts/v15_replay.py --binary target/linux-tool/release/formal-coverage --container fcsurvey-main \
  --out logs/v1.5-linux/survey/compiled-files --case vest --case anvil --case firecracker -- --size-mode compiled-files
python3 scripts/v15_size_check.py --run whole-crate=logs/v1.5-linux/survey/whole-crate \
  --run compiled-files=logs/v1.5-linux/survey/compiled-files
```

**What linked-lines does not count.** All three modes count `.rs` files of cargo
packages. The Firecracker binary also links aws-lc's C and assembly (161
compile units, ~108,116 distinct linked C/asm lines by `readelf`). Its debug
paths are remapped to `/aws-lc/...`, so `aws-lc-sys` counts 0 linked lines.
That 0 means no Rust lines, not no code. Static musl adds ~3,600 more lines.

The macOS v1.5 runs of these modes fell back for every survey dependency (no
aarch64-linux standard library or binaries on the host). The table above
replaces them.

The first-party change between whole-crate and the other modes is structural test
detection alone. whole-crate reproduces the v1.4.2 reports byte-for-byte apart
from the listed new fields. Vest's schema-2 config
([`docs/survey/vest.toml`](survey/vest.toml)) restricts security to `vest_lib`, so
the default-rules-only figure shows its effect: 28.05% with inputs against 3.35%
under default rules only in whole-crate mode. Each of its ten bare globs now
carries an `override has no reason` warning.

## Classification and retained regressions

Runtime/proof denominators and distinct escape inventories remain unchanged. Vest's five active escapes remain unchecked
slice/UTF-8 operations, not assume/admit. Its complete `vest_lib` inventory still
contains 33 trusted sites (246 SLoC) and 15 unsafe sites; the cfg-active receipt
uses 31 (234 SLoC) and five unsafe escapes. The old no-feature Verus command and
three pinned driver cfg names are unchanged. Inactive code stays uncredited.
Firecracker retains exactly the original 412 distinct first-party unsafe sites;
its proof-only `kani::assume` inputs are not escapes. Its 69 proof-code functions
(906 SLoC) remain excluded from runtime denominators.

The v1.1-c survey snapshot is retained under `historical_v1_1_c` in results.json.
The existing archived Vest hashes, Firecracker unsafe-site equality and all
scratch-server regressions remain checked. No new scratch-server verifier runs
were performed. Stages 12/13 v1.2 runtime-closure reports retain **644/1,363 = 47.25% proven, relying on
0 trusted items (0 SLoC) and 2472 escapes**; their proof counts remain
39/39 and 200/200. See [scratch-server results](scratch-server.md).

## Reproduce and audit

[Run setup and replay instructions](../scripts/verify-runs/README.md) explain
image pins, exact recorded commands, generated locks, read-only same-path mounts,
resumable named runs and offline reporting. Build/verifier output must be
redirected to files; inspect only `tail -n 30` or grep. Do not rerun the old
survey.py host-verifier mode for v1.4.

```sh
python3 scripts/verify-runs/dependencies.py > runs/v1.4/dependencies.log 2>&1
python3 scripts/verify-runs/kani_credit.py > runs/v1.4/credits.log 2>&1
python3 scripts/verify-runs/survey_snapshot.py
python3 scripts/check_survey.py > runs/v1.4/check-survey.log 2>&1
CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo test --frozen > runs/v1.4/tests.log 2>&1
CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo clippy --frozen -- -D warnings > runs/v1.4/clippy.log 2>&1
CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo fmt --check > runs/v1.4/fmt.log 2>&1
git diff --check
```

The checker requires the retained local run artifacts and read-only clones.
These are local agent-run checks; independent acceptance has not been performed.
No pushes, new branches, host installs, Docker Desktop setting changes, external
messages, VM boots, integration tests or /dev/kvm access were performed.
