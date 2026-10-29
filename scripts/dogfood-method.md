
## Reading the progression

`main` and `nix-scratch` have no verifier script, so their locally proven share is
zero. `verified-boundary` introduces the contracted HTTP parser. Browser grammar
changes retain that evidence boundary. HTTPS adds substantial unverified provider,
transport and dependency code; upstream primitive claims do not prove those
adapters or whole crates. `https-headers` extends the grammar; `idle-connections`
adds unverified transport handling. Exact measured changes are in the two tables;
the broad dependency denominator can hide the parser improvement, so the
first-party view is also shown. No proof of end-to-end TLS is implied.

## Method, evidence and limitations

- Reproduce: `CARGO_HOME="$PWD/.cargo-home" cargo build` then
  `python3 scripts/dogfood.py`; acceptance: `python3 scripts/check_dogfood.py`.
  Exports, downloaded sources and isolated verifier toolchains live under /tmp.
  The runner builds v1 commit `4685da4841e9f9bf2e74442d2e9d24e82317ead3`
  under ignored logs/v1-tool for direct comparison; committed `.v1.json` files
  retain its summary, while complete v1 reports remain in ignored logs/.
  Cargo/verifier output lives in this repository's ignored logs/.
- Every available `./verify.sh` is run on its exported branch. Its output is
  retained as `.verify-script.log`. Each root named by that script is then run
  independently, with its own `.verus.log` and `.evidence.json`; all root receipts
  go into **one** v1.1-b CLI call per branch. Stages 8–13 have three roots;
  stages 3–7 have one; main/nix-scratch have none. No historical scratch-server
  verifier logs are reused. JSON records every receipt and its credited units.
- Dependency archives are downloaded from static.crates.io and checked against
  each branch's Cargo.lock SHA256. All registry source SLoC is measured locally,
  including optional/build/test and platform-specific code. This deliberately
  overstates the runtime dependency boundary. Acceptance rejects unknown sizes
  for these branch measurements.
- Syntactic identifier reachability is approximate. Unknown calls/macros include
  all first-party functions. The example config explicitly excludes port parsing,
  standalone healthcheck and test modules; all matches are recorded as overrides.
  Production TLS, transport, dispatch and parser remain security-relevant.
- Verus `proven` covers contracted functions in complete invoked roots and
  conservatively followed plain module files. Spec/proof functions can be
  verified without explicit contracts. `intent.rs` is reached by the parser's
  plain `mod intent;`. Transport's `#[path] mod http;` remains uncredited by
  that receipt, with a warning; the separate parser receipt credits the parser
  and intent module. Non-Verus IO, TLS, entry points and tests remain outside
  these run scopes. Source-span SLoC includes contracts and proof statements. The verifier's query count is not the function
  count. Tests are only a reference signal; no application tests/containers were run.
- Registry entries preserve TLS-PROVIDER.md's **unconfirmed** exact release/target
  correspondence. Partial primitive coverage never promotes whole dependency
  units. See [evidence rules and primary sources](../docs/evidence.md).
- The pinned v1 comparison retains the single parser-root receipt used by M5
  and v1.1-a. At stage 12 that gives 502/1685 (29.79%); at stage 13,
  502/6095 (8.24%). The earlier external union of three v1 reports was
  715/1685 and 715/6095. v1.1-b uses all receipts internally and excludes erased
  proof code, so the table explicitly compares different evidence scopes.
  No post-processing union supplies v1.1-b headline numbers.
