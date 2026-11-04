#!/usr/bin/env python3
"""usage: compare.py OLD.json NEW.json  (repo-root paths normalised to <repo>, .tmp/v1.7.2 to runs/v1.7.1) -- list changed leaves (path, old, new); lists are compared by index only when same length, else as set sizes."""
import json, os, sys

REPO = os.getcwd()

def load(path):
    text = open(path).read().replace(REPO + "/.tmp/v1.7.2", "<repo>/runs/v1.7.1").replace(REPO, "<repo>")
    return json.loads(text)

def walk(a, b, path, out):
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a: out.append((path + "/" + k, "<absent>", b[k]))
            elif k not in b: out.append((path + "/" + k, a[k], "<absent>"))
            else: walk(a[k], b[k], path + "/" + k, out)
    elif isinstance(a, list) and isinstance(b, list) and len(a) == len(b):
        for i, (x, y) in enumerate(zip(a, b)): walk(x, y, f"{path}[{i}]", out)
    elif a != b:
        out.append((path, a, b))

if __name__ == "__main__":
    out = []
    walk(load(sys.argv[1]), load(sys.argv[2]), "", out)
    for p, a, b in out:
        sa, sb = json.dumps(a)[:110], json.dumps(b)[:110]
        print(f"{p}: {sa} -> {sb}")
    print(f"{len(out)} changed leaves")
