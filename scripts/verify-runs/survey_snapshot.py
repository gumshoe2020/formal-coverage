#!/usr/bin/env python3
"""Write the local v1.4 survey snapshot after all project summaries exist."""
from datetime import datetime
import json
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_runs as v


def main():
    path = v.REPO / 'docs/survey/results.json'
    old = json.loads(path.read_text())
    historical = old.get('historical_v1_1_c', old)
    rows = [json.loads((v.RUNS / name / ('release-summary.json' if name == 'firecracker' else 'summary.json')).read_text()) for name in ['vest', 'anvil', 'firecracker']]
    assert all('pending' not in row.get('verifier_outcome', {}).get('counts', {}) for row in rows)
    document = dict(version='1.4', measured_at=datetime.now().astimezone().isoformat(),
                    exports=str(v.EXPORTS), projects=rows, historical_v1_1_c=historical)
    v.write(path, document)
    text = '''# Real-world survey: v1.4

Real Docker verifier attempts on the unchanged archived sources, measured
2026-10-28–29. Exact pins, commands, timings, exits and hashes are in the
[results snapshot](survey/results.json) and committed
[run manifests](../runs/v1.4/). Full logs, source-bound receipts, dependency
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
'''
    text += '\n'.join(v.outcome_row(row['project'], row) for row in rows) + '\n'
    text += '''
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

'''
    failures = [h for h in rows[2]['verifier_outcome']['harnesses'] if h['outcome'] not in ['pass', 'not-compiled-on-aarch64']]
    for h in failures:
        text += f'- `{h["id"]}`: **{h["outcome"]}** — {h["reason"]}.\n'
    text += '''
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

## Dependency sizes: before → after

Fetch-only containers populated separate project Cargo homes; host reports read
those caches offline without executing target code. Vest and Firecracker use
archived lockfiles; Anvil uses the explicitly recorded generated lock. All
three now use **Cargo metadata (runtime)**, rather than lockfile fallback, with
**zero unknown runtime or build-time source sizes**. The prior lockfile inventory
and the new target-filtered runtime inventory have different package sets.

| Project | Prior lockfile packages / unknown sizes | Runtime packages / SLoC after | Build-time packages / SLoC after |
| --- | ---: | ---: | ---: |
'''
    text += '\n'.join(v.dependency_row(row) for row in rows) + '\n'
    text += '''
Vest dependency runtime SLoC changes **0 known (207 unknown packages) →
1,035,488 measured (121 resolved runtime packages)**. Its first-party proof
numerator and denominator are preserved. Headline reliance includes the newly measured
dependency inventory; receipt-scope reliance is reported separately above. Build-time
packages remain a separate trusted supply-chain inventory; dev-only packages
are excluded where Cargo confirms the graph. Features can still be unified by
Cargo, and package inventories include cfg/test/platform-specific source.

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
'''
    (v.REPO / 'docs/survey.md').write_text(text)
    dependencies = [{k: row[k] for k in ['project', 'target', 'closure', 'closure_reason',
                    'dependency_before', 'dependency_count', 'dependency_sloc', 'unknown_dependencies',
                    'build_dependency_count', 'build_dependency_sloc', 'build_unknown_dependencies',
                    'dev_dependencies_excluded']} for row in rows]
    v.write(v.REPO / 'docs/survey/v1.4-dependencies.json', dependencies)


if __name__ == '__main__':
    main()
