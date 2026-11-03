#!/usr/bin/env python3
"""Check the shipped Bertie dependency identities survived workspace lock extension."""
import json
from pathlib import Path
import sys
import tomllib


def identity(package):
    return package['name'], package['version'], package.get('source')


def resolve(reference, packages):
    text, _, source = reference.partition(' (')
    fields = text.split()
    candidates = [p for p in packages if p['name'] == fields[0]
                  and (len(fields) == 1 or p['version'] == fields[1])
                  and (not source or p.get('source') == source.rstrip(')'))]
    assert len(candidates) == 1, (reference, len(candidates))
    return candidates[0]


def audit(old, new):
    known = {identity(p): p for p in old}
    current = {identity(p): p for p in new}
    shipped = next(p for p in old if p['name'] == 'bertie')
    local = next(p for p in new if p['name'] == 'bertie' and 'source' not in p)
    selected = {identity(resolve(ref, new)) for ref in local.get('dependencies', [])}
    pending = [resolve(ref, old) for ref in shipped.get('dependencies', [])]
    direct = {identity(p) for p in pending}
    missing = direct - selected
    closure = set()
    while pending:
        package = pending.pop()
        key = identity(package)
        if key in closure:
            continue
        closure.add(key)
        pending.extend(resolve(ref, old) for ref in package.get('dependencies', []))
    changed = [key for key in closure if key not in current
               or known[key].get('checksum') != current[key].get('checksum')]
    return {'missing_or_changed_direct_bertie_dependencies': sorted(missing, key=str),
            'missing_or_changed_shipped_transitives': sorted(changed, key=str),
            'shipped_bertie_dependency_closure': sorted(closure, key=str),
            'additional_workspace_dev_packages': [p for p in new if identity(p) not in known and p['name'] != 'bertie'],
            'libcrux': [p for p in new if p['name'].startswith('libcrux')]}


if __name__ == '__main__':
    old, new = [tomllib.loads(Path(p).read_text())['package'] for p in sys.argv[1:]]
    result = audit(old, new)
    print(json.dumps(result, indent=2))
    sys.exit(bool(result['missing_or_changed_direct_bertie_dependencies'] or result['missing_or_changed_shipped_transitives']))
