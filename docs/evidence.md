# Evidence contract and primary sources

`proven` requires an accepted source-bound successful verifier run. Without run
evidence, a safe unit remains `tested` or `unverified`. A syntactic test reference
is a signal, not evidence of executing tests or exercising every line. Verus
runtime credit still requires a contract (`requires`/`ensures`); Verus spec/proof
functions need no explicit contract. Uncontracted exec functions receive no
inferred proof credit.

## Receipts and union

`--verus-log run.log` / `--kani-log run.log` require `run.log.evidence.json`:

```json
{
  "tool": "verus",
  "tool_version": "actual version output",
  "command": ["verus", "--crate-type", "lib", "src/lib.rs"],
  "exit_code": 0,
  "cfg": ["feature=\"std\"", "feature=\"alloc\""],
  "files": {"src/lib.rs": "SHA256 of exact source bytes"},
  "log_sha256": "SHA256 of exact log bytes",
  "harnesses": [],
  "covered_units": {}
}
```

Record a receipt only after process exit. Hash **all inventoried first-party
source files**, including functionless files, not just the invoked root. A
missing/stale source hash rejects that receipt. Bare logs, malformed receipts,
nonzero exits, missing versions, mismatched log hashes, empty/partial/failing
summaries and unsupported commands establish no proof. Receipts are
caller-supplied evidence, not authenticated attestations of execution. Test
receipts are explicitly synthetic policy tests, not mathematical verification.

Both CLI commands accept repeated `--verus-log` and `--kani-log`. Each receipt is
validated independently; coverage is the union by unit, never a sum of duplicate
units or borrowed mappings/bounds. Rejected receipts credit nothing. JSON
`evidence_runs` and Markdown list acceptance, reasons, credited IDs, verified
files, effective cfg, and cfg-active trusted/escape sites for each run.

Only complete direct Verus library invocations are supported, allowing explicit
`--cfg VALUE`, `--cfg=VALUE` and `--edition=VALUE`. All other command flags fail
closed. Pinned Verus `0.2026.10.26.d17fbd0` help confirms `--verify-root`,
`--verify-module`, `--verify-only-module`, `--verify-function` and `--no-verify`;
these restricted/skipped commands remain rejected. Unsupported flags have
**unconfirmed** scope and are not interpreted as complete runs.

## cfg and module scope

The optional `cfg` array records enabled names and key/value settings, including
features as `feature="NAME"`. Explicit command cfg values are also evaluated.
An absent cfg on a supported direct Verus invocation implies no enabled features;
Cargo feature defaults are never guessed. `all`, `any`, `not`, `feature`, `test`
and ordinary names are evaluated per receipt. Unrecorded target settings and
unconfirmed Verus-specific settings remain unknown. Unknown syntax blocks only
its owning item/module and emits a warning naming the predicate and owner.
Nested functions, inline modules and `verus!` wrappers inherit their owners' cfg.

A `cfg_attr` adding only `no_std`, `feature(...)`, `allow`, `warn`, `deny` or `doc`
never blocks existence credit, even with an unknown predicate. Potentially
item-changing attributes with unknown activation stay conservative. Lint escapes
are still scanned independently, so this existence rule cannot hide them.

