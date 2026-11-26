# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
import copy
import sys
from pathlib import Path
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from coverage_metrics import baseline
from coverage_lib import check, snapshot, touched

COMMIT = 'a' * 40
SECURITY_DIFF = 'diff --git a/src/parser.rs b/src/parser.rs\n--- a/src/parser.rs\n+++ b/src/parser.rs\n@@ -10 +10 @@\n-old\n+new\n'
DOC_DIFF = 'diff --git a/README.md b/README.md\n--- a/README.md\n+++ b/README.md\n@@ -1 +1 @@\n-old\n+new\n'
EXCEPTION = f'## Exceptions\n\n| Commit | Approving maintainer | Reason | Restoration obligation |\n| --- | --- | --- | --- |\n| {COMMIT} | Andrew VanLoo | urgent security fix | next change restores pre-fix floor |\n'


def fixture(proven=20, tested=30, unverified=50):
    return dict(schema=4, dependency_inventory_complete=True, unparseable_files=[],
                summary=dict(first_party=dict(security_relevant=dict(sloc=proven+tested+unverified,
                    statuses={'proven': 0, 'proven-extracted': proven, 'tested': tested, 'unverified': unverified}))),
                proof_ratio=dict(proof_sloc=200, ratio=200/proven if proven else None),
                units=[dict(file='src/parser.rs', line=8, end_line=20, first_party=True, security=True, category='runtime')])


def with_depth(report, own, functional):
    proven = snapshot(report)['proven']
    report['proof_ratio'].update(proven_runtime_sloc=0, proven_extracted_runtime_sloc=proven)
    report['summary']['proof_depth'] = dict(safety_only_sloc=proven-own-functional,
                                          functional_own_code_only_sloc=own, functional_sloc=functional)
    return report


