#!/usr/bin/env python3
"""Inventory one tree's fresh extraction: hash every file, compare to the committed snapshot.

usage: v171_inventory.py SOURCES_EXPORT TREE_DIR   (writes TREE_DIR/extraction-inventory.json)
"""
import difflib
import hashlib
import json
import sys
from pathlib import Path

EXTRACTION = "proofs/fstar/extraction"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def line_changes(old, new):
    old_lines = old.read_text().splitlines() if old.exists() else []
    diff = difflib.unified_diff(old_lines, new.read_text().splitlines(), lineterm="", n=0)
    lines = [l for l in diff if l[:1] in "+-" and l[:3] not in ("+++", "---")]
    return {"added": sum(l[0] == "+" for l in lines), "removed": sum(l[0] == "-" for l in lines)}


def inventory(export, tree):
    snapshot = Path(export) / EXTRACTION
    fresh = Path(tree) / "ws/bertie" / EXTRACTION
    files = {}
    for path in sorted(fresh.glob("*.fst*")):
        old = snapshot / path.name
        files[path.name] = {
            "sha256": sha(path),
            "snapshot_sha256": sha(old) if old.exists() else None,
            "changed_from_snapshot": not old.exists() or sha(old) != sha(path),
            **line_changes(old, path),
        }
    return {"fresh_files": files, "snapshot_only": sorted(p.name for p in snapshot.glob("*.fst*") if p.name not in files)}


def main(argv):
    export, tree = argv[1:3]
    result = inventory(export, tree)
    Path(tree, "extraction-inventory.json").write_text(json.dumps(result, indent=2) + "\n")
    changed = sum(f["changed_from_snapshot"] for f in result["fresh_files"].values())
    print(len(result["fresh_files"]), "fresh,", changed, "differ from snapshot, snapshot-only:", result["snapshot_only"])


if __name__ == "__main__":
    main(sys.argv)
