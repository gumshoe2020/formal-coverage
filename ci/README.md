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
| `badges` | Array of endpoint names: `proven`, `tested`, `unverified`, `proof-ratio`, `ratchet` |
| `labels` | Optional overrides of the five displayed badge labels |
| `proven_thresholds` | Descending lower bounds `[90,75,50,25]`: brightgreen, green, yellow, orange; below is red |
| `unverified_thresholds` | Ascending exclusive upper bounds `[10,25,50,75]`: brightgreen, green, yellow, orange; at/above last is red |
| `proof_ratio` | `stage`: `development` or `established`; inclusive ranges `development: [5,15]`, `established: [10,20]`; nonempty `reason` |

Proven bounds may only increase, unverified bounds may only decrease: custom
colours cannot flatter the reference policy. Tested stays blue; ratchet is
brightgreen for pass, red for fail. Proof/code is lightgrey for unavailable,
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
Outputs: `ratchet` (measurement/check step outcome: success/failure/skipped) and
`artifact-name`. It uploads endpoints plus a separate `<artifact-name>-report`
artifact containing report JSON/Markdown, summary, floor, diff and kit logs.
On failure it stages a red ratchet from the committed baseline if readable;
those other badge numbers describe that baseline, not a successful new run.
Do not use the ratchet output as a substitute for the whole job result.
Use a separate job per project; output paths are fixed within a job.

`publish-badges` inputs: `artifact-name` and required `coverage-result`
(`${{ needs.coverage.result }}`). No outputs. Requires `contents: write` on its
job and only uses `github.token` (GITHUB_TOKEN). It independently rejects events
other than push to main, validates the downloaded endpoint files, and marks the
ratchet red for any coverage job failure. Git work happens in an isolated temp
directory; only `refs/heads/badges` is updated, using force-with-lease to reject
concurrent overwrites. Main and the caller's working tree are never modified.
No missing artifact or network failure is silently ignored.

## Rules and local use

The committed baseline and measured section of COVERAGE.md must match the new
report exactly. The baseline at the PR merge base (or push's `before` commit) is
the floor. Unverified security lines cannot increase; proven share cannot drop.
A diff intersecting security-runtime spans in either inventory must strictly
increase proven share. Docs/CI/test-only changes are exempt from strict increase,
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
