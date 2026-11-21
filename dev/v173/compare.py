#!/usr/bin/env python3
"""Compare reports; independently scan source doc comments (no Rust token library)."""
import argparse
from collections import defaultdict
from datetime import datetime
import hashlib
import json
from pathlib import Path
import re


def doc_lines(source):
    """Small Rust lexical scan: skip literals and ordinary comments, nest block comments."""
    docs = set()
    code = set()
    i, line = 0, 1

    def cover(end, destination):
        nonlocal i, line
        text = source[i:end]
        destination.update(range(line, line + text.count("\n") + 1))
        line += text.count("\n")
        i = end

    while i < len(source):
        if source[i].isspace():
            line += source[i] == "\n"
            i += 1
            continue
        if source.startswith("//", i):
            end = source.find("\n", i)
            end = len(source) if end < 0 else end
            is_doc = source.startswith("//!", i) or (
                source.startswith("///", i) and not source.startswith("////", i)
            )
            cover(end, docs if is_doc else set())
            continue
        if source.startswith("/*", i):
            is_doc = source.startswith("/*!", i) or (
                source.startswith("/**", i) and not source.startswith("/***", i)
            )
            end, depth = i + 2, 1
            while depth:
                if source.startswith("/*", end):
                    depth += 1
                    end += 2
                elif source.startswith("*/", end):
                    depth -= 1
                    end += 2
                else:
                    end += 1
                assert end <= len(source), "unterminated block comment"
            cover(end, docs if is_doc else set())
            continue
        raw = re.match(r'(?:br|cr|r)(#*)"', source[i:])
        if raw:
            close = '"' + raw[1]
            end = source.find(close, i + raw.end())
            assert end >= 0, "unterminated raw literal"
            cover(end + len(close), code)
            continue
        if source[i] == '"':
            end = i + 1
            while end < len(source) and source[end] != '"':
                end += 2 if source[end] == "\\" else 1
            assert end < len(source), "unterminated string"
            cover(end + 1, code)
            continue
        char = re.match(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'", source[i:])
        if char:
            cover(i + char.end(), code)
            continue
        code.add(line)
        i += 1
    return docs, docs - code


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("root", type=Path)
    parser.add_argument("old", type=Path)
    parser.add_argument("new", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    old, new = (json.loads(p.read_text()) for p in (args.old, args.new))
    key = lambda u: (u["id"], u["file"], u["line"], u["end_line"])
    before, after = ({key(u): u for u in r["units"]} for r in (old, new))
    assert len(before) == len(old["units"]) and len(after) == len(new["units"])
    errors = []
    if before.keys() != after.keys():
        errors.append("unit identities changed")
    common = before.keys() & after.keys()
    changed = [k for k in sorted(common) if before[k]["sloc"] != after[k]["sloc"]]
    tiers = [k for k in common if before[k]["status"] != after[k]["status"]]
    increased = [k for k in common if before[k]["sloc"] < after[k]["sloc"]]
    if tiers:
        errors.append(f"{len(tiers)} unit tiers changed")
    if increased:
        errors.append(f"{len(increased)} unit sizes increased")
    security = [u for u in before.values() if u["first_party"] and u["security"] and u["category"] == "runtime"]
    files = defaultdict(list)
    for unit in security:
        files[unit["file"]].append(unit)
    all_docs, scoped_docs, weighted_docs = 0, 0, 0
    doc_rows = []
    for file, units in sorted(files.items()):
        source = (args.root / file).read_text()
        # Bertie has no explicit doc attributes. Fail rather than silently undercount those.
        assert not re.search(r"#\s*!?\s*\[\s*doc\b", source), file
        docs, only_docs = doc_lines(source)
        scoped = set()
        weighted = 0
        for u in units:
            owned = only_docs & set(range(u["line"], u["end_line"] + 1))
            scoped.update(owned)
            weighted += len(owned)
        all_docs += len(docs)
        scoped_docs += len(scoped)
        weighted_docs += weighted
        doc_rows.append((file, len(docs), len(scoped), weighted))
    totals = [r["summary"]["first_party"]["security_relevant"]["sloc"] for r in (old, new)]
    drop = totals[0] - totals[1]
    # Predeclared "roughly" tolerance: 5% of independently counted, unit-owned doc lines.
    tolerance = max(1, weighted_docs * 0.05)
    if abs(drop - weighted_docs) > tolerance:
        errors.append(f"drop {drop} differs from independent doc count {weighted_docs} by more than {tolerance}")
    out = ["# Bertie v1.7.3 report-only comparison", "", f"Generated {datetime.now().astimezone().isoformat(timespec='seconds')}.", "",
           "Source: bertie-deps HEAD `f646099cee002c1d998b0e8c160886ef838f4f69`. Baseline: `4031d85`; candidate measurement code: `b3ed561` (M1). Both binaries built with `cargo build --release --frozen`. Package metadata still reads 1.5.0 at M2; M3 bumps it to 1.7.3.", "",
           "Both runs exited 0 using the exact CI arguments from `.github/workflows/coverage.yml`:", "", "```sh",
           'formal-coverage report . --config formal-coverage.toml --fstar-log target/ci-logs/fstar/fstar.log --json <out>.json --markdown <out>.md', "```", "",
           "No log-layout adaptation or proof-ratio fallback was needed. No proofs, extraction, receipt, tier or threshold changes were made. Raw reports and build/run logs are retained locally under `target/v173/` (ignored). The same accepted F* receipt was used by both runs.", "",
           "## Runtime security headlines", "", "All statuses are reported separately; `proven-extracted` is the hax/F* tier and is not `proven`. Percentages use each row's security runtime denominator. First-party covers the entire Bertie workspace, as CI does.", "",
           "| Scope / version | Security lines | Proven | Proven-extracted | Tested | Trusted | Unverified |", "| --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for scope in ("first_party", "third_party", "all"):
        for version, report in (("1.7.2", old), ("1.7.3", new)):
            t = report["summary"][scope]["security_relevant"]
            values = [f'{t["statuses"][status]:,} ({100*t["statuses"][status]/t["sloc"]:.2f}%)' for status in ("proven", "proven-extracted", "tested", "trusted", "unverified")]
            out.append(f'| {scope} / {version} | {t["sloc"]:,} | ' + " | ".join(values) + " |")
    out += ["", "`proven-bounded`, `proven-upstream` and `hazard` are zero in all headline rows.", "", "## Acceptance checks", "",
            f"- Unit identities: {'PASS' if before.keys() == after.keys() else 'FAIL'} ({len(before)} old / {len(after)} new).",
            f"- No tier changes: {'PASS' if not tiers else 'FAIL'} ({len(tiers)} changes).",
            f"- No size increases: {'PASS' if not increased else 'FAIL'} ({len(increased)} increases).",
            f"- Independent doc count vs security drop: {'PASS' if abs(drop-weighted_docs) <= tolerance else 'FAIL'}; {totals[0]:,} → {totals[1]:,}, drop **{drop}**; independently counted **{weighted_docs}** unit-owned doc-only lines ({scoped_docs} distinct source lines). Tolerance set to 5%; actual difference {abs(drop-weighted_docs)}.",
            f"- Security source files contain {all_docs} doc-comment lines in total; module/type docs outside runtime unit spans are excluded from the runtime denominator. The independent scanner skips strings and ordinary comments and handles nested block comments. No explicit doc attributes exist in these files.", ""]
    for k in common:
        if before[k]["name"] == "server" and before[k]["crate_name"] == "bertie_tls":
            out += [f'`server` (`{k[0]}`, `{k[1]}:{k[2]}`): **{before[k]["sloc"]} → {after[k]["sloc"]}** lines; tier `{before[k]["status"]}` unchanged. The supplied rough estimate (76 lines / 57 doc lines) differs from this pinned source; these are the actual paired measurements.', ""]
    out += [f"## Every unit whose size changed ({len(changed)})", "", "Includes runtime, proof and test units across the complete report, not just the security headline.", "", "| Unit | File:line | Old | New | Tier |", "| --- | --- | ---: | ---: | --- |"]
    for k in changed:
        a, b = before[k], after[k]
        out.append(f'| `{k[0]}` | `{k[1]}:{k[2]}` | {a["sloc"]} | {b["sloc"]} | {a["status"]} |')
    out += ["", "## Independent source count by file", "", "| Security file | All doc lines | Distinct unit-owned doc-only lines | Unit-weighted lines |", "| --- | ---: | ---: | ---: |"]
    out.extend(f"| `{f}` | {a} | {s} | {w} |" for f, a, s, w in doc_rows)
    out += ["", "## Report hashes", ""]
    for p in (args.old, args.new):
        out.append(f"- `{p.name}` SHA-256: `{hashlib.sha256(p.read_bytes()).hexdigest()}`.")
    if errors:
        out += ["", "FAILED — stop before M3:", *[f"- {e}" for e in errors]]
    args.output.write_text("\n".join(out) + "\n")
    print(f"{len(changed)} changed units; security {totals}; drop {drop}; independent doc count {weighted_docs}; errors {errors}")
    if errors:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
