# v1.12.1 offline consumer-flow replay

Reproduce: `cargo build --locked && python3 dev/v1121/consumer_flow.py . target/v1121/flow`
(scratch under target/, local bare origin, no network). Commit SHAs differ per run.
"new status entries" are `git status --porcelain --ignored -uall` lines a step added
(`??` untracked, `!!` ignored). `uses:` steps (cache, upload-artifact) are not replayed:
the cache path is runner.temp and uploads only read.

### committed / clean
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 0; new status entries: ['!! target/.rustc_info.json', '!! target/coverage-ci/badges/proof-ratio.json', '!! target/coverage-ci/badges/proven.json', '!! target/coverage-ci/badges/ratchet.json', '!! target/coverage-ci/badges/tested.json', '!! target/coverage-ci/badges/unverified.json', '!! target/coverage-ci/diff.patch', '!! target/coverage-ci/floor.json', '!! target/coverage-ci/report.json', '!! target/coverage-ci/report.md', '!! target/coverage-ci/summary.md', '?? logs/ci-kit/coverage.log']
  ```
  None matched.
  
  ## Run receipts
  
  
  ## Evidence and limitations
  
  - No entry points: all first-party functions conservatively security-relevant.
  - closure: lockfile (fallback): cargo metadata: error: cannot create the lock file $SCRATCH/committed/clean/workspace/Cargo.lock because --frozen was passed to prevent this help: to generate the lock file without accessing the network, remove the --frozen flag and use --offline instead.. Entire prior lockfile closure retained; build/dev/optional exclusions unconfirmed.
  
  coverage baseline and document match measurement
  ratchet PASS
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: skipped (failure())
- step `Stage badges including failed ratchets`: exit 0; new status entries: none

### committed / tracked-edit
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 1; new status entries: ['?? logs/ci-kit/coverage.log']
  ```
  coverage FAIL: uncommitted changes in $SCRATCH/committed/tracked-edit/workspace; commit them first: the ratchet compares 83b61e30d5bc0a66e1ca2bb219c5ad8a73a6e9b7 to HEAD
    src/lib.rs
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: exit 0; new status entries: none
- step `Stage badges including failed ratchets`: exit 0; new status entries: ['!! target/coverage-ci/badges/proof-ratio.json', '!! target/coverage-ci/badges/proven.json', '!! target/coverage-ci/badges/ratchet.json', '!! target/coverage-ci/badges/tested.json', '!! target/coverage-ci/badges/unverified.json']

### committed / untracked-rs
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 1; new status entries: ['?? logs/ci-kit/coverage.log']
  ```
  coverage FAIL: uncommitted changes in $SCRATCH/committed/untracked-rs/workspace; commit them first: the ratchet compares 83b61e30d5bc0a66e1ca2bb219c5ad8a73a6e9b7 to HEAD
    src/new_parser.rs
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: exit 0; new status entries: none
- step `Stage badges including failed ratchets`: exit 0; new status entries: ['!! target/coverage-ci/badges/proof-ratio.json', '!! target/coverage-ci/badges/proven.json', '!! target/coverage-ci/badges/ratchet.json', '!! target/coverage-ci/badges/tested.json', '!! target/coverage-ci/badges/unverified.json']

### recorded / clean
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 0; new status entries: ['!! target/.rustc_info.json', '!! target/coverage-ci/badges/proof-ratio.json', '!! target/coverage-ci/badges/proven.json', '!! target/coverage-ci/badges/ratchet.json', '!! target/coverage-ci/badges/tested.json', '!! target/coverage-ci/badges/unverified.json', '!! target/coverage-ci/diff.patch', '!! target/coverage-ci/floor.json', '!! target/coverage-ci/record/COVERAGE-MEASURED.md', '!! target/coverage-ci/record/baseline.json', '!! target/coverage-ci/record/branch', '!! target/coverage-ci/record/sha', '!! target/coverage-ci/report.json', '!! target/coverage-ci/report.md', '!! target/coverage-ci/summary.md', '?? logs/ci-kit/coverage.log']
  ```
  
  None matched.
  
  ## Run receipts
  
  
  ## Evidence and limitations
  
  - No entry points: all first-party functions conservatively security-relevant.
  - closure: lockfile (fallback): cargo metadata: error: cannot create the lock file $SCRATCH/recorded/clean/workspace/Cargo.lock because --frozen was passed to prevent this help: to generate the lock file without accessing the network, remove the --frozen flag and use --offline instead.. Entire prior lockfile closure retained; build/dev/optional exclusions unconfirmed.
  
  ratchet PASS
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: skipped (failure())
- step `Stage badges including failed ratchets`: exit 0; new status entries: none

