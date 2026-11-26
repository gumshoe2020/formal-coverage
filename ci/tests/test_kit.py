import copy
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import textwrap
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import config
import publish
import run
import test_badges
import test_write_coverage
from test_ratchet import fixture, SECURITY_DIFF, DOC_DIFF, EXCEPTION, COMMIT
from coverage_lib import check, snapshot

ROOT = Path(__file__).resolve().parents[2]


class ConfigTests(unittest.TestCase):
    def load(self, data):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory, 'config.json')
            path.write_text(json.dumps(data))
            return config.load(path)

    def test_example_equals_defaults(self):
        self.assertEqual(config.load(ROOT / 'ci/project.example.json'), config.load())

    def test_partial_config_merges_labels(self):
        data = self.load({'labels': {'proven': 'proven (hax trusted)'}, 'badges': ['proven']})
        self.assertEqual(data['badges'], ['proven'])
        self.assertEqual(data['labels']['tested'], 'tested')

    def test_fail_closed_on_invalid_config(self):
        for data in ({'typo': 1}, {'schema': 2}, {'badges': ['unknown']},
                     {'badges': ['proven', 'proven']}, {'labels': {'proven': ''}},
                     {'proof_ratio': {'reason': ''}}, {'proof_ratio': {'stage': 'unknown'}},
                     {'proof_ratio': {'development': [15, 5]}},
                     {'proven_thresholds': [80, 75, 50, 25]},
                     {'unverified_thresholds': [11, 25, 50, 75]},
                     {'proven_thresholds': [90, 75, 50, float('nan')]}):
            with self.subTest(data=data), self.assertRaises(ValueError):
                self.load(data)

    def test_badge_set_labels_and_established_range(self):
        data = self.load({'badges': ['proof-ratio'], 'labels': {'proof-ratio': 'cost'},
                          'proof_ratio': {'stage': 'established'}})
        baseline = dict(total=100, proven=20, tested=30, unverified=50, proof_ratio=17)
        badges = test_badges.write_badges.build(baseline, 'pass', data)
        self.assertEqual(set(badges), {'proof-ratio'})
        self.assertEqual(badges['proof-ratio']['label'], 'cost')
        self.assertEqual(badges['proof-ratio']['color'], 'brightgreen')

    def test_stricter_colors_permitted(self):
        self.load({'proven_thresholds': [95, 80, 60, 30]})

    def test_argument_array_no_shell_evaluation(self):
        self.assertEqual(run.report_arguments('["--fstar-log", "a b;$(false)"]'),
                         ['--fstar-log', 'a b;$(false)'])
        for value in ('"--json evil"', '[1]', '["--json=evil"]', '["--markdown", "evil"]'):
            with self.assertRaises(ValueError):
                run.report_arguments(value)


class ExtraRatchetTests(unittest.TestCase):
    def test_ci_only_exempt_but_cannot_lower_floor(self):
        report = fixture()
        diff = DOC_DIFF.replace('README.md', '.github/workflows/ci.yml')
        self.assertEqual(check(snapshot(report), snapshot(report), report, diff), [])
        worse = fixture(19, 30, 51)
        with self.assertRaises(ValueError):
            check(snapshot(report), snapshot(worse), worse, diff)

    def test_exception_requires_reason_restoration_and_full_sha(self):
        report = fixture()
        for text in (EXCEPTION.replace('urgent security fix', ''),
                     EXCEPTION.replace('next change restores pre-fix floor', ''),
                     EXCEPTION.replace(COMMIT, COMMIT[:7])):
            with self.assertRaises(ValueError):
                check(snapshot(report), snapshot(report), report, SECURITY_DIFF, text, [COMMIT])


class PublishTests(unittest.TestCase):
    def test_publication_restricted_before_network(self):
        with patch.dict(os.environ, {'GITHUB_EVENT_NAME': 'pull_request', 'GITHUB_REF': 'refs/heads/main'}):
            with self.assertRaisesRegex(ValueError, 'push on main'):
                publish.publish(Path('/unused'), 'success')

    def test_artifact_validation_and_failure_color(self):
        with tempfile.TemporaryDirectory() as directory:
            source = Path(directory)
            badge = dict(schemaVersion=1, label='ratchet', message='pass', color='brightgreen')
            (source / 'ratchet.json').write_text(json.dumps(badge))
            failed = json.loads(publish.validate(source, 'failure')['ratchet.json'])
            self.assertEqual((failed['message'], failed['color']), ('fail', 'red'))
            badge.update(message='off', color='lightgrey')
            (source / 'ratchet.json').write_text(json.dumps(badge))
            self.assertEqual(json.loads(publish.validate(source, 'failure')['ratchet.json']), badge)
            (source / 'unexpected.json').write_text('{}')
            with self.assertRaises(ValueError):
                publish.validate(source, 'success')


