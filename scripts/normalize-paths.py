#!/usr/bin/env python3
"""Normalize retained evidence and transitively refresh byte hashes from a snapshot.

Run with --before PATH (v1.4.1 tree plus extracted bundle) and --author-prefix
PREFIX. The private prefix is an input, never stored in the mapping. Without
--apply this only probes. This round-1 normalizer keeps archives and Cargo registry sources immutable.
Use redact-archives.py for the separately authorized scratch archive redactions.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess
from urllib.parse import quote


def sha(data):
    return hashlib.sha256(data).hexdigest()


def protected(path):
    return 'cargo-home/' in path or path.endswith(('.tar', '.crate', '.zst'))


def substitutions(prefix):
    # Longest first: preserve the project-relative suffix of each path.
    home = '/'.join(prefix.split('/')[:3])
    hidden = next(part for part in prefix.split('/') if part.startswith('.'))
    pairs = [(prefix, '/work/projects'), (home + '/' + hidden, '/work/projects'),
             (home, '/work/projects'), (hidden, '/work/projects')]
    variants = {}
    for old, new in pairs:
        for encode in [lambda s: s, lambda s: s.replace('/', r'\/'),
                       lambda s: quote(s, safe=''), lambda s: quote(s, safe='/'),
                       lambda s: s.replace('/', r'\u002f'),
                       lambda s: s.replace('.', r'\u002e')]:
            a, b = encode(old).encode(), encode(new).encode()
            variants[a] = b
    return sorted(variants.items(), key=lambda pair: (-len(pair[0]), pair[0]))


def normalize(data, pairs):
    lower = data.lower()
    if not any(old.lower() in lower for old, _ in pairs):
        return data
    for old, new in pairs:
        data = re.sub(re.escape(old), lambda m: new, data, flags=re.IGNORECASE)
    return data


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--before', required=True, type=Path)
    parser.add_argument('--author-prefix', required=True)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    before = args.before.resolve()
    pairs = substitutions(args.author_prefix.rstrip('/'))
    bundled = {str(p.relative_to(before)) for p in before.rglob('*') if p.is_file()}
    tracked = set(subprocess.check_output(['git', 'ls-files', '-z'], cwd=root).decode().strip('\0').split('\0'))
    names = sorted(bundled | tracked)
    originals, initial, probe = {}, {}, []
    for name in names:
        path = root / name
        if not path.is_file():
            raise SystemExit(f'missing input: {name}')
        data = path.read_bytes()
        normalized = normalize(data, pairs) if 'cargo-home/' not in name else data
        if normalized != data:
            if protected(name):
                print(f'IMMUTABLE path hit: {name}')
            probe.append(dict(file=name, occurrences=sum(len(re.findall(re.escape(a), data, re.I)) for a, _ in pairs)))
        if protected(name):
            if name in bundled:
                assert data == (before / name).read_bytes(), name
            continue
        # Keep only editable evidence in memory, never third-party caches/archives.
        originals[name] = (before / name).read_bytes() if name in bundled else data
        initial[name] = normalized
    print(json.dumps(dict(files=probe, changed_path_files=len(probe)), indent=2))
    if not args.apply:
        return
    current = initial.copy()
    digest_pattern = re.compile(rb'(?<![a-fA-F0-9])[a-f0-9]{64}(?![a-fA-F0-9])')
    for _ in range(len(current) + 1):
        hashes = {}
        for name in current:
            old, new = sha(originals[name]), sha(current[name])
            if old != new:
                assert old not in hashes or hashes[old] == new, name
                hashes[old] = new
        updated = {name: digest_pattern.sub(lambda m: hashes.get(m[0].decode(), m[0].decode()).encode(), data)
                   for name, data in initial.items()}
        if updated == current:
            break
        current = updated
    else:
        raise SystemExit('hash dependency cycle')
    mapping = [dict(file=name, old_sha256=sha(originals[name]), new_sha256=sha(data))
               for name, data in sorted(current.items()) if data != originals[name]]
    # Stronger than checking rounded figures: every report and summary byte must
    # match, allowing only the exact path substitution and mapped digest tokens.
    compared = []
    for name, data in current.items():
        if name not in bundled or not (Path(name).name.startswith('report.') or 'summary' in Path(name).name
                                       or name.endswith('.json') and name != 'logs/release-evidence.json'):
            continue
        expected = normalize(originals[name], pairs)
        expected = digest_pattern.sub(lambda m: hashes.get(m[0].decode(), m[0].decode()).encode(), expected)
        assert expected == data, f'non-path/non-hash report change: {name}'
        compared.append(name)
    for name, data in current.items():
        if (root / name).read_bytes() != data:
            (root / name).write_bytes(data)
    out = root / 'logs/path-normalization.json'
    out.write_text(json.dumps(dict(placeholder='/work/projects', files=mapping, compared_reports=compared), indent=2) + '\n')
    audit = root / 'logs/release-v1.4.2'
    audit.mkdir(parents=True, exist_ok=True)
    (audit / 'report-summary.diff').write_bytes(b'')
    print(f'{len(mapping)} changed files; {len(compared)} complete reports/summaries identical modulo paths and mapped hashes')


if __name__ == '__main__':
    main()
