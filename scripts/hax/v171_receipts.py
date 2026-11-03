#!/usr/bin/env python3
"""Write the v1.7 `--fstar-log` receipt for one typecheck run, from its recorded per-module results.

usage: v171_receipts.py TREE_DIR TIER FSTAR_VERSION
Reads TREE_DIR/typecheck-TIER-VERSION.{log,results.tsv}; writes <log>.evidence.json next to the log.
Nothing is inferred: a module is `success` only if its recorded exit code is 0, and the validator
separately requires F*'s own `Verified module:` line.
"""
import hashlib
import json
import shlex
import sys
from pathlib import Path

LABEL = ["python3", "hax-driver.py", "extract-fstar"]
TIER_DIR = {"extraction": "proofs/fstar/extraction", "panic-free": "proofs/fstar/extraction-panic-free"}


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def hashed(base, patterns, directories):
    return {str(p.relative_to(base)): sha(p) for d in directories for pat in patterns for p in sorted((base / d).glob(pat))}


def rust_tree_files(root):
    files = {str(p.relative_to(root)): sha(p) for p in sorted(root.rglob("*.rs")) if "target" not in p.relative_to(root).parts}
    for pattern in ["**/Cargo.toml", "Cargo.lock", "hax-driver.py"]:
        files.update({str(p.relative_to(root)): sha(p) for p in root.glob(pattern)})
    return files


def logged_commands(log_text):
    commands = {}
    for line in log_text.splitlines():
        if line.startswith("+ fstar.exe"):
            args = shlex.split(line[2:])
            commands[args[-1]] = args
    return commands


def module_outcomes(base, log_text, results):
    commands, modules = logged_commands(log_text), []
    for line in results.read_text().splitlines():
        file, code, _seconds = line.split("\t")
        code = int(code)
        outcome = "success" if code == 0 else "timeout" if code == 124 else "failed"
        modules.append({"file": file, "command": commands[file], "exit_code": code, "outcome": outcome})
    return modules


def build(base, tier, version):
    log = base / f"typecheck-{tier}-{version}.log"
    log_text = log.read_text(errors="replace")
    library_dirs = sorted({p.parent.relative_to(base) for p in (base / "libs").rglob("*.fst*")})
    checked = hashed(base, ["*.fst*"], library_dirs + [Path(TIER_DIR[tier])])
    hax_commit = next(l.split("=")[1] for l in (base / "extract/hax-version.log").read_text().splitlines() if l.startswith("commit="))
    hax_version = next(l.split("=")[1] for l in (base / "extract/hax-version.log").read_text().splitlines() if l.startswith("version="))
    return {
        "tool": "hax-fstar", "hax_version": hax_version, "hax_commit": hax_commit,
        "fstar_version": (base / f"typecheck-{tier}-{version}.fstar-version.txt").read_text().split()[1],
        "extraction_command": LABEL, "extraction_exit_code": int((base / "extract/extract.exit").read_text()),
        "files": rust_tree_files(base / "root"),
        "fresh": hashed(base, ["*.fst*"], ["fresh"]), "checked": checked,
        "patches": {}, "log_sha256": sha(log),
        "modules": module_outcomes(base, log_text, base / f"typecheck-{tier}-{version}.results.tsv"),
    }


if __name__ == "__main__":
    base, tier, version = Path(sys.argv[1]), sys.argv[2], sys.argv[3]
    receipt = build(base, tier, version)
    out = base / f"typecheck-{tier}-{version}.log.evidence.json"
    out.write_text(json.dumps(receipt, indent=2) + "\n")
    print(out, {k: len(v) for k, v in receipt.items() if isinstance(v, (dict, list))})
