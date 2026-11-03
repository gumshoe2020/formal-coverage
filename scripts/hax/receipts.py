#!/usr/bin/env python3
"""Preserve actual failed/partial run evidence. Never manufacture fresh extraction."""
import hashlib
import json
from pathlib import Path
import shlex
import sys

out = Path(sys.argv[1]).resolve()
manifest = json.loads((out/'manifest.json').read_text())
sha = lambda path: hashlib.sha256(path.read_bytes()).hexdigest()
for tree in ['upstream', 'shipped']:
    work = out/'work'/tree
    files = {str(p.relative_to(work)): sha(p) for p in sorted(work.rglob('*.rs'))
             if not any(part in ['target', 'cargo-home'] for part in p.relative_to(work).parts)}
    for pattern in ['**/Cargo.toml', 'Cargo.lock', 'hax-driver.py']:
        for p in work.glob(pattern):
            files[str(p.relative_to(work))] = sha(p)
    fresh = {str(p.relative_to(work)): sha(p) for p in sorted((work/'fresh').glob('*.fst*'))}
    patches = {str(p.relative_to(work)): sha(p) for p in sorted((work/'proofs/fstar').glob('*.patch'))}
    for kind, directory in [('extraction', 'extraction'), ('panic-free', 'extraction-panic-free')]:
        log = work/f'typecheck-{kind}.log'
        checked = {str(p.relative_to(work)): sha(p)
                   for p in sorted((work/'proofs/fstar'/directory).glob('*.fst*'))}
        commands = {}
        for line in log.read_text(errors='replace').splitlines():
            try:
                args = shlex.split(line)
            except ValueError:
                continue
            if args and Path(args[0]).name == 'fstar.exe':
                inputs = [a for a in args if a.endswith(('.fst', '.fsti')) and '*' not in a]
                if len(inputs) == 1:
                    commands[Path(inputs[0]).name] = args
        modules = []
        for file in checked:
            if not file.endswith('.fst'):
                continue
            name = Path(file).name
            cmd = commands.get(name)
            verified = f'Verified module: {Path(name).stem}' in log.read_text(errors='replace')
            modules.append({'file': file, 'command': cmd or ['not-executed'],
                            'exit_code': 0 if verified and cmd else 125,
                            'outcome': 'success' if verified and cmd else 'skipped'})
        receipt = {'tool': 'hax-fstar',
                   'hax_version': (out/'hax-version.log').read_text(errors='replace').strip(),
                   'hax_commit': manifest['hax_commit'],
                   'fstar_version': (out/'fstar-version.log').read_text(errors='replace').strip(),
                   'extraction_command': ['python3', 'hax-driver.py', 'extract-fstar'],
                   'extraction_exit_code': int((work/'extract.exit').read_text()),
                   'files': files, 'fresh': fresh, 'checked': checked, 'patches': patches,
                   'log_sha256': sha(log), 'modules': modules}
        Path(str(log)+'.evidence.json').write_text(json.dumps(receipt, indent=2)+'\n')
        print(tree, kind, 'fresh files', len(fresh), 'module outcomes', len(modules))
