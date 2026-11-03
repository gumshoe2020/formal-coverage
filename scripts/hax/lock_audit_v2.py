#!/usr/bin/env python3
"""Lock audit v2: strict, fail-closed check for the in-workspace Bertie harness.

Separate from scripts/hax/audit-lock.py (the v1.7 audit), which is unchanged.
Compares the shipped scratch-server Cargo.lock with the lock cargo wrote for the
extraction workspace, where Bertie is a path member instead of a git dependency.
"""
import hashlib
import json
import sys
import tomllib
from pathlib import Path

BERTIE = "bertie"
IGNORED_PREFIXES = ("target/", ".git/")


def identity(package):
    return package["name"], package["version"], package.get("source")


def name_version(package):
    return package["name"], package["version"]


def resolve_reference(reference, packages):
    """Map a lock dependency string to its target's "name version" (cargo shortens it when unambiguous)."""
    fields = reference.split(" ")
    candidates = [p for p in packages if p["name"] == fields[0] and (len(fields) == 1 or p["version"] == fields[1])]
    if len(fields) > 2:
        candidates = [p for p in candidates if p.get("source") == fields[2].strip("()")]
    if len(candidates) != 1:
        raise ValueError("unresolvable lock reference %r (%d candidates)" % (reference, len(candidates)))
    return "%s %s" % name_version(candidates[0])


def canonical_deps(package, packages):
    return sorted(resolve_reference(r, packages) for r in package.get("dependencies", []))


def finding(kind, package, detail, blocking=True):
    return {"kind": kind, "package": package, "detail": detail, "blocking": blocking}


def label(key):
    return "%s %s" % key[:2]


def audit_package_identity(shipped, current):
    """Return (findings, matched pairs) for entries that must be byte-identical."""
    findings, matched = [], []
    by_nv = {name_version(p): p for p in current}
    current_names = {}
    for p in current:
        current_names.setdefault(p["name"], []).append(p)
    for old in shipped:
        if old["name"] == BERTIE:
            continue
        new = by_nv.get(name_version(old))
        if new is None:
            versions = sorted(p["version"] for p in current_names.get(old["name"], []))
            kind = "version-change" if versions else "removed"
            findings.append(finding(kind, label(identity(old)), "now: %s" % (versions or "absent")))
            continue
        if old.get("source") != new.get("source"):
            findings.append(finding("source-change", label(identity(old)), "%s -> %s" % (old.get("source"), new.get("source"))))
        elif old.get("checksum") != new.get("checksum"):
            findings.append(finding("checksum-change", label(identity(old)), "%s -> %s" % (old.get("checksum"), new.get("checksum"))))
        else:
            matched.append((old, new))
    return findings, matched


def audit_edges(matched, shipped, current, closure):
    findings, additions = [], []
    for old, new in matched:
        before, after = canonical_deps(old, shipped), canonical_deps(new, current)
        if before == after:
            continue
        who = label(identity(old))
        for ref in sorted(set(before) - set(after)):
            findings.append(finding("dropped-edge", who, ref))
        in_closure = name_version(old) in closure
        for ref in sorted(set(after) - set(before)):
            if in_closure:
                findings.append(finding("closure-internal-edge-addition", who, ref))
            else:
                additions.append({"kind": "edge-outside-closure", "package": who, "detail": ref})
    return findings, additions


def audit_added_packages(shipped, current, closure):
    known = {name_version(p) for p in shipped}
    findings, additions = [], []
    for package in current:
        if package["name"] == BERTIE or name_version(package) in known:
            continue
        key = name_version(package)
        if key in closure:
            findings.append(finding("closure-internal-addition", label(key), "new package inside Bertie's closure"))
        else:
            additions.append({"kind": "package", "package": label(key), "detail": package.get("source")})
    return findings, additions


def audit_bertie_entry(shipped, current, dev_only_targets):
    old = [p for p in shipped if p["name"] == BERTIE]
    new = [p for p in current if p["name"] == BERTIE]
    if len(old) != 1 or len(new) != 1:
        return [finding("bertie-entry", BERTIE, "expected one entry each, found %d shipped / %d current" % (len(old), len(new)))], []
    old, new = old[0], new[0]
    findings, additions = [], []
    if new.get("source") is not None:
        findings.append(finding("bertie-source-not-path", BERTIE, "source=%s" % new["source"]))
    if old["version"] != new["version"]:
        findings.append(finding("version-change", BERTIE, "%s -> %s" % (old["version"], new["version"])))
    before, after = set(canonical_deps(old, shipped)), set(canonical_deps(new, current))
    for ref in sorted(before - after):
        findings.append(finding("dropped-edge", BERTIE, ref))
    for ref in sorted(after - before):
        if tuple(ref.split(" ")) in dev_only_targets:
            additions.append({"kind": "bertie-dev-edge", "package": BERTIE, "detail": ref})
        else:
            findings.append(finding("closure-internal-edge-addition", BERTIE, ref))
    return findings, additions


