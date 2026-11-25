#!/usr/bin/env python3
"""Audit changed source units against named mapping forms; never tune to totals."""
from collections import Counter
import json
from controls import check, ROOT, OUT


def read(name):
    return json.loads((OUT / name).read_text())


def require(condition, message):
    if not condition:
        raise AssertionError('HONEST-FINDING: ' + message)


def security(report):
    return report['summary']['first_party']['security_relevant']


def headline(report):
    row = security(report)
    statuses = row['statuses']
    return (f"{statuses['proven-extracted']} proven-extracted, {statuses['tested']} tested, "
            f"{statuses['unverified']} unverified, {statuses['trusted']} trusted / "
            f"{row['sloc']} security runtime SLoC "
            f"({100 * statuses['proven-extracted'] / row['sloc']:.2f}% proven-extracted).")


def main():
    lines = ['# v1.9.1 paired replay', '',
             'Check (b) amended 2026-11-25: strictly checked extraction-only proof-code helpers',
             'may become verified under the unchanged v1.8.1 classifier; they contribute no runtime totals.', '',
             'Baseline: tag `v1.9.0`; candidate: v1.9.1 working tree.',
             'Bertie: staged bab3c13, Coverage run 45615087282, unchanged ship configuration.',
             'CI receipt/log bytes and receipt-bound hashes verified. Replay runs neither hax nor F*.',
             'Current stable, RUSTUP_TOOLCHAIN unset: ' + (OUT / 'toolchain.txt').read_text().strip() + '.', '',
             'Codex-run checks; independent Andrew/Dudley acceptance remains outstanding.', '']
    require(read('bertie-separate-before.json')['units'] == read('ci-report.json')['units'],
            '(a) baseline Bertie units differ from staged CI')
    lines += ['(a) Baseline units exactly equal CI units: PASS.', '']
    for mode in ('separate', 'runtime'):
        a, b = (read(f'bertie-{mode}-{label}.json') for label in ('before', 'after'))
        for report in (a, b):
            require(report['evidence_runs'] and all(r['accepted'] for r in report['evidence_runs']),
                    mode + ': missing/rejected evidence')
        identity = lambda u: (u['id'], u['file'], u['line'], u['end_line'])
        old, new = ({identity(u): u for u in report['units']} for report in (a, b))
        require(len(old) == len(a['units']) and len(new) == len(b['units']), 'duplicate source identities')
        require(old.keys() == new.keys(), mode + ': inventory changed')
        targets = {}
        for d in b['fstar_definitions']:
            reason = d.get('mapping_reason', '') or ''
            if reason.startswith('form ') and reason[5:6] in 'ABCDE' and d['rust_unit']:
                form = reason[5]
                if d['classification'] == ('trusted' if form == 'D' else 'proven-extracted'):
                    targets.setdefault(d['rust_unit'], []).append((form, d['name']))
        lines += [f'## Bertie {mode}', '', f'Before: {headline(a)}', '', f'After: {headline(b)}', '',
                  '| Rust unit (source line) | Old → new | SLoC | Form and definition |',
                  '| --- | --- | ---: | --- |']
        delta = Counter()
        changed = 0
        inactive_count = 0
        for key, before in old.items():
            after = new[key]
            uid = before['id']
            # Check (e) by source identity, never by id alone: twins share ids.
            inactive = ((before['name'] in ('aead_encrypt', 'aead_decrypt') and
                         '#[cfg(not(feature = "chacha20poly1305"))]' in
                         '\n'.join((OUT / 'bertie' / before['file']).read_text().splitlines()[max(0, before['line']-8):before['line']]))
                        or (before['name'] == 'tlserr' and
                            before['line'] > next(u['line'] for u in a['units'] if u['name'] == 'tlserr')))
            if inactive:
                inactive_count += 1
                require(before == after, f'(e) inactive cfg twin changed: {key}')
            if before == after:
                continue
            changed += 1
            require(uid in targets, f'(b) changed unit has no form A–E target: {key}')
            forms = {form for form, _ in targets[uid]}
            require(after['status'] == 'proven-extracted' or
                    (after['status'] == 'trusted' and forms == {'D'}) or
                    (after['status'] == 'verified' and
                     before['category'] == after['category'] == 'proof-code' and
                     forms and forms <= set('ABCDE')),
                    f'(b) unexpected destination: {key}: {after["status"]}')
            require(before['status'] in ('unverified', 'tested', 'unmapped') and
                    (after['status'] == 'proven-extracted' or before['status'] != 'tested'),
                    f'(c) tier loss or existing proof changed: {key}: {before["status"]} -> {after["status"]}')
            require({k: v for k, v in before.items() if k not in ('status', 'proof_scope')} ==
                    {k: v for k, v in after.items() if k not in ('status', 'proof_scope')},
                    f'(b) unrelated metadata change: {key}')
            runtime_delta_before = delta.copy()
            if before['security'] and before['category'] == 'runtime':
                delta[before['status']] -= before['sloc']
                delta[after['status']] += after['sloc']
            if 'proof-code' in (before['category'], after['category']):
                require(before['category'] == after['category'] == 'proof-code' and
                        delta == runtime_delta_before,
                        f'(b) proof-code change moved runtime totals: {key}')
            lines.append(f'| `{uid}` ({before["line"]}) | {before["status"]} → {after["status"]} | {before["sloc"]} | ' +
                         ', '.join(f'{form}: `{name}`' for form, name in sorted(set(targets[uid]))) + ' |')
        require(inactive_count == 3, f'(e) expected three inactive twins, found {inactive_count}')
        require(security(a)['sloc'] == security(b)['sloc'], mode + ': denominator changed')
        for status, count in security(a)['statuses'].items():
            require(security(b)['statuses'][status] - count == delta[status], mode + ': unexplained totals')
        lines += ['', f'{changed} changed source units.', '',
                  '(b) Every changed unit explained by form A–E: PASS.',
                  '(c) No tier loss; unchanged fields and totals arithmetic: PASS.',
                  '(e) Inactive cfg twins unchanged: PASS.', '']
    rows = check()
    lines += ['## Controls', '', *['- ' + row for row in rows], '',
              '(d) Exact JSON + Markdown equality: PASS.', '',
              '## Scope', '',
              'B stays unmapped: the receipt contains no explicit extraction feature/cfg set.',
              'D records retain their trusted F* classification; linking them attributes trust to previously unverified Rust units.',
              'Unsupported references, aliases, conditional blocks and generic binder identities remain unmapped.', '',
              'Rerun: `python3 dev/v191/replay.py` recreates target/v191 from staged inputs.', '']
    (ROOT / 'dev/v191/compare.md').write_text('\n'.join(lines))
    print('(a) baseline CI units: PASS\n(b) explained A–E targets: PASS\n(c) no tier loss: PASS\n(e) inactive cfg twins: PASS')
    print('\n'.join(rows))
    print('(d) exact controls: PASS')


if __name__ == '__main__':
    main()
