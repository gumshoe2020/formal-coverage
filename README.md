# formal-coverage

[![CI](https://github.com/gumshoe2020/formal-coverage/actions/workflows/ci.yml/badge.svg)](https://github.com/gumshoe2020/formal-coverage/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/tag/gumshoe2020/formal-coverage?label=release)](https://github.com/gumshoe2020/formal-coverage/releases)
[![License: MIT](https://img.shields.io/github/license/gumshoe2020/formal-coverage)](LICENSE)

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

## v1.9.1: more hax naming forms

Hax's trailing numeric escapes, typed trait instance records and empty-segment
inherent impl names now have conservative fallback mappings. Trait records
match the Rust trait, Self type and generic arguments; only explicit method
body fields can receive credit. Interface records retain trusted classification
and require the F* version checked by the [M0 canary](dev/v191/NOTES.md).
Cfg twins stay unmapped when the receipt lacks an explicit extraction cfg set.
Ambiguous identities, unsupported generic binders and missing type guards stay
unmapped with reasons in JSON and Markdown. Existing mappings retain precedence
and new mappings cannot share a Rust unit with another definition.
See the [paired replay](dev/v191/compare.md).

## v1.9.0: reusable coverage CI

The [CI kit](ci/README.md) wraps the existing measurement tool with a git-based
ratchet, committed baseline/document checks and shields.io endpoint badges.
Measurement behavior is unchanged from v1.8.5. Proof cost stays informational.
See [Use in CI](#use-in-ci) below and the [changelog](CHANGELOG.md).

## v1.8.5: numbered hax impl methods

Numbered `impl_<N>__method` definitions now map to a specific inherent Rust impl
block when the module's whole method group selects exactly one block and the
F* definition text mentions that block's `t_Type` token. Different numbered
groups cannot select the same block. This uses method sets and type tokens;
hax's impl numbers do not encode source order. Trait impls are excluded.
Ambiguous groups and missing type tokens stay unmapped with a visible reason
in JSON and Markdown. Existing direct, named-type and uppercase value mappings
retain their rules. Mapped definitions use the existing evidence classification:
strictly checked bodies can earn proven-extracted credit; interface-only
methods remain trusted. See the [paired replay](dev/v185/compare.md).

## v1.8.4: code that doesn't ship isn't counted

Optional `[ship]` configuration limits first-party coverage to files rustc reads
for the named packages and the workspace members reached through their normal
runtime dependencies, with the configured Cargo features. Unrelated workspace
members and cfg-disabled files leave every total, including proven numerators
and security/non-security denominators. Without `[ship]`, the existing inventory
of all workspace members stays in scope.
`[ship]` controls what is counted; tests and proof evidence anywhere in the full
first-party inventory still confer the same tiers on shipped units.

This fails toward counting: missing dep-info or a failed `cargo check` keeps the
uncertain code in scope and emits a warning. Reports keep excluded files visible
in a **Not shipped** summary line and a **Not shipped** section listing each file,
whole-file SLoC and its units' would-be tiers. This is file-level build scope;
inline cfg-disabled code inside a compiled file can still count. See the
[paired replay](dev/v184/compare.md).

## v1.8.1: extraction-only Rust is proof code

In first-party hax crates, items whose own or enclosing `cfg` attributes imply
`hax` are standalone proof code in both ghost modes. Accepted F* credit reports
these units as `verified`; uncredited units retain their status. Inside runtime
functions, cfg(hax) statements and blocks move to inline proof in `separate`
mode and stay runtime in `runtime` mode. Shared runtime lines stay runtime;
overlapping annotations count once. See the [paired replay](dev/v181/compare.md).

## v1.8: hax/F* proof cost

Proof ratios now include `proven-extracted` runtime in the denominator and
whitelisted hax/F* annotations in inline proof. Reports separately show
annotations, strictly verified patch additions, hand-written F* proof files,
and model SLoC (trusted assumptions, excluded from the ratio). No tier rule,
receipt grammar or threshold changed. See the [release notes](docs/v1.8.md)
and [paired replay](dev/v18/compare.md).

## v1.7.3: doc comments no longer count as code

SLoC now excludes lines occupied only by Rust doc comments or explicit `doc`
attributes, including inline Verus ghost sizing. Other attributes and lines with
real code still count. This changes measured sizes and coverage denominators;
status-weighted line totals and percentages may change. No tier rule, receipt
grammar or threshold changed. See the [release notes](docs/v1.7.3.md) and
[Bertie comparison](dev/v173/compare.md).

## Proof cost ratio (v1.6)

`--ghost-lines separate` is the default. In first-party runtime functions,
lines containing only supported Verus ghost constructs or whitelisted hax/F*
annotations, plus cfg(hax) statements in hax crates, move from runtime to inline
proof SLoC. Mixed runtime/ghost lines stay runtime. `--ghost-lines runtime`
keeps these lines in runtime. The first report line records the mode; a table
shows proven runtime figures under both modes using the same accepted evidence.
Other Rust code and dependency sizing are unaffected.

The ratio is **(standalone proof + inline proof + F* patch additions +
hand-written F* proof SLoC) / (proven + proven-extracted runtime SLoC)**.
F* annotations and cfg(hax) statements are already part of inline proof and are never added twice. Reports show first-party security-relevant and all-first-party headlines,
crate/module rows, the standalone/inline split, and bounded runtime proofs in a
separate column. A zero combined denominator means `n/a`. Unattributed root proof code
still counts in crate and overall totals.

The default acceptance range is **1.0–10.0**, inclusive. A schema-3 config can set:

```toml
[proof_ratio]
min = 1.0
max = 10.0
reason = "review budget for proof maintenance per proven runtime line"
```

Either bound may be omitted, but not both; bounds must be finite, nonnegative
and ordered. A non-empty reason is required; a TODO reason fails `gate`.
`--proof-ratio-range MIN:MAX` overrides config for that run (CLI > config >
default); `2:` and `:15` are open-ended. Reports record the range, source, reason
and `within`, `below`, `above` or `n/a` status. `gate` fails below/above the range
for the first-party security headline; module exceptions are warnings only.
For a ratio gate with an explicit zero coverage threshold, use
`formal-coverage gate path/to/workspace --min-security-proven 0 --proof-ratio-range 1:10`.
`--no-proof-ratio-gate` disables that check and is recorded. Existing coverage,
completeness, hazard and input-reason checks still apply.

Optional schema-2/3 extraction declaration (v1.8.2):

```toml
[fstar.extraction]
inactive_features = ["hax-pv"]
reason = "hax-driver.py enables hax-pv only for ProverIf extraction"
```

A nonempty feature list requires a nonempty reason. Only plain
`cfg_attr(feature = "X", ...)` predicates qualify; `all`/`any`/`not` and unknown
predicates retain trust. Recognized `proverif`, `coq` and `ssprove` attribute
paths are non-F* trust independently of this declaration. Reports list these
annotations with locations, affected units and SLoC separately from F* trust.
Unconditional opaque, F* replacement and unknown paths retain trust. Removing
an F* trust annotation's effect still requires normal source-bound F* evidence
for credit. Inherent `impl_<Type>__<method>` definitions map only to a unique
Rust method; trait impl names remain unsupported. Since v1.8.3, if both direct
and inherent lookups find no unit, `v_<Rest>` also tries `<Rest>` only when
its first character is ASCII uppercase. The same rule applies to the method
part of `impl_<Type>__v_<Method>`. All lookups require exactly one Rust unit;
ambiguous existing matches stop without fallback. Normal F* evidence rules
still determine credit.

Optional schema-2/3 configuration for hand-written F* inputs:

```toml
[fstar.proof]
patches = ["proofs/fstar/extraction-panic-free.patch"]
files = []                                     # hand-written .fst/.fsti proof
models = ["proofs/fstar/libcrux-models/*.fsti"] # trusted assumptions only
```

Globs are relative to the workspace root; unmatched globs are errors. Absent
configuration adds no patch/file/model cost. Patch additions exclude headers,
blank lines and comment-only lines; each target must be a strictly verified
module in an accepted F* receipt. Lax, admitted, hazard, interface-only and
absent modules count zero, with each excluded hunk and reason listed. Module
names must match exactly. Hand-written files additionally need matching
receipt-bound bytes. Generated extraction and configured models never count
as hand-written proof. Without an accepted receipt, patch/file cost is zero;
models are still reported separately.

The hax whitelist is centralized in [src/hax.rs](src/hax.rs), with a test for
every exact path. It covers the approved contracts, F* quoting variants,
proof helpers and explicitly named int/prop helpers; both `hax_lib::` and
`hax::` spellings and scoped `use hax_lib as NAME` aliases work. Unknown paths
are listed for review and stay runtime. ProVerif and SSProve add no proof cost.
`assume` retains its escape treatment. Annotation cost follows the same
first-party, runtime-unit and security filters as Verus inline proof.
Unmapped external lemmas/models are conservatively security relevant in the
root crate's `<unattributed>` module row.

JSON report schema is **4**; configuration remains schema 2/3. The new
`proof_ratio` fields are `fstar_annotation_sloc`, `fstar_patch_sloc`,
`fstar_file_sloc`, `fstar_model_sloc`, `proven_extracted_runtime_sloc` and
`excluded_patch_hunks`, also present in crate/module rows. v1.8.1 adds
`fstar_cfg_hax_sloc` (standalone) and `fstar_cfg_hax_inline_sloc` (inline),
also exposed in both-mode figures. Distinct
`unknown_hax_paths` are reported beside the ratio. `proven_runtime_sloc`
continues to mean Verus proven runtime alone; bounded runtime remains separate.

cfg implication uses the shared test/hax algebra: `cfg(hax)` and
`cfg(all(hax, ...))` qualify; `any(...)` qualifies only when every arm implies
hax. `not(hax)`, `cfg_attr(hax, ...)`, `feature = "hax"` and other cfg names
qualify for no cfg proof credit. The existing hax project marker is an
inventoried source containing `hax_lib`, applied at first-party crate scope.
Annotations in standalone proof units belong to standalone SLoC and contribute
zero `fstar_annotation_sloc`. Gated `use` items remain uncounted. Receipt grammar,
evidence acceptance and thresholds are unchanged.

This is a **cost signal, not a quality score**. `ensures true` can require zero
proof statements; its low cost says nothing about whether the specification is
useful. Syntactic line counts also cannot measure proof difficulty or establish
end-to-end correctness.

Build with stable Rust and Cargo. The test, clippy, fmt and CI quality gates run
on current stable Rust (1.100.0 for v1.8.4), without a toolchain override:

```sh
cargo build --release --locked
./target/release/formal-coverage report path/to/workspace --json report.json --markdown report.md
```

Reports read Cargo metadata offline. Use `--target TRIPLE` to select a dependency
graph; the default is the host. If frozen metadata fails, the report labels its
conservative lockfile fallback. Unknown dependency sizes stay unknown and block
security gates.

Runtime dependencies are sized in one of three modes (`--size-mode`). The mode,
target and profile are on the first line of every Markdown report:

- `compiled-files` (default): only the `.rs` files rustc read for each crate, from
  cargo dep-info after `cargo check --frozen --target TRIPLE`, minus spec/proof
  functions. Tests, benches, examples and cfg-disabled modules drop out. This runs
  `cargo check`, which compiles and executes the workspace's build scripts and
  proc macros; use `whole-crate` where that is not acceptable.
- `linked-lines` (`--binary PATH`): distinct (file, line) pairs in the binary's DWARF
  line table per dependency, i.e. the code actually linked after dead-code
  elimination. The unit is linked lines, not SLoC. Build with debug info; on macOS
  a UUID-matched dSYM is used. Use it for a shipped binary.

Measure on the target's OS: a host without the target's standard library cannot
run `cargo check` for it, and every dependency falls back. The survey and
scratch-server v1.5 numbers come from a Linux build of this tool run in an aarch64
Linux container (`scripts/v15_replay.py --container`).
- `whole-crate`: every `.rs` file under each package, the pre-1.5 measure. It reads
  sources only, never compiles; add `--ghost-lines runtime` to reproduce v1.4.2 output.

Missing evidence never shrinks a count. A dependency without dep-info or debug info
keeps its whole-crate size, and these fallbacks are listed at the top of the report.

Test code is found structurally, whatever the module is named: functions with
`#[test]`-style attributes, anything under `#[cfg(test)]` at any depth, and
integration-test and bench targets. It is reported on its own line and excluded
from runtime totals (not in `whole-crate` mode).

A `formal-coverage.toml` (schema 3) identifies the boundary. Every input needs a
reason; a pattern or dependency key that matches nothing is an error:

```toml
entry_points = ["server::receive"]

[[security.include]]
pattern = "src/parser.rs"
reason = "parses attacker-controlled bytes"

[[security.exclude]]
pattern = "server::logging::*"
reason = "formats fixed strings; never sees request data"

[dependencies."foo@1.2.3"]      # "name" alone is an error when two versions are present
sloc = 1200                     # and/or security = true|false
reason = "measured by hand: see docs/foo-size.md"
```

Optional schema-3 shipped-artifact scope in `formal-coverage.toml`:

```toml
[ship]
packages = ["bertie-tls"]
features = ["default"]
reason = "Ship the bertie-tls library with default features"
```

`packages` names workspace packages. `features` is the explicit Cargo feature
set: `"default"` enables default features; omit it (or use an empty list) to
disable defaults. Other entries are passed through `--features`. A non-empty
`reason` is required. Scope comes from `cargo check --frozen -p PACKAGE --lib
--bins` dep-info for each selected package (binary-only packages omit `--lib`),
using the report target and features. Build scripts and proc macros can execute.
Normal workspace dependency edges extend first-party scope; dev/build-only
edges do not. Dependency coverage and evidence acceptance rules stay unchanged.
Excluded files appear under **Not shipped**, outside all coverage and proof
cost totals. JSON exposes the selection as `ship` and excluded files as
`not_shipped`; both fields are omitted when absent/empty.

Each security headline is printed twice: with these inputs and under default rules
only. An Inputs section shows the SLoC each input moved. `gate` uses the with-inputs
figure; `--max-override-shift PCT` fails when the two first-party figures differ by
more than PCT points, and a `TODO` reason always fails `gate`. Schema-2 files (flat
`security = [...]` / `not_security = [...]`) still work, with a warning per entry;
`formal-coverage migrate-config OLD.toml` rewrites them with `TODO` reasons. See
[the worked scratch-server config](examples/scratch-server.formal-coverage.toml).

Supply retained evidence with repeatable `--verus-log run.log` and
`--kani-log run.log`; each needs a `run.log.evidence.json` receipt. Receipts bind
logs to source hashes and commands. They are **caller-supplied evidence, not
attestations**. Rejected receipts credit nothing and retain their reasons.
Every proven headline states its whole-inventory trusted items and escapes;
receipt-scope reliance is shown separately.

The identifier graph and source sizes are approximate. Inline cfg/platform source
inside compiled files can remain in denominators. Closed command grammars deliberately reject
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

v1.7 adds repeatable `--fstar-log LOG` for source-bound hax/F* receipts.
**Proven-extracted** has its own column and explicitly trusts hax extraction;
it never enters Verus `proven` or its gates. Patched models and interface-only
contracts are trusted, never Rust proof credit. Reports show checked
postconditions and a labelled **self-consistency heuristic**: agreement with
one's own parser does not establish external format conformance. See the
[v1.7 receipt contract](docs/evidence.md#haxf-receipts-v17) and Bertie survey.

## Use in CI

Install a reviewed initial `coverage/baseline.json` and `COVERAGE.md` using the
[local commands and document template](ci/README.md#rules-and-local-use). Copy
[ci/project.example.json](ci/project.example.json) into your project's
`coverage/ci.json`. Keep the project's prover setup and receipt generation before
the coverage action; pass receipts through `report-args` (for example
`'["--verus-log","target/verification.log"]'` or
`'["--fstar-log","target/fstar.log"]'`). With `[]`, code without accepted evidence
remains unproven. Install/fetch locked dependencies before measuring; incomplete
inventory is blocking.

Copy this to `.github/workflows/coverage.yml`. Replace **both**
`FORMAL_COVERAGE_COMMIT_SHA` placeholders with the same full 40-character commit of
the release you pin, for example
`git ls-remote https://github.com/gumshoe2020/formal-coverage 'refs/tags/v1.9.1^{}'`.
Only the publication job has write permission; every blocking step propagates
failure. Do not run this under `pull_request_target`. The project must enable
up-to-date branch checks before relying on concurrent PR ratchets.

```yaml
name: Coverage
on:
  pull_request:
    branches: [main]
  push:
    branches: [main]
permissions:
  contents: read
concurrency:
  group: coverage-${{ github.ref }}
  cancel-in-progress: true
jobs:
  coverage:
    runs-on: ubuntu-latest
    timeout-minutes: 60
    steps:
      - uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683 # v4.2.2
        with:
          fetch-depth: 0
          persist-credentials: false
      # Insert this project's pinned prover setup and fresh receipt generation here.
      - uses: gumshoe2020/formal-coverage/.github/actions/coverage@FORMAL_COVERAGE_COMMIT_SHA
        with:
          workspace: '.'
          report-args: '[]'
          baseline: coverage/baseline.json
          coverage: COVERAGE.md
          config: coverage/ci.json
          artifact-name: coverage-badges
  publish-badges:
    needs: coverage
    if: always() && github.event_name == 'push' && github.ref == 'refs/heads/main' && needs.coverage.result != 'cancelled'
    runs-on: ubuntu-latest
    timeout-minutes: 10
    permissions:
      contents: write
    steps:
      - uses: gumshoe2020/formal-coverage/.github/actions/publish-badges@FORMAL_COVERAGE_COMMIT_SHA
        with:
          artifact-name: coverage-badges
          coverage-result: ${{ needs.coverage.result }}
```

The coverage action outputs `ratchet` and `artifact-name`; the publication action
has no outputs. The latter updates only the `badges` branch with GITHUB_TOKEN.
Embed its `proven.json`, `tested.json`, `unverified.json`, `proof-ratio.json` and
`ratchet.json` through shields.io endpoint URLs. See [inputs, configuration,
thresholds and failure behavior](ci/README.md). formal-coverage itself has no
proven badge: this tool is not formally verified.
