"""Strict, standard-library-only project presentation configuration."""
import copy
import json
import math
import re
from pathlib import Path

DEFAULT = {
    'schema': 1,
    'floor': 'committed',
    'data_branch': 'coverage-data',
    'bootstrap_baseline': '',
    'badges': ['proven', 'tested', 'unverified', 'proof-ratio', 'ratchet'],
    'labels': {'proven': 'proven', 'tested': 'tested', 'unverified': 'unverified',
               'proof-ratio': 'proof/code', 'ratchet': 'ratchet'},
    'proven_thresholds': [90, 75, 50, 25],
    'unverified_thresholds': [10, 25, 50, 75],
    'proof_ratio': {'stage': 'development', 'development': [5, 15],
                    'established': [10, 20],
                    'reason': 'Informational proof maintenance cost per proven runtime line'},
}


def valid_branch(name):
    """A plain data branch name; never the default branch or the badges ref."""
    return (isinstance(name, str)
            and re.fullmatch(r'[A-Za-z0-9_][A-Za-z0-9_.-]*(/[A-Za-z0-9_][A-Za-z0-9_.-]*)*', name) is not None
            and '..' not in name and not name.endswith('.lock')
            and name not in ('main', 'master', 'badges', 'HEAD'))


def load(path=None):
    data = copy.deepcopy(DEFAULT)
    if path:
        supplied = json.loads(Path(path).read_text())
        if not isinstance(supplied, dict) or set(supplied) - set(data):
            raise ValueError('unknown project config keys')
        for key, value in supplied.items():
            if key in ('labels', 'proof_ratio') and isinstance(value, dict):
                if set(value) - set(data[key]):
                    raise ValueError(f'unknown {key} keys')
                data[key].update(value)
            else:
                data[key] = value
    if type(data['schema']) is not int or data['schema'] != 1:
        raise ValueError('expected project config schema 1')
    if data['floor'] not in ('committed', 'recorded'):
        raise ValueError('floor must be "committed" or "recorded"')
    if not valid_branch(data['data_branch']):
        raise ValueError('invalid data_branch')
    if not isinstance(data['bootstrap_baseline'], str) or data['bootstrap_baseline'].startswith('/') or '..' in Path(data['bootstrap_baseline']).parts:
        raise ValueError('bootstrap_baseline must be a relative path inside the project')
    if data['bootstrap_baseline'] and data['floor'] != 'recorded':
        raise ValueError('bootstrap_baseline requires floor "recorded"')
    badges = data['badges']
    if not isinstance(badges, list) or not badges or len(set(badges)) != len(badges) or set(badges) - set(DEFAULT['badges']):
        raise ValueError('invalid badge set')
    if not isinstance(data['labels'], dict) or set(data['labels']) != set(DEFAULT['labels']) or any(
            not isinstance(v, str) or not v.strip() for v in data['labels'].values()):
        raise ValueError('invalid badge labels')
    for key in ('proven_thresholds', 'unverified_thresholds'):
        values = data[key]
        if not isinstance(values, list) or len(values) != 4 or any(
                type(v) not in (int, float) or not math.isfinite(v) or not 0 <= v <= 100 for v in values):
            raise ValueError(f'invalid {key}')
        expected = sorted(values, reverse=key == 'proven_thresholds')
        if values != expected or len(set(values)) != 4:
            raise ValueError(f'unordered {key}')
        # Projects may make colours stricter, never flatter than the source policy.
        if any((v < floor if key == 'proven_thresholds' else v > floor)
               for v, floor in zip(values, DEFAULT[key])):
            raise ValueError(f'{key} may not flatter the reference policy')
    ratio = data['proof_ratio']
    if not isinstance(ratio, dict) or ratio['stage'] not in ('development', 'established') or not isinstance(ratio['reason'], str) or not ratio['reason'].strip():
        raise ValueError('invalid proof_ratio stage/reason')
    for stage in ('development', 'established'):
        bounds = ratio[stage]
        if not isinstance(bounds, list) or len(bounds) != 2 or any(
                type(v) not in (int, float) or not math.isfinite(v) or v < 0 for v in bounds) or bounds[0] > bounds[1]:
            raise ValueError('invalid proof_ratio bounds')
    return data
