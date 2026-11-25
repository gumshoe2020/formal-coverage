#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
"""Write shields.io endpoint JSON badges from the committed baseline.

usage: write-badges.py BASELINE_JSON OUT_DIR RATCHET_RESULT   (RATCHET_RESULT: pass|fail)
Colors follow the plain thresholds documented in COVERAGE.md and never flatter.
The proof-to-code ratio is formal-coverage's native ratio, projected into the
baseline by coverage_metrics. Its badge is informational: it is never red.
"""
import argparse
import json
from config import load
from pathlib import Path
import sys

RATIO_RANGE = (5.0, 15.0)


def proven_color(percent, thresholds=None):
    for floor, color in zip(thresholds or [90, 75, 50, 25], ['brightgreen', 'green', 'yellow', 'orange']):
        if percent >= floor:
            return color
    return 'red'


def unverified_color(percent, thresholds=None):
    for ceiling, color in zip(thresholds or [10, 25, 50, 75], ['brightgreen', 'green', 'yellow', 'orange']):
        if percent < ceiling:
            return color
    return 'red'


def ratio_badge(ratio, bounds=RATIO_RANGE):
    if ratio is None:
        return 'n/a', 'lightgrey'
    low, high = bounds
    return f'{ratio:.1f}', 'brightgreen' if low <= ratio <= high else 'yellow'


def badge(label, message, color):
    return dict(schemaVersion=1, label=label, message=message, color=color)


def build(baseline, ratchet_result, config=None):
    config = load() if config is None else config
    total = baseline['total']
    percent = lambda lines: 100 * lines / total
    ratio_message, ratio_color = ratio_badge(baseline['proof_ratio'], config['proof_ratio'][config['proof_ratio']['stage']])
    passed = ratchet_result == 'pass'
    result = {
        'proven': badge('proven', f"{percent(baseline['proven']):.1f}%", proven_color(percent(baseline['proven']), config['proven_thresholds'])),
        'tested': badge('tested', f"{percent(baseline['tested']):.1f}%", 'blue'),
        'unverified': badge('unverified', f"{baseline['unverified']} lines",
                            unverified_color(percent(baseline['unverified']), config['unverified_thresholds'])),
        'proof-ratio': badge('proof/code', ratio_message, ratio_color),
        'ratchet': badge('ratchet', 'pass' if passed else 'fail', 'brightgreen' if passed else 'red'),
    }

    for name, value in result.items():
        value['label'] = config['labels'][name]
    return {name: result[name] for name in config['badges']}


def main(argv):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('baseline_path')
    parser.add_argument('out_dir')
    parser.add_argument('ratchet_result', choices=['pass', 'fail'])
    parser.add_argument('--config')
    args = parser.parse_args(argv)
    baseline_path, out_dir, ratchet_result = args.baseline_path, args.out_dir, args.ratchet_result
    if ratchet_result not in ('pass', 'fail'):
        raise SystemExit('RATCHET_RESULT must be pass or fail')
    badges = build(json.loads(Path(baseline_path).read_text()), ratchet_result, load(args.config))
    Path(out_dir).mkdir(parents=True, exist_ok=True)
    for name, content in badges.items():
        (Path(out_dir) / f'{name}.json').write_text(json.dumps(content, sort_keys=True) + '\n')


if __name__ == '__main__':
    main(sys.argv[1:])
