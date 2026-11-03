#!/usr/bin/env python3
"""Compare the lock cargo derived for the standalone report root with the audited workspace lock.

usage: v171_rootlock.py WORKSPACE_LOCK DERIVED_LOCK
Blocking: any package present in both with a different source or checksum, or a package whose
name+version exists in the workspace lock under a different version in the derived lock (version drift).
Listed, not blocking: packages only in one lock, and resolved dependency-target differences for shared packages.
(This is not an exception to lock audit v2; the extraction itself ran against the audited workspace lock.)
"""
import json
import sys

from lock_audit_v2 import canonical_deps, load_packages, name_version


def label(key):
    return "%s %s" % key


def edges(package, packages):
    try:
        return canonical_deps(package, packages)
    except ValueError as error:
        return ["unresolvable: %s" % error]


def compare(workspace, derived):
    ws, dr = {name_version(p): p for p in workspace}, {name_version(p): p for p in derived}
    problems, edge_differences = [], []
    for key in sorted(set(ws) & set(dr)):
        if (ws[key].get("source"), ws[key].get("checksum")) != (dr[key].get("source"), dr[key].get("checksum")):
            problems.append("identity change: " + label(key))
        before, after = edges(ws[key], workspace), edges(dr[key], derived)
        if before != after:
            edge_differences.append({"package": label(key), "only_workspace": sorted(set(before) - set(after)), "only_derived": sorted(set(after) - set(before))})
    workspace_names = {}
    for key in ws:
        workspace_names.setdefault(key[0], set()).add(key[1])
    for key in sorted(set(dr) - set(ws)):
        if key[0] in workspace_names and key[0] != "bertie":
            problems.append("version drift: %s (workspace lock has %s)" % (label(key), sorted(workspace_names[key[0]])))
    return {"ok": not problems, "problems": problems,
            "only_workspace": sorted(label(k) for k in set(ws) - set(dr)),
            "only_derived": sorted(label(k) for k in set(dr) - set(ws)),
            "edge_differences": edge_differences}


def main(argv):
    result = compare(load_packages(argv[1]), load_packages(argv[2]))
    print(json.dumps(result, indent=2))
    return 0 if result["ok"] else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
