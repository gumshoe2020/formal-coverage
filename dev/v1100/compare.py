#!/usr/bin/env python3
"""Run the default (committed-floor) kit on the test fixture; print sha256 of every output.

usage: compare.py KIT_ROOT   -- diff the output of two kit checkouts to prove byte-identity.
"""
import hashlib, json, subprocess, sys, tempfile
from pathlib import Path

kit = Path(sys.argv[1]).resolve()
sys.path.insert(0, str(kit / 'ci/tests'))
sys.path.insert(0, str(kit / 'ci'))
import test_write_coverage
from test_ratchet import fixture


def git(work, *a):
    return subprocess.check_output(['git', '-C', str(work), '-c', 'user.name=T', '-c', 'user.email=t@e.invalid', *a], text=True, stderr=subprocess.DEVNULL).strip()


def scenario(name, report, work):
    work.mkdir()
    git(work, 'init', '-q')
    (work / 'fixture.json').write_text(json.dumps(report))
    (work / 'COVERAGE.md').write_text('Intro\n<!-- measured coverage: begin -->\n<!-- measured coverage: end -->\n\n## Exceptions\n\n| Commit | Approving maintainer | Reason | Restoration obligation |\n| --- | --- | --- | --- |\n')
    subprocess.run([sys.executable, str(kit / 'ci/write-coverage.py'), 'fixture.json'], cwd=work, check=True, capture_output=True)
    git(work, 'add', '.'); git(work, 'commit', '-qm', 'floor')
    base = git(work, 'rev-parse', 'HEAD')
    (work / 'README.md').write_text('docs\n'); git(work, 'add', '.'); git(work, 'commit', '-qm', 'docs')
    tool = work / 'tool'
    tool.write_text('#!' + sys.executable + '\nimport sys,shutil\nfrom pathlib import Path\na=sys.argv\nshutil.copyfile("fixture.json",a[a.index("--json")+1])\nPath(a[a.index("--markdown")+1]).write_text("fixture")\n')
    tool.chmod(0o755)
    out = work / 'out'
    r = subprocess.run([sys.executable, str(kit / 'ci/run.py'), '--tool', str(tool), '--workspace', str(work), '--base', base, '--output', str(out)], capture_output=True, text=True)
    print(f'{name}: exit={r.returncode}')
    print(f'{name}: stdout={r.stdout.strip()!r}')
    for path in sorted([work / 'COVERAGE.md', work / 'coverage/baseline.json', *out.rglob('*')]):
        if path.is_file() and path.name not in ('tool',):
            print(f'{name}: {path.relative_to(work)} {hashlib.sha256(path.read_bytes()).hexdigest()}')


with tempfile.TemporaryDirectory() as d:
    for name, report in (('default', test_write_coverage.WriteCoverageTests().report()),):
        scenario(name, report, Path(d) / name)