class RunnerTests(unittest.TestCase):
    def test_action_ratchet_output(self):
        action = (ROOT / '.github/actions/coverage/action.yml').read_text()
        self.assertIn("default: 'false'", action.split('  enforce-ratchet:', 1)[1].split('  workspace:', 1)[0])
        self.assertIn('value: ${{ steps.ratchet.outputs.result }}', action)
        step = action.split('    - id: ratchet\n')[1].split('    - name:')[0]
        script = textwrap.dedent(step.split('      run: |\n')[1])
        for enforce in ('true', 'false'):
            for outcome in ('success', 'failure', 'skipped'):
                with self.subTest(enforce=enforce, outcome=outcome), tempfile.NamedTemporaryFile() as output:
                    subprocess.run(['bash', '-ec', script], check=True, env=dict(
                        os.environ, ENFORCE_RATCHET=enforce, RESULT=outcome, GITHUB_OUTPUT=output.name))
                    expected = outcome if enforce == 'true' else 'not-enforced'
                    self.assertEqual(output.read().decode().strip(), 'result=' + expected)

    def test_local_pipeline_and_missing_floor(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            def git(*args):
                return subprocess.check_output(['git', '-C', directory, *args], stderr=subprocess.DEVNULL, text=True).strip()
            git('init')
            report = test_write_coverage.WriteCoverageTests().report()
            (work / 'fixture.json').write_text(json.dumps(report))
            (work / 'COVERAGE.md').write_text('Intro\n<!-- measured coverage: begin -->\n<!-- measured coverage: end -->\n')
            subprocess.run([sys.executable, str(ROOT / 'ci/write-coverage.py'), 'fixture.json'],
                           cwd=work, capture_output=True, check=True)
            git('add', '.')
            git('-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-m', 'initial reviewed floor')
            base = git('rev-parse', 'HEAD')
            (work / 'README.md').write_text('Docs only\n')
            git('add', '.')
            git('-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-m', 'docs')
            tool = work / 'tool'
            tool.write_text('#!' + sys.executable + '\nimport sys,shutil\nfrom pathlib import Path\na=sys.argv\nshutil.copyfile("fixture.json",a[a.index("--json")+1])\nPath(a[a.index("--markdown")+1]).write_text("fixture")\n')
            tool.chmod(0o755)
            output = work / 'out'
            command = [sys.executable, str(ROOT / 'ci/run.py'), '--tool', str(tool),
                       '--workspace', str(work), '--base', base, '--output', str(output)]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn('ratchet PASS', (output / 'summary.md').read_text())
            self.assertEqual(json.loads((output / 'badges/ratchet.json').read_text())['message'], 'pass')
            self.assertIn('informational only', (output / 'summary.md').read_text())
            with self.assertRaises(ValueError):
                run.floor_and_diff(work, 'coverage/baseline.json', '')
            with self.assertRaises(subprocess.CalledProcessError):
                run.floor_and_diff(work, 'missing-baseline.json', base)
            off = command + ['--enforce-ratchet', 'false']
            for invalid in ('False', 'TRUE', '', '0'):
                result = subprocess.run(command + ['--enforce-ratchet', invalid], capture_output=True, text=True)
                self.assertNotEqual(result.returncode, 0)
                self.assertIn('invalid choice', result.stderr)
            baseline_path = work / 'coverage/baseline.json'
            baseline_path.write_text(baseline_path.read_text() + '\n')
            summary = work / 'job-summary'
            result = subprocess.run(off, capture_output=True, text=True, env=dict(os.environ, GITHUB_STEP_SUMMARY=str(summary)))
            self.assertEqual(result.returncode, 0, result.stderr)
            notice = 'ratchet: not enforced (would report ratchet FAIL: baseline.json or COVERAGE.md differs'
            self.assertIn(notice, result.stdout)
            self.assertIn(notice, summary.read_text())
            result = subprocess.run(command + ['--enforce-ratchet', 'true'], capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('baseline.json or COVERAGE.md differs', result.stderr)
            valid = baseline_path.read_text()
            invalid = json.loads(valid)
            invalid['total'] = 0
            baseline_path.write_text(json.dumps(invalid))
            git('add', 'coverage/baseline.json')
            git('-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-m', 'bad floor')
            baseline_path.write_text(valid)
            result = subprocess.run(off + ['--base', 'HEAD'], capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('invalid merge-base totals', result.stderr)
            report['dependency_inventory_complete'] = False
            (work / 'fixture.json').write_text(json.dumps(report))
            result = subprocess.run(off, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('incomplete inventory', result.stderr)
            # Every tool failure blocks, even when it produced a plausible report.
            tool.write_text('#!' + sys.executable + '\nraise SystemExit(7)\n')
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)


if __name__ == '__main__':
    unittest.main()
