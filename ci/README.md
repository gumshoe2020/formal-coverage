# Coverage CI kit

Requires Linux, Bash, Git, Python 3.9+ and stable Rust/Cargo. Python uses only the
standard library. The caller checks out its project with full history, installs
its prover and dependencies, and generates fresh source-bound receipts. This kit
contains no prover setup. Pin both actions by the same full public commit SHA.
The coverage action builds that action repository's Rust sources, resolved from
`github.action_path`, so one pin selects both the kit and measurement tool.

## Configuration

Copy [project.example.json](project.example.json) to `coverage/ci.json`. It is
JSON schema 1, distinct from the tool's `formal-coverage.toml`. Unknown keys and
invalid values fail. Keys may be omitted to retain defaults:

| Key | Meaning/default |
| --- | --- |
| `schema` | `1` |
| `floor` | `committed` (default): floor is `coverage/baseline.json` at the merge base. `recorded`: floor is main's recorded measurement, see [Recorded floor](#recorded-floor) |
| `data_branch` | Recorded-floor branch, default `coverage-data`; not `main`, `master` or `badges` |
| `bootstrap_baseline` | Recorded floor only: project-relative path of a baseline used for the first adoption commit, default none |
| `badges` | Array of endpoint names: `proven`, `tested`, `unverified`, `proof-ratio`, `ratchet` |
| `labels` | Optional overrides of the five displayed badge labels |
| `proven_thresholds` | Descending lower bounds `[90,75,50,25]`: brightgreen, green, yellow, orange; below is red |
| `unverified_thresholds` | Ascending exclusive upper bounds `[10,25,50,75]`: brightgreen, green, yellow, orange; at/above last is red |
| `proof_ratio` | `stage`: `development` or `established`; inclusive ranges `development: [5,15]`, `established: [10,20]`; nonempty `reason` |

Proven bounds may only increase, unverified bounds may only decrease: custom
colours cannot flatter the reference policy. Tested stays blue; ratchet is
lightgrey for off, brightgreen for pass, red for fail. Proof/code is lightgrey for unavailable,
brightgreen inside the selected range, yellow outside, and **never blocking**.
Established module ratios above the established upper bound emit review warnings.
Keep the chosen thresholds and ranges documented in the project's COVERAGE.md.
These settings affect presentation/warnings; they do not change report evidence,
classification or measurement configuration.

## Actions

`coverage` inputs: `workspace` (`.`), `report-args` (JSON string array, `[]`),
`baseline` (`coverage/baseline.json`), `coverage` (`COVERAGE.md`), `config` (optional
project JSON path), `artifact-name` (`coverage-badges`). Baseline, document, config
and receipt paths are relative to `workspace`. Extra report arguments are passed
as argv without shell evaluation. The runner controls `--json` and `--markdown`.
The `enforce-ratchet` input accepts exactly `'true'` or `'false'` (default).
Both modes measure coverage and evaluate the same floor and exceptions. With
enforcement off, stale committed outputs and policy regressions appear in the log
and summary with the would-be result. Measurement errors and invalid floors
still fail the job. Set `enforce-ratchet: 'true'` to block on policy failures.

Outputs: `ratchet` (`not-enforced` when off, otherwise success/failure/skipped) and
`artifact-name`. It uploads endpoints plus a separate `<artifact-name>-report`
artifact containing report JSON/Markdown, summary, floor, diff and kit logs.
On an enforced failure it stages a red ratchet from the committed baseline if readable;
those other badge numbers describe that baseline, not a successful new run.
Do not use the ratchet output as a substitute for the whole job result.
Use a separate job per project; output paths are fixed within a job.

`publish-badges` inputs: `artifact-name` and required `coverage-result`
(`${{ needs.coverage.result }}`). No outputs. Requires `contents: write` on its
job and only uses `github.token` (GITHUB_TOKEN). It independently rejects events
other than push to main, validates the downloaded endpoint files, and marks the
enforced ratchet red for any coverage job failure. Git work happens in an isolated temp
directory; only `refs/heads/badges` is updated, using force-with-lease to reject
concurrent overwrites. Main and the caller's working tree are never modified.
No missing artifact or network failure is silently ignored.

## Recorded floor

Opt in with `"floor": "recorded"` in `coverage/ci.json`. The default `committed`
mode is unchanged. Ratchet rules and exact-rational arithmetic are identical; only
the floor's source differs, and no numbers are committed in PRs.

