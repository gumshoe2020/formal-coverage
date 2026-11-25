# formal-coverage: README cleanup (docs only)

Repo: this directory, branch `main`. Docs-only change requested by Andrew on 2026-11-25:
"The intro at the top is good, but we should move the release notes to the changelog (check for
duplication when we do this). We should specify which prover frameworks we support and maybe have
a few simple usage examples."

## What to produce

1. **README.md** restructured, in this order:
   - Title, badges and the existing intro (lines 1–19) unchanged in substance.
   - `## Supported provers`: a short table, one row per framework the tool actually credits, with
     the tier it earns, the CLI flag that supplies evidence, and one line on what is trusted. Ground
     every row in the source (`src/evidence.rs`, `src/kani.rs`, `src/fstar.rs`, `src/hax.rs`,
     `src/main.rs`): expected Verus → proven (`--verus-log`), Kani → proven-bounded
     (`--kani-log`), hax/F* → proven-extracted (`--fstar-log`). Then one line on what is recognised
     but never credited (ProVerif, SSProve/Coq attribute paths: reported as non-F* trust) and that
     anything else (Creusot, Prusti, Aeneas, Lean, etc.) is not supported, if the source confirms
     that. Don't claim anything the source doesn't establish.
   - `## Quick start`: build command, then 4–6 short, copy-pasteable examples, each one line of
     prose plus a code block: a basic report; a report with Verus evidence; with hax/F* evidence;
     `gate` with a threshold; using a `formal-coverage.toml`; pointer to Use in CI. Every command
     and flag must exist (`formal-coverage report --help`, `gate --help`). Run each example against
     an existing fixture or example under `tests/` or `examples/` where possible and record the
     command and exit code in NOTES.md; where an example needs real evidence that isn't in the repo,
     say so in NOTES.md and check the flags with `--help` instead.
   - Reference sections, kept but reorganised under clear topic headings instead of version
     headings (for example: Configuration, Shipped scope, Dependency sizing, Test detection,
     Evidence and receipts, hax/F* specifics, Proof cost ratio, Limitations). Keep all
     still-current reference content; drop only text that is purely "what changed in vX".
     Fix stale wording while moving (e.g. "current stable Rust (1.100.0 for v1.8.4)" → just "current
     stable Rust"; the orphaned "v1.7 adds repeatable `--fstar-log`…" paragraph after the licence
     line belongs in the hax/F* section).
   - `## Use in CI` (existing content), then `## License` last.
   - End with a link line to CHANGELOG.md, docs/, and dev/README.md.

2. **CHANGELOG.md**: one entry per version that the README currently describes (1.9.1, 1.9.0,
   1.8.5, 1.8.4, 1.8.1, 1.8, 1.7.3, 1.7, 1.6), newest first, in the existing changelog style
   (dated heading, short bullets, links to `dev/vNNN/compare.md` and `docs/v1.x.md` where the
   README had them). Dates come from `git log` / tags, not guesses. **Deduplicate:** 1.9.1 and
   1.9.0 already exist; merge anything the README said that they don't, without repeating what
   they already say. A version's behaviour belongs in the changelog once; the README describes
   current behaviour without version labels (a brief "since vX" is fine only where a user needs it
   to pick a version).

## Rules
- Docs only: touch README.md, CHANGELOG.md and dev/readme-cleanup/NOTES.md. No code, tests,
  version bump, or other files. No push, no tag.
- Keep every still-true fact; the reviewer will diff old vs new README+CHANGELOG for lost content.
  In NOTES.md, list every README paragraph you deleted outright (not moved) and why.
- Keep links working: run a relative-link check over README.md and CHANGELOG.md (every linked
  path exists, every `#anchor` matches a heading) and record the result in NOTES.md.
- Plain, short sentences. No marketing language.
- Run `scripts/check-public.sh` is not needed; do not add local paths, names or private terms.
- One commit, message `docs: reorganise README; move release notes to CHANGELOG`, trailer
  `Assisted-by: Codex (GPT-6.1)` or the model you actually are. Then write `README-READY` as the
  last line of NOTES.md and stop.
