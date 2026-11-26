#!/usr/bin/env python3
"""Local acceptance adapter. Never edits the read-only consumer checkout.

Run from formal-coverage after archiving Bertie 110802a under target/v1120/bertie
and copying ci/fstar-cache's verify-fstar.py there. All F* runs use one worker.
The scratch-only verifier supports a module list; this adapter fills that list
after its unconditional fresh extraction, just before strict checking.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'target/v1120'
BERTIE = WORK / 'bertie'
OUT = BERTIE / 'target/ci-logs/fstar'
TOOLS = Path(os.environ['BERTIE_DOCKER_TOOLS'])  # pinned Docker toolchain checkout; see README
SHA = '110802aae37cb2e4adce165a1410bdea4be01644'
TOOL = ROOT / 'target/debug/formal-coverage'
sys.path.insert(0, str(ROOT / 'ci'))
import receipts


def load_verifier():
    spec = importlib.util.spec_from_file_location('bertie_verifier', BERTIE / 'scripts/verify-fstar.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def execute(args, log, **kwargs):
    start = time.monotonic()
    with Path(log).open('w') as stream:
        result = subprocess.run(list(map(str, args)), stdout=stream, stderr=subprocess.STDOUT, **kwargs)
    if result.returncode:
        raise RuntimeError(f'exit {result.returncode}: {log}')
    return time.monotonic() - start


def identity(*paths):
    return hashlib.sha256(''.join(receipts.sha(TOOLS / p) for p in paths).encode()).hexdigest()


def make_context(verifier):
    includes = ['libs/rust_primitives', 'libs/core', 'libs/hax-lib', 'libs/libcrux-models', 'fresh']
    inc = [x for p in includes for x in ('--include', p)]
    commands = [dict(file=f'fresh/Bertie_tls.{m}', command=['fstar.exe', *inc, f'fresh/Bertie_tls.{m}']) for m in verifier.MODULES]
    dep = OUT / 'dep.txt'
    args = ['docker', 'run', '--rm', '--platform', 'linux/amd64', '--network', 'none',
            '--cpus', '1', '--memory', '1g', '-v', f'{TOOLS}:/tools:ro',
            '-v', f'{BERTIE}:{BERTIE}', '-w', str(OUT), 'fc-verify-fstar-amd64:v17',
            'bash', '-ec', 'export PATH=/tools/fstar-bin-2025.10.06/fstar/bin:$PATH; exec "$@"', 'deps',
            'fstar.exe', '--dep', 'full', *inc, *[m['file'] for m in commands]]
    execute(args, dep)
    toolchain = dict(fstar='2025.10.06@' + identity('fstar-bin-2025.10.06/fstar/bin/fstar.exe'),
                    z3='4.13.3@' + identity('fstar-bin-2025.10.06/fstar/lib/fstar/z3-4.13.3/bin/z3'),
                    hax='0.4.2@' + verifier.HAX_COMMIT + '@' + identity('hax/cargo-hax', 'hax/hax-engine'))
    path = OUT / 'fstar.log.context.json'
    path.write_text(receipts.encoded(receipts.context(OUT, dep.read_text(), commands, toolchain, [f'/tools={TOOLS}'])))
    return path


def report(name, reuse=None, tool=TOOL):
    args = [tool, 'report', '.', '--fstar-log', 'target/ci-logs/fstar/fstar.log',
            '--json', str(WORK / f'{name}.json'), '--markdown', str(WORK / f'{name}.md')]
    if reuse:
        args += ['--fstar-reuse', str(reuse)]
    return execute(args, WORK / 'logs' / f'{name}-report.log', cwd=BERTIE)


def install_module_list_adapter():
    path = BERTIE / 'scripts/verify-fstar.py'
    text = path.read_text()
    if 'FC_MODULE_LIST' in text:
        return
    text = text.replace('modules = list(pool.map(verify, MODULES))',
        "selected = json.loads(Path(os.environ['FC_MODULE_LIST']).read_text()) if os.environ.get('FC_MODULE_LIST') else MODULES\n        modules = list(pool.map(verify, selected))")
    text = text.replace("for m in MODULES))", "for m in selected))")
    path.write_text(text)


def incremental(name):
    install_module_list_adapter()
    verifier = load_verifier()
    selection = WORK / f'{name}-modules.json'
    manifest = WORK / f'{name}-reuse.json'
    context_time, selection_time = [], []

    class SelectPool(ThreadPoolExecutor):
        def __enter__(self):
            # At this point the unchanged verifier has freshly extracted all
            # source and copied all libraries. Only its list of checks varies.
            start = time.monotonic()
            ctx = make_context(verifier)
            context_time.append(time.monotonic() - start)
            start = time.monotonic()
            roots = receipts.select(TOOL, ctx, WORK / 'data-reader', SHA, 'coverage-data', manifest)
            selection_time.append(time.monotonic() - start)
            suffixes = [Path(m).name.removeprefix('Bertie_tls.') for m in roots]
            selection.write_text(json.dumps(suffixes))
            os.environ['FC_MODULE_LIST'] = str(selection)
            print(f'{name}: selected {suffixes}', flush=True)
            return super().__enter__()

    verifier.ThreadPoolExecutor = SelectPool
    start = time.monotonic()
    if verifier.main():
        raise RuntimeError('strict incremental verifier failed')
    duration = time.monotonic() - start
    measurement = report(name, manifest)
    result = dict(seconds=duration, context_seconds=context_time[0], selection_seconds=selection_time[0],
                  report_seconds=measurement, modules=json.loads(selection.read_text()))
    (WORK / f'{name}-timing.json').write_text(receipts.encoded(result))
    return result


def baseline():
    verifier = load_verifier()
    context = make_context(verifier)
    elapsed = (OUT / 'fstar.log.evidence.json').stat().st_mtime - json.loads((WORK / 'fresh-start.json').read_text())['timestamp']
    (WORK / 'a-fresh-timing.json').write_text(receipts.encoded(dict(seconds=elapsed)))
    report('a-fresh')
    old = WORK / 'v1110-build/debug/formal-coverage'
    if old.exists():
        report('a-v1110', tool=old)
        assert (WORK / 'a-fresh.json').read_bytes() == (WORK / 'a-v1110.json').read_bytes()
        assert (WORK / 'a-fresh.md').read_bytes() == (WORK / 'a-v1110.md').read_bytes()
        print('Default JSON and Markdown exactly equal v1.11.0', flush=True)
    stage = WORK / 'main-receipts'
    receipts.prepare(TOOL, BERTIE, OUT / 'fstar.log', context, SHA, 'coverage-data', stage)
    # An isolated local Git data branch provides real git-show reads without
    # fetching, publishing or mutating the consumer repository's refs.
    data = WORK / 'data-reader'
    data.mkdir(exist_ok=True)
    execute(['git', 'init', '-q', '-b', 'coverage-data', data], WORK / 'logs/data-init.log')
    target = data / f'receipts/{SHA}'
    target.mkdir(parents=True, exist_ok=True)
    for p in stage.glob('*.json'):
        shutil.copy2(p, target / p.name)
    execute(['git', '-C', data, 'add', '.'], WORK / 'logs/data-add.log')
    execute(['git', '-C', data, '-c', 'user.name=Dudley', '-c', 'user.email=dudley@local',
             'commit', '-qm', 'Local fixture of main strict receipts'], WORK / 'logs/data-commit.log')
    execute(['git', '-C', BERTIE, 'fetch', '-q', data, 'coverage-data:refs/remotes/origin/coverage-data'], WORK / 'logs/data-fetch.log')
    shutil.copytree(OUT, WORK / 'a-inputs', ignore=shutil.ignore_patterns('workspace'), dirs_exist_ok=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('phase', choices=['baseline', 'unchanged', 'touched', 'reverted', 'tamper'])
    args = parser.parse_args()
    # Use this checkout's source inventory and local Git object fixtures only.
    os.environ['FSTAR_JOBS'] = '1'
    os.environ['BERTIE_DOCKER_TOOLS'] = str(TOOLS)
    if args.phase == 'baseline':
        baseline()
    elif args.phase == 'unchanged':
        result = incremental('b-unchanged')
        assert result['modules'] == [], result
        assert (WORK / 'a-fresh.json').read_bytes() == (WORK / 'b-unchanged.json').read_bytes()
    elif args.phase == 'touched':
        path = BERTIE / 'src/der.rs'
        original = path.read_bytes()
        (WORK / 'der.original').write_bytes(original)
        # Add one harmless public constant: original semantics/proofs preserved.
        # hax puts it in Der.fsti; Der.fst stays identical. Both its own check
        # and the Tls13formats consumer must be invalidated by that interface.
        text = original.decode().replace('pub const RAW_SIGNATURE_LEN: usize = 64;',
            'pub const RAW_SIGNATURE_LEN: usize = 64;\npub const RECEIPT_M3_CANARY: usize = 0;')
        assert text != original.decode()
        path.write_text(text)
        try:
            result = incremental('c-touched')
            assert set(result['modules']) == {'Der.fst', 'Tls13formats.fst'}, result
        finally:
            path.write_bytes(original)
    elif args.phase == 'reverted':
        result = incremental('c-reverted')
        assert result['modules'] == [], result
        assert (WORK / 'a-fresh.json').read_bytes() == (WORK / 'c-reverted.json').read_bytes()
    else:
        manifest = json.loads((WORK / 'c-reverted-reuse.json').read_text())
        first = next(iter(manifest['receipts'].values()))
        dep = next(p for p in first['dependencies'] if p.startswith('external/'))
        first['dependencies'][dep] = '0' * 64
        tampered = WORK / 'tampered-manifest.json'
        tampered.write_text(receipts.encoded(manifest))
        try:
            report('tampered', tampered)
        except RuntimeError:
            text = (WORK / 'logs/tampered-report.log').read_text()
            assert first['file'] in text and dep in text, text
            print(f'Tamper rejected: {first["file"]}, dependencies hash: {dep}')
        else:
            raise AssertionError('tampered receipt was accepted')


if __name__ == '__main__':
    main()
