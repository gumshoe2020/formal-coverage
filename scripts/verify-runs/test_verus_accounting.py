"""Reject replayed Cargo diagnostics and dependency-only proof summaries."""
import unittest
from verus import verus_result


class FreshScope(unittest.TestCase):
    def test_fresh_own_scope_required(self):
        real = ('Checking verifiable-controllers v0.1.0\n'
                'note: verifying module composition::compose_all\n'
                'verification results:: 7 verified, 0 errors (partial verification)\n')
        self.assertEqual(verus_result(real, 'composition'),
                         (7, 0, ['composition::compose_all'], True))
        self.assertFalse(verus_result(real, 'vreplicaset_controller')[3])
        cached = 'note: verifying module kubernetes_api_objects\nFinished in 0.5s\n'
        self.assertEqual(verus_result(cached, 'composition'), (None, None, [], False))
        deps = 'verification results:: 2060 verified, 0 errors\nChecking verifiable-controllers v0.1.0\n'
        self.assertIsNone(verus_result(deps, 'composition')[0])


if __name__ == '__main__':
    unittest.main()
