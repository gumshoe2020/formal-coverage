"""Module receipt kit tests; local temporary Git remotes only."""
import copy
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import gitenv
from gitenv import git
import receipts
import publish_receipts

SHA = 'a' * 40
TOOL = Path(__file__).resolve().parents[2] / 'target/debug/formal-coverage'


class ReceiptTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.base = self.root / 'inputs'
        self.base.mkdir()
        for name in ['Root.fst', 'Root.fsti', 'Dep.fsti']:
            (self.base / name).write_text(name)
        self.ctx = dict(schema=1, toolchain=dict(fstar='F*', z3='Z3', hax='hax'), modules=[
            dict(file='Root.fst', command=['fstar.exe', 'Root.fst'],
                 dependencies=['Dep.fsti', 'Root.fst', 'Root.fsti'])])
        self.context = self.base / 'fstar.log.context.json'
        self.context.write_text(receipts.encoded(self.ctx))
        self.r = dict(schema=1, module='Root', main_commit=SHA, file='Root.fst',
                      toolchain=self.ctx['toolchain'], command=['fstar.exe', 'Root.fst'],
                      dependencies={p: receipts.sha(self.base / p) for p in self.ctx['modules'][0]['dependencies']},
                      log='Verified module: Root\n')
        self.source = self.root / 'record'
        self.source.mkdir()
        (self.source / 'sha').write_text(SHA)
        (self.source / 'branch').write_text('coverage-data')
        self.save()
        self.remote = self.root / 'remote.git'
        git(self.root, 'init', '--bare', '--quiet', str(self.remote))

    def save(self):
        (self.source / 'Root.json').write_text(receipts.encoded(self.r))

    def test_dependency_closure_includes_interface_project_and_library(self):
        library = self.root / 'toolchain'
        library.mkdir()
        (library / 'Lib.fsti').write_text('library')
        text = (f'Root.fst.checked: Root.fst Root.fsti.checked\n'
                f'Root.fsti.checked: Root.fsti Dep.fsti.checked\n'
                f'Dep.fsti.checked: Dep.fsti /tools/Lib.fsti.checked\n'
                f'/tools/Lib.fsti.checked: /tools/Lib.fsti\n')
        ctx = receipts.context(self.base, text, self.ctx['modules'], self.ctx['toolchain'], [f'/tools={library}'])
        self.assertEqual(ctx['modules'][0]['dependencies'], ['Dep.fsti', 'Root.fst', 'Root.fsti', 'external/0/Lib.fsti'])
        self.assertEqual((self.base / 'external/0/Lib.fsti').read_text(), 'library')
        self.assertFalse(list(self.base.rglob('*.checked')))

    def test_dependency_analysis_fails_closed(self):
        for text in ('diagnostic only', 'Root.fst.checked: Root.fst Missing.checked',
                     'Root.fst.checked: Root.fst Root.fst.checked',
                     'Root.fst.checked: Root.fst @response',
                     'Root.fst.checked: Root.fst\nRoot.fst.checked: Root.fst',
                     'Root.fst.checked: Root.fst /unknown/Lib.fsti.checked'):
            with self.subTest(text=text), self.assertRaises(ValueError):
                receipts.context(self.base, text, self.ctx['modules'], self.ctx['toolchain'])

    def test_publisher_is_append_only_and_coexists_with_baselines(self):
        self.assertTrue(publish_receipts.record(self.source, str(self.remote)))
        clone = self.root / 'clone'
        git(self.root, 'clone', '--quiet', '--branch', 'coverage-data', str(self.remote), str(clone))
        before = git(clone, 'rev-parse', 'HEAD')
        self.assertFalse(publish_receipts.record(self.source, str(self.remote)))
        self.r['log'] += 'different bytes\n'
        self.save()
        with self.assertRaisesRegex(ValueError, 'append-only'):
            publish_receipts.record(self.source, str(self.remote))
        self.assertEqual(git(self.remote, 'rev-parse', 'coverage-data'), before)
        self.assertEqual(git(self.remote, 'for-each-ref', '--format=%(refname)').splitlines(), ['refs/heads/coverage-data'])
        (clone / 'baselines').mkdir()
        (clone / 'baselines/floor.json').write_text('retained')
        git(clone, 'add', '.')
        git(clone, 'commit', '-qm', 'baseline')
        git(clone, 'push', '-q', 'origin', 'coverage-data')
        (self.source / 'sha').write_text('b' * 40)
        self.r['main_commit'] = 'b' * 40
        self.save()
        self.assertTrue(publish_receipts.record(self.source, str(self.remote)))
        self.assertEqual(git(self.remote, 'show', 'coverage-data:baselines/floor.json'), 'retained')
        self.assertIn(f'receipts/{SHA}/Root.json', git(self.remote, 'ls-tree', '-r', '--name-only', 'coverage-data'))

    def test_publisher_event_sha_token_and_invalid_fields(self):
        env = dict(GITHUB_EVENT_NAME='pull_request', GITHUB_REF='refs/heads/main',
                   GITHUB_SHA=SHA, GITHUB_REPOSITORY='owner/repo', GITHUB_TOKEN='github-token')
        with patch.dict(os.environ, env), self.assertRaisesRegex(ValueError, 'restricted'):
            publish_receipts.publish(self.source)
        env.update(GITHUB_EVENT_NAME='push', GITHUB_SHA='b' * 40)
        with patch.dict(os.environ, env), self.assertRaisesRegex(ValueError, 'pushed commit'):
            publish_receipts.publish(self.source)
        env['GITHUB_SHA'] = SHA
        with patch.dict(os.environ, env), patch.object(publish_receipts, 'record') as record:
            publish_receipts.publish(self.source)
            self.assertEqual(record.call_args.args[1], 'https://github.com/owner/repo.git')
            self.assertIn('GIT_CONFIG_VALUE_0', record.call_args.args[2])
        original = copy.deepcopy(self.r)
        for field, value in [('module', 'Other'), ('main_commit', 'short'), ('log', ''),
                             ('command', ['fstar.exe', '--cache_checked_modules', 'Root.fst']),
                             ('dependencies', {}), ('schema', 2)]:
            self.r = original | {field: value}
            self.save()
            with self.subTest(field=field), self.assertRaises(ValueError):
                publish_receipts.load_record(self.source)

    def test_real_rust_selector_reuses_only_exact_merge_base_matches(self):
        self.assertTrue(TOOL.is_file(), 'build cargo binary before kit tests')
        publish_receipts.record(self.source, str(self.remote))
        workspace = self.root / 'workspace'
        git(self.root, 'clone', '--quiet', '--branch', 'coverage-data', str(self.remote), str(workspace))
        output = self.root / 'reuse.json'
        def select(base=SHA):
            return receipts.select(TOOL, self.context, workspace, base, 'coverage-data', output)
        self.assertEqual(select(), [])
        self.assertEqual(set(json.loads(output.read_text())['receipts']), {'Root.fst'})
        cli = gitenv.run([sys.executable, str(Path(receipts.__file__)), 'select',
                          '--tool', str(TOOL), '--context', str(self.context),
                          '--workspace', str(workspace), '--base', SHA,
                          '--output', str(output)], capture_output=True, text=True, check=True)
        self.assertEqual(cli.stdout, '')  # An empty module list means zero checks.
        self.assertEqual(json.loads(output.read_text())['data_branch'], 'coverage-data')
        (self.base / 'Dep.fsti').write_text('changed')
        self.assertEqual(select(), ['Root.fst'])
        self.assertEqual(json.loads(output.read_text())['receipts'], {})
        self.assertEqual(select('b' * 40), ['Root.fst'])
        (self.base / 'Dep.fsti').write_text('Dep.fsti')
        # Malformed records are selected fresh, never placed in a reuse manifest.
        broken = workspace / f'receipts/{SHA}/Root.json'
        broken.write_text('{}')
        git(workspace, 'add', '.')
        git(workspace, 'commit', '-qm', 'malformed')
        self.assertEqual(select(), ['Root.fst'])


if __name__ == '__main__':
    unittest.main()
