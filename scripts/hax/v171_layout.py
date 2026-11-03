#!/usr/bin/env python3
"""Lay out one tree's receipt base: hashed F* libraries plus the fresh extraction as the checked tier.

usage: v171_layout.py TREE_DIR HAX_CHECKOUT LIBCRUX_MODELS_DIR BERTIE_EXPORT
The report root is TREE_DIR/root. Receipt paths are relative to TREE_DIR; the v1.7 F* grammar only allows `--include DIR` for
directories whose files are all receipt-hashed, so the libraries are copied in and hashed.
"""
import difflib
import re
import shutil
import sys
from pathlib import Path

LIBS = {
    "libs/rust_primitives": "proof-libs/fstar/rust_primitives",
    "libs/core": "proof-libs/fstar/core",
    "libs/hax-lib": "hax-lib/proofs/fstar/extraction",
}


def copy_fstar(source, destination):
    destination.mkdir(parents=True, exist_ok=True)
    count = 0
    for path in sorted(Path(source).glob("*.fst*")):
        shutil.copy2(path, destination / path.name)
        count += 1
    return count


WORKSPACE_TABLE = re.compile(r"^\[workspace\]\n(?:(?!\[)[^\n]*\n)*", re.MULTILINE)


def build_root(tree, export):
    """The report root: the bertie package alone (its other workspace members are not in the audited closure) plus the audited lock."""
    shutil.rmtree(tree / "root", ignore_errors=True)
    shutil.copytree(export, tree / "root", symlinks=True)
    manifest = tree / "root/Cargo.toml"
    before = manifest.read_text()
    if len(WORKSPACE_TABLE.findall(before)) != 1:
        raise SystemExit("expected exactly one [workspace] table in Bertie's Cargo.toml")
    after = WORKSPACE_TABLE.sub("[workspace]\n", before, count=1)
    manifest.write_text(after)
    diff = difflib.unified_diff(before.splitlines(), after.splitlines(), "a/Cargo.toml", "b/Cargo.toml", lineterm="")
    (tree / "root-manifest.diff").write_text("\n".join(diff) + "\n")
    shutil.copy2(tree / "ws/Cargo.lock", tree / "root/Cargo.lock")


def main(argv):
    tree, hax, models, export = Path(argv[1]), Path(argv[2]), Path(argv[3]), Path(argv[4])
    build_root(tree, export)
    for target, relative in LIBS.items():
        shutil.rmtree(tree / target, ignore_errors=True)
        print(target, copy_fstar(hax / relative, tree / target))
    shutil.rmtree(tree / "libs/libcrux-models", ignore_errors=True)
    print("libs/libcrux-models", copy_fstar(models, tree / "libs/libcrux-models"))
    shutil.rmtree(tree / "proofs/fstar/extraction", ignore_errors=True)
    print("proofs/fstar/extraction", copy_fstar(tree / "fresh", tree / "proofs/fstar/extraction"))


if __name__ == "__main__":
    main(sys.argv)
