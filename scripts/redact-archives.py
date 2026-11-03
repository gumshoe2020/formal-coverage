#!/usr/bin/env python3
"""Redact pinned scratch exports; preserve tar metadata and refresh evidence hashes.

Private inputs are supplied at runtime. Rule IDs and byte positions are public.
The baseline must contain the original archives and v1.4.1 evidence outputs.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path
import re
import subprocess
import tarfile


def sha(data):
    return hashlib.sha256(data).hexdigest()


def redact(data, rules):
    events = []
    # Compute positions in original bytes, then substitute without recursive matches.
    matches = []
    for rule, pattern, replacement in rules:
        matches.extend((m.start(), m.end(), rule, replacement(m)) for m in pattern.finditer(data))
    cursor, pieces = 0, []
    for start, end, rule, replacement in sorted(matches):
        if start < cursor:
            raise ValueError('overlapping redaction rules')
        pieces.extend([data[cursor:start], replacement])
        events.append(dict(line=data.count(b'\n', 0, start) + 1,
                           column=start - data.rfind(b'\n', 0, start), rule=rule))
        cursor = end
    return b''.join(pieces + [data[cursor:]]), events


def rebuild(data, rules):
    changes, replacements, blockers = [], {}, []
    with tarfile.open(fileobj=io.BytesIO(data)) as tar:
        members = tar.getmembers()
        for member in members:
            if not member.isfile():
                continue
            old = tar.extractfile(member).read()
            new, events = redact(old, rules)
            if old == new:
                continue
            if member.name.endswith('.rs') or Path(member.name).name in ('Cargo.toml', 'Cargo.lock'):
                blockers.append(member.name)
            replacements[member.offset_data - 512] = new
            changes.append(dict(member=member.name, old_sha256=sha(old), new_sha256=sha(new), events=events))
    if blockers:
        return data, changes, blockers
    # Copy raw tar records, including git's global PAX commit header, verbatim.
    # Only size/checksum fields of changed regular-file headers need updating.
    output, offset = bytearray(), 0
    while data[offset:offset + 512].strip(b'\0'):
        header = bytearray(data[offset:offset + 512])
        size = int(header[124:136].rstrip(b'\0 ') or b'0', 8)
        end = offset + 512 + ((size + 511) // 512) * 512
        if offset in replacements:
            content = replacements[offset]
            header[124:136] = f'{len(content):011o}\0'.encode()
            header[148:156] = b' ' * 8
            header[148:156] = f'{sum(header):06o}\0 '.encode()
            output.extend(header + content + b'\0' * (-len(content) % 512))
        else:
            output.extend(data[offset:end])
        offset = end
    # Keep the original tar blocking convention (git archive uses 10 KiB).
    output.extend(b'\0' * 1024)
    output.extend(b'\0' * (-len(output) % 10240))
    result = bytes(output) if changes else data
    with tarfile.open(fileobj=io.BytesIO(result)) as tar:
        after = tar.getmembers()
        assert len(after) == len(members)
        for old, new in zip(members, after):
            a, b = old.get_info(), new.get_info()
            for key in ('size', 'chksum'):
                a.pop(key)
                b.pop(key)
            assert a == b and old.pax_headers == new.pax_headers, (old.name, a, b)
    with tarfile.open(fileobj=io.BytesIO(data)) as before_tar, tarfile.open(fileobj=io.BytesIO(result)) as after_tar:
        actual_changes = []
        for member in members:
            if member.isfile():
                old_bytes = before_tar.extractfile(member.name).read()
                new_bytes = after_tar.extractfile(member.name).read()
                if old_bytes != new_bytes:
                    actual_changes.append(member.name)
                    assert not member.name.endswith('.rs')
                    assert Path(member.name).name not in ('Cargo.toml', 'Cargo.lock')
        assert actual_changes == [change['member'] for change in changes]
    return result, changes, []


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--before', required=True, type=Path)
    parser.add_argument('--denylist', required=True, type=Path)
    parser.add_argument('--author-prefix', required=True)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    rules = [('author-prefix', re.compile(re.escape(args.author_prefix.encode())), lambda m: b'/work/projects'),
             ('lan-address', re.compile(rb'192\.168\.\d{1,3}\.(\d{1,3})(?![\d.])'),
              lambda m: b'192.0.2.' + m[1])]
    private = [p for p in args.denylist.read_bytes().splitlines() if p and not p.startswith((b'/', b'.', b'192.'))]
    for index, word in enumerate(private, 1):
        placeholder = b'[workstation]' if b' ' in word else b'[private]'
        rules.append((f'private-term-{index:02}', re.compile(rb'(?<!\w)' + re.escape(word) + rb'(?!\w)'),
                      lambda m, value=placeholder: value))
    audit = root / 'logs/release-v1.4.2'
    proof, rebuilt, events = [], {}, []
    for path in sorted((args.before / 'logs/evidence/archives').glob('scratch-*.tar')):
        name = str(path.relative_to(args.before))
        original = path.read_bytes()
        result, changes, blockers = rebuild(original, rules)
        with tarfile.open(fileobj=io.BytesIO(original)) as old_tar, tarfile.open(fileobj=io.BytesIO(result)) as new_tar:
            protected = []
            for m in old_tar.getmembers():
                if m.isfile() and (m.name.endswith('.rs') or Path(m.name).name in ('Cargo.toml', 'Cargo.lock')):
                    old_hash = sha(old_tar.extractfile(m.name).read())
                    new_hash = sha(new_tar.extractfile(m.name).read())
                    assert old_hash == new_hash
                    protected.append(dict(member=m.name, old_sha256=old_hash, new_sha256=new_hash))
        proof.append(dict(archive=name, old_sha256=sha(original), new_sha256=sha(result),
                          changed_members=changes, protected_members=protected,
                          metadata_preserved=True, blockers=blockers))
        for change in changes:
            events.extend(dict(archive=name, member=change['member'], **event) for event in change['events'])
        if blockers:
            events.append(dict(archive=name, rule='BLOCKER', members=blockers))
        rebuilt[name] = result
    (audit / 'archive-redactions.log').write_text(''.join(json.dumps(e, sort_keys=True) + '\n' for e in events))
    (audit / 'archive-member-proof.json').write_text(json.dumps(proof, indent=2) + '\n')
    assert not any(p['blockers'] for p in proof), 'protected member redaction blocked; no archives written'
    assert sum(bool(p['changed_members']) for p in proof) == 11
    for name, data in rebuilt.items():
        (root / name).write_bytes(data)
    mapping_path = root / 'logs/path-normalization.json'
    mapping = json.loads(mapping_path.read_text())
    hashes = {p['old_sha256']: p['new_sha256'] for p in proof if p['changed_members']}
    previous = {e['file']: e['new_sha256'] for e in mapping['files']}
    for record in proof:
        if record['archive'] in previous:
            hashes[previous[record['archive']]] = record['new_sha256']
    # Refresh every retained occurrence, including original metadata copies, but
    # never rewrite caches, source archives, or the old/new audit mapping itself.
    names = set(subprocess.check_output(['git', 'ls-files', '-z'], cwd=root).decode().strip('\0').split('\0'))
    names.update(str(p.relative_to(args.before)) for p in args.before.rglob('*') if p.is_file())
    names.discard('logs/path-normalization.json')
    originals = {n: (root / n).read_bytes() for n in names if (root / n).is_file()
                 and 'cargo-home/' not in n and not n.endswith(('.tar', '.zst', '.crate'))}
    current = originals.copy()
    token = re.compile(rb'(?<![a-fA-F0-9])[a-f0-9]{64}(?![a-fA-F0-9])')
    for _ in range(len(current) + 1):
        for n, data in current.items():
            if data != originals[n]:
                hashes[sha(originals[n])] = sha(data)
        updated = {n: token.sub(lambda m: hashes.get(m[0].decode(), m[0].decode()).encode(), d)
                   for n, d in originals.items()}
        if updated == current:
            break
        current = updated
    else:
        raise ValueError('hash dependency cycle')
    entries = {e['file']: e for e in mapping['files']}
    for name, data in {**current, **rebuilt}.items():
        old = (args.before / name).read_bytes() if (args.before / name).is_file() else originals.get(name, data)
        if name in entries or old != data:
            entries[name] = dict(file=name, old_sha256=entries.get(name, {}).get('old_sha256', sha(old)), new_sha256=sha(data))
        if name in current and data != originals[name]:
            (root / name).write_bytes(data)
    mapping['files'] = [entries[n] for n in sorted(entries)]
    mapping_path.write_text(json.dumps(mapping, indent=2) + '\n')
    print(f'{len(proof)} archives checked; 11 rebuilt; {len(events)} replacements; no protected member changes')


if __name__ == '__main__':
    main()
