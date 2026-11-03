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
open/closed qualifiers are proof code by syntax. Before v1.6, exec contracts/ghost blocks
stayed with the runtime function; the v1.6 split below is now the default. `#[kani::proof]` harnesses and functions under
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

## Reproduce the numbers (v1.4.2)

The small survey and scratch summaries are in git. Full reports, exact source
archives, path-normalized verifier logs, receipts and offline Cargo source caches are in
`formal-coverage-evidence-v1.4.2.tar.zst`. The bundle restores paths relative to
the checkout root. It contains caller-supplied evidence, not an attestation.

```sh
git clone https://github.com/gumshoe2020/formal-coverage.git
cd formal-coverage
git checkout v1.4.2
curl -fLO https://github.com/gumshoe2020/formal-coverage/releases/download/v1.4.2/formal-coverage-evidence-v1.4.2.tar.zst
curl -fLO https://github.com/gumshoe2020/formal-coverage/releases/download/v1.4.2/formal-coverage-evidence-v1.4.2.tar.zst.sha256
shasum -a 256 -c formal-coverage-evidence-v1.4.2.tar.zst.sha256
tar --zstd -xf formal-coverage-evidence-v1.4.2.tar.zst
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
default output is `../release/formal-coverage-evidence-v1.4.2.tar.zst` and the
adjacent `.sha256`. Regeneration of the pinned measurements is documented by
`scripts/prepare_release.py --help` and requires the original read-only clones,
the pinned branch list, and retained checksum-verified crate archives. No upload
or host installation is part of these scripts.

## Path normalization in v1.4.2

The author-specific project-directory prefix in retained logs and evidence
metadata is replaced with the fixed `/work/projects` placeholder. The
normalizer also probes user-home and agent-directory spellings, JSON slash
and Unicode escapes, URL encoding, Debug-quoted text, and Kani/Verus output.
Only plain prefixes occurred in this snapshot. Relative suffixes are retained;
logs are otherwise byte-identical to v1.4.1. Receipt, manifest, report and
summary hashes covering changed files are refreshed transitively. Cargo
manifest/lock source-provenance hashes also reflect the package version bump
from 0.1.0 to 1.4.2; no measurement code or rule changed.

The bundled `logs/path-normalization.json` maps each changed file to its old
and new SHA-256 and lists the complete reports/summaries compared.
`scripts/normalize-paths.py --before SNAPSHOT --author-prefix PREFIX` probes a
checkout with the original evidence extracted; add `--apply` to normalize and
rehash. SNAPSHOT is a separate v1.4.1 tree with the original bundle extracted.
The private PREFIX is supplied locally and never written into the mapping.
The script preserves formatting and compares complete report/summary bytes,
allowing only the substitution and mapped hashes.

The scratch archives are `git archive` exports of the pinned commits in
`runs.json`. Eleven exports have non-Rust files redacted by
`scripts/redact-archives.py`; the other two scratch exports are unchanged.
The changed members are top-level specs (`BROWSER-WHITELIST.md`,
`CHROMIUM-HEADERS.md`, `FIREFOX-HEADERS.md`, `HTTPS-HEADERS.md`, `HTTPS.md`,
`IDLE-CONNECTIONS.md`, `PRAGMA-HEADER.md`, `PROVEN-TRANSPORT.md`,
`VERIFIED-BOUNDARY.md`), `CODEX_PROMPT.md`, `NOTES.md`, `PROGRESS.md`, `README.md`,
`scripts/acceptance.py`, `scripts/make-lan-cert.py`, and
`tests/safari-{favicon,https-favicon,https-root,root}.http`.
Only these rules change contents: author prefix to `/work/projects`, private
LAN addresses to the documentation range `192.0.2.x` preserving the last octet,
and case-sensitive whole private terms to `[private]` (workstation phrases to
`[workstation]`). Private rule inputs come from `<notes>/public-denylist.txt`.
No Rust source, Cargo manifest or lockfile changed; their hashes are as recorded
in the receipts. Member order, modes, mtimes, tar format and git commit PAX
metadata are preserved. Size and checksum header fields reflect changed bytes.

The bundle includes `logs/release-v1.4.2/archive-redactions.log` (archive, member,
original line/column and rule ID, never original text) and
`logs/release-v1.4.2/archive-member-proof.json` (every changed member with old/new
hashes and a protected-member check for every scratch archive).
`logs/path-normalization.json` maps file names to v1.4.1 and current SHA-256
values, including rebuilt archives and transitively refreshed metadata hashes.
Every source-bound receipt retains its source hashes. All third-party source archives
and cached dependency sources remain byte-identical.

Scan the checkout and an extracted bundle with the same private denylist:

```sh
scripts/check-public.sh <notes>/public-denylist.txt
scripts/check-public.sh <notes>/public-denylist.txt EXTRACTED_BUNDLE
```

The bundle scan prints these allowances before scanning, and has no others:

- Excluded `*/cargo-home/registry/src/**`: byte-identical third-party crate sources.
- Excluded `*/cargo-home/git/checkouts/**`: byte-identical third-party Git
  dependency sources.
- Excluded `logs/evidence/archives/firecracker.tar`: the unmodified `git archive`
  export of public upstream Firecracker at the pinned commit.
- Masked the upstream CI runner work-path substring: build paths inside the
  Verus binary's panic output.
- Masked the GitHub API user-URL prefix: API URLs in pinned metadata.
- Masked the exact PGP signature and PGP public key block armor headers:
  public commit signatures in pinned Anvil metadata.

Masks remove only the exact substring, so any other hit on the same line still
counts, including every other armor header such as a private key marker. All
other archives, including the scratch exports and the Vest and Anvil exports,
are scanned. Tree mode (no bundle argument) has no allowances.

Before relying on the Firecracker exclusion, `scripts/check-upstream-archive.py`
checks the extracted bundle: the pinned commit in `logs/release-evidence.json`
and the Firecracker run manifests matches the archive's git PAX comment header;
the archive SHA-256 equals the v1.4.1 record; the archive is not in the rebuild
mapping; and the raw archive bytes contain no author path, no private LAN
address and no private term as a whole case-sensitive word. Private inputs are
supplied at runtime and the log names only rule IDs. Without the exclusion,
its scan hits are upstream example network addresses, a documentation URL,
case-insensitive substring matches in a test binary and a C header, and
upstream's public signing key.

With these allowances the v1.4.2 bundle scan reports 0 matching lines, and the
tree scan reports 0. The original v1.4.1 bundle and checksum are left untouched.

## Security scope and dependency size modes (v1.5)

Reports are schema 3. Line 1 of the Markdown and the JSON fields `size_mode`,
`size_profile`, `target` and (for linked-lines) `binary_sha256` record how runtime
dependencies were sized.

**Size modes.** `--size-mode` selects one of three:

| Mode | What counts per runtime dependency | Use when |
| --- | --- | --- |
| `compiled-files` (default) | `.rs` files listed in the crate's cargo dep-info (`<target-dir>/<triple>/debug/deps/<crate>-<hash>.d`, mapped through `cargo check --message-format=json` package ids) after `cargo check --frozen --workspace --keep-going --target <triple>`; generated `OUT_DIR` files count when listed. Spec/proof functions (the first-party `proof-code` rule) are subtracted line by line. | The usual case: drops tests, benches, examples, other-OS modules and Verus ghost functions without needing a binary. Runs the workspace's build scripts and proc macros. |
| `linked-lines` (`--binary`) | Distinct `(file, line)` pairs in the binary's DWARF line table whose file lies under the package source directory (longest prefix wins; a registry `name-version/` path component also matches). On Mach-O, an adjacent `.dSYM` or a fresh `dsymutil` output is used only when its UUID matches the binary. Unit: linked lines, not SLoC. | Measuring a shipped binary after dead-code elimination, inlining and monomorphization. Build it with debug info. |
| `whole-crate` | Every `.rs` file under the package source directory. | Comparison with earlier releases, or when compiling the target is not acceptable. With `--ghost-lines runtime`, output is v1.4.2's, including v1.4.2 first-party categories (no structural test detection). |

Fail closed: a dependency whose evidence is missing keeps its whole-crate size,
gets `size_mode: whole-crate (fallback)` with the reason, and is listed under
**SIZE FALLBACKS** at the top of the report. Compiled-files falls back when
`cargo check` cannot run (including when the toolchain lacks the target's
standard library; then `cargo check` is not started), produced no library
artifact for the package, or the `.d` file or a listed source is missing or
unreadable. Linked-lines falls back for every dependency when the binary is
unreadable or has no line rows for first-party sources (built without debug
info), and for one dependency when a root-manifest profile override disables its
debug info, or when it has no line rows but the symbol table is stripped or
contains symbols from its library crate. A dependency with no line rows and no
symbols counts 0 and says so. Inline cfg-disabled items inside compiled files
still count in compiled-files mode. Dependencies sized from `dependency_sources`
have no package id and fall back in both new modes.

Not counted in any mode: only `.rs` files of cargo packages count. Native code a
package compiles (C or assembly through a build script) and a statically linked C
runtime (musl) are outside all three figures. In linked-lines, such a package counts
0 when its native debug paths lie outside its source directory and it defines no
Rust symbols (aws-lc-sys remaps its paths to `/aws-lc/...`); 0 then means no Rust
lines, not no code. The aarch64 Linux measurements report those native line counts
separately ([survey](survey.md), [scratch-server](scratch-server.md)).

**Structural test detection** (compiled-files and linked-lines). A first-party
function is `test-code` when it has a test attribute (`#[test]`, any path ending
in `::test`, `bench`, `test_case`, `rstest`, `wasm_bindgen_test`, `quickcheck`,
`proptest`), when an enclosing item at any depth has a `#[cfg(...)]` that only
holds under `test` (`test`, `all(.., test, ..)`, an `any` of such terms), when it
is in an out-of-line module declared under such a cfg (`#[cfg(test)] mod name;`,
followed by module path; `#[path]` is not followed), or when it is in an
integration-test or bench target (`tests/`, `benches/`, or a test/bench target
source; cargo builds these with `--test`, which sets `cfg(test)`). Examples stay
runtime. Test code stays in the reachability graph, is excluded from runtime
totals, and is reported as a TEST CODE line and `summary.test_code`.

**Explicit inputs (config schema 3).** `[[security.include]]` and
`[[security.exclude]]` take `pattern` and a required non-empty `reason`;
`[dependencies."name"]` or `[dependencies."name@version"]` take `security`
and/or `sloc` and a `reason`. Default rules apply first, then includes, then
excludes (exclude wins). A pattern or dependency key that matches nothing, a bare
name with two versions in the closure, and a dependency entry without `security`
or `sloc` are errors. Schema-2 files (flat `security`/`not_security` arrays) keep
their v1.4.2 meaning with one `override has no reason` warning per entry.
`formal-coverage migrate-config OLD.toml [--output NEW.toml]` writes schema 3 with
`reason = "TODO"` (comments are not carried over), and any reason containing the
word TODO fails `gate`.

**Two figures.** Every SECURITY-RELEVANT headline line is printed `(with inputs)`
and `(default rules only)`: the same analysis with every explicit input undone
(units keep `default_security` in JSON when an input moved them; dependencies
keep `default_security` / `default_sloc`). JSON has `summary` and
`summary_default_rules`. The Inputs section lists each input's kind, key,
direction, reason, matches and the runtime SLoC it moved into or out of the
security side (credited to the last matching input, which decided the unit), plus
any dependency size change. `gate` thresholds use the with-inputs figure;
`gate --max-override-shift PCT` fails when first-party security proven with
inputs and under default rules only differ by more than PCT percentage points.

**Regression against v1.4.2.** `scripts/v15_replay.py` replays spec-lemmas (the
bundle case and a host-metadata export) and Vest, Anvil and Firecracker from the
v1.4.2 evidence bundle; `scripts/v15_regression.py BASELINE CURRENT` compares a
v1.4.2-binary replay with a v1.5 `--size-mode whole-crate` replay that uses the
unchanged v1.4.2 config (`fixtures/v1.5/v1.4.2-scratch-server.toml`). It removes only
these new constructs and then requires byte equality: JSON keys `size_mode`,
`size_profile`, `binary_sha256`, `size_fallbacks`, `config_schema`, `inputs`,
`summary_default_rules`; dependency keys `size_mode`, `size_reason`,
`whole_crate_sloc`, `default_security`, `default_sloc`, `default_size_unknown`,
`override_reason`; unit key `default_security`; `schema` 3 → 2; warnings starting
`override has no reason: `; Markdown line 1 and its blank line, `(default rules
only)` lines, the ` (with inputs)` label, the `## Inputs` section and the matching
warning bullets. All five cases compare equal (see
[scratch-server](scratch-server.md) and [survey](survey.md) for the v1.5 numbers).
`scripts/v15_replay.py --container NAME [--target TRIPLE]` runs a Linux build of
the tool through `docker exec` in a container that mounts the repository at `/fc`,
so `cargo metadata`, `cargo check` and build scripts run on Linux; the container
prefix is removed from reports like the host prefix. `scripts/v15_size_check.py`
checks per-dependency ordering between modes (compiled-files ≤ whole-crate,
linked-lines ≤ compiled-files, fallbacks labelled) and equality against a
reference run. The published v1.5 survey and scratch-server numbers were measured
this way on aarch64 Linux.
The release checkers (`check_survey.py`, `check_dogfood.py`, `check_perf.py`)
verify the tool sources recorded in the v1.4.2 bundle and therefore report a tool
mismatch for v1.5 sources until a v1.5 bundle exists.

## Inline ghost lines and proof cost (v1.6)

`--ghost-lines separate|runtime` records its choice on Markdown line 1 and in
JSON `ghost_lines`. `separate` is the default. Within a first-party Verus runtime
unit, the token walker recognizes signature `requires`, `ensures`, `recommends`
and `decreases` clauses; `while`, `loop` and `for` clauses (`invariant`,
`invariant_except_break`, `ensures`, `decreases`); complete `proof { ... }` blocks;
`assert(...)`, `assert(...) by { ... }`, `assert forall ... by { ... }`,
`assume(...)`, and `let ghost` / `let tracked` statements. A line moves only
when **every non-comment token**, including delimiters and semicolons, belongs
to these ghost constructs. Mixed lines stay runtime. Comments and strings do
not establish constructs; arbitrary macro inputs and uncertain syntax stay
runtime. Non-Verus units are unchanged. Unparseable first-party files retain
every nonblank line as runtime, are listed in `unparseable_files`, and have an
explicit file-scope hazard because their escape scan is unconfirmed.

Each unit has `runtime_sloc` and `inline_proof_sloc` (zero for standalone proof
and structural test units), plus mode-independent `ghost_sloc` for the
counterfactual denominator. `sloc` continues to drive runtime status totals.
In `runtime` mode inline proof is zero and ghost lines remain runtime. No source
bytes, evidence eligibility, receipts, trust/escape classification or security
rules change between modes. Proven runtime figures under both modes use exactly
the same accepted evidence. Dependencies retain the v1.5 sizing rules.

JSON `proof_ratio` contains the first-party security headline:
`standalone_proof_sloc`, `inline_proof_sloc`, their sum `proof_sloc`,
`proven_runtime_sloc`, `proven_bounded_runtime_sloc`, `ratio` (null at zero proven),
`range: {min, max, source, reason}`, `status`, and `gate_enabled`. It also contains
`all_first_party`, `crates`, `modules` and `runtime_proven_modes`. Crate/module
rows cover all first-party units in that scope; the security headline includes
only security-relevant units. Root proof units without a module appear in the
`<unattributed>` module row and remain in crate/overall totals. Proof lines count
as inventory regardless of proof status; the denominator requires unbounded
accepted `proven` status. Bounded runtime is separate and never added to it.

The default range is inclusive 1.0:10.0; config `[proof_ratio]` needs at least
one finite nonnegative bound, ordered when both exist, and a non-empty reason.
CLI `--proof-ratio-range MIN:MAX` takes precedence, retaining the configured
input and its reason in Inputs. A TODO config reason still fails the gate when
overridden or when `--no-proof-ratio-gate` is used. Below/above security ratios
fail the gate unless disabled; `n/a` and module warnings do not. Range checks
use the unrounded ratio; Markdown displays two decimals. This measures cost,
not quality: `ensures true` can need zero proof statements.

`scripts/v16_regression.py BASELINE CURRENT` requires exact v1.5 JSON/Markdown
bytes after removing only enumerated v1.6 fields/additions. It checks the same
five cases as `scripts/v15_regression.py`, with unchanged configs,
`--size-mode whole-crate --ghost-lines runtime`. See the script's explicit
field list and the v1.6 progress record for local evidence.

## hax/F* receipts (v1.7)

Repeat `--fstar-log LOG`. The adjacent `LOG.evidence.json` has tool
`hax-fstar`, `hax_version`, full `hax_commit`, `fstar_version`,
`extraction_command` (`["python3","hax-driver.py","extract-fstar"]`),
`extraction_exit_code`, `files` (Rust tree hashes including every inventoried
source, Cargo.toml, Cargo.lock and driver), `fresh`, `checked`, `patches`
(path-to-SHA256 maps), `log_sha256`, and `modules`. Artifact paths are relative
to the log directory and cannot leave it. Every F* sibling must be hashed.
Each module records `file`, complete `command`, `exit_code`, `outcome`
(`success`, `failed`, `timeout`, `skipped`). Only success with exit zero can
credit that module. The closed F* grammar accepts `fstar.exe`, one hashed input,
and repeated distinct `--include` directories containing hashed inputs.
All other flags, including lax, admission and loading checked caches, reject
the receipt. No success is inferred from extraction or dependency checks.

`proven-extracted` remains separate from Verus `proven` and Kani
`proven-bounded`. Every associated headline explicitly trusts hax. Receipts
are caller-supplied evidence, not attestations; they cannot establish that the
caller actually executed the recorded commands in a clean environment.

Mapping uses exact, unique `crate::module::function` names obtained by
lowercasing hax's dotted module path and retaining the function name. Unsupported
impl/mangled/recursive names are disclosed as unmapped, never guessed. Credit
requires a checked implementation, an identical fresh `.fst` and (when present)
`.fsti`, and a recognized contract. Any module/interface byte difference trusts
all definitions of that module; this conservative rule may undercredit functions
whose own bodies were unchanged. Interface-only vals and `assume val` are trusted;
`assume`, `admit`, `admitP` are also reported as hazards. Rust hax opaque and
replacement annotations remain explicit syntactic trust, including inactive cfgs.

Contract classification comes from the checked interface: `Tot` or an explicit
True Pure postcondition is `safety-only`; a meaningful `ensures` is `functional`;
unknown forms are `unclassified` and receive no credit. Functional contracts
mentioning local function names without an external spec namespace receive the
**self-consistency heuristic** flag. This is a syntactic warning, not a theorem
about specification intent. Every definition shows its postcondition and mapping.
