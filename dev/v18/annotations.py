#!/usr/bin/env python3
"""Independent lexical source count for pinned Bertie (no tool-code imports).

The whitelist is independently transcribed from the approved categories. Each
counted source line and each changed unit is retained for review.
"""
import argparse
import json
import re
from pathlib import Path

ALLOWED = set('requires ensures decreases loop_invariant fstar opaque attributes lemma assert assume implies forall exists int prop'.split())
ALLOWED.update('fstar::' + s for s in 'before after replace replace_body options expr prop unsafe_expr'.split())
ALLOWED.update(['int::Int', 'int::Int::new', 'int::Int::pow2', 'prop::Prop', 'prop::Prop::from_bool'])
ALLOWED.update('prop::constructors::' + s for s in 'from_bool and or not eq ne implies forall exists'.split())


def tokens(src):
    result = []
    i, line = 0, 1
    while i < len(src):
        start, first = i, line
        if src[i].isspace():
            line += src[i] == '\n'
            i += 1
            continue
        if src.startswith('//', i):
            end = src.find('\n', i)
            i = len(src) if end < 0 else end
            continue
        if src.startswith('/*', i):
            depth, i = 1, i + 2
            while depth:
                assert i < len(src)
                if src.startswith('/*', i):
                    depth, i = depth + 1, i + 2
                elif src.startswith('*/', i):
                    depth, i = depth - 1, i + 2
                else:
                    line += src[i] == '\n'
                    i += 1
            continue
        raw = re.match(r'(?:br|cr|r)(#*)"', src[i:])
        char = re.match(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'", src[i:])
        if raw:
            end = src.find('"' + raw[1], i + raw.end())
            assert end >= 0
            i = end + 1 + len(raw[1])
        elif src[i] == '"':
            i += 1
            while src[i] != '"':
                i += 2 if src[i] == '\\' else 1
            i += 1
        elif char:
            i += char.end()
        else:
            ident = re.match(r'[A-Za-z_][A-Za-z_0-9]*', src[i:])
            i += ident.end() if ident else 1
        line += src[start:i].count('\n')
        result.append((src[start:i], first, line))
    return result


def scan(src):
    ts = tokens(src)
    matching, stack = {}, []
    for i, (t, _, _) in enumerate(ts):
        if t in ('(', '[', '{'):
            stack.append(i)
        elif t in (')', ']', '}'):
            j = stack.pop()
            assert ts[j][0] == {')': '(', ']': '[', '}': '{'}[t]
            matching[j] = i
    aliases = {'hax_lib', 'hax'}
    aliases.update(re.findall(r'\buse\s+hax_lib\s+as\s+(\w+)', src))
    ghost, runtime = set(), set()

    def path(i):
        parts = []
        while i < len(ts) and re.fullmatch(r'[A-Za-z_]\w*', ts[i][0]):
            parts.append(ts[i][0])
            i += 1
            if i + 1 < len(ts) and ts[i][0] == ts[i+1][0] == ':':
                i += 2
            else:
                break
        return parts, i

    def known(parts):
        return len(parts) > 1 and parts[0] in aliases and '::'.join(parts[1:]) in ALLOWED

    def attr(i, end):
        parts, j = path(i)
        if known(parts):
            return True
        if parts == ['cfg_attr'] and j in matching:
            k = j + 1
            while k < matching[j] and ts[k][0] != ',':
                k = matching.get(k, k) + 1
            return k < matching[j] and attr(k+1, matching[j])
        return False

    i = 0
    while i < len(ts):
        end, proof = i + 1, False
        t = ts[i][0]
        if t == '#':
            j = i + 1
            if ts[j][0] == '!':
                j += 1
            if ts[j][0] == '[':
                if ts[j+1][0] == 'doc':
                    i = matching[j] + 1
                    continue
                if attr(j+1, matching[j]):
                    end, proof = matching[j] + 1, True
        if not proof:
            parts, j = path(i)
            if known(parts):
                if ts[j][0] == '!':
                    j += 1
                if j in matching:
                    end, proof = matching[j] + 1, True
                    if end < len(ts) and ts[end][0] == ';':
                        end += 1
        dest = ghost if proof else runtime
        for _, first, last in ts[i:end]:
            dest.update(range(first, last+1))
        i = end
    return ghost - runtime


def main():
    p = argparse.ArgumentParser()
    p.add_argument('root', type=Path)
    p.add_argument('before', type=Path)
    p.add_argument('after', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    before, after = (json.loads(p.read_text()) for p in (a.before, a.after))
    old = {(u['id'], u['file'], u['line']): u for u in before['units']}
    files, rows, changes = {}, [], []
    security = total = 0
    for u in after['units']:
        prev = old[(u['id'], u['file'], u['line'])]
        assert prev['status'] == u['status'], f"HONEST-FINDING: {u['id']} tier changed"
        if u['first_party'] and u['category'] == 'runtime':
            if u['file'] not in files:
                files[u['file']] = sorted(scan((a.root / u['file']).read_text()))
            lines = [n for n in files[u['file']] if u['line'] <= n <= u['end_line']]
            assert len(lines) == u['fstar_annotation_sloc'], (u['id'], lines, u['fstar_annotation_sloc'])
            assert prev['sloc'] - u['sloc'] == len(lines), (u['id'], 'size delta')
            rows.append({'unit': u['id'], 'file': u['file'], 'lines': lines})
            total += len(lines)
            if u['security']:
                security += len(lines)
        else:
            assert prev['sloc'] == u['sloc']
        if prev['sloc'] != u['sloc']:
            changes.append({'unit': u['id'], 'file': u['file'], 'line': u['line'], 'before': prev['sloc'], 'after': u['sloc'], 'status': u['status']})
    assert security == after['proof_ratio']['fstar_annotation_sloc']
    assert total == after['proof_ratio']['all_first_party']['fstar_annotation_sloc']
    a.output.write_text(json.dumps(dict(security=security, all_first_party=total, units=rows, changed_sizes=changes), indent=2)+'\n')
    print(f'PASS: exact independent match: security {security}, all first-party {total}; {len(changes)} size changes; no tier changes')


if __name__ == '__main__':
    main()
