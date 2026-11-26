#!/usr/bin/env python3
"""Replay staged bertie-tls evidence through formal-coverage v1.10.0 and the working tree.

Never runs hax or F*. Builds the baseline from a git worktree of tag v1.10.0, runs both
binaries over the same inputs in both ghost-line modes, checks (a)-(e) from
dev/v1110/PLAN.md and writes dev/v1110/compare.md. Exit 0 only when every check passes.
"""
import hashlib
import json
import os
from datetime import datetime, timezone
from pathlib import Path
import shutil
import subprocess
import sys

REPO = Path(__file__).resolve().parents[2]
OUT = REPO / 'target/v1110'
INPUTS = REPO / 'target/v1110-inputs/bertie'
COMPARE = REPO / 'dev/v1110/compare.md'
NOTES = REPO / 'dev/v1110/NOTES.md'
MODES = ('separate', 'runtime')
CRATE = 'bertie_tls'
SAFETY, SELF, FUNCTIONAL = 'safety-only', 'functional, checked only against own code', 'functional'

# (unit, expected kind, expected self-consistency or None for n/a, required)
CONTROLS = (
    [(f'tls13formats::{n}', 'functional', True, True)
     for n in ('client_hello', 'server_hello', 'server_certificate', 'certificate_verify', 'finished')]
    + [('der::decode_ecdsa_signature', 'functional', True, True),
       ('der::encode_ecdsa_signature', 'functional', True, False),
       ('der::encode_uint', 'functional', True, True),
       ('der::skip_zeros', 'functional', False, True),
       ('der::uint_to_32_bytes', 'functional', True, True)]
    + [(f'tls13utils::{n}', 'functional', False, True)
       for n in ('check_eq', 'check_eq1', 'check_eq_slice', 'check_eq_option', 'check_eq_inner', 'eq_slice')]
    + [(f'tls13formats::{n}', 'safety-only', None, True)
       for n in ('parse_client_hello', 'check_extension', 'check_server_extension')]
    + [(f'tls13utils::{n}', 'safety-only', None, True)
       for n in ('length_u24_encoded', 'encode_length_u16', 'bytes2', 'Bytes::len', 'Bytes::concat',
                 'check_eq_with_slice')]
    + [('tls13crypto::HashAlgorithm::hash_len', 'safety-only', None, True),
       ('tls13cert::has_oid', 'safety-only', None, True),
       ('tls13record::padlen', 'safety-only', None, True)]
)


def run(args, log, cwd=REPO):
    log.parent.mkdir(parents=True, exist_ok=True)
    with log.open('w') as stream:
        stream.write('$ ' + ' '.join(map(str, args)) + '\n')
        stream.flush()
        result = subprocess.run(list(map(str, args)), cwd=cwd, stdout=stream,
                                stderr=subprocess.STDOUT)
    if result.returncode:
        raise RuntimeError(f'exit {result.returncode}: {log.relative_to(REPO)}')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def reset_output():
    worktree = OUT / 'baseline-src'
    if worktree.exists():
        subprocess.run(['git', 'worktree', 'remove', '--force', worktree], cwd=REPO, check=True)
    if OUT.exists():
        shutil.rmtree(OUT)
    subprocess.run(['git', 'worktree', 'prune'], cwd=REPO, check=True)
    OUT.mkdir(parents=True)


def check_staged_inputs(root):
    evidence = root / 'target/ci-logs/fstar'
    receipt = json.loads((evidence / 'fstar.log.evidence.json').read_text())
    assert digest(evidence / 'fstar.log') == receipt['log_sha256'], 'fstar.log digest'
    for name, expected in receipt['files'].items():
        assert digest(root / name) == expected, name
    for field in ('fresh', 'checked', 'patches'):
        for name, expected in receipt[field].items():
            assert digest(evidence / name) == expected, name
    assert '[ship]' in (root / 'formal-coverage.toml').read_text()


