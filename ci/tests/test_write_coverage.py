# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from test_ratchet import fixture
from coverage_metrics import snapshot

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
            self.assertEqual(baseline, snapshot(report))
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