The data-branch fetch handles credentials persisted by `actions/checkout`. It
resets extra headers before setting the job token, so Git sends one Authorization
header. The workflow template recommends `persist-credentials: false` to avoid
leaving a credential in the checkout.

- **Floor.** For a PR, the measurement recorded for its merge-base commit, read
  from `baselines/<full-sha>.json` on the data branch (same schema as
  `baseline.json`). The coverage action fetches the branch first. If the file is
  missing (that main commit was never measured, or predates adoption), or is
  unreadable or not a schema-1 baseline, the run fails closed with a message naming the
  commit.
- **Bootstrap.** For the first adoption only, set `bootstrap_baseline` to a path
  (for example the project's last committed `coverage/baseline.json`). It is read
  from the merge-base commit, not the PR's tree, and is used only while the data
  branch holds no `baselines/*.json`. After the first recording it is ignored, so
  remove it then.
- **No committed numbers.** The action does not check `coverage/baseline.json` or
  the measured section of `COVERAGE.md`; delete them and link to
  `COVERAGE-MEASURED.md` on the data branch. Prose and the `## Exceptions` table in
  `COVERAGE.md` stay in force.
- **Recording.** On push to main, the `publish-baseline` action writes
  `baselines/<sha>.json` and a rendered `COVERAGE-MEASURED.md` to the data branch.
  It needs `contents: write` on its job, uses only `GITHUB_TOKEN`, pushes only the
  data branch and never forces. An existing sha file is never overwritten: identical
  content is a no-op, different content is an error. Run it even when the coverage
  job failed (`if: always()`, as for badges): main's real state must become the
  next floor, in either enforcement mode. If measurement itself
  failed there is no artifact and the job fails.
- **Exceptions.** Unchanged: an exact commit in the PR's range with approver,
  reason and restoration obligation in `COVERAGE.md` `## Exceptions`. The commit
  is recorded like any other, so the next floor is the post-exception state.
- **Concurrency.** Each PR is compared with its own merge base. After two PRs
  merge, main's push run measures the combination and, when enforcement is on, fails if it breaks
  the ratchet against the previous recorded sha. To avoid that, enable "Require
  branches to be up to date" or a merge queue; the kit never changes repository
  settings.

Add this job beside `publish-badges`; the coverage job and `artifact-name` are
the same. The `coverage` job's `permissions` stay `contents: read`.

```yaml
  publish-baseline:
    needs: coverage
    if: always() && github.event_name == 'push' && github.ref == 'refs/heads/main' && needs.coverage.result != 'cancelled'
    runs-on: ubuntu-latest
    timeout-minutes: 10
    permissions:
      contents: write
    steps:
      - uses: gumshoe2020/formal-coverage/.github/actions/publish-baseline@FORMAL_COVERAGE_COMMIT_SHA
        with:
          artifact-name: coverage-badges
```

Adopting: set `floor`/`bootstrap_baseline`, merge, let the first main push record,
then delete `bootstrap_baseline`, `coverage/baseline.json` and the measured section.

## Rules and local use

The committed baseline and measured section of COVERAGE.md must match the new
report exactly. The baseline at the PR merge base (or push's `before` commit) is
the floor. Unverified security lines cannot increase; proven share cannot drop.
A diff intersecting security-runtime spans in either inventory must increase
proven share or, at equal share, increase the count of proven security runtime
lines with a functional contract (both functional depth categories combined).
The depth comparison requires depth in both the floor and the measured report.
In recorded mode, main must first write a record with this version and depth
data, and the PR must use that commit's record as its floor.
Docs/CI/test-only changes are exempt from strict increase,
not from the other two rules. Deletions and pure source renames count. Diffs use
`git diff -U0`; shares use exact rational comparisons.

Only an exact full SHA in the checked commit range, a named approving maintainer,
a reason and restoration obligation in `## Exceptions` can waive those ratchet
rules. Exceptions cannot waive stale/edited baseline checks or incomplete
inventories. Human review authenticates approval and enforces restoration. The
project must enable up-to-date PR checks to serialize competing floor changes.
Missing base history or a missing base baseline fails closed; install the initial
reviewed baseline before enabling this workflow. CI never commits to main.

The writer preserves all text outside these markers (create a document containing
both markers and a `## Exceptions` table before first use):

```markdown
<!-- measured coverage: begin -->
<!-- measured coverage: end -->

## Exceptions

| Commit | Approving maintainer | Reason | Restoration obligation |
| --- | --- | --- | --- |
```

From the project directory, after generating receipts with the project's prover:

```sh
/path/to/formal-coverage report . --fstar-log target/fstar.log --json report.json
python3 /path/to/kit/ci/write-coverage.py report.json  # developer updates, review and commit
python3 /path/to/kit/ci/run.py --tool /path/to/formal-coverage \
  --workspace . --report-args '["--fstar-log","target/fstar.log"]' \
  --config coverage/ci.json --base "$(git merge-base origin/main HEAD)" \
  --output target/coverage-ci > coverage-ci.log 2>&1
```

`report` itself has no ratio gate; every nonzero report exit blocks. The kit never
retries a failed report with audits disabled. The native ratio is displayed
unchanged. `proven` here sums `proven` and `proven-extracted` only; hax trust stays
explicit. Trusted, hazardous, bounded and upstream-only lines remain unverified
for this ratchet. Tested means syntactically referenced, not executed coverage.

Run `python3 -m unittest discover -s ci/tests -v` for kit tests and
`python3 ci/lint-actions.py` for actionlint. Actionlint accepts workflow files,
not composite metadata: the adapter lints CI and the README workflow directly,
and wraps each composite's unchanged steps in a workflow with declared inputs.
It checks their expressions, step references and shell scripts; this is not a
GitHub-hosted execution test. Publishing must be exercised after review/release.

The metric, ratchet, writers and original tests were adapted from bertie-tls
`26b54ae` (Apache-2.0); see THIRD_PARTY_NOTICES.md. New runner/actions do not alter
Rust measurement behavior. See the root README for a copy-paste workflow.

## Opt-in per-module F* reuse (v1.12.0)

Default evidence rules and full main checks are unchanged. A `.checked` file is
never evidence. Main must still freshly extract and check every root strictly,
without a project proof cache. PRs always freshly extract and run `fstar.exe
--dep full` with the same includes and roots as their checks. The integration
must retain a current source-bound `LOG.evidence.json`, including when no root
needs a fresh check (then `modules` is empty and LOG may be empty).

**Trust added:** historical receipts on the data branch were produced by main's
strict job. Protect write access to that branch and the workflow/artifact chain.
These are caller-supplied evidence, not signed execution attestations. Fresh
extraction, dependency analysis, tool identities and their truthful execution
remain caller responsibilities, as with existing run evidence. A PR cannot
publish receipts. Never ingest artifacts from an untrusted PR into a main
publisher. Pin the kit/tool/action to one reviewed full SHA.

`ci/receipts.py context` turns the fresh `--dep full` make output into
`LOG.context.json` beside the current extraction. Supply `--commands` as a JSON
list of `{ "file": "fresh/Project.Module.fst", "command": ["fstar.exe", ...] }`
for **all** requested implementation roots, and `--toolchain` as a JSON object:

```json
{
  "fstar": "2025.10.06@sha256-of-fstar-executable-and-distribution-identity",
  "z3": "4.13.3@sha256-of-z3-executable",
  "hax": "0.4.2@FULL_HAX_COMMIT@sha256-of-hax-executable"
}
```

Use identities from the installed pinned tools, not values copied from old
receipts. F*'s identity must start with the run receipt's version plus `@`; hax's
must start with its version, `@`, full commit and `@`. Record all remaining
execution identity (including wrappers/distribution changes) in those strings.
Every string and every command argument must match exactly to reuse a check.
The accepted argv grammar is unchanged: `fstar.exe`, unique `--include DIR`
pairs and exactly one explicit root; no hints, cache, lax or admission switches.

The closure recursively follows every `.checked` target to **source** files;
these target names are graph edges only. Includes do not become evidence.
Interfaces, project dependencies, hax libraries and F* standard-library files
are all hashed. Unknown edges, duplicate targets, cycles, missing sources and
unmapped paths fail closed. Paths containing spaces/make escapes are unsupported
and fail closed. Relative paths are rooted beside LOG. Absolute project paths
are normalized there. Use stable-order `--external /virtual/prefix=/local/prefix`
for paths outside that directory; sources are copied under `external/0/`, etc.
For Docker `/tools` this maps to the read-only host toolchain. Never copy cached
artifacts. Keep the same mappings/order on main and PRs. Run the helper after
extraction and libraries exist, before selecting checks:

```sh
python3 KIT/ci/receipts.py context --dep target/fstar/dep.txt \
  --commands target/fstar/commands.json --toolchain target/fstar/toolchain.json \
  --external /tools=/path/to/pinned/tools --output target/fstar/fstar.log.context.json
python3 KIT/ci/receipts.py select --tool KIT/target/release/formal-coverage \
  --workspace . --base MERGE_BASE_FULL_SHA --context target/fstar/fstar.log.context.json \
  --output target/fstar/reuse.json > target/fstar/needed-modules.txt
```

Before selection, fetch `coverage-data` as for recorded floors (use
`fetch_data_branch.py` with the recorded-floor config), or otherwise make its
local `refs/remotes/origin/coverage-data` available. Selection uses only Git
reads; it performs no network access. An absent receipt schedules a fresh check.
Invalid/mismatched historical receipts also schedule fresh checks and name the
reason on stderr. Only matching receipts from
`receipts/<exact-merge-base>/<Module>.json` enter the reuse manifest. No nearest
ancestor fallback. `--branch` selects another valid data branch.

Give the printed full root paths (one per line, empty means zero checks) to the
project's verifier. Its module-list interface must perform fresh extraction
unconditionally, check exactly that list strictly without caches, retain each
`<root-basename>.log`, join only fresh slices into LOG, and write the current
source/extraction receipt with only those fresh outcomes. Dependency context
must describe that same final extraction. Do not interpret an empty list as
"all modules". Projects may instead separate their extraction/check phases.
The kit does not install or run provers.

Pass `--fstar-log LOG --fstar-reuse REUSE.json` through the coverage action's
`report-args` (or directly to report/gate). One log is required in reuse mode.
The Rust tool independently checks schema/module/main commit, exact dependency
path set, every current source hash, tool identities, exact strict command and
the historical success slice, then applies all existing current-source,
extraction, mapping, hazard and proof-cost rules. It rejects unknown fields,
missing interface entries, overlapping fresh/reused outcomes and incomplete
module sets. Invalid reuse returns nonzero, naming the module and field; it is
never silently credited or retried without audits. Markdown/stdout gives a
per-module fresh/reused table and historical main commit. Coverage JSON remains
byte-stable for equal coverage; provenance is in Markdown and the retained
manifest/receipts rather than new JSON coverage fields. Pass an empty reuse
manifest on a fully fresh run to get the same provenance table.

After main's full strict check, prepare the **main-only** artifact:

```sh
python3 KIT/ci/receipts.py prepare --tool KIT/target/release/formal-coverage \
  --workspace . --log target/fstar/fstar.log --context target/fstar/fstar.log.context.json \
  --commit MAIN_FULL_SHA --output target/coverage-receipts
```

Preparation validates the full source-bound strict receipt using the report
command and refuses skipped/reused/failed roots. It hashes the full closures and
requires each retained per-module slice to be contained in the strict combined
log. Upload `target/coverage-receipts/*` as `coverage-receipts` only on push to
main, using the pinned upload-artifact action as in the existing coverage kit.
Add a separate publisher job, beside `publish-baseline`:

```yaml
  publish-receipts:
    needs: coverage
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - uses: gumshoe2020/formal-coverage/.github/actions/publish-receipts@FORMAL_COVERAGE_COMMIT_SHA
        with:
          artifact-name: coverage-receipts
```

The action independently requires push to main and the artifact's SHA to equal
`GITHUB_SHA`. It uses only `github.token` (GITHUB_TOKEN) and appends one JSON per
module under `receipts/<main-sha>/` beside `baselines/`. Identical existing bytes
are a no-op; different bytes are an error. It never overwrites or forces, never
pushes main, preserves other data-branch files, and retries ref races by reading
the updated branch. Keep main runs uncancelled per SHA, as for baseline records.
The first PR without receipts simply checks every root freshly.

`check-fstar-receipt --context LOG.context.json --receipt MODULE.json` exposes
the exact Rust comparison used both by selection and by report. Use this for
diagnostics; it is not a replacement for validating current source-bound run
evidence. All new Python uses the standard library only.

The schema-1 reuse manifest contains `schema`, `data_branch`, `main_commit` and
`receipts` (map from full root path to receipt object). The report also compares
every reused object with its exact main record read via local Git from the
workspace's data-branch ref. It performs no fetch itself. Copying a receipt into
a manifest and rewriting its hashes/log to match a PR therefore fails the origin
check even if all current hashes happen to match. The selected branch must be
available in the **reported workspace**, not just in the helper's working tree.
