# M3 equality acceptance — 2026-11-26

PASS on Bertie origin/main `110802aae37cb2e4adce165a1410bdea4be01644`,
using the prescribed Docker toolchain and `FSTAR_JOBS=1`. No proof retries or
exit 137. Full fresh checks: 18/18 exit zero. No cached project proofs.

| Case | Fresh roots checked | Verifier phase (s) | Debug report (s) |
| --- | ---: | ---: | ---: |
| Full fresh | 18 | 1689.2 | not separately timed |
| Unchanged | 0 | 24.8 | 120.6 |
| Leaf changed | 2 | 463.6 | 119.8 |
| Leaf reverted | 0 | 25.6 | 116.6 |

Full-fresh time includes extraction and all strict checks. Incremental verifier
phases include fresh extraction, fresh dependency analysis, selection and only
selected strict checks. Report measurements use the local **debug** binary;
these are shared-machine observations, not hosted-CI release-build benchmarks.

The temporary addition of `RECEIPT_M3_CANARY` in `src/der.rs` changes only
`Der.fsti`; `Der.fst` remains identical. Exactly Der and Tls13formats are selected
and both exit zero; the other 16 roots reuse main receipts. Rust source bytes
are restored in `finally`; `git diff --exit-code -- src/der.rs` passes. Fresh
re-extraction after restoration selects zero roots.

`a-fresh.json`, `b-unchanged.json`, `c-reverted.json` are byte-identical,
4,557,458 bytes each, SHA-256:

`924c77d9d316173406feacd553c5ccbbfaff017cbe7a3d5146e9962452c860e8`

Default-mode JSON **and Markdown** also exactly equal the immutable v1.11.0
`cac0f85` build on the same full strict evidence. All comparisons are literal,
without normalizing paths, removing evidence runs or dropping fields.

First-party security: 4,125/5,092 proven-extracted; proof ratio
0.1423030303030303; accepted evidence credits 310 Rust units. The existing
compiled-files fallbacks for chacha20@0.10.2 and getrandom@0.4.3 remain equal to
v1.11.0 (no library artifact from cargo check). Original `[ship]` retained.

End-to-end report CLI tamper PASS (expected failure): changing a recorded
standard-library dependency hash returns nonzero before writing a report:
`module fresh/Bertie_tls.Der.fst receipt` / `dependencies hash:
external/0/fstar-bin-2025.10.06/fstar/lib/fstar/ulib/FStar.Attributes.fsti`.

Artifacts: target/v1120 holds per-case reports, reuse manifests, selections,
timings and outer logs; a-inputs preserves the original full strict run's
extraction, libraries, context, receipt and raw per-root slices. main-receipts
and isolated local Git fixtures preserve the historical main records. The
project verifier clears its transient check directory on each extraction.
Current final source/extraction inputs remain in the scratch Bertie checkout.
No consumer checkout edits or real remote publication. Hosted actions and an
independent acceptance rerun remain unverified.