class RatchetTests(unittest.TestCase):
    def setUp(self):
        self.report = fixture()
        self.floor = snapshot(self.report)

    def run_check(self, report=None, diff=DOC_DIFF, committed=None, **kw):
        report = self.report if report is None else report
        return check(self.floor, baseline(report) if committed is None else committed, report, diff, **kw)

    def test_docs_exempt_at_equal_share(self):
        self.assertEqual(self.run_check(), [])

    def test_security_requires_strict_increase(self):
        with self.assertRaisesRegex(ValueError, 'without increasing'):
            self.run_check(diff=SECURITY_DIFF)

    def test_security_improvement_passes(self):
        self.assertEqual(self.run_check(fixture(21, 30, 49), SECURITY_DIFF), [])

    def test_equal_share_depth_progress(self):
        self.floor = baseline(with_depth(fixture(), 3, 2))
        for own, functional in ((4, 2), (3, 3)):
            self.assertEqual(self.run_check(with_depth(fixture(), own, functional), SECURITY_DIFF), [])
        with self.assertRaisesRegex(ValueError, 'functional security lines: 5 -> 5'):
            self.run_check(with_depth(fixture(), 2, 3), SECURITY_DIFF)

    def test_depth_gain_preserves_other_rules(self):
        self.floor = baseline(with_depth(fixture(), 0, 0))
        for report, error in ((fixture(19, 31, 50), 'proven share decreased'),
                              (fixture(20, 29, 51), 'unverified security lines increased')):
            with self.subTest(error=error), self.assertRaisesRegex(ValueError, error):
                self.run_check(with_depth(report, 1, 0), SECURITY_DIFF)

    def test_missing_depth_requires_share_gain(self):
        with self.assertRaisesRegex(ValueError, 'without increasing proven share'):
            self.run_check(with_depth(fixture(), 1, 0), SECURITY_DIFF)
        self.floor = baseline(with_depth(fixture(), 0, 0))
        with self.assertRaisesRegex(ValueError, 'without increasing proven share'):
            self.run_check(fixture(), SECURITY_DIFF)

    def test_committed_depth_requires_integer_counts(self):
        report = with_depth(fixture(), 1, 0)
        committed = baseline(report)
        committed['proof_depth']['functional_own_code_only'] = True
        with self.assertRaisesRegex(ValueError, 'does not add up'):
            self.run_check(report, committed=committed)

    def test_malformed_depth_cannot_be_waived(self):
        self.floor = baseline(with_depth(fixture(), 1, 0))
        self.floor['proof_depth']['functional'] += 1
        with self.assertRaisesRegex(ValueError, 'does not add up'):
            self.run_check(markdown=EXCEPTION, commits=[COMMIT])
        report = with_depth(fixture(), 1, 0)
        report['summary']['proof_depth']['functional_sloc'] += 1
        with self.assertRaisesRegex(ValueError, 'does not add up'):
            self.run_check(report, committed=self.floor)

    def test_unverified_increase_fails_even_with_better_share(self):
        with self.assertRaisesRegex(ValueError, 'unverified security lines increased'):
            self.run_check(fixture(40, 30, 51))

    def test_proven_share_decrease_fails_even_with_less_unverified(self):
        with self.assertRaisesRegex(ValueError, 'proven share decreased'):
            self.run_check(fixture(19, 32, 49))

    def test_urgent_exception_names_exact_commit_and_approver(self):
        self.assertTrue(self.run_check(fixture(19, 30, 51), SECURITY_DIFF,
                                      markdown=EXCEPTION, commits=[COMMIT]))

    def test_unrelated_exception_does_not_authorize(self):
        with self.assertRaises(ValueError):
            self.run_check(diff=SECURITY_DIFF, markdown=EXCEPTION, commits=['b'*40])

    def test_missing_approver_is_rejected(self):
        with self.assertRaises(ValueError):
            self.run_check(diff=SECURITY_DIFF, markdown=EXCEPTION.replace('Andrew VanLoo', ''), commits=[COMMIT])

    def test_stale_baseline_fails(self):
        with self.assertRaisesRegex(ValueError, 'does not match'):
            self.run_check(fixture(21, 30, 49), committed=self.floor)

    def test_hand_edited_baseline_fails_even_with_exception(self):
        changed = copy.deepcopy(self.floor)
        changed['proven'] += 1
        with self.assertRaisesRegex(ValueError, 'does not match'):
            self.run_check(committed=changed, markdown=EXCEPTION, commits=[COMMIT])

    def test_ratio_is_informational(self):
        self.report['proof_ratio']['proof_sloc'] = 999999
        self.report['proof_ratio']['ratio'] = 999999
        self.assertEqual(self.run_check(), [])

    def test_schema_3_is_rejected(self):
        self.report['schema'] = 3
        with self.assertRaisesRegex(ValueError, 'schema 3: expected schema 4'):
            snapshot(self.report)

    def test_native_ratio_is_preserved(self):
        for ratio in (0.927, 0.0, None):
            self.report['proof_ratio']['ratio'] = ratio
            measured = snapshot(self.report)
            self.assertEqual(measured['proof_ratio'], ratio)
            self.assertNotIn('native_proof_ratio', measured)

    def test_proof_code_is_excluded_from_runtime_inventory(self):
        proof = dict(self.report['units'][0], line=30, end_line=40,
                     category='proof-code', status='verified')
        self.report['units'].append(proof)
        self.assertEqual(snapshot(self.report), self.floor)

    def test_deleted_security_is_classified_from_old_inventory(self):
        diff = SECURITY_DIFF.replace('+++ b/src/parser.rs', '+++ /dev/null').replace('@@ -10 +10 @@', '@@ -10 +0,0 @@')
        self.assertTrue(touched(diff, [self.floor['inventory'], []]))

    def test_inline_test_edit_exempt(self):
        diff = SECURITY_DIFF.replace('@@ -10 +10 @@', '@@ -100 +100 @@')
        self.assertEqual(self.run_check(diff=diff), [])

    def test_renaming_security_without_proof_is_not_improvement(self):
        diff = 'diff --git a/src/parser.rs b/src/renamed.rs\nsimilarity index 100%\nrename from src/parser.rs\nrename to src/renamed.rs\n'
        with self.assertRaisesRegex(ValueError, 'without increasing'):
            self.run_check(diff=diff)

    def test_incomplete_inventory_fails_closed(self):
        self.report['dependency_inventory_complete'] = False
        with self.assertRaisesRegex(ValueError, 'incomplete inventory'):
            self.run_check()

    def test_unparseable_source_fails_closed(self):
        self.report['unparseable_files'] = ['src/parser.rs']
        with self.assertRaisesRegex(ValueError, 'unparseable'):
            self.run_check()

    def test_trusted_and_bounded_remain_unverified(self):
        statuses = self.report['summary']['first_party']['security_relevant']['statuses']
        statuses['unverified'] = 40
        statuses['trusted'] = 5
        statuses['proven-bounded'] = 5
        self.assertEqual(snapshot(self.report)['unverified'], 50)


if __name__ == '__main__':
    unittest.main()
