#!/usr/bin/env python3
"""v1.5 offline replays: spec-lemmas (bundle and host-metadata exports) and the three survey repos.

Runs one formal-coverage binary over each case with optional extra CLI arguments and writes
portable report.json/report.md per case. Never writes into scratch-server or survey checkouts:
sources are git-archive or bundle-archive exports under this repository's target/.

With --container NAME the binary (a Linux build) runs through `docker exec` in an existing
container that mounts this repository at CONTAINER_ROOT, so cargo metadata/check and every
build script run on Linux. Paths under the repository are passed as container paths, and the
container prefix is removed from the reports like the host prefix.
"""
import argparse
import io
import json
import os
from pathlib import Path, PurePosixPath
import subprocess
import tarfile

from evidence_support import REPO, extract, portable, sha, write

SCRATCH = REPO.parent / 'scratch-server'
SPEC_LEMMAS = '2fa5eb78a6f96a8433229dc64ec4b82c3e5f9bc2'
HOST_EXPORT = REPO / 'target/v1.5-scratch/spec-lemmas'
CONTAINER_ROOT = PurePosixPath('/fc')


def host_triple(container=None):
    prefix = ['docker', 'exec', container] if container else []
    out = subprocess.check_output(prefix + ['rustc', '-vV'], text=True)
    return next(line[6:] for line in out.splitlines() if line.startswith('host: '))


def export_host(config):
    """Read-only git archive of the pinned spec-lemmas commit (v1.2 host-metadata closure)."""
    if not (HOST_EXPORT / 'Cargo.toml').is_file():
        HOST_EXPORT.mkdir(parents=True, exist_ok=True)
        archive = subprocess.check_output(['git', '-C', str(SCRATCH), 'archive', SPEC_LEMMAS])
        with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
            tar.extractall(HOST_EXPORT, members=[m for m in tar.getmembers() if m.isfile() or m.isdir()], filter='data')
    (HOST_EXPORT / 'formal-coverage.toml').write_bytes(Path(config).read_bytes())
    return HOST_EXPORT


def prepare_bundle_case(case):
    destination = REPO / case['source']
    assert sha((REPO / case['archive']).read_bytes()) == case['archive_sha256'], case['name']
    extract(REPO / case['archive'], destination)
    for archive in case.get('vendor_archives', []):
        extract(REPO / archive, destination / 'vendor')
    if case.get('lock'):
        (destination / 'Cargo.lock').write_bytes((REPO / case['lock']).read_bytes())
    if case.get('config'):
        (destination / 'formal-coverage.toml').write_bytes((REPO / case['config']).read_bytes())
    if case.get('isolate_workspace'):
        import tomllib
        manifest = destination / 'Cargo.toml'
        if 'workspace' not in tomllib.loads(manifest.read_text()):
            manifest.write_text(manifest.read_text() + '\n[workspace]\n')
    return destination


def cases(names, host_config, container=None):
    index = json.loads((REPO / 'logs/release-evidence.json').read_text())
    known = {c['name']: c for c in index['scratch'] + index['survey']}
    for name in names:
        if name == 'spec-lemmas-host':
            receipts = ['spec-lemmas.verus.log', 'spec-lemmas.root-2.verus.log', 'spec-lemmas.root-3.verus.log']
            yield name, dict(source=str(export_host(host_config).relative_to(REPO)), target=host_triple(container),
                             cargo_home='.cargo-home',
                             evidence=[['verus', f'docs/scratch-server/{r}'] for r in receipts])
        else:
            case = known[name]
            prepare_bundle_case(case)
            yield name, case


def container_path(value):
    """An absolute path inside the repository as its container path; anything else unchanged.

    Relative paths need no mapping: both runs use the repository root as working directory.
    """
    path = Path(value)
    if not path.is_absolute():
        return value
    try:
        relative = path.resolve().relative_to(REPO.resolve())
    except ValueError:
        return value
    return str(CONTAINER_ROOT / relative.as_posix())


def command_line(binary, case, output, extra, command='report', container=None):
    """(argv, environment overrides) for one case, on the host or through docker exec."""
    root = Path(str(CONTAINER_ROOT)) if container else REPO.resolve()
    tool = container_path(str(Path(binary).resolve())) if container else str(Path(binary).resolve())
    args = [tool, command, case['source'], '--target', case['target'],
            '--json', str(output / 'report.json'), '--markdown', str(output / 'report.md')]
    for tool_name, log in case['evidence']:
        args += [f'--{tool_name}-log', log]
    args += extra
    env = {'CARGO_HOME': str(root / case['cargo_home']), 'TMPDIR': str(root / '.tmp'),
           'CARGO_NET_OFFLINE': 'true'}
    if not container:
        return args, env
    args = [container_path(a) for a in args]
    wrapped = ['docker', 'exec', '-w', str(CONTAINER_ROOT)]
    for key, value in env.items():
        wrapped += ['-e', f'{key}={value}']
    return wrapped + [container] + args, {}


def portable_report(value, container=None):
    value = portable(value, REPO)
    return portable(value, Path(str(CONTAINER_ROOT))) if container else value


def run(binary, case, output, extra, command='report', container=None):
    output.mkdir(parents=True, exist_ok=True)
    args, overrides = command_line(binary, case, output, extra, command, container)
    env = {**os.environ, **overrides}
    (REPO / '.tmp').mkdir(exist_ok=True)
    with (output / 'report.log').open('w') as log:
        log.write('$ ' + ' '.join(args) + '\n')
        log.flush()
        code = subprocess.run(args, cwd=REPO, env=env, stdout=log, stderr=subprocess.STDOUT).returncode
    if code != 0:
        return code
    report = portable_report(json.loads((output / 'report.json').read_text()), container)
    write(output / 'report.json', report)
    (output / 'report.md').write_text(portable_report((output / 'report.md').read_text(), container))
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', required=True)
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--case', action='append', default=[])
    parser.add_argument('--host-config', default=str(REPO / 'examples/scratch-server/formal-coverage.toml'))
    parser.add_argument('--container', help='run the (Linux) binary in this container via docker exec')
    parser.add_argument('--target', help='override every case target triple')
    parser.add_argument('extra', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    extra = args.extra[1:] if args.extra[:1] == ['--'] else args.extra
    names = args.case or ['scratch-spec-lemmas', 'spec-lemmas-host', 'vest', 'anvil', 'firecracker']
    failures = 0
    for name, case in cases(names, args.host_config, args.container):
        if args.target:
            case = {**case, 'target': args.target}
        code = run(args.binary, case, args.out / name, extra, container=args.container)
        print(f'{name}: exit {code}', flush=True)
        failures += code != 0
    return 1 if failures else 0


if __name__ == '__main__':
    raise SystemExit(main())