def prepare():
    if 'RUSTUP_TOOLCHAIN' in os.environ:
        raise RuntimeError('Run with RUSTUP_TOOLCHAIN unset')
    reset_output()
    check_staged_inputs(INPUTS)
    source = OUT / 'baseline-src'
    run(['git', 'worktree', 'add', '--detach', source, 'v1.10.0'], OUT / 'worktree.log')
    run(['cargo', 'build', '--release', '--locked', '--manifest-path', source / 'Cargo.toml',
         '--target-dir', OUT / 'baseline-build'], OUT / 'baseline-build.log')
    run(['cargo', 'build', '--release', '--locked', '--target-dir', OUT / 'candidate-build'],
        OUT / 'candidate-build.log')
    for label in ('baseline', 'candidate'):
        shutil.copy2(OUT / f'{label}-build/release/formal-coverage', OUT / label)
    shutil.copytree(INPUTS, OUT / 'bertie', ignore=shutil.ignore_patterns('target', '.git'))
    shutil.copytree(INPUTS / 'target/ci-logs/fstar', OUT / 'bertie/target/ci-logs/fstar')


def generate():
    host = next(line[6:] for line in subprocess.check_output(
        ['rustc', '-vV'], text=True).splitlines() if line.startswith('host: '))
    for mode in MODES:
        for label in ('baseline', 'candidate'):
            name = f'bertie-{mode}-{label}'
            run([OUT / label, 'report', '.', '--target', host,
                 '--fstar-log', 'target/ci-logs/fstar/fstar.log', '--ghost-lines', mode,
                 '--json', OUT / f'{name}.json', '--markdown', OUT / f'{name}.md'],
                OUT / f'{name}.log', cwd=OUT / 'bertie')
            print(name + ': generated', flush=True)


def load(label, mode):
    return json.loads((OUT / f'bertie-{mode}-{label}.json').read_text())


# ---- measurements -----------------------------------------------------------------------

def unit_key(u):
    return (u['id'], u['file'], u['line'])


def unit_view(u):
    return {k: u[k] for k in ('category', 'security', 'first_party', 'status', 'sloc')}


def runtime_sloc(u, mode):
    inline = u['ghost_sloc'] if mode == 'separate' else 0
    return u['sloc'] + (u['inline_proof_sloc'] if mode == 'separate' else 0) - inline


def definition_depth(d):
    if d['contract_kind'] != 'functional':
        return SAFETY
    return SELF if d['self_consistency_heuristic'] else FUNCTIONAL


def depth_totals(report, mode):
    """Depth over proven + proven-extracted first-party security runtime SLoC, lowest per unit."""
    rank = {SAFETY: 0, SELF: 1, FUNCTIONAL: 2}
    lowest = {}
    for d in report['fstar_definitions']:
        if d['classification'] == 'proven-extracted' and d['rust_unit']:
            depth = definition_depth(d)
            if d['rust_unit'] not in lowest or rank[depth] < rank[lowest[d['rust_unit']]]:
                lowest[d['rust_unit']] = depth
    totals = {SAFETY: 0, SELF: 0, FUNCTIONAL: 0}
    for u in report['units']:
        if not (u['first_party'] and u['security'] and u['category'] == 'runtime'
                and u['status'] in ('proven', 'proven-extracted')):
            continue
        depth = lowest.get(u['id'], SAFETY) if u['status'] == 'proven-extracted' else SAFETY
        totals[depth] += runtime_sloc(u, mode)
    return totals


def proven_runtime(report):
    p = report['proof_ratio']
    return p['proven_runtime_sloc'] + p['proven_extracted_runtime_sloc']


# ---- checks -----------------------------------------------------------------------------

def check_a(before, after):
    old = {unit_key(u): unit_view(u) for u in before['units']}
    new = {unit_key(u): unit_view(u) for u in after['units']}
    differing = [k for k in old.keys() | new.keys() if old.get(k) != new.get(k)]
    return not differing, f'{len(old)} units; {len(differing)} differ' + (f': {differing[:5]}' if differing else '')


