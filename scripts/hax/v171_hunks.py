#!/usr/bin/env python3
"""Per-hunk record of a failed `patches.sh apply`: hunks per file in the patch vs rejected.

usage: v171_hunks.py TREE_DIR   (reads TREE_DIR/ws/bertie/proofs/fstar/*.patch and TREE_DIR/patches/apply.log)
"""
import json
import re
import sys
from pathlib import Path

HUNK = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@", re.MULTILINE)
FAILED = re.compile(r"^(\d+) out of (\d+) hunks? FAILED -- saving rejects to file (\S+)", re.MULTILINE)
REJ_GROUP = re.compile(r"^::group::cat (\S+)\.rej\n(.*?)^::endgroup::", re.MULTILINE | re.DOTALL)


def patch_hunks(text):
    """file name -> list of '@@' header strings, from a unified diff."""
    files, current = {}, None
    for line in text.splitlines():
        if line.startswith("+++ "):
            current = Path(line[4:].split("\t")[0]).name
            files[current] = []
        elif line.startswith("@@") and current:
            files[current].append(line.split(" @@")[0] + " @@")
    return files


def rejected_hunks(log):
    return {name: [m.group(0) for m in HUNK.finditer(body)] for name, body in REJ_GROUP.findall(log)}


def record(tree):
    tree = Path(tree)
    log = (tree / "patches/apply.log").read_text(errors="replace")
    patch = tree / "ws/bertie/proofs/fstar/extraction-lax.patch"
    total = patch_hunks(patch.read_text())
    failed = {name.removesuffix(".rej"): (int(f), int(t)) for f, t, name in FAILED.findall(log)}
    rejected = rejected_hunks(log)
    files = {}
    for name, headers in sorted(total.items()):
        bad, of = failed.get(name, (0, len(headers)))
        files[name] = {"hunks": len(headers), "failed": bad, "rejected_headers": rejected.get(name, [])}
    return {
        "patch": "extraction-lax.patch (first of two; extraction-panic-free.patch applies to its output and was not attempted)",
        "hunks_total": sum(f["hunks"] for f in files.values()),
        "hunks_failed": sum(f["failed"] for f in files.values()),
        "files": files,
    }


if __name__ == "__main__":
    result = record(sys.argv[1])
    Path(sys.argv[1], "patches/hunks.json").write_text(json.dumps(result, indent=2) + "\n")
    print(result["hunks_failed"], "of", result["hunks_total"], "hunks failed;",
          {n: (f["failed"], f["hunks"]) for n, f in result["files"].items() if f["failed"]})
