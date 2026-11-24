#!/usr/bin/env python3
"""Fail on no-ship differences or any retained-unit tier/size/metadata change."""
from collections import Counter
import hashlib
import json
from pathlib import Path
from controls import check, ROOT, OUT


def read(name):
    return json.loads((OUT / name).read_text())


def require(condition, message):
    if not condition:
        raise AssertionError('HONEST-FINDING: ' + message)


def frozen(unit):
    return json.dumps(unit, sort_keys=True)


def security(report):
    return report['summary']['first_party']['security_relevant']


def headline(report):
    row = security(report)
    statuses = row['statuses']
    return (f"{statuses['proven-extracted']} proven-extracted, {statuses['tested']} tested, "
            f"{statuses['unverified']} unverified, {statuses['trusted']} trusted / "
            f"{row['sloc']} security SLoC; "
            f"{100 * statuses['proven-extracted'] / row['sloc']:.2f}% proven-extracted")


def main():
    lines = ['# v1.8.4 paired replay', '',
             'Baseline: tag `v1.8.3`; candidate: HEAD (v1.8.4).',
             'Bertie: archived `c66a986`, copied from `target/v183/bertie` into `target/v184/bertie`.',
             'Original CI log/receipt and all receipt-bound source, extraction and library hashes verified.',
             f"Both binaries built and run on current stable ({(OUT / 'toolchain.txt').read_text().strip()}), with RUSTUP_TOOLCHAIN unset.",
             f"Bertie target: {read('bertie-separate-before.json')['target']} (host); compiled-files, default features. "
             'Host dep-info exercises the shipped build; the original Linux CI first-party baseline is checked separately.',
             'Controls retain v183 targets, evidence and whole-crate settings. '
             'All replay mutations stay under target/v184; no external checkout is accessed.', '',
             'Shipped selection (temporarily appended to the archived config; restored after reporting):', '',
             '```toml', '[ship]', 'packages = ["bertie-tls"]', 'features = ["default"]',
             'reason = "Ship the bertie-tls library with default features"', '```', '',
             '## Controls without [ship]', '', *['- ' + row for row in check()], '']
    ci = read('ci-c66a986/report.json')
    for mode in ('separate', 'runtime'):
        a = read(f'bertie-{mode}-before.json')
        b = read(f'bertie-{mode}-after.json')
        c = read(f'bertie-{mode}-shipped.json')
        for suffix in ('json', 'md'):
            require((OUT / f'bertie-{mode}-before.{suffix}').read_bytes() ==
                    (OUT / f'bertie-{mode}-after.{suffix}').read_bytes(),
                    f'Bertie {mode} no-ship {suffix} differs')
        for report in (a, b, c):
            require(report['evidence_runs'] and all(r['accepted'] for r in report['evidence_runs']),
                    f'{mode}: rejected/missing receipt')
        # CI mode is separate; runtime legitimately changes ghost sizing.
        if mode == 'separate':
            accepted = read('v183-accepted-separate.json')
            require(Counter((u['id'], u['status']) for u in accepted['units']) ==
                    Counter((u['id'], u['status']) for u in a['units']), 'accepted v183 unit tiers')
            require(accepted['summary']['first_party'] == a['summary']['first_party'],
                    'accepted v183 first-party totals')
            require(security(ci)['sloc'] == 5402 and all(security(ci)['statuses'][key] == value
                    for key, value in {'proven-extracted': 3635, 'tested': 452,
                                       'unverified': 1197, 'trusted': 118}.items()), 'original CI baseline')
            lines += ['## Original CI and accepted v1.8.3 baseline', '',
                      f'Original CI: {headline(ci)}.',
                      f'Rebuilt tag v1.8.3: {headline(a)}.',
                      'Original CI report predates the v1.8.3 R1 fix. The rebuilt tag matches the accepted '
                      'v183 replay first-party totals and every unit tier: PASS. Original CI receipt and '
                      'source hashes are preserved; no old CI totals are relabelled as v1.8.3.', '']
        require(not any(w.startswith('ship:') for w in c['warnings']), f'{mode}: ship fallback')
        dropped = c['not_shipped']
        require(dropped, f'{mode}: no dropped files (scope not exercised)')
        paths = {f['file'] for f in dropped}
        retained = [u for u in b['units'] if u['file'] not in paths]
        require(retained == c['units'], f'{mode}: retained units changed (full identity, including tier/SLoC)')
        old_dropped = [u for u in b['units'] if u['file'] in paths]
        require(Counter(map(frozen, old_dropped)) == Counter(
            frozen(u) for f in dropped for u in f['units']), f'{mode}: dropped would-be units changed')
        require(set(c['source_files']) == set(b['source_files']) - paths, f'{mode}: source inventory')
        for key in ('approximation', 'binary_sha256', 'cargo_version', 'closure',
                    'closure_reason', 'config_schema', 'declared_external_dependencies',
                    'dependencies', 'build_dependencies', 'dependency_inventory_complete',
                    'dev_dependencies_excluded', 'fstar_extraction', 'fstar_proof',
                    'ghost_lines', 'no_proof_ratio_gate', 'overrides', 'proof_ratio_range',
                    'schema', 'size_fallbacks', 'size_mode', 'size_profile', 'target',
                    'upstream_claims'):
            require(b[key] == c[key], f'{mode}: {key} changed')
        for key in ('summary', 'summary_default_rules'):
            require(b[key]['third_party'] == c[key]['third_party'], f'{mode}: {key} third-party totals')
        removed = Counter()
        for unit in old_dropped:
            if unit['category'] == 'runtime' and unit['security']:
                removed[unit['status']] += unit['sloc']
        require(security(b)['sloc'] - security(c)['sloc'] == sum(removed.values()), f'{mode}: denominator')
        for status, count in security(b)['statuses'].items():
            require(count - security(c)['statuses'][status] == removed[status], f'{mode}: {status} arithmetic')
        lines += [f'## Bertie, {mode}', '',
                  'Without [ship]: exact JSON + Markdown equality PASS; accepted CI evidence PASS.',
                  f'Before: {headline(b)}.', f'With [ship]: {headline(c)}.',
                  f'Retained-units check PASS: all {len(retained)} entries identical in every JSON field '
                  '(all fields, including tier, SLoC, proof and trust metadata).',
                  'Dropped would-be units check PASS; dependency inventories and third-party totals unchanged PASS; security arithmetic PASS.',
                  'Build/target/configuration metadata, extraction/proof inputs and upstream claims unchanged PASS.',
                  f"Dependency size fallbacks: {len(b['size_fallbacks'])}, identical before/after/ship.",
                  f"Removed security runtime: {dict(sorted(removed.items()))} ({sum(removed.values())} SLoC).", '',
                  '| Dropped file | Whole-file SLoC | Security runtime SLoC by would-be tier | Other units: SLoC by category/tier |',
                  '| --- | ---: | --- | --- |']
        for item in dropped:
            sec, other = Counter(), Counter()
            for u in item['units']:
                if u['security'] and u['category'] == 'runtime':
                    sec[u['status']] += u['sloc']
                else:
                    other[f"{u['category']}/{u['status']}"] += u['sloc']
            describe = lambda counts: ', '.join(f'{tier}: {n}' for tier, n in sorted(counts.items())) or '0'
            lines.append(f"| `{item['file']}` | {item['sloc']} | {describe(sec)} | {describe(other)} |")
        lines += ['', 'Whole-file SLoC includes non-runtime source; unit SLoC is the coverage measure. '
                  'Empty files remain visible. The ~324-SLoC estimate is a guide, not an assertion.', '']
    lines += ['## Artifacts', '']
    for path in sorted(OUT.glob('bertie-*.json')):
        lines.append(f'- `{path.relative_to(ROOT)}` SHA-256 `{hashlib.sha256(path.read_bytes()).hexdigest()}`')
    lines += ['', '## Rerun', '', '```sh', 'python3 dev/v184/replay.py', '```', '',
              'Recreates target/v184, rebuilds both binaries and regenerates this comparison. '
              'Requires the existing v183 archived Bertie/CI inputs, release evidence archives and populated offline Cargo caches. '
              'Original CI evidence is replayed; no new F*/Verus verification is claimed. '
              'Checks are Codex-run; independent acceptance remains Andrew/Dudley’s.']
    (ROOT / 'dev/v184/compare.md').write_text('\n'.join(lines) + '\n')
    print('PASS: all controls, Bertie no-ship equality, CI baseline, retained units and dropped accounting (both ghost modes)')


if __name__ == '__main__':
    main()
