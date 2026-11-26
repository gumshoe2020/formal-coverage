#!/usr/bin/env python3
"""Fresh F* dependency contexts, immutable main receipts, and PR selection.

All inputs are argv/JSON, never shell code. No prover setup or network access.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

from config import valid_branch
from run import data_refs, show


def encoded(value):
    return json.dumps(value, indent=2, sort_keys=True) + '\n'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def full_sha(value):
    if not isinstance(value, str) or not re.fullmatch('[0-9a-f]{40}', value):
        raise ValueError('expected full main commit SHA')
    return value


def module_name(file):
    name = Path(file).stem
    if not re.fullmatch(r'[A-Za-z][A-Za-z0-9_.]*', name) or not file.endswith('.fst'):
        raise ValueError(f'invalid module file: {file}')
    return name


def inside(base, name):
    path = (base / name).resolve(strict=True)
    if Path(name).is_absolute() or not path.is_relative_to(base.resolve()):
        raise ValueError(f'path outside context: {name}')
    return path


def rules(text):
    """F* --dep full make rules. Ignore diagnostics and non-checked outputs."""
    result = {}
    for line in text.replace('\\\n', ' ').splitlines():
        target, sep, rest = line.partition(':')
        target = target.strip()
        if not sep or not target.endswith('.checked'):
            continue
        deps = rest.split()
        if (not deps or any(not p.endswith(('.fst', '.fsti', '.checked')) for p in deps)
                or any(c in target for c in '\\ \t') or target in result):
            raise ValueError(f'unsupported or duplicate dependency rule: {target}')
        result[target] = deps
    if not result:
        raise ValueError('no F* dependency rules')
    return result


def context(base, dep_text, commands, toolchain, external=()):
    """Normalize paths and copy external source bytes; never copy checked files."""
    base = base.resolve()
    mappings = []
    for number, mapping in enumerate(external):
        virtual, local = mapping.split('=', 1)
        if not Path(virtual).is_absolute():
            raise ValueError('external prefix must be absolute')
        mappings.append((Path(virtual), Path(local).resolve(), f'external/{number}'))

    def normalize(name, copy=False):
        path = Path(name)
        if not path.is_absolute():
            path = base / path
        if path.is_relative_to(base):
            return str(path.relative_to(base))
        for prefix, local, logical in mappings:
            if path.is_relative_to(prefix):
                relative = path.relative_to(prefix)
                target = Path(logical) / relative
                if copy:
                    source = (local / relative).resolve(strict=True)
                    if not source.is_relative_to(local):
                        raise ValueError(f'external source escapes prefix: {name}')
                    dest = base / target
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copyfile(source, dest)
                return str(target)
        raise ValueError(f'unmapped dependency path: {name}')

    graph = {}
    for target, deps in rules(dep_text).items():
        key = normalize(target)
        if key in graph:
            raise ValueError(f'ambiguous dependency target: {key}')
        graph[key] = [normalize(p, copy=not p.endswith('.checked')) for p in deps]
    memo, visiting = {}, set()

    def closure(target):
        if target in memo:
            return memo[target]
        if target in visiting or target not in graph:
            raise ValueError(f'cyclic or missing dependency rule: {target}')
        visiting.add(target)
        files = set()
        for dep in graph[target]:
            if dep.endswith('.checked'):
                files.update(closure(dep))
            else:
                inside(base, dep)
                files.add(dep)
        visiting.remove(target)
        memo[target] = files
        return files

    modules = []
    for item in commands:
        file, command = item['file'], item['command']
        module_name(file)
        modules.append(dict(file=file, command=command,
                            dependencies=sorted(closure(file + '.checked'))))
    return dict(schema=1, toolchain=toolchain, modules=modules)


def validate(tool, context_path, receipt_path):
    return subprocess.run([str(tool), 'check-fstar-receipt', '--context', str(context_path),
                           '--receipt', str(receipt_path)], capture_output=True, text=True)


def select(tool, context_path, workspace, base, branch, output):
    full_sha(base)
    if not valid_branch(branch):
        raise ValueError('invalid data branch')
    ctx = json.loads(context_path.read_text())
    reused, needed = {}, []
    with tempfile.TemporaryDirectory(prefix='fc-select-') as directory:
        candidate = Path(directory) / 'receipt.json'
        for m in ctx['modules']:
            name = module_name(m['file'])
            path = f'receipts/{base}/{name}.json'
            text = next((t for ref in data_refs(branch)
                         if (t := show(workspace, f'{ref}:{path}')) is not None), None)
            if text is None:
                needed.append(m['file'])
                print(f'{name}: fresh (no merge-base receipt)', file=sys.stderr)
                continue
            candidate.write_text(text)
            result = validate(tool, context_path, candidate)
            try:
                r = json.loads(text)
                matches = r['file'] == m['file'] and r['main_commit'] == base
            except (ValueError, KeyError, TypeError):
                matches = False
            if result.returncode or not matches:
                needed.append(m['file'])
                print(f'{name}: fresh ({result.stderr.strip() or "module/main_commit mismatch"})', file=sys.stderr)
            else:
                reused[m['file']] = r
    output.write_text(encoded(dict(schema=1, data_branch=branch, main_commit=base, receipts=reused)))
    return needed


def prepare(tool, workspace, log, context_path, commit, branch, output):
    full_sha(commit)
    if not valid_branch(branch):
        raise ValueError('invalid data branch')
    if context_path.resolve() != Path(str(log) + '.context.json').resolve():
        raise ValueError('context must be the log context sidecar')
    ctx = json.loads(context_path.read_text())
    evidence = json.loads(Path(str(log) + '.evidence.json').read_text())
    if (len(evidence['modules']) != len(ctx['modules']) or
            any(m['exit_code'] != 0 or m['outcome'] != 'success' for m in evidence['modules'])):
        raise ValueError('publication requires every module freshly checked successfully')
    output.mkdir(parents=True, exist_ok=True)
    # Validate all old evidence rules as well as the fresh context. Zero reused modules.
    with tempfile.TemporaryDirectory(prefix='fc-prepare-') as directory:
        empty = Path(directory) / 'reuse.json'
        empty.write_text(encoded(dict(schema=1, data_branch=branch, main_commit=commit, receipts={})))
        subprocess.run([str(tool.resolve()), 'report', str(workspace.resolve()), '--size-mode', 'whole-crate',
                        '--fstar-log', str(log.resolve()), '--fstar-reuse', str(empty)],
                       stdout=subprocess.DEVNULL, check=True)
    for m in ctx['modules']:
        name = module_name(m['file'])
        slice_path = log.parent / (Path(m['file']).name + '.log')
        r = dict(schema=1, module=name, main_commit=commit, file=m['file'],
                 toolchain=ctx['toolchain'], command=m['command'],
                 dependencies={p: sha(inside(context_path.parent, p)) for p in m['dependencies']},
                 log=slice_path.read_text())
        # Require the slice actually came from the retained strict combined log.
        if r['log'].encode() not in log.read_bytes():
            raise ValueError(f'{name}: log slice not in strict run log')
        dest = output / f'{name}.json'
        text = encoded(r)
        if dest.exists() and dest.read_text() != text:
            raise ValueError(f'{name}: staging receipts are append-only')
        dest.write_text(text)
        result = validate(tool, context_path, dest)
        if result.returncode:
            raise ValueError(result.stderr)
    (output / 'sha').write_text(commit + '\n')
    (output / 'branch').write_text(branch + '\n')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='operation', required=True)
    ctx = sub.add_parser('context')
    ctx.add_argument('--dep', type=Path, required=True)
    ctx.add_argument('--commands', type=Path, required=True, help='JSON list of {file, command}')
    ctx.add_argument('--toolchain', type=Path, required=True, help='JSON {fstar, z3, hax} identities')
    ctx.add_argument('--external', action='append', default=[], help='virtual-prefix=local-prefix; stable order')
    ctx.add_argument('--output', type=Path, required=True, help='LOG.context.json beside current extraction')
    for name in ('select', 'prepare'):
        p = sub.add_parser(name)
        p.add_argument('--tool', type=Path, required=True)
        p.add_argument('--context', type=Path, required=True)
        p.add_argument('--workspace', type=Path, default=Path('.'))
        p.add_argument('--base' if name == 'select' else '--commit', required=True)
        p.add_argument('--branch', default='coverage-data')
        p.add_argument('--output', type=Path, required=True)
        if name == 'prepare':
            p.add_argument('--log', type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.operation == 'context':
            args.output.write_text(encoded(context(args.output.parent, args.dep.read_text(),
                json.loads(args.commands.read_text()), json.loads(args.toolchain.read_text()), args.external)))
        elif args.operation == 'select':
            needed = select(args.tool.resolve(), args.context.resolve(), args.workspace,
                            args.base, args.branch, args.output)
            sys.stdout.write(''.join(m + '\n' for m in needed))
        else:
            prepare(args.tool, args.workspace, args.log, args.context, args.commit, args.branch, args.output)
    except (ValueError, KeyError, TypeError, OSError, subprocess.CalledProcessError) as exc:
        parser.exit(1, f'receipts FAIL: {exc}\n')


if __name__ == '__main__':
    main()
