#!/usr/bin/env python3
"""Prove the bundled Firecracker export is the unmodified pinned upstream archive.

The public scan excludes this archive, so check provenance first: the pinned
commit in the bundle metadata must match git's PAX comment header, the archive
hash must equal the v1.4.1 record, and no private string may occur anywhere in
the raw archive bytes. Private inputs are supplied at runtime; the log names
rule IDs, never the strings themselves.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path
import re
import tarfile

ARCHIVE = 'logs/evidence/archives/firecracker.tar'
PIN_FILES = ['runs/v1.4/firecracker/manifest.json', 'runs/v1.4/firecracker/export.json']


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--bundle', required=True, type=Path, help='extracted evidence bundle')
    parser.add_argument('--baseline-index', required=True, type=Path,
                        help='logs/release-evidence.json from the SHA-verified v1.4.1 bundle')
    parser.add_argument('--denylist', required=True, type=Path)
    parser.add_argument('--author-prefix', required=True)
    parser.add_argument('--lan-address', required=True)
    args = parser.parse_args()
    failures, lines = [], []

    def check(name, ok, detail=''):
        lines.append(f"{'PASS' if ok else 'FAIL'} {name}{': ' + detail if detail else ''}")
        if not ok:
            failures.append(name)

    data = (args.bundle / ARCHIVE).read_bytes()
    digest = hashlib.sha256(data).hexdigest()
    index = json.loads((args.bundle / 'logs/release-evidence.json').read_text())
    case = next(c for c in index['survey'] if c['archive'] == ARCHIVE)
    pins = {'logs/release-evidence.json': case['commit']}
    for name in PIN_FILES:
        pins[name] = json.loads((args.bundle / name).read_text())['commit']
    commit = case['commit']
    check('pinned-commit-consistent', len(set(pins.values())) == 1 and re.fullmatch('[0-9a-f]{40}', commit) is not None,
          ', '.join(f'{k}={v}' for k, v in pins.items()))
    with tarfile.open(fileobj=io.BytesIO(data)) as tar:
        tar.next()
        comment = tar.pax_headers.get('comment')
    check('pax-comment-matches-pin', comment == commit, f'pax comment={comment}')

    baseline = json.loads(args.baseline_index.read_text())
    recorded = next(c for c in baseline['survey'] if c['archive'] == ARCHIVE)
    check('sha256-matches-v1.4.1', digest == recorded['archive_sha256'] == case['archive_sha256'],
          f'archive={digest} v1.4.1={recorded["archive_sha256"]} current-index={case["archive_sha256"]}')
    check('commit-matches-v1.4.1', recorded['commit'] == commit)
    mapping = json.loads((args.bundle / 'logs/path-normalization.json').read_text())
    remapped = [e for e in mapping['files'] if e['file'] == ARCHIVE and e['old_sha256'] != e['new_sha256']]
    check('not-in-rebuild-mapping', not remapped)

    prefix = args.author_prefix.encode()
    home = b'/'.join(prefix.split(b'/')[:3])
    rules = [('author-prefix', re.compile(re.escape(prefix))),
             ('author-home', re.compile(re.escape(home) + rb'(?![\w.-])')),
             ('lan-address', re.compile(rb'(?<![\d.])' + re.escape(args.lan_address.encode()) + rb'(?![\d])'))]
    private = [p for p in args.denylist.read_bytes().splitlines() if p and not p.startswith((b'/', b'.', b'192.'))]
    for number, word in enumerate(private, 1):
        rules.append((f'private-term-{number:02}', re.compile(rb'(?<!\w)' + re.escape(word) + rb'(?!\w)')))
    for rule, pattern in rules:
        count = sum(1 for _ in pattern.finditer(data))
        check(f'no-{rule}', count == 0, f'{count} occurrences in raw archive bytes')

    lines.append(f'archive {ARCHIVE}: {len(data)} bytes, sha256 {digest}, commit {commit}')
    lines.append('RESULT ' + ('PASS: exclusion justified' if not failures else 'BLOCKED: ' + ', '.join(failures)))
    print('\n'.join(lines))
    raise SystemExit(1 if failures else 0)


if __name__ == '__main__':
    main()
