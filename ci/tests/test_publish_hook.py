"""Exercise the public publishers with a hook environment and a file-only remote."""
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

import test_receipts
import test_record
from test_hook_env import SCENARIOS, changed, git, hook_env, make_decoy, snapshot
import publish
from git_temp import temp_repo_env
from gitauth import github_env
from gitenv import LOCAL_ENV_VARS


class PublisherHookTests(unittest.TestCase):
    def test_temp_env_preserves_auth_config(self):
        original = github_env('fixture')
        inherited = dict.fromkeys(LOCAL_ENV_VARS, 'decoy') | original
        cleaned = temp_repo_env(inherited)
        self.assertFalse((set(LOCAL_ENV_VARS) - {'GIT_CONFIG_COUNT'}) & cleaned.keys())
        expected = {k: v for k, v in original.items()
                    if k not in LOCAL_ENV_VARS or k == 'GIT_CONFIG_COUNT'}
        self.assertTrue(cleaned == expected, 'temporary git lost non-repository configuration')

    def check_publication(self, root, remote, sha, branch, message, files, run):
        decoy = make_decoy(root)
        before = snapshot(decoy)
        config = root / 'gitconfig'
        # The public entry points build a GitHub URL. Rewrite just that URL at transport time.
        git(root, 'config', '--file', str(config), f'url.{remote.as_uri()}.insteadOf',
            'https://github.com/example/fixture.git')
        env = dict(hook_env(decoy, SCENARIOS[1]), GIT_CONFIG_GLOBAL=str(config),
                   GIT_ALLOW_PROTOCOL='file', GITHUB_EVENT_NAME='push', GITHUB_REF='refs/heads/main',
                   GITHUB_REPOSITORY='example/fixture', GITHUB_SHA=sha,
                   GH_TOKEN='fixture', GITHUB_TOKEN='fixture')
        with patch.dict(os.environ, env):
            try:
                run()
            finally:
                self.assertEqual(changed(before, snapshot(decoy)), [])
        self.assertEqual(git(remote, 'for-each-ref', '--format=%(refname)'), f'refs/heads/{branch}')
        self.assertEqual(git(remote, 'rev-list', '--count', branch), '1')
        self.assertEqual(git(remote, 'show', '-s', '--format=%s', branch), message)
        self.assertEqual(git(remote, 'ls-tree', '-r', '--name-only', branch).splitlines(), sorted(files))
        for path, content in files.items():
            self.assertEqual(git(remote, 'rev-parse', f'{branch}:{path}'),
                             git(root, 'hash-object', str(content)))

    def test_record(self):
        fixture = test_record.RecordTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        sha = test_record.SHA1
        source = fixture.source(sha)
        self.check_publication(fixture.root, fixture.remote, sha, 'coverage-data',
                               f'Record coverage for {sha}',
                               {f'baselines/{sha}.json': source / 'baseline.json',
                                'COVERAGE-MEASURED.md': source / 'COVERAGE-MEASURED.md'},
                               lambda: test_record.record.publish(source))

    def test_receipts(self):
        fixture = test_receipts.ReceiptTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        sha = test_receipts.SHA
        self.check_publication(fixture.root, fixture.remote, sha, 'coverage-data',
                               f'Record F* receipts for {sha}',
                               {f'receipts/{sha}/Root.json': fixture.source / 'Root.json'},
                               lambda: test_receipts.publish_receipts.publish(fixture.source))

    def test_badges(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            remote = root / 'remote.git'
            git(root, 'init', '--bare', '--quiet', str(remote))
            source = root / 'badges'
            source.mkdir()
            badge = source / 'tested.json'
            badge.write_text(json.dumps(dict(schemaVersion=1, label='tested', message='1', color='green'),
                                        sort_keys=True) + '\n')
            sha = 'c' * 40
            self.check_publication(root, remote, sha, 'badges', f'Update coverage badges for {sha}',
                                   {badge.name: badge}, lambda: publish.publish(source, 'success'))
