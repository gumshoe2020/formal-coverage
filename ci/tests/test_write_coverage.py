# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from test_ratchet import fixture
from coverage_metrics import proof_depth, snapshot

ROOT = Path(__file__).resolve().parents[2]


class WriteCoverageTests(unittest.TestCase):
    def render(self, report):
        target = ROOT / 'target'
        target.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(dir=target) as directory:
            work = Path(directory)
            page = work / 'COVERAGE.md'
            page.write_text('Intro\n<!-- measured coverage: begin -->\nold\n'
                            '<!-- measured coverage: end -->\n## Exceptions\nuntouched\n')
            path = work / 'report.json'
            path.write_text(json.dumps(report))
            command = [sys.executable, str(ROOT / 'ci/write-coverage.py'), str(path)]
            result = subprocess.run(command, cwd=work, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            baseline = json.loads((work / 'coverage/baseline.json').read_text())
            expected = snapshot(report)
            if proof_depth(report) is not None:
                expected['proof_depth'] = proof_depth(report)
            self.assertEqual(baseline, expected)
            result = subprocess.run(command + ['--check'], cwd=work, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            text = page.read_text()
            self.assertTrue(text.startswith('Intro\n'))
            self.assertTrue(text.endswith('## Exceptions\nuntouched\n'))
            return text

    def report(self):
        report = fixture()
        report['proof_ratio'] = dict(
            proof_sloc=509, ratio=509/549, proven_runtime_sloc=0,
            proven_extracted_runtime_sloc=549, fstar_annotation_sloc=268,
            fstar_cfg_hax_sloc=197, fstar_cfg_hax_inline_sloc=57,
            fstar_model_sloc=18,
            modules=[dict(crate_name='bertie_tls', module='parser', proof_sloc=509,
                          proven_runtime_sloc=0, proven_extracted_runtime_sloc=549,
                          ratio=509/549)])
        return report

    def test_native_ratio_components_and_module_table(self):
        text = self.render(self.report())
        self.assertIn('Proof-to-code ratio: **0.93**', text)
        self.assertIn('509 proof SLoC / 549 proven + proven-extracted runtime SLoC', text)
        self.assertIn('268 hax annotation SLoC', text)
        self.assertIn('197 standalone `cfg(hax)` proof-code SLoC', text)
        self.assertIn('57 inline `cfg(hax)` SLoC (overlaps count once)', text)
        self.assertIn('18 SLoC of trusted assumptions, excluded from the ratio', text)
        self.assertIn('| bertie_tls::parser | 549 | 509 | 0.93 |', text)
        self.assertNotIn('not measured', text)

    def test_proof_depth_table_and_metrics(self):
        report = self.report()
        totals = report['summary']['first_party']['security_relevant']
        totals['statuses']['proven-extracted'] = 549
        totals['sloc'] = sum(totals['statuses'].values())
        report['summary']['proof_depth'] = dict(
            safety_only_sloc=400, functional_own_code_only_sloc=50, functional_sloc=99)
        text = self.render(report)
        self.assertIn('### Proof depth', text)
        self.assertIn('| safety-only | 400 | 72.86% |', text)
        self.assertIn('| functional, checked only against own code | 50 | 9.11% |', text)
        self.assertIn('| functional | 99 | 18.03% |', text)
        self.assertIn('it says nothing about what the code computes.', text)
        self.assertEqual(proof_depth(report), dict(
            safety_only=400, functional_own_code_only=50, functional=99))
        self.assertNotIn('proof_depth', snapshot(report))

    def test_reports_without_depth_have_no_depth_table(self):
        report = self.report()
        self.assertIsNone(proof_depth(report))
        self.assertNotIn('Proof depth', self.render(report))

    def test_depth_must_add_up_to_proven_runtime_lines(self):
        report = self.report()
        report['summary']['proof_depth'] = dict(
            safety_only_sloc=1, functional_own_code_only_sloc=0, functional_sloc=0)
        with self.assertRaisesRegex(ValueError, 'does not add up'):
            proof_depth(report)

    def test_zero_ratio_is_measured(self):
        report = self.report()
        report['proof_ratio'].update(proof_sloc=0, ratio=0.0, modules=[])
        self.assertIn('Proof-to-code ratio: **0.00**', self.render(report))

    def test_missing_ratio_is_na(self):
        report = self.report()
        report['proof_ratio'].update(ratio=None, proven_extracted_runtime_sloc=0)
        report['proof_ratio']['modules'][0].update(ratio=None, proven_extracted_runtime_sloc=0)
        text = self.render(report)
        self.assertIn('Proof-to-code ratio: **n/a**', text)
        self.assertIn('| bertie_tls::parser | 0 | 509 | n/a |', text)


if __name__ == '__main__':
    unittest.main()
