#!/usr/bin/env python3
"""Build the v1.7.1 extraction workspace: Bertie as a path member of scratch-server.

usage: v171_workspace.py build SCRATCH_EXPORT BERTIE_EXPORT OUT_WS | check OUT_WS
Rust sources are copied untouched. The only manifest edits are the three allowed ones:
drop Bertie's own [workspace] table; add the member line and the [patch] entry to
scratch-server's Cargo.toml. Every edit is recorded with before/after hashes and a diff.
"""
import difflib
import hashlib
import json
import re
import shutil
import sys
from pathlib import Path

from lock_audit_v2 import check_tree, file_sha256, tree_digest, tree_manifest

IGNORED = ("target/", ".git/", "Cargo.lock")

BERTIE_URL = "https://github.com/gumshoe2020/bertie"
PATCH_ENTRY = '\n[patch."%s"]\nbertie = { path = "bertie" }\n' % BERTIE_URL
WORKSPACE_TABLE = re.compile(r"^\[workspace\]\n(?:(?!\[)[^\n]*\n)*", re.MULTILINE)
MEMBERS_LINE = re.compile(r'^(members = \[)([^\]]*)\]', re.MULTILINE)


def drop_bertie_workspace_table(text):
    """Remove the [workspace] table; refuse anything but exactly one."""
    if len(WORKSPACE_TABLE.findall(text)) != 1:
        raise ValueError("expected exactly one [workspace] table in Bertie's Cargo.toml")
    return WORKSPACE_TABLE.sub("", text, count=1)


def add_member_and_patch(text):
    """Append "bertie" to scratch-server's workspace members and add the [patch] entry."""
    if len(MEMBERS_LINE.findall(text)) != 1 or "[patch" in text:
        raise ValueError("unexpected scratch-server manifest shape")
    text = MEMBERS_LINE.sub(lambda m: '%s%s, "bertie"]' % (m.group(1), m.group(2).rstrip()), text, count=1)
    return text.rstrip("\n") + "\n" + PATCH_ENTRY


def record_edit(path, rel, edit):
    before = path.read_text()
    after = edit(before)
    path.write_text(after)
    diff = difflib.unified_diff(before.splitlines(), after.splitlines(), "a/" + rel, "b/" + rel, lineterm="")
    return {"before": text_sha256(before), "after": text_sha256(after), "diff": list(diff)}


def text_sha256(text):
    return hashlib.sha256(text.encode()).hexdigest()


def copy_tree(source, destination):
    shutil.copytree(source, destination, symlinks=True)


def build(scratch_export, bertie_export, ws):
    ws = Path(ws)
    if ws.exists():
        raise SystemExit("refusing to overwrite %s" % ws)
    copy_tree(scratch_export, ws)
    copy_tree(bertie_export, ws / "bertie")
    edits = {
        "Cargo.toml": record_edit(ws / "Cargo.toml", "Cargo.toml", add_member_and_patch),
        "bertie/Cargo.toml": record_edit(ws / "bertie/Cargo.toml", "bertie/Cargo.toml", drop_bertie_workspace_table),
    }
    return edits


def archive_manifest(scratch_export, bertie_export):
    archive = tree_manifest(scratch_export, IGNORED)
    archive.update({"bertie/" + rel: d for rel, d in tree_manifest(bertie_export, IGNORED).items()})
    return archive


def check(archive, ws, edits):
    current = tree_manifest(ws, IGNORED)
    tree = check_tree(archive, current, {rel: {"before": e["before"], "after": e["after"]} for rel, e in edits.items()})
    tree["archive_tree_sha256"], tree["workspace_tree_sha256"] = tree_digest(archive), tree_digest(current)
    return tree


def write_json(path, value):
    Path(path).write_text(json.dumps(value, indent=2) + "\n")


def cmd_build(scratch_export, bertie_export, ws):
    edits = build(scratch_export, bertie_export, ws)
    archive = archive_manifest(scratch_export, bertie_export)
    out = Path(ws).parent
    write_json(out / "archive-tree.json", archive)
    write_json(out / "workspace.json", {
        "scratch_export": scratch_export, "bertie_export": bertie_export, "edits": edits,
        "shipped_lock_sha256": file_sha256(Path(ws) / "Cargo.lock"),
    })
    return cmd_check(ws)


def cmd_check(ws):
    out = Path(ws).parent
    record = json.loads((out / "workspace.json").read_text())
    tree = check(json.loads((out / "archive-tree.json").read_text()), ws, record["edits"])
    write_json(out / "tree-check.json", tree)
    print(json.dumps({"tree_ok": tree["ok"], "problems": tree["problems"][:5]}))
    return 0 if tree["ok"] else 1


def main(argv):
    if argv[1] == "build":
        return cmd_build(*argv[2:5])
    if argv[1] == "check":
        return cmd_check(argv[2])
    raise SystemExit("usage: v171_workspace.py build SCRATCH BERTIE WS | check WS")


if __name__ == "__main__":
    sys.exit(main(sys.argv))
