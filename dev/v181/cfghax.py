#!/usr/bin/env python3
"""Independent lexical cfg(hax) audit. Never imports formal-coverage code.

Uses the v1.8 independent Rust lexer; cfg parsing, region discovery and ownership
are separate Python implementations. Outputs exact source lines per unit.
"""
import argparse
import json
from pathlib import Path
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'v18'))
from annotations import tokens, ALLOWED
import re


def regions(source, atom="hax"):
    ts = tokens(source)
    pairs, stack = {}, []
    for i, (t, _, _) in enumerate(ts):
        if t in ['(', '[', '{']:
            stack.append(i)
        elif t in [')', ']', '}']:
            start = stack.pop()
            pairs[start] = i
    def implies(start, end):
        words = [t[0] for t in ts[start:end]]
        if words == [atom]:
            return True
        if start + 1 >= end or words[0] not in ['all', 'any'] or ts[start+1][0] != '(':
            return False
        if pairs[start+1] != end-1:
            return False
        args, k, begin = [], start+2, start+2
        while k < end-1:
            if ts[k][0] == ',':
                args.append(implies(begin, k))
                begin = k+1
            k = pairs.get(k, k)+1
        if begin < end-1:
            args.append(implies(begin, end-1))
        return any(args) if words[0] == 'all' else bool(args) and all(args)
    result = []
    for i, (t, _, _) in enumerate(ts):
        if t != '#' or i+4 >= len(ts) or ts[i+1][0] != '[':
            continue
        end = pairs[i+1]
        is_test = atom == 'test' and [t[0] for t in ts[i+2:end]] == ['test']
        if not is_test and (ts[i+2][0] != 'cfg' or ts[i+3][0] != '(' or not implies(i+4, pairs[i+3])):
            continue
        k = end+1
        # Skip all subsequent outer attributes before finding the guarded construct.
        while k < len(ts) and ts[k][0] == '#' and ts[k+1][0] == '[':
            k = pairs[k+1]+1
        begin = k
        while k < len(ts):
            if ts[k][0] == ';':
                break
            if ts[k][0] == '{':
                k = pairs[k]
                if k+1 < len(ts) and ts[k+1][0] == ';':
                    k += 1
                break
            k = pairs.get(k, k)+1
        assert k < len(ts), (i, 'unterminated guarded construct')
        result.append((i, begin, k))
    return ts, pairs, result


def code_lines(ts, pairs, start, end):
    result, i = set(), start
    while i <= end:
        if ts[i][0] == '#' and i+2 < len(ts) and ts[i+1][0] == '[' and ts[i+2][0] == 'doc':
            i = pairs[i+1]+1
            continue
        result.update(range(ts[i][1], ts[i][2]+1))
        i += 1
    return result


def ownership(ts, pairs, regs):
    """Independently mark lexical annotation/cfg regions, then resolve shared lines."""
    aliases = {'hax_lib', 'hax'}
    for i,t in enumerate(ts[:-3]):
        if t[0] == 'use' and ts[i+1][0] in aliases and ts[i+2][0] == 'as':
            aliases.add(ts[i+3][0])
    def path(i):
        parts=[]
        while i < len(ts) and re.fullmatch(r'[A-Za-z_]\w*', ts[i][0]):
            parts.append(ts[i][0]); i+=1
            if i+1 < len(ts) and ts[i][0] == ts[i+1][0] == ':': i+=2
            else: break
        return parts,i
    def known(parts):
        return len(parts)>1 and parts[0] in aliases and '::'.join(parts[1:]) in ALLOWED
    def attr(i):
        parts,j=path(i)
        if known(parts): return True
        if parts == ['cfg_attr'] and j in pairs:
            k=j+1
            while k < pairs[j] and ts[k][0] != ',': k=pairs.get(k,k)+1
            return k < pairs[j] and attr(k+1)
        return False
    ann,docs=set(),set()
    i=0
    while i < len(ts):
        end=i
        if ts[i][0] == '#' and i+2 < len(ts) and ts[i+1][0] == '[':
            if ts[i+2][0] == 'doc':
                docs.update(range(i,pairs[i+1]+1)); i=pairs[i+1]+1; continue
            if attr(i+2): end=pairs[i+1]
        if end == i:
            parts,j=path(i)
            if known(parts) and j < len(ts):
                if ts[j][0] == '!': j+=1
                if j in pairs:
                    end=pairs[j]
                    if end+1 < len(ts) and ts[end+1][0] == ';': end+=1
        if end > i: ann.update(range(i,end+1))
        i=end+1
    cfg=set().union(*(set(range(start,end+1)) for start,_,end in regs)) if regs else set()
    def lines(indices):
        return set().union(*(set(range(ts[i][1],ts[i][2]+1)) for i in indices-docs)) if indices-docs else set()
    runtime=lines(set(range(len(ts)))-cfg-ann-docs)
    return lines(cfg)-runtime, lines(ann)-runtime, lines(cfg|ann)-runtime


