#!/usr/bin/env python3
"""Replay pinned scratch-server archives and retained receipts; never execute target code."""
import io
import json
from pathlib import Path
import subprocess
import tarfile

from evidence_support import REPO, arguments, require_bundle, check_tool, replay, validate_receipts


def export_all(destination, scratch, records):
    """Read only committed bytes, never refs or the concurrently edited working tree."""
    for row in records:
        target = destination / row['branch']
        target.mkdir(parents=True, exist_ok=True)
        data = subprocess.check_output(['git', '-C', str(scratch), 'archive', row['commit']])
        with tarfile.open(fileobj=io.BytesIO(data)) as tar:
            tar.extractall(target, members=[m for m in tar.getmembers() if m.isfile() or m.isdir()], filter='data')
    return destination


def main():
    parser = arguments(__doc__, scratch=True)
    parser.add_argument('--export-only', action='store_true')
    parser.add_argument('--directory', type=Path, default=REPO / 'target/release-scratch')
    args = parser.parse_args()
    index = require_bundle(args.evidence_root)
    check_tool(index)
    if args.export_only:
        export_all(args.directory, args.scratch_server, json.loads((REPO / 'docs/scratch-server/runs.json').read_text()))
        return
    for case in index['scratch']:
        report = replay(args.evidence_root, args.binary, case, args.evidence_root / 'logs/replay' / case['name'])
        validate_receipts(args.evidence_root, case, report)
        expected = json.loads((args.evidence_root / case['report']).read_text())
        assert report['summary'] == expected['summary'], case['name']
        print(case['name'], 'pinned receipt replay passed')


if __name__ == '__main__':
    main()
