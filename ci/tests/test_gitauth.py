"""Check Git's ordered extraHeader configuration without contacting a remote."""
import base64
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from gitauth import github_env


class GitAuthTests(unittest.TestCase):
    def test_checkout_header_is_reset_before_kit_header(self):
        key = 'http.https://github.com/.extraheader'
        token = 'synthetic-test-token'
        expected = 'AUTHORIZATION: basic ' + base64.b64encode(
            ('x-access-token:' + token).encode()).decode()
        with tempfile.TemporaryDirectory() as repo:
            env = dict(github_env(token), GIT_CONFIG_NOSYSTEM='1',
                       GIT_CONFIG_GLOBAL='/dev/null')
            subprocess.run(['git', 'init', '-q', repo], env=env, check=True)
            for persisted in (False, True):
                with self.subTest(persisted=persisted):
                    previous = ['Authorization: checkout'] if persisted else []
                    if persisted:
                        subprocess.run(['git', '-C', repo, 'config', '--local', key, previous[0]],
                                       env=env, check=True)
                    values = subprocess.check_output(
                        ['git', '-C', repo, 'config', '--get-all', key], env=env, text=True)
                    # HTTP clears its accumulated list at the empty value. --get-all
                    # shows that reset and the entries on either side in read order.
                    self.assertTrue(values.splitlines() == previous + ['', expected],
                                    'expected a reset followed by only the kit header')
