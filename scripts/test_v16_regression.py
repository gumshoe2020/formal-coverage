#!/usr/bin/env python3
"""Ensure compatibility normalization can remove only the documented additions."""
import unittest
from v16_regression import legacy_json, legacy_markdown


class Compatibility(unittest.TestCase):
    def test_only_new_json_fields_removed(self):
        old = {'schema': 3, 'units': [{'sloc': 5, 'status': 'proven'}], 'warnings': ['keep']}
        new = {**old, 'ghost_lines': 'runtime', 'unparseable_files': [],
               'proof_ratio_range': {'min': 1, 'max': 10},
               'no_proof_ratio_gate': False, 'proof_ratio': {'ratio': 2}}
        new['units'] = [{**old['units'][0], 'runtime_sloc': 5, 'inline_proof_sloc': 0, 'ghost_sloc': 2}]
        self.assertEqual(legacy_json(new), old)
        new['units'][0]['sloc'] = 4
        self.assertNotEqual(legacy_json(new), old)
        new['warnings'] = []
        self.assertNotEqual(legacy_json(new), old)

    def test_only_exact_markdown_additions_removed(self):
        old = 'Size mode: `whole-crate`;\n\nold body\n\n## Units\n\nold units\n'
        new = old.replace(';\n', '; Ghost lines: `runtime`;\n', 1)
        self.assertEqual(legacy_markdown(new), old)
        section = '## Proof ratio\n\nnew table\n\n'
        with_ratio = new.replace('\n## Units\n', section + '\n## Units\n')
        self.assertEqual(legacy_markdown(with_ratio), old)
        self.assertNotEqual(legacy_markdown(new + 'unknown addition\n'), old)
        with self.assertRaises(AssertionError):
            legacy_markdown(new.replace('`runtime`', '`separate`'))


if __name__ == '__main__':
    unittest.main()
