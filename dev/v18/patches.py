#!/usr/bin/env python3
"""Independent Bertie patch audit: added code lines per exact target module.

No Rust-tool imports. Pinned Bertie's legacy Bertie.* targets are absent from
its Bertie_tls.* receipt; require that fact explicitly rather than renaming.
"""
import argparse
from collections import defaultdict
import json
from pathlib import Path


def added_code(lines):
    depth, string = 0, False
    count = 0
    for added, text in lines:
        i, code = 0, False
        while i < len(text):
            if depth:
                if text.startswith('(*', i):
                    depth, i = depth+1, i+2
                elif text.startswith('*)', i):
                    depth, i = depth-1, i+2
                else:
                    i += 1
            elif string:
                code = True
                if text[i] == '\\':
                    i += 2
                elif text[i] == '"':
                    string, i = False, i+1
                else:
                    i += 1
            elif text.startswith('//', i):
                break
            elif text.startswith('(*', i):
                depth, i = 1, i+2
            else:
                code |= not text[i].isspace()
                string = text[i] == '"'
                i += 1
        count += added and code
    return count


def main():
    p = argparse.ArgumentParser()
    p.add_argument('root', type=Path)
    p.add_argument('report', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    patch = a.root / 'proofs/fstar/extraction-panic-free.patch'
    receipt = json.loads((a.root / 'target/ci-logs/fstar/fstar.log.evidence.json').read_text())
    report = json.loads(a.report.read_text())
    assert any(r['tool'] == 'hax-fstar' and r['accepted'] for r in report['evidence_runs'])
    names = {Path(f).name for f in receipt['checked']}
    raw, hunk_counts, eligible = defaultdict(int), defaultdict(int), defaultdict(int)
    target, hunk, content = '', None, []
    hunks = []

    def finish():
        if hunk is None:
            return
        module = Path(target).stem
        count = added_code(content)
        raw[module] += count
        hunk_counts[module] += 1
        # This acceptance audit is deliberately pinned to the actual input. If a
        # later patch target appears in a receipt, extend the independent strict
        # check explicitly rather than pretending this absent-target audit applies.
        assert Path(target).name not in names, ('new matching target requires independent strict check', target)
        hunks.append(dict(target=target, module=module, line=hunk, raw_added_sloc=count, eligible_sloc=0, reason='module absent from accepted receipt'))
        eligible[module] += 0

    for i, line in enumerate(patch.read_text().splitlines(), 1):
        if line.startswith('+++ '):
            finish()
            hunk, content = None, []
            target = line[4:].split('\t')[0]
        elif line.startswith('@@ '):
            finish()
            hunk, content = i, []
        elif hunk is not None and line[:1] in ['+', ' ']:
            content.append((line[0] == '+', line[1:]))
    finish()
    tool = defaultdict(int)
    entries = [i for i in report['fstar_proof']['inputs'] if i['kind'] == 'patch']
    for i in entries:
        tool[i['target_module']] += i['sloc']
        assert i['reason'] == 'module absent from accepted receipt'
    assert dict(tool) == dict(eligible)
    assert len(entries) == len(hunks) == len(report['proof_ratio']['excluded_patch_hunks'])
    assert report['proof_ratio']['fstar_patch_sloc'] == 0
    a.output.write_text(json.dumps(dict(raw_added_sloc=dict(raw), eligible_sloc=dict(eligible), hunks_by_module=dict(hunk_counts), hunks=hunks), indent=2)+'\n')
    print(f'PASS: {len(hunks)} excluded hunks; {sum(raw.values())} raw added code lines across {len(raw)} modules; exact eligible count 0 for every module')


if __name__ == '__main__':
    main()