The pinned [Verus driver source](https://github.com/verus-lang/verus/blob/d17fbd0/source/rust_verify/src/driver.rs)
was fetched and read (`logs/v1.1-c-verus-driver.rs`): verification adds
`verus_only`, `verus_keep_ghost` and `verus_keep_ghost_body` at lines 289–291.
Only those unconditional Verus names are inferred. Conditional core/no-vstd
settings are not assumed for the supported default invocation.

From the invoked root, follow active `mod name;` declarations using `name.rs` or
`name/mod.rs`, recursively, including inline module directories. Every followed
file must have a matching receipt hash. Missing/ambiguous module files warn and
remain uncredited. `#[path]` and verifier-exempt modules remain excluded.
Arbitrary macros are not expanded. cfg-disabled code remains in the source
inventory/denominators but cannot receive verifier credit from an inactive run.

## Trusted base and escapes

`#[verifier::external_body]` and `assume_specification` are **trusted**, never
proven. JSON and Markdown list every trusted site, affected functions and SLoC,
including non-function trusted declarations. See the
[Verus assume_specification reference](https://verus-lang.github.io/verus/guide/reference-assume-specification.html):
these specifications are unchecked. Verus checks callers against the trusted
item's specification; crediting those callers therefore explicitly relies on
that trusted specification. This is conservative about what was checked, not an
independent validation of the specification or the trusted implementation.

`assume`, `admit`, `#[verifier::external]`, verification-lint suppression and
`unsafe` remain hazards. Affected units are `hazard`; unrelated covered units
can receive credit, explicitly marked as relying on the run's trust and escapes.
An unattributed file-scope hazard conservatively affects all functions in that
file; if there are none, it stays an explicit file-scope site. Inherited
verifier-exempt owners also affect their functions.

Every proven percentage has the adjacent statement **“proven, relying on N
trusted items (S SLoC) and E escapes.”** The main headline conservatively lists
the complete inventory, including out-of-scope sites; each accepted run lists
its own cfg-active dependency counts and site references. A site's existence in
a cfg-disabled or unrelated file never silently removes it from the inventory
or escape gate. Escapes are called out at the top of every report.

Sites are distinct `(file, line, kind)` tuples. `affected_units` is a separate
list; a file-level site is not duplicated per affected function. Baseline gate
comparison also uses distinct sites, including legacy reports with duplicates.
Out-of-scope escapes still fail `--no-new-escape-hatches` against a baseline
without those sites. Proof-side and unattributed file-scope hazards retain their
independent gate failures. Trusted items are listed separately and are not
unsound escapes. Third-party unsafe counts also use distinct source sites;
unknown source/tokenization yields an unconfirmed count, never an assertion of
safety. Partial upstream primitive claims do not establish whole-crate proof.

## Kani and proof code (schema 2)

Verus `spec fn`, `proof fn` and `broadcast proof fn` with any visibility and
open/closed qualifiers are proof code by syntax. Exec contracts/ghost blocks
stay with the runtime function. `#[kani::proof]` harnesses and functions under
`#[cfg(kani)]` are also proof code, excluded from both runtime denominators.
Names, strings and comments do not establish function mode or attributes.

`kani::assume` constrains input models inside Kani harnesses and proof-only
`cfg(kani)` helpers (including `Arbitrary` generators/stubs); it is not an
escape there. Outside that syntactically identified proof context it remains
conservatively reported. Other assumptions are not exempted.

Kani requires one explicit, source-resolved harness per serial log. Andrew's
v1.4 M2b decision expands the closed grammar to `cargo kani` with exactly one
`--harness ID`; the receipt must list that exact logged ID. Legacy crate-qualified
IDs and Kani's crate-less pretty names resolve only to a unique hashed harness.
Substring filters selecting another harness, wildcard patterns, workspace runs,
parallel jobs, duplicate selectors and unknown flags are rejected.

The only experimental `-Z` values are `unstable-options`, `stubbing`,
`function-contracts` and `restrict-vtable`. Other allowed options are
`--output-format regular|terse` (also `--output-format=...`),
`--harness-timeout DURATION`, `--package NAME`/`-p NAME`, and positive
`--unwind N`. Check-disabling options, CBMC arguments, default-unwind, playback
and all other flags remain rejected. The result rule is unchanged: exactly one
matching Checking line, one plain SUCCESSFUL line, and no failed/error/expected
panic output. `#[kani::should_panic]` never establishes safety evidence.

Bounds come from the command, then the hashed harness's `#[kani::unwind(N)]`,
then **Kani default: no unwind limit, unwinding assertions on**. Source unwind 0
is preserved as an actual source bound. Every case remains `proven-bounded`,
never unbounded `proven`. Scope labels derive stubs and contracts from hashed
attributes: A replaced by stub B; F assumed to meet its contract for verified
stubs; F's requires/ensures contract for proof-for-contract harnesses. Vtable
restriction is stated explicitly. Conditional or unsupported Kani scope
attributes fail closed. Caller-supplied scope text cannot override this model.

For runtime target credit, an explicit `covered_units` mapping is still required
and the harness must directly call an unambiguous target. Assumptions, hashes,
trust/escape handling and receipt-local union rules remain enforced. The v1.4
M2b replay credits eligible harnesses themselves; it infers no runtime mappings.
The three uncredited raw passes are an expected-panic test and two multi-harness
logs whose filters need the unallowlisted `--exact` switch. Their original logs
and rejection reasons remain visible; no rerun or flag exception raises a count.

The proof section shows SLoC, units and verified/unverified counts with an
informational security split. Trusted, hazardous and bounded units count in
unverified totals, with statuses and lists separately visible. Proof-code
hazards never disappear from the gate. `verified` means covered by an accepted
Verus run, not independent validation of specification intent.

Schema 1 baselines require `--allow-schema-change`; the incomparable percentage
comparison is skipped while thresholds, completeness and escape checks remain.
The schema remains 2; regenerate baselines when adopting v1.1-c's Kani category
and cfg/trust policies rather than treating an old inventory as an unchanged
measurement method.

## Other primary evidence

The [Verus summary emitter](https://github.com/verus-lang/verus/blob/main/source/rust_verify/src/verifier.rs),
[Kani output guide](https://model-checking.github.io/verify-rust-std/tools/kani.html),
[attributes reference](https://model-checking.github.io/kani/reference/attributes.html)
and [unwinding tutorial](https://model-checking.github.io/kani/tutorial-loop-unwinding.html)
were read during v1. Summary counts are verifier queries, not a function/result
mapping. The nine published crate archives cited in `registry/upstream.toml`
were also read. Exact libcrux release/proof/target correspondence remains
**unconfirmed**; F* was not run. Registry claims carry explicit operation,
property, tool and primary-source fields. Only an explicitly confirmed
`whole_crate = true` claim can promote an entire dependency; no seed claim has
that scope.

## Target dependency closure (v1.2)

`report` and `gate` accept `--target <triple>`; the host from `rustc -vV` is the
default. Reports record the target and Cargo version. The tool invokes
`cargo metadata --format-version 1 --frozen --filter-platform <triple>` and walks
normal `dep_kinds` edges from every workspace member. Disabled optional packages
are absent. Resolved proc macros, explicit build dependencies and their normal/
build transitives form a separate build-time inventory with SLoC and unsafe
counts. They never enter runtime denominators unless independently needed by a
runtime path. Build-time code can affect the binary and is trusted, not measured
by runtime coverage. Dev-only packages and their transitives are excluded with
a package count; mixed runtime/build/dev packages are retained where needed.

If full metadata or source inventory fails, `closure: lockfile (fallback)` and
its reason appear near the top. The former whole lockfile set is retained,
including optional/build/dev/platform packages; exclusions are unconfirmed.
Missing sources remain unknown, not zero-sized evidence, and unknown security
sizes or incomplete lock inventories block gates. Workspace-local configured
source overrides still validate package name/version. Metadata source roots are
read from Cargo's resolved manifests, without building dependency code.

The first report line is first-party runtime security coverage, with the same
explicit trust/escape statement. Both SECURITY-RELEVANT and EVERYTHING ELSE
have first-party, third-party runtime and combined figures. JSON stores them in
`summary.first_party`, `summary.third_party`, and `summary.all` for each block.
`--min-security-proven` retains its combined meaning;
`--min-first-party-security-proven` adds an independent first-party threshold.
Neither bypasses existing completeness or hazard failures.

The call graph and package source sizes remain approximate: first-party cfg/test
code and full source within each selected dependency package remain inventoried;
source symlinks fail closed. Cargo metadata may unify features required by dev/
build code into normal edges. Those packages remain in the conservative graph.
Also, `cargo tree -e normal` includes proc macros and their dependencies; it is
not exactly the separately measured runtime graph. The literal v1.2 acceptance-C
comparison therefore fails on spec-lemmas: fifteen tree-only build-time packages
and two metadata-runtime-only packages. Exact names, commands, both sets and
current measurements are in [v1.2 results](scratch-server/v1.2-results.json).
No claim of exact equality is made. See [scratch-server results](scratch-server.md)
for the numbers and [survey results](survey.md) for actual pinned verifier runs.
Self-measurement remains later work.

## Reproduce the numbers (v1.4.1)

The small survey and scratch summaries are in git. Full reports, exact source
archives, raw verifier logs, receipts and offline Cargo source caches are in
`formal-coverage-evidence-v1.4.1.tar.zst`. The bundle restores paths relative to
the checkout root. It contains caller-supplied evidence, not an attestation.

```sh
git clone https://github.com/gumshoe2020/formal-coverage.git
cd formal-coverage
git checkout v1.4.1
curl -fLO https://github.com/gumshoe2020/formal-coverage/releases/download/v1.4.1/formal-coverage-evidence-v1.4.1.tar.zst
curl -fLO https://github.com/gumshoe2020/formal-coverage/releases/download/v1.4.1/formal-coverage-evidence-v1.4.1.tar.zst.sha256
shasum -a 256 -c formal-coverage-evidence-v1.4.1.tar.zst.sha256
tar --zstd -xf formal-coverage-evidence-v1.4.1.tar.zst
mkdir -p .tmp logs
CARGO_HOME="$PWD/.cargo-home" TMPDIR="$PWD/.tmp" cargo build --release --frozen > logs/build-release.log 2>&1
python3 scripts/check_survey.py > logs/check-survey.log 2>&1
python3 scripts/check_dogfood.py > logs/check-dogfood.log 2>&1
python3 scripts/check_perf.py > logs/check-perf.log 2>&1
tail -n 30 logs/check-survey.log
tail -n 30 logs/check-dogfood.log
tail -n 30 logs/check-perf.log
```

The release URL is the intended publication location; uploading and tagging are
outside this release pass. These three checkers run **offline from receipts**.
They need Python 3.12+, Rust/Cargo, and the extracted bundle; extraction also needs
zstd support. The retained dependency measurements target aarch64; reproducing
the exact inventory was checked on an aarch64 macOS host. A report's source hash
map binds it to the checkout's measurement code and registry, rather than the
machine-dependent bytes of a compiled binary. `check_perf.py` replays all sixteen
inputs and checks unit attribution, coverage totals and receipt decisions; times
are informational, with no new performance threshold.

Use `--evidence-root PATH` or `FC_EVIDENCE_ROOT` to select extracted artifacts and
`--binary PATH` or `FC_BINARY` to select a fresh binary. `dogfood.py` and the scratch
checkers accept `--scratch-server PATH` (`FC_SCRATCH_SERVER`, default
`../scratch-server`); export-only mode uses `git archive` of `runs.json`'s pinned
commits. Ordinary receipt checks use bundled archives and do not need that clone.
The full scratch reports preserve the historical vendor/lockfile denominator;
the separately retained v1.2 summary table measures the later runtime closure.
Both retain the published first-party proof figures. No verifier runs in dogfood.

Fresh **verifier execution needs Docker**, the separately retained immutable
images, and read-only archives: see [run instructions](../scripts/verify-runs/README.md).
The release iovec rerun uses the pinned Kani image, `--network none`, one read-only
source mount and one project-local writable output mount. Never mount host home,
SSH/Cargo homes or the Docker socket. Historical v1.4 commands are preserved as
recorded; their generic helper did not explicitly disable container networking.
Do not mistake receipt replay for rerunning a verifier or fetching dependencies.

Maintainers regenerate the bundle with `scripts/make-evidence-bundle.sh`; its
default output is `../release/formal-coverage-evidence-v1.4.1.tar.zst` and the
adjacent `.sha256`. Regeneration of the pinned measurements is documented by
`scripts/prepare_release.py --help` and requires the original read-only clones,
the pinned branch list, and retained checksum-verified crate archives. No upload
or host installation is part of these scripts.
