# formal-coverage

A conservative source inventory of formal verification coverage in Rust.
It reports **security-relevant code** separately from everything else, with
first-party, runtime dependency and combined SLoC-weighted figures.

- **Proven**: accepted source-bound Verus evidence for eligible code.
- **Proven-bounded**: accepted Kani evidence under recorded bounds and model assumptions.
- **Tested**: referenced by test code; this does not establish test execution or line coverage.
- **Trusted**: unchecked specifications or implementations that proofs rely on.
  Unsafe code and verifier escapes are reported separately as hazards.

Proof code has its own inventory and stays outside runtime denominators.
Build dependencies and proc macros are a separate trusted supply-chain inventory.
Partial upstream claims do not promote entire dependency crates.

Build with stable Rust and Cargo:

```sh
cargo build --release --locked
./target/release/formal-coverage report path/to/workspace --json report.json --markdown report.md
```

Reports read Cargo metadata offline and never compile or execute target code.
Use `--target TRIPLE` to select a dependency graph; the default is the host.
If frozen metadata fails, the report labels its conservative lockfile fallback.
Unknown dependency sizes stay unknown and block security gates.

A small `formal-coverage.toml` can identify the boundary:

```toml
entry_points = ["server::receive"]
security = ["src/parser.rs"]
not_security = ["server::logging::*"]
```

Supply retained evidence with repeatable `--verus-log run.log` and
`--kani-log run.log`; each needs a `run.log.evidence.json` receipt. Receipts bind
logs to source hashes and commands. They are **caller-supplied evidence, not
attestations**. Rejected receipts credit nothing and retain their reasons.
Every proven headline states its whole-inventory trusted items and escapes;
receipt-scope reliance is shown separately.

The identifier graph and source sizes are approximate. cfg/test/platform source
can remain in denominators. Closed command grammars deliberately reject
unsupported scope: Anvil's `cargo verus focus` receipts receive 0 credit, giving
0.00% proven. M2b explicitly extended the closed Kani grammar in `src/kani.rs` and
`src/evidence.rs`, with regression tests. It credits eligible proof-code harnesses
only; the survey's runtime targets receive 0 credit. Retained verification runs
are aarch64; x86-only Firecracker harnesses were not verified. The current Vest
release replay took 8.19 seconds on the aarch64 macOS host with populated caches
(2026-10-29); timings depend on inputs and caches.

See the [survey](docs/survey.md), [evidence rules and reproduction](docs/evidence.md),
and [scratch-server measurements](docs/scratch-server.md). Third-party verifier
execution uses pinned Docker images and isolated mounts; receipt replay is offline.
The [build record](dev/README.md) retains the specifications and implementation logs.

Licensed under [MIT](LICENSE).
