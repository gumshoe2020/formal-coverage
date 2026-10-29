#!/usr/bin/env python3
"""Fetch locked dependencies in fenced containers, preserving prior manifests."""
import json
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import verify_runs as v


def main():
    for name in ['vest', 'anvil', 'firecracker']:
        out = v.RUNS / name
        path = out / 'manifest.json'
        original = path.read_bytes()
        try:
            step = v.container(name, v.BASE, 'm3-fetch', 'cargo +1.98.0 fetch --locked', 1200)
        finally:
            path.write_bytes(original)
        v.write(out / 'm3-manifest.json', dict(project=name, steps=[step], lockfile_sha256=v.sha((v.EXPORTS/name/'Cargo.lock').read_bytes())))
        assert step['exit_code'] == 0
        if name == 'firecracker':
            # Existing M2b report already reads this exact cache offline and
            # preserves all receipt/model accounting; do not overwrite M1.
            row = json.loads((out/'m2b-summary.json').read_text())
        else:
            import summarize
            row = summarize.summary(name)
        assert not row['unknown_dependencies'] and not row['build_unknown_dependencies']
        assert row['closure'] == 'cargo metadata (runtime)'
    keys = ['project', 'target', 'closure', 'closure_reason', 'dependency_before', 'dependency_count',
            'dependency_sloc', 'unknown_dependencies', 'build_dependency_count', 'build_dependency_sloc',
            'build_unknown_dependencies', 'dev_dependencies_excluded']
    rows = [json.loads((v.RUNS/name/('m2b-summary.json' if name=='firecracker' else 'summary.json')).read_text()) for name in ['vest','anvil','firecracker']]
    v.write(v.REPO/'docs/survey/v1.4-dependencies.json', [{k:r[k] for k in keys} for r in rows])
    print('PASS: three locked fetches; offline runtime and build inventories have zero unknown sizes', flush=True)


if __name__ == '__main__':
    main()
