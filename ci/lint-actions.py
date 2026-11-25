#!/usr/bin/env python3
"""Lint workflows and composite run steps with actionlint (which accepts workflows).

Wrap each action's actual steps unchanged in a workflow_call job. Declare its
string inputs so actionlint also validates input and step-output references.
This adapter handles this kit's composite metadata layout, not arbitrary YAML.
"""
from pathlib import Path
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def main():
    subprocess.run(['actionlint', *map(str, sorted((ROOT / '.github/workflows').glob('*.yml')))], check=True)
    example = (ROOT / 'README.md').read_text().split('```yaml\n', 1)[1].split('```', 1)[0]
    with tempfile.NamedTemporaryFile(mode='w', suffix='.yml') as stream:
        stream.write(example)
        stream.flush()
        subprocess.run(['actionlint', stream.name], check=True)
    print('actionlint: README consumer workflow passed')
    for action in sorted((ROOT / '.github/actions').glob('*/action.yml')):
        text = action.read_text()
        inputs = text.split('\ninputs:\n', 1)[1].split('\noutputs:', 1)[0].split('\nruns:', 1)[0]
        names = re.findall(r'^  ([a-z][a-z-]*):$', inputs, re.M)
        steps = text.split('\n  steps:\n', 1)[1]
        workflow = 'name: Composite lint\non:\n  workflow_call:\n    inputs:\n'
        for name in names:
            workflow += f'      {name}:\n        type: string\n'
        workflow += 'jobs:\n  action:\n    runs-on: ubuntu-latest\n    steps:\n'
        workflow += ''.join('  ' + line + '\n' for line in steps.splitlines())
        with tempfile.NamedTemporaryFile(mode='w', suffix='.yml') as stream:
            stream.write(workflow)
            stream.flush()
            result = subprocess.run(['actionlint', stream.name])
        if result.returncode:
            raise SystemExit(f'actionlint failed for composite steps: {action.relative_to(ROOT)}')
        print(f'actionlint: {action.relative_to(ROOT)} steps passed')


if __name__ == '__main__':
    main()
