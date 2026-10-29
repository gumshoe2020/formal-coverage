#!/usr/bin/env python3
"""Replay all sixteen pinned report inputs offline and check coverage and receipt decisions."""
import json
import time
from evidence_support import arguments, require_bundle, check_tool, replay


def main():
    args = arguments(__doc__, scratch=True).parse_args()
    root = args.evidence_root.resolve()
    index = require_bundle(root)
    check_tool(index)
    timings = []
    for case in index['survey'] + index['scratch']:
        start = time.monotonic()
        report = replay(root, args.binary, case, root / 'logs/perf-release' / case['name'])
        expected = json.loads((root / case['report']).read_text())
        assert report['summary'] == expected['summary'], case['name']
        identity = lambda r: [(u['id'], u['sloc'], u['status'], u['category'], u['security']) for u in r['units']]
        assert identity(report) == identity(expected), case['name']
        decisions = lambda r: [(e['tool'], e['log'], e['accepted'], e['credited_units'], e['reasons']) for e in r['evidence_runs']]
        assert decisions(report) == decisions(expected), case['name']
        elapsed = time.monotonic() - start
        timings.append(dict(name=case['name'], seconds=elapsed))
        print(f'{case["name"]}: identical coverage and receipt decisions; {elapsed:.3f}s', flush=True)
    assert len(timings) == 16
    from evidence_support import write
    write(root / 'logs/perf-release/timings.json', timings)


if __name__ == '__main__':
    main()
