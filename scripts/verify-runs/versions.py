"""Run inside the container; retain actual tool versions without log scraping."""
import json
from pathlib import Path
import subprocess


def output(args):
    p = subprocess.run(args, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    return {'command': args, 'exit_code': p.returncode, 'output': p.stdout.strip()}


facts = [output(['rustup', 'toolchain', 'list']), output(['rustc', '-vV']),
         output(['cargo', '--version']), output(['cargo', 'kani', '--version'])]
if Path('/root/.kani/kani-0.67.0').exists():
    facts.append(output(['rustc', '+nightly-2025-11-21', '-vV']))
for tool in ['/opt/verus/source/target-verus/release/verus', '/opt/verus/source/z3']:
    if Path(tool).exists():
        facts.append(output([tool, '--version']))
Path('/out/tool-versions.json').write_text(json.dumps(facts, indent=2) + '\n')
Path('/out/packages.txt').write_bytes(Path('/packages.txt').read_bytes())
