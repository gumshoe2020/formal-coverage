"""Regression for real Kani substring filters matching multiple harness names."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from verify_runs import kani_result


class KaniAccounting(unittest.TestCase):
    def test_complete_overlapping_filter_credits_each_named_result_once(self):
        text = ('Checking harness ethernet::verify_from_bytes...\nVERIFICATION:- SUCCESSFUL\n'
                'Checking harness ethernet::verify_from_bytes_unchecked...\nVERIFICATION:- SUCCESSFUL\n'
                'Complete - 2 successfully verified harnesses, 0 failures, 2 total.\n')
        result = kani_result(text, 'ethernet::verify_from_bytes', 0)
        self.assertTrue(result['passed'])
        self.assertEqual(len(result['checked_harnesses']), 2)
        self.assertFalse(kani_result(text, 'other::verify_from_bytes', 0)['passed'])
        self.assertFalse(kani_result(text.replace('2 total', '3 total'), 'ethernet::verify_from_bytes', 0)['passed'])
        self.assertFalse(kani_result(text.replace('VERIFICATION:- SUCCESSFUL', 'VERIFICATION:- FAILED', 1), 'ethernet::verify_from_bytes', 0)['passed'])
        self.assertFalse(kani_result(text, 'ethernet::verify_from_bytes', 1)['passed'])


if __name__ == '__main__':
    unittest.main()