def file_sha256(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def tree_manifest(root, ignore_prefixes=IGNORED_PREFIXES):
    root = Path(root)
    entries = {}
    for path in sorted(p for p in root.rglob("*") if p.is_file() or p.is_symlink()):
        rel = path.relative_to(root).as_posix()
        if rel.startswith(ignore_prefixes):
            continue
        digest = hashlib.sha256(path.readlink().as_posix().encode()).hexdigest() if path.is_symlink() else file_sha256(path)
        entries[rel] = digest
    return entries


def tree_digest(manifest):
    text = "".join("%s %s\n" % (digest, rel) for rel, digest in sorted(manifest.items()))
    return hashlib.sha256(text.encode()).hexdigest()


def check_tree(archive, current, edits):
    """archive/current: relpath -> sha256. edits: relpath -> {before, after}."""
    problems = []
    for rel in sorted(set(archive) | set(current)):
        want, have = archive.get(rel), current.get(rel)
        edit = edits.get(rel)
        if edit is not None:
            if want != edit["before"] or have != edit["after"]:
                problems.append("%s: recorded edit does not match archive/current hashes" % rel)
        elif want != have:
            problems.append("%s: archive=%s current=%s" % (rel, want, have))
    return {"ok": not problems, "problems": problems}


def audit(shipped, current, closure, dev_only_targets, tree):
    """Return the audit verdict. closure/dev_only_targets: sets of (name, version)."""
    try:
        return audit_checked(shipped, current, closure, dev_only_targets, tree)
    except ValueError as error:
        blocking = [finding("unresolvable-reference", "lock", str(error))]
        return {"verdict": "block", "blocking": blocking, "additions": [], "libcrux": [], "closure_size": len(closure)}


def audit_checked(shipped, current, closure, dev_only_targets, tree):
    findings, matched = audit_package_identity(shipped, current)
    edge_findings, edge_additions = audit_edges(matched, shipped, current, closure)
    added_findings, package_additions = audit_added_packages(shipped, current, closure)
    bertie_findings, bertie_additions = audit_bertie_entry(shipped, current, dev_only_targets)
    findings += edge_findings + added_findings + bertie_findings
    if not tree["ok"]:
        findings += [finding("tree-hash-mismatch", BERTIE, p) for p in tree["problems"]]
    blocking = [f for f in findings if f["blocking"]]
    return {
        "verdict": "block" if blocking else "pass",
        "blocking": blocking,
        "additions": package_additions + edge_additions + bertie_additions,
        "libcrux": sorted(label(identity(p)) for p in current if p["name"].startswith("libcrux")),
        "closure_size": len(closure),
    }


def load_packages(path):
    return tomllib.loads(Path(path).read_text())["package"]


def closure_from_cargo_tree(text):
    """Parse `cargo tree --prefix none --format "{p}"`: `name vX.Y.Z (source) (*)` lines."""
    closure = set()
    for line in text.splitlines():
        fields = line.split()
        if len(fields) >= 2 and fields[1].startswith("v"):
            closure.add((fields[0], fields[1][1:]))
    return closure


def dev_only_targets_from_metadata(metadata):
    ids = {p["id"]: p for p in metadata["packages"]}
    node = next(n for n in metadata["resolve"]["nodes"] if ids[n["id"]]["name"] == BERTIE)
    targets = set()
    for dep in node["deps"]:
        if all(k["kind"] == "dev" for k in dep["dep_kinds"]):
            targets.add(name_version(ids[dep["pkg"]]))
    return targets


def main(argv):
    shipped_lock, current_lock, tree_json, metadata_json, *cargo_tree_files = argv[1:]
    tree = json.loads(Path(tree_json).read_text())
    closure = set().union(*(closure_from_cargo_tree(Path(f).read_text()) for f in cargo_tree_files))
    dev_only = dev_only_targets_from_metadata(json.loads(Path(metadata_json).read_text()))
    result = audit(load_packages(shipped_lock), load_packages(current_lock), closure, dev_only, tree)
    print(json.dumps(result, indent=2))
    return 0 if result["verdict"] == "pass" else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