### recorded / tracked-edit
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 1; new status entries: ['?? logs/ci-kit/coverage.log']
  ```
  coverage FAIL: uncommitted changes in $SCRATCH/recorded/tracked-edit/workspace; commit them first: the ratchet compares 1e015d4f327217fb00374aca9b350446a03cb51a to HEAD
    src/lib.rs
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: exit 0; new status entries: none
- step `Stage badges including failed ratchets`: exit 1; new status entries: none

### recorded / untracked-rs
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 1; new status entries: ['?? logs/ci-kit/coverage.log']
  ```
  coverage FAIL: uncommitted changes in $SCRATCH/recorded/untracked-rs/workspace; commit them first: the ratchet compares 1e015d4f327217fb00374aca9b350446a03cb51a to HEAD
    src/new_parser.rs
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: exit 0; new status entries: none
- step `Stage badges including failed ratchets`: exit 1; new status entries: none

### false-pass / false-pass-v1.12.0
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 0; new status entries: ['!! target/.rustc_info.json', '!! target/coverage-ci/badges/proof-ratio.json', '!! target/coverage-ci/badges/proven.json', '!! target/coverage-ci/badges/ratchet.json', '!! target/coverage-ci/badges/tested.json', '!! target/coverage-ci/badges/unverified.json', '!! target/coverage-ci/diff.patch', '!! target/coverage-ci/floor.json', '!! target/coverage-ci/report.json', '!! target/coverage-ci/report.md', '!! target/coverage-ci/summary.md', '?? logs/ci-kit/coverage.log']
  ```
  None matched.
  
  ## Run receipts
  
  
  ## Evidence and limitations
  
  - No entry points: all first-party functions conservatively security-relevant.
  - closure: lockfile (fallback): cargo metadata: error: cannot create the lock file $SCRATCH/false-pass/false-pass-v1.12.0/workspace/Cargo.lock because --frozen was passed to prevent this help: to generate the lock file without accessing the network, remove the --frozen flag and use --offline instead.. Entire prior lockfile closure retained; build/dev/optional exclusions unconfirmed.
  
  coverage baseline and document match measurement
  ratchet PASS
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: skipped (failure())
- step `Stage badges including failed ratchets`: exit 0; new status entries: none

### false-pass / false-pass-v1.12.1
- step `source`: exit 0; new status entries: none
- step `Build pinned formal-coverage`: exit 0; new status entries: ['?? logs/ci-kit/build.log']
- step `Fetch recorded floor (floor "recorded" only)`: exit 0; new status entries: none
- step `measure`: exit 1; new status entries: ['?? logs/ci-kit/coverage.log']
  ```
  coverage FAIL: uncommitted changes in $SCRATCH/false-pass/false-pass-v1.12.1/workspace; commit them first: the ratchet compares 6a6b4b70d7c936571b119605adee8e542a8d327a to HEAD
    src/lib.rs
  ```
- step `ratchet`: exit 0; new status entries: none
- step `Record coverage failure in job summary`: exit 0; new status entries: none
- step `Stage badges including failed ratchets`: exit 0; new status entries: ['!! target/coverage-ci/badges/proof-ratio.json', '!! target/coverage-ci/badges/proven.json', '!! target/coverage-ci/badges/ratchet.json', '!! target/coverage-ci/badges/tested.json', '!! target/coverage-ci/badges/unverified.json']
