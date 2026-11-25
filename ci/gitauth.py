"""Environment for git commands that authenticate to github.com with GITHUB_TOKEN.

The token travels in an environment-provided HTTP header, never in a remote URL
or a printed command.
"""
import base64
import os


def github_env(token):
    header = base64.b64encode(('x-access-token:' + token).encode()).decode()
    return {**os.environ, 'GIT_CONFIG_COUNT': '1',
            'GIT_CONFIG_KEY_0': 'http.https://github.com/.extraheader',
            'GIT_CONFIG_VALUE_0': 'AUTHORIZATION: basic ' + header}