def check_b(before, after):
    keys = ('ratio', 'proof_sloc', 'proven_runtime_sloc', 'proven_extracted_runtime_sloc', 'runtime_sloc')
    same_ratio = all(before['proof_ratio'][k] == after['proof_ratio'][k] for k in keys)
    share = lambda r: r['summary']['first_party']['security_relevant']
    same_share = share(before) == share(after)
    p = after['proof_ratio']
    return same_ratio and same_share, (
        f"proof ratio {p['ratio']}, proven {p['proven_runtime_sloc']}+{p['proven_extracted_runtime_sloc']} "
        f"of {p['runtime_sloc']} runtime SLoC; identical to v1.10.0: {same_ratio and same_share}")


def check_d(after, mode):
    native = after['summary']['proof_depth']
    computed = depth_totals(after, mode)
    parts = (native['safety_only_sloc'], native['functional_own_code_only_sloc'], native['functional_sloc'])
    agrees = parts == (computed[SAFETY], computed[SELF], computed[FUNCTIONAL])
    adds_up = sum(parts) == proven_runtime(after)
    return agrees and adds_up, f'depth {parts}, proven runtime {proven_runtime(after)}; recomputed from definitions: {agrees}'


def check_controls(after):
    by_unit = {}
    for d in after['fstar_definitions']:
        if d['rust_unit']:
            by_unit.setdefault(d['rust_unit'], []).append(d)
    rows, failures = [], []
    for name, kind, consistent, required in CONTROLS:
        found = by_unit.get(f'{CRATE}::{name}', [])
        if not found:
            rows.append((name, kind, consistent, 'missing', '', not required))
            if required:
                failures.append(name)
            continue
        actual = {(d['contract_kind'], d['self_consistency_heuristic'], d['contract_reason']) for d in found}
        ok = all(k == kind and (consistent is None or sc == consistent) for k, sc, _ in actual)
        shown = '; '.join(f'{k}/{r}/sc={sc}' for k, sc, r in sorted(actual))
        rows.append((name, kind, consistent, shown, found[0]['postcondition'], ok))
        if not ok:
            failures.append(name)
    return rows, failures


def unparsed(after):
    seen, listed = set(), []
    for d in after['fstar_definitions']:
        if d['contract_reason'] == 'unparsed' and (d['name'], d['line']) not in seen:
            seen.add((d['name'], d['line']))
            listed.append(d)
    return listed


# ---- compare.md -------------------------------------------------------------------------

def excerpt(text, width=110):
    return ' '.join(text.split())[:width].replace('|', '\\|')


def changed_contracts(before, after, mode):
    old = {(d['name'], d['file'], d['line']): d for d in before['fstar_definitions']}
    units = {u['id']: u for u in after['units'] if u['first_party']}
    rows, seen = [], set()
    for d in after['fstar_definitions']:
        key = (d['name'], d['file'], d['line'])
        if key in seen or key not in old:
            continue
        seen.add(key)
        was = (old[key]['contract_kind'], old[key]['self_consistency_heuristic'])
        now = (d['contract_kind'], d['self_consistency_heuristic'])
        if was != now:
            unit = units.get(d['rust_unit'] or '')
            rows.append((d['rust_unit'] or d['name'], runtime_sloc(unit, mode) if unit else 0,
                         f'{was[0]}/sc={was[1]}', f"{now[0]}/{d['contract_reason']}/sc={now[1]}",
                         d['postcondition']))
    return sorted(rows, key=lambda r: (-r[1], r[0]))


def depth_table(totals):
    total = sum(totals.values())
    return ''.join(f'| {label} | {lines} | {100 * lines / total if total else 0:.2f}% |\n'
                   for label, lines in totals.items())


