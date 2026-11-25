# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
import importlib.util
from pathlib import Path
import unittest
import sys

sys.path.insert(0, str(Path(__file__).parents[1]))
from coverage_metrics import snapshot
from test_ratchet import fixture

spec = importlib.util.spec_from_file_location('write_badges', Path(__file__).parents[1] / 'write-badges.py')
write_badges = importlib.util.module_from_spec(spec)
spec.loader.exec_module(write_badges)

BASE = dict(total=1000, proven=100, tested=200, unverified=700, proof_ratio=None)


class BadgeTests(unittest.TestCase):
    def test_low_proven_share_is_red_not_flattering(self):
        self.assertEqual(write_badges.build(BASE, 'pass')['proven']['color'], 'red')

    def test_proven_thresholds(self):
        colors = [write_badges.proven_color(p) for p in (0, 24.9, 25, 50, 75, 90)]
        self.assertEqual(colors, ['red', 'red', 'orange', 'yellow', 'green', 'brightgreen'])

    def test_unverified_thresholds(self):
        colors = [write_badges.unverified_color(p) for p in (0, 10, 25, 50, 75, 100)]
        self.assertEqual(colors, ['brightgreen', 'green', 'yellow', 'orange', 'red', 'red'])

    def test_missing_ratio_is_not_zero(self):
        badge = write_badges.build(BASE, 'pass')['proof-ratio']
        self.assertEqual((badge['message'], badge['color']), ('n/a', 'lightgrey'))

    def test_ratio_never_red(self):
        for ratio in (0.0, 0.1, 9.0, 80.0):
            self.assertNotEqual(write_badges.ratio_badge(ratio)[1], 'red')

    def test_schema_4_native_ratio_reaches_badge(self):
        report = fixture()
        report['proof_ratio']['ratio'] = 0.927
        badge = write_badges.build(snapshot(report), 'pass')['proof-ratio']
        self.assertEqual((badge['message'], badge['color']), ('0.9', 'yellow'))

    def test_ratchet_fail_is_red(self):
        badge = write_badges.build(BASE, 'fail')['ratchet']
        self.assertEqual((badge['message'], badge['color']), ('fail', 'red'))

    def test_endpoint_schema(self):
        for badge in write_badges.build(BASE, 'pass').values():
            self.assertEqual(set(badge), {'schemaVersion', 'label', 'message', 'color'})
            self.assertEqual(badge['schemaVersion'], 1)


if __name__ == '__main__':
    unittest.main()
