#!/usr/bin/env python3
"""Static archived-model inventory, explicitly not fresh proof credit."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re


def mask(text):
    result = list(text)
    i, depth, string = 0, 0, False
    while i < len(text):
        if not string and text[i:i+2] == '(*':
            depth += 1
            result[i:i+2] = '  '
            i += 2
            continue
        if depth and text[i:i+2] == '*)':
            depth -= 1
            result[i:i+2] = '  '
            i += 2
            continue
        if not depth and not string and text[i:i+2] == '//':
            while i < len(text) and text[i] != '\n':
                result[i] = ' '
                i += 1
            continue
        if not depth and text[i] == '"':
            string = not string
            result[i] = ' '
            i += 1
            continue
        if string and text[i] == '\\':
            result[i] = ' '
            i += 1
            if i < len(text):
                if text[i] != '\n':
                    result[i] = ' '
                i += 1
            continue
        if (depth or string) and text[i] != '\n':
            result[i] = ' '
        i += 1
    assert not depth and not string
    return ''.join(result)


def inventory(root):
    output = {'method': 'static archived upstream models, no fresh extraction and no proof credit',
              'modules': {}, 'files': {}, 'worked_examples': {}}
    for tree in ['extraction', 'extraction-lax', 'extraction-panic-free']:
        directory = root / 'proofs/fstar' / tree
        for path in sorted(directory.glob('*.fst*')):
            raw = path.read_text()
            text = mask(raw)
            relative = path.relative_to(root).as_posix()
            escapes = [{'file': relative, 'line': text[:m.start()].count('\n')+1, 'kind': m.group()}
                       for m in re.finditer(r'\b(?:assume|admit|admitP)\b', text)]
            decls = list(re.finditer(r'^(let|val|assume val) ([A-Za-z_][A-Za-z_0-9]*)\b', text, re.M))
            baseline = root / 'proofs/fstar/extraction' / path.name
            altered = not baseline.exists() or baseline.read_bytes() != path.read_bytes()
            sibling = path.with_suffix('.fst')
            bodies = set(re.findall(r'^let ([A-Za-z_][A-Za-z_0-9]*)\b', mask(sibling.read_text()), re.M)) if sibling.exists() else set()
            trusted = []
            contracts = Counter()
            for i, decl in enumerate(decls):
                syntax, name = decl.groups()
                if name == '_':
                    continue
                body = text[decl.start():decls[i+1].start() if i+1 < len(decls) else len(text)]
                line = text[:decl.start()].count('\n')+1
                if altered or syntax == 'assume val' or (syntax == 'val' and name not in bodies):
                    trusted.append({'file': relative, 'line': line, 'name': name,
                                    'kind': 'patch-altered module' if altered else 'interface-only/assume val'})
                if path.suffix != '.fsti':
                    continue
                if '(ensures' in body:
                    kind = 'functional'
                elif 'Prims.l_True' in body or re.search(r'\bTot\b', body):
                    kind = 'safety-only'
                else:
                    kind = 'unclassified'
                contracts[kind] += 1
                if name in ['certificate_verify', 'ecdsa_signature']:
                    output['worked_examples'][f'{tree}/{name}'] = {
                        'file': relative, 'line': line, 'contract_kind': kind,
                        'self_consistency_heuristic': name == 'certificate_verify' and 'parse_certificate_verify' in body,
                        'postcondition_or_signature': body.strip(), 'credited': False}
            output['files'][relative] = {'sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
                                       'trusted_sites': trusted, 'escapes': escapes,
                                       'contract_kinds': dict(contracts), 'patch_altered_from_tracked_extraction': altered}
            module = f'{tree}/{path.stem}'
            row = output['modules'].setdefault(module, {'proven_extracted_sloc': 0,
                        'trusted_sites': 0, 'escapes': 0, 'contract_kinds': {}})
            row['trusted_sites'] += len(trusted)
            row['escapes'] += len(escapes)
            for key, value in contracts.items():
                row['contract_kinds'][key] = row['contract_kinds'].get(key, 0) + value
    return output


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('source', type=Path)
    parser.add_argument('out', type=Path)
    args = parser.parse_args()
    args.out.write_text(json.dumps(inventory(args.source), indent=2)+'\n')
