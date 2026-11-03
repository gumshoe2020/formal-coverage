"""Regression checks; run with the private denylist path as the only argument."""
import importlib.util
import io
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[1]


def archive_checks():
    spec = importlib.util.spec_from_file_location('redact', ROOT / 'scripts/redact-archives.py')
    redact = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(redact)
    rules = [('private-term-01', re.compile(rb'(?<!\w)PrivateName(?!\w)'), lambda m: b'[private]')]
    content, _ = redact.redact(b'PrivateName PrivateNames privatename\nPrivateName', rules)
    assert content == b'[private] PrivateNames privatename\n[private]'
    for name, blocked in [('NOTES.md', False), ('src/lib.rs', True),
                          ('Cargo.toml', True), ('Cargo.lock', True)]:
        stream = io.BytesIO()
        with tarfile.open(fileobj=stream, mode='w', format=tarfile.PAX_FORMAT) as tar:
            member = tarfile.TarInfo(name)
            member.size, member.mode, member.mtime = 11, 0o640, 123456
            tar.addfile(member, io.BytesIO(b'PrivateName'))
        original = stream.getvalue()
        output, changes, blockers = redact.rebuild(original, rules)
        assert bool(blockers) == blocked
        assert (output == original) == blocked
        assert changes[0]['events'] == [dict(line=1, column=1, rule='private-term-01')]
    print('Archive substitution boundaries, case, metadata and protected-member blockers pass')


def scanner_checks(denylist):
    with tempfile.TemporaryDirectory() as temporary:
        scan = Path(temporary)
        allowed = b'/Us' b'ers/runner/work/abc api.github.com/us' b'ers/example'
        lan = b'192.' b'168.1.1'
        sample = scan / 'sample.txt'
        command = ['bash', str(ROOT / 'scripts/check-public.sh'), str(denylist), str(scan)]

        def status():
            return subprocess.run(command, stdout=subprocess.DEVNULL, check=False).returncode

        sample.write_bytes(allowed + b'\n')
        assert status() == 0
        sample.write_bytes(allowed + b' ' + lan + b'\n')
        assert status() == 1
        armor = b'-----' + b'BEGIN '
        sample.write_bytes(armor + b'PGP SIGNATURE-----\n' + armor + b'PGP PUBLIC KEY BLOCK-----\n')
        assert status() == 0
        for private in [armor + b'PRIVATE KEY-----', armor + b'RSA PRIVATE KEY-----',
                        armor + b'PGP PRIVATE KEY BLOCK-----', armor + b'PGP SIGNATURE',
                        armor + b'PGP SIGNATURE-----' + armor + b'PRIVATE KEY-----',
                        armor + b'PGP SIGNATURE-----' b'BEGIN PRIVATE KEY-----']:
            sample.write_bytes(private + b'\n')
            assert status() == 1, private
        sample.unlink()
        archive = scan / 'logs/evidence/archives/firecracker.tar'
        archive.parent.mkdir(parents=True)
        archive.write_bytes(lan)
        assert status() == 0
        (archive.parent / 'scratch-main.tar').write_bytes(lan)
        assert status() == 1
        (archive.parent / 'scratch-main.tar').unlink()
        archive.rename(archive.with_name('firecracker.tar.bak'))
        assert status() == 1
        archive.with_name('firecracker.tar.bak').unlink()
        for folder in ['cargo-home/registry/src/pkg', 'cargo-home/git/checkouts/pkg']:
            path = scan / folder
            path.mkdir(parents=True)
            (path / 'sample').write_bytes(lan)
        assert status() == 0
        path = scan / 'cargo-home/git/other'
        path.mkdir()
        (path / 'sample').write_bytes(lan)
        assert status() == 1
    print('Scanner substring masks preserve other same-line hits; nearby paths and private key markers fail')
    with tempfile.TemporaryDirectory() as temporary:
        tracked = Path(temporary)
        subprocess.run(['git', 'init', '-q'], cwd=tracked, check=True)
        (tracked / 'scripts').mkdir()
        (tracked / 'scripts/check-public.sh').write_bytes((ROOT / 'scripts/check-public.sh').read_bytes())
        (tracked / 'sample.txt').write_bytes(b'-----' + b'BEGIN PGP SIGNATURE-----\n')
        subprocess.run(['git', 'add', '.'], cwd=tracked, check=True)
        result = subprocess.run(['bash', 'scripts/check-public.sh', str(denylist)], cwd=tracked,
                                stdout=subprocess.DEVNULL, check=False)
        assert result.returncode == 1
    print('Tree mode keeps every marker as a hit')


if __name__ == '__main__':
    archive_checks()
    scanner_checks(Path(sys.argv[1]).resolve())