def audit(root, before, after, inline=False):
    old = {(u['id'], u['file'], u['line']): u for u in before['units']}
    cache, moved, line_rows = {}, [], []
    annotation_totals = {False: 0, True: 0}
    matched_signatures=set()
    for u in after['units']:
        key = (u['id'], u['file'], u['line'])
        a = old[key]
        if u['file'] not in cache:
            source = (root/u['file']).read_text()
            lexed = regions(source)
            test_regs = regions(source, 'test')[2]
            ts, pairs, regs = lexed
            regs = [r for r in regs if not any(begin <= r[0] <= end for _, begin, end in test_regs)]
            cache[u['file']] = (ts, pairs, regs, ownership(ts, pairs, regs), test_regs)
        ts, pairs, regs, owned, test_regs = cache[u['file']]
        # Locate the signature independently, then ask if a guarded item encloses it.
        signatures = [i for i,t in enumerate(ts[:-1]) if t[0] == 'fn' and ts[i+1][0] == u['name'] and u['line'] <= t[1] <= u['end_line']]
        assert signatures, u['id']
        signature = signatures[0]
        test_scoped = any(begin <= signature <= end for _,begin,end in test_regs)
        guarded = not test_scoped and any(begin <= signature <= end for _,begin,end in regs)
        assert guarded == u.get('fstar_cfg_hax', False), (u['id'], guarded)
        if guarded:
            matched_signatures.add((u['file'],u['name'],ts[signature][1]))
            assert a['category'] == 'runtime' and u['category'] == 'proof-code', u['id']
            assert u['status'] != 'hazard', f"HONEST-FINDING: cfg(hax) hazard {u['id']}"
            expected_status = 'verified' if a['status'] == 'proven-extracted' else a['status']
            assert u['status'] == expected_status, f"HONEST-FINDING: {u['id']} status"
            # Count source tokens, not report deltas; discard explicit docs.
            selected = [i for i,t in enumerate(ts) if u['line'] <= t[1] <= u['end_line']]
            lines = code_lines(ts, pairs, selected[0], selected[-1])
            assert len(lines) == u['sloc'], (u['id'], sorted(lines), u['sloc'])
            assert u['fstar_annotation_sloc'] == 0
            moved.append(dict(unit=u['id'], file=u['file'], line=u['line'], lines=sorted(lines), before=a, after=u))
        else:
            assert a['status'] == u['status'], f"HONEST-FINDING: {u['id']} tier"
            assert a['category'] == u['category'], f"HONEST-FINDING: {u['id']} category"
            if inline and u['category'] == 'runtime':
                cfg,ann,union = [{n for n in lines if u['line'] <= n <= u['end_line']} for lines in owned]
                assert len(cfg) == u.get('fstar_cfg_hax_inline_sloc', 0), (u['id'], sorted(cfg), u.get('fstar_cfg_hax_inline_sloc', 0))
                assert len(ann) == u['fstar_annotation_sloc'], (u['id'], sorted(ann), u['fstar_annotation_sloc'])
                annotation_totals[False] += len(ann)
                if u['security']: annotation_totals[True] += len(ann)
                mode = after['ghost_lines']
                expected_inline = len(union) if mode == 'separate' else 0
                assert u['inline_proof_sloc'] == expected_inline, (u['id'], sorted(union))
                assert u['ghost_sloc'] == len(union), (u['id'], 'raw ghost lines', sorted(union))
                old_inline = a['inline_proof_sloc']
                assert a['sloc'] + old_inline == u['sloc'] + expected_inline, f"HONEST-FINDING: {u['id']} size"
                if cfg:
                    line_rows.append(dict(unit=u['id'],file=u['file'],security=u['security'],lines=sorted(cfg),annotations=sorted(ann),union=sorted(union)))
            else:
                assert a['sloc'] == u['sloc'], f"HONEST-FINDING: {u['id']} size"
    source_signatures=set()
    for file,(ts,_,regs,_,test_regs) in cache.items():
        for i,t in enumerate(ts[:-1]):
            if t[0]=='fn' and re.fullmatch(r'[A-Za-z_]\w*',ts[i+1][0]) and any(begin <= i <= end for _,begin,end in regs) and not any(begin <= i <= end for _,begin,end in test_regs):
                source_signatures.add((file,ts[i+1][0],t[1]))
    assert source_signatures == matched_signatures, ('independent source function set', sorted(source_signatures-matched_signatures), sorted(matched_signatures-source_signatures))
    assert after['summary']['proof_code']['total']['hazard_units'] == before['summary']['proof_code']['total']['hazard_units'], 'HONEST-FINDING: proof hazard gate'
    if inline:
        for security, metrics in [(True, after['proof_ratio']), (False, after['proof_ratio']['all_first_party'])]:
            standalone=sum(len(row['lines']) for row in moved if not security or row['after']['security'])
            inline_count=sum(len(row['lines']) for row in line_rows if not security or row['security'])
            assert metrics['fstar_cfg_hax_sloc'] == standalone
            assert metrics['fstar_cfg_hax_inline_sloc'] == (inline_count if after['ghost_lines']=='separate' else 0)
            assert metrics['fstar_annotation_sloc'] == (annotation_totals[security] if after['ghost_lines']=='separate' else 0)
    return dict(moved=moved, inline=line_rows, standalone_sloc=sum(len(u['lines']) for u in moved), inline_sloc=sum(len(u['lines']) for u in line_rows))


def main():
    p = argparse.ArgumentParser()
    p.add_argument('root', type=Path)
    p.add_argument('before', type=Path)
    p.add_argument('after', type=Path)
    p.add_argument('output', type=Path)
    p.add_argument('--inline', action='store_true')
    a=p.parse_args()
    result=audit(a.root, json.loads(a.before.read_text()), json.loads(a.after.read_text()), a.inline)
    a.output.write_text(json.dumps(result, indent=2)+'\n')
    print(f"PASS: {len(result['moved'])} independent cfg(hax) units, {result['standalone_sloc']} standalone SLoC, {result['inline_sloc']} cfg inline SLoC")

if __name__ == '__main__':
    main()
