# v1.8.5 decisions and blockers

## 2026-11-24 — implementation scope

Existing untracked v184 milestone logs and v185/run.log are left untouched. Internal inventory metadata will preserve inherent impl spans without changing serialized control reports. Numbered groups combine declarations from checked sibling files in each F* module; malformed module text fails toward no mapping. No source-order inference. Connected memory tools are unavailable in this session; no memory search or diary write can be performed.

## 2026-11-24T22:24:27.354597+00:00 — conservative mapping details

Impl identity includes line and column endpoints, so same-line blocks remain distinct. Qualified/generic inventory identities unsupported by the existing module mapping receive no numbered credit. A duplicate direct Rust identity cannot fall back to numbered matching. Mapping reasons stay inside the existing Markdown table cell. Initial clippy failures (tuple migration and owned comparison) were fixed; passing reruns are in PROGRESS.md.

## 2026-11-24T22:25:28.891389+00:00 — replay build profile

Stopped the initial debug replay before comparisons to include the final M1 refinements. Baseline and candidate will both use release builds for replay, reducing dependency-inventory CPU time. This changes neither report options nor evidence. The final acceptance run recreates target/v185 from empty.

## 2026-11-24T22:26:38.752642+00:00 — final identifier audit

Type-token guards treat F* apostrophes as identifier characters; a suffixed t_Type token cannot satisfy the guard. Numbered identifiers with unsupported apostrophe continuations fail closed instead of using the shared parser’s truncated prefix. New rejection tests pass. Restarted preliminary replay before comparisons for these final conservative checks. No (b)/(c) check has failed.

## 2026-11-24T22:28:51.691475+00:00 — CI baseline reproduced

The rebuilt v1.8.4 tag on the host target reproduces every full unit in the staged Linux CI report exactly (448 units, no normalization). Security runtime is 5,078 SLoC; proven-extracted is 3,776. Staged source, fresh/checked/patch hashes and original CI receipt/log bytes validated.

## 2026-11-24T22:30:51.201888+00:00 — replay unit identity

Bertie has cfg alternatives sharing unit ids (including aead_encrypt/decrypt); an id-only dictionary would collapse distinct source units and falsely report changes. Comparison keys use (id, file, line, end_line) and require identical inventories and full unchanged fields outside status/proof_scope. The separate-mode pair has 39 actual changed source units, all expected tier promotions. This is a comparison identity correction, not a mapping-rule change; no (b)/(c) failure occurred.

## 2026-11-24T22:32:16.967377+00:00 — M2 verified

Final empty-output replay exited 0: (a) full CI-unit equality, (b) every changed unit explained by a numbered mapping, (c) no tier loss, (d) all 16 controls byte-identical JSON and Markdown. Each ghost mode promotes 39 source units. Separate mode: 3,776 → 4,027 proven-extracted SLoC (+251), denominator 5,078, 79.30%. Runtime mode: 4,091 → 4,382 (+291), denominator 5,448, 80.43%. The expected line estimate was a guide, not a target. No tier, sizing, evidence or shipping rule was adjusted to fit the results.

All six requested commands exited 0, current stable with RUSTUP_TOOLCHAIN unset; version stdout is formal-coverage 1.8.5. No hax/F* execution, push or tag. Full changed-unit tables are in compare.md. Independent Andrew/Dudley acceptance remains outstanding.
