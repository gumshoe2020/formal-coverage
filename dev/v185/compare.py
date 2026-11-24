#!/usr/bin/env python3
"""Audit every changed unit and stop on unexplained changes or tier losses."""
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
    tiers = row['statuses']
    return (f"{tiers['proven-extracted']} proven-extracted, {tiers['tested']} tested, "
            f"{tiers['unverified']} unverified, {tiers['trusted']} trusted / "
            f"{row['sloc']} security runtime SLoC "
            f"({100 * tiers['proven-extracted'] / row['sloc']:.2f}% proven-extracted).")


def main():
    lines = ['# v1.8.5 paired replay', '',
             'Baseline: tag `v1.8.4`; candidate: v1.8.5 working tree.',
             'Bertie: staged 9617409 with its own unchanged [ship] configuration.',
             'Original CI receipt/log bytes and all receipt-bound hashes verified; no hax or F* executed.',
             'Both binaries use current stable with RUSTUP_TOOLCHAIN unset: ' +
             (OUT / 'toolchain.txt').read_text().strip() + '.', '',
             'Checks are Codex-run; Andrew/Dudley independent acceptance remains outstanding.', '']
    ci = read('ci-report.json')
    a = read('bertie-separate-before.json')
    require(a['units'] == ci['units'], '(a) baseline Bertie units differ from original CI')
    lines += ['(a) Baseline units exactly equal CI units: PASS.', '']
    for mode in ('separate', 'runtime'):
        a = read(f'bertie-{mode}-before.json')
        b = read(f'bertie-{mode}-after.json')
        for report in (a, b):
            require(report['evidence_runs'] and all(r['accepted'] for r in report['evidence_runs']),
                    mode + ': missing/rejected evidence')
        # cfg alternatives can legitimately share an id; preserve each source unit.
        identity = lambda u: (u['id'], u['file'], u['line'], u['end_line'])
        old = {identity(u): u for u in a['units']}
        new = {identity(u): u for u in b['units']}
        require(len(old) == len(a['units']) and len(new) == len(b['units']), 'duplicate source unit identities')
        require(old.keys() == new.keys(), mode + ': inventory changed')
        numbered = {}
        for d in b['fstar_definitions']:
            name = d['name'].rsplit('.', 1)[-1]
            rest = name.removeprefix('impl_')
            n, sep, method = rest.partition('__')
            if name.startswith('impl_') and sep and n.isascii() and n.isdigit() and method and d['rust_unit']:
                numbered.setdefault(d['rust_unit'], []).append(d['name'])
        lines += [f'## Bertie {mode}', '',
                  f"Before: {headline(a)}", '', f"After: {headline(b)}", '',
                  '| Rust unit | Old → new | SLoC | Numbered definition |',
                  '| --- | --- | ---: | --- |']
        delta = Counter()
        for key, before in old.items():
            uid = before['id']
            after = new[key]
            if before == after:
                continue
            require(before['status'] in ('unmapped', 'tested', 'unverified') and
                    after['status'] in ('proven-extracted', 'trusted'),
                    f'(b)/(c) {mode}: unexplained tier change {uid}: {before["status"]} -> {after["status"]}')
            require(uid in numbered, f'(b) {mode}: changed unit is not numbered target: {uid}')
            require({k: v for k, v in before.items() if k not in ('status', 'proof_scope')} ==
                    {k: v for k, v in after.items() if k not in ('status', 'proof_scope')},
                    f'(b) {mode}: unrelated unit metadata change: {uid}')
            if before['security'] and before['category'] == 'runtime':
                delta[before['status']] -= before['sloc']
                delta[after['status']] += after['sloc']
            lines.append(f'| `{uid}` | {before["status"]} → {after["status"]} | {before["sloc"]} | ' +
                         ', '.join(f'`{n}`' for n in sorted(set(numbered[uid]))) + ' |')
        require(security(a)['sloc'] == security(b)['sloc'], mode + ': denominator changed')
        for status, value in security(a)['statuses'].items():
            require(security(b)['statuses'][status] - value == delta[status], mode + ': totals unexplained')
        lines += ['', '(b) Every changed unit is an eligible numbered target: PASS.',
                  '(c) No unit lost a tier; unchanged unit fields and totals arithmetic: PASS.', '']
    rows = check()
    lines += ['## Controls', '', *['- ' + row for row in rows], '',
              '(d) All v1.8.4 controls exact JSON + Markdown equality: PASS.', '',
              '## Rerun', '', '`python3 dev/v185/replay.py` rebuilds both binaries and replaces target/v185.',
              'Requires staged target/v185-inputs, release archives and populated control Cargo caches.',
              'Host target is used for replay; check (a) compares every full CI unit without normalization.', '']
    (ROOT / 'dev/v185/compare.md').write_text('\n'.join(lines))
    print('(a) baseline CI units: PASS\n(b) explained numbered targets: PASS\n(c) no tier loss: PASS')
    print('\n'.join(rows))
    print('(d) exact controls: PASS')


if __name__ == '__main__':
    main()