def write_compare(results, failures):
    before, after = load('baseline', 'separate'), load('candidate', 'separate')
    text = ['# v1.11.0 compare: v1.10.0 vs working tree on bertie-tls (bab3c13 staged inputs)\n\n']
    text.append('## Proof depth, proven + proven-extracted security runtime SLoC (ghost lines: separate)\n\n')
    text.append('| Depth | v1.10.0 lines | v1.10.0 share | v1.11.0 lines | v1.11.0 share |\n| --- | ---: | ---: | ---: | ---: |\n')
    old, new = depth_totals(before, 'separate'), depth_totals(after, 'separate')
    for label in old:
        text.append(f'| {label} | {old[label]} | {100 * old[label] / sum(old.values()):.2f}% '
                    f'| {new[label]} | {100 * new[label] / sum(new.values()):.2f}% |\n')
    text.append('\nv1.10.0 had no safety-only/functional split of size facts: its `functional` row is every '
                'credited line whose postcondition mentioned anything but `result`/`True`.\n')
    text.append('\n## Checks\n\n' + ''.join(f'- {line}\n' for line in results))
    text.append('\n## Controls (separate mode)\n\n| Unit | Expected | Self-consistent | Actual | Match | Postcondition |\n| --- | --- | --- | --- | --- | --- |\n')
    rows, _ = check_controls(after)
    for name, kind, consistent, shown, post, ok in rows:
        expected_sc = 'n/a' if consistent is None else str(consistent).lower()
        text.append(f'| {name} | {kind} | {expected_sc} | {shown} | {"yes" if ok else "**NO**"} | {excerpt(post)} |\n')
    text.append('\n## Contracts the classifier could not read (reason `unparsed`)\n\n')
    listed = unparsed(after)
    text.append('None.\n' if not listed else ''.join(
        f"- `{d['name']}` ({d['file']}:{d['line']}): `{excerpt(d['postcondition'], 400)}`\n" for d in listed))
    text.append('\n## Contracts whose kind or self-consistency changed\n\n| Unit | SLoC | Old | New | Postcondition |\n| --- | ---: | --- | --- | --- |\n')
    for unit, sloc, was, now, post in changed_contracts(before, after, 'separate'):
        text.append(f'| {unit} | {sloc} | {was} | {now} | {excerpt(post)} |\n')
    if failures:
        text.append(f'\n## {stamp()} — HONEST-FINDING\n\n' + '\n'.join(failures) + '\n')
    COMPARE.write_text(''.join(text))


def stamp():
    return datetime.now(timezone.utc).isoformat(timespec='seconds')


def main():
    failures, results = [], []
    try:
        prepare()
        generate()
        for mode in MODES:
            before, after = load('baseline', mode), load('candidate', mode)
            checks = [('a', *check_a(before, after)), ('b', *check_b(before, after)),
                      ('d', *check_d(after, mode))]
            _, control_failures = check_controls(after)
            checks.append(('c', not control_failures,
                           'all controls as in PLAN.md' if not control_failures
                           else 'control mismatches: ' + ', '.join(control_failures)))
            listed = unparsed(after)
            checks.append(('e', True, f'{len(listed)} unparsed contract(s), each listed in compare.md'))
            for name, ok, detail in checks:
                line = f"{'PASS' if ok else 'FAIL'} ({name}) [{mode}] {detail}"
                results.append(line)
                print(line, flush=True)
                if not ok:
                    failures.append(line)
    except Exception as error:
        failures.append(f'replay stopped: {error}')
        print(failures[-1], file=sys.stderr)
    if COMPARE.parent.exists() and (OUT / 'bertie-separate-candidate.json').exists():
        write_compare(results, failures)
    if failures:
        with NOTES.open('a') as notes:
            notes.write(f'\n## {stamp()} — HONEST-FINDING / replay\n\n' + '\n'.join(f'- {f}' for f in failures) + '\n')
        sys.exit(1)


if __name__ == '__main__':
    main()
