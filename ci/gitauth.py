"""Environment for git commands that authenticate to github.com with GITHUB_TOKEN.

The token travels in an environment-provided HTTP header, never in a remote URL
or a printed command.
"""
import base64
import os


def github_env(token):
    header = base64.b64encode(('x-access-token:' + token).encode()).decode()
    key = 'http.https://github.com/.extraheader'
    # An empty extraHeader clears checkout's persisted headers for this command.
    return {**os.environ, 'GIT_CONFIG_COUNT': '2',
            'GIT_CONFIG_KEY_0': key, 'GIT_CONFIG_VALUE_0': '',
            'GIT_CONFIG_KEY_1': key,
            'GIT_CONFIG_VALUE_1': 'AUTHORIZATION: basic ' + header}
