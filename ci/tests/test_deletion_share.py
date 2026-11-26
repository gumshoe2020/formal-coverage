import copy
import hashlib
import json
from pathlib import Path
import shutil
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import gitenv
from coverage_lib import RatchetFailure, check
from coverage_metrics import baseline
from test_ratchet import fixture, with_depth

ROOT = Path(__file__).resolve().parents[2]
PATCH = ('diff --git a/parser.rs b/parser.rs\n--- a/parser.rs\n+++ b/parser.rs\n'
         '@@ -8,2 +7,0 @@\n-a\n-b\n')


def report(proven=4065, tested=500, unverified=467, functional=0):
    r = with_depth(fixture(proven, tested, unverified), 0, functional)
    def unit(name, status, line, sloc):
        return dict(file='parser.rs', id=name, line=line, end_line=line+sloc-1,
                    first_party=True, security=True, category='runtime', status=status,
                    inline_proof_sloc=0, sloc=sloc)
    r['units'] = [unit('f', 'proven-extracted', 1, proven-4000),
                  unit('g', 'proven-extracted', 100, 4000),
                  unit('t', 'tested', 5000, tested), unit('u', 'unverified', 6000, unverified)]
    return r


class DeletionShareTests(unittest.TestCase):
    def compare(self, r, floor=None, diff=PATCH):
        return check(baseline(report()) if floor is None else floor, baseline(r), r, diff)

    def test_bertie(self):
        self.assertEqual(self.compare(report(4063, functional=12)), [])
        with self.assertRaisesRegex(RatchetFailure, 'functional security lines: 0 -> 0'):
            self.compare(report(4063))

    def test_bertie_u16_contract(self):
        data = ROOT / 'ci/tests/fixtures/bertie-u16'
        diff = (data / 'change.diff').read_text()

        def encoder_report(name, functional):
            unit = json.loads((data / (name + '.json')).read_text())
            # One unchanged tested line makes a proven deletion lower the share.
            r = fixture(unit['sloc'] + 369, tested=1, unverified=0)
            r['units'] = [unit, dict(unit, file='other.rs', id='stable_proof', line=1,
                                     end_line=369, sloc=369), dict(file=unit['file'], id='background', line=1, end_line=1,
                                     first_party=True, security=True, category='runtime',
                                     sloc=1, status='tested', inline_proof_sloc=0)]
            proof = unit['inline_proof_sloc']
            r['proof_ratio'].update(proof_sloc=proof, ratio=proof / unit['sloc'])
            return with_depth(r, 0, functional)

        floor = baseline(encoder_report('floor', 369))
        measured = encoder_report('measured', 381)
        measured['units'].append(json.loads((data / 'added-test.json').read_text()))
        self.assertEqual(check(floor, baseline(measured), measured, diff), [])
        shallow = encoder_report('measured', 369)
        shallow['units'].append(measured['units'][-1])
        with self.assertRaisesRegex(RatchetFailure, '^security code changed.*functional security lines: 369 -> 369'):
            check(floor, baseline(shallow), shallow, diff)

    def test_replacement_of_deleted_proven_unit(self):
        diff = PATCH + '@@ -20,0 +19,2 @@\n+fn replacement() {\n+}\n'
        for evidence, category, depth, reason in [
            ('none', 'runtime', 12, 'unmatched replacement'),  # renamed proven function
            ('cfg-test', 'test', 12, None),
            ('cfg-test', 'test', 0, 'functional security lines: 0 -> 0'),
        ]:
            with self.subTest(evidence=evidence, depth=depth):
                r = report(4063, functional=depth)
                old = copy.deepcopy(r)
                old['units'].append(dict(r['units'][0], id='gone', line=8, end_line=9, sloc=2))
                old['summary'] = report()['summary']
                old['proof_ratio'] = report()['proof_ratio']
                replacement = dict(old['units'][-1], id='replacement', line=19, end_line=20,
                                   category=category, test_evidence=evidence)
                if category == 'runtime':
                    r['units'][0]['sloc'] -= 2
                r['units'].append(replacement)
                if reason:
                    with self.assertRaisesRegex(RatchetFailure, reason):
                        self.compare(r, baseline(old), diff)
                else:
                    self.assertEqual(self.compare(r, baseline(old), diff), [])

    def test_unrecognized_test_attribute_replacement(self):
        r = report(4063, functional=12)
        r['units'][-1]['sloc'] -= 2
        helper = dict(r['units'][-1], id='authorize', line=19, end_line=20, sloc=2,
                      test_evidence='none')
        r['units'].append(helper)
        diff = PATCH + '@@ -20,0 +19,2 @@\n+#[attrs::test]\n+fn authorize() {}\n'
        with self.assertRaisesRegex(RatchetFailure, 'unmatched replacement.*authorize'):
            self.compare(r, diff=diff)

    def test_old_floor(self):
        floor = baseline(report())
        floor['schema'] = 1
        floor['inventory'] = [{k: u[k] for k in ('file', 'line', 'end_line')} for u in floor['inventory']]
        with self.assertRaisesRegex(RatchetFailure, 'merge-base floor lacks per-unit statuses'):
            self.compare(report(4063, functional=12), floor)

    def test_demotion_hidden_by_deletion(self):
        r = report(4063, functional=12)
        # Demote a two-line proven unit and remove two previously unverified lines.
        old = report()
        old['units'][0]['sloc'] -= 2
        old['units'].append(dict(old['units'][0], id='demoted', line=70, end_line=71, sloc=2))
        r['units'].append(dict(old['units'][-1], status='unverified'))
        r['units'][-2]['sloc'] -= 2
        with self.assertRaisesRegex(RatchetFailure, 'demoted to unverified'):
            self.compare(r, baseline(old))

    def test_ambiguous_or_excluded(self):
        for change, reason in (({'id': 'renamed'}, 'unmatched'), ({'security': False}, 'left security'),
                               ({'category': 'test'}, 'left security')):
            r = report(4063, functional=12)
            if 'id' in change:
                r['units'][0].update(change)
            else:
                r['units'].append(dict(r['units'][0], **change))
                r['units'][0]['id'] = 'replacement'
            with self.subTest(change=change), self.assertRaisesRegex(RatchetFailure, reason):
                self.compare(r)
        r = report(4063, functional=12)
        r['units'].append(dict(r['units'][0], security=False))
        with self.assertRaisesRegex(RatchetFailure, 'ambiguous'):
            self.compare(r)

    def test_moved_to_excluded_file(self):
        moved = PATCH + ('diff --git a/other.rs b/other.rs\n--- /dev/null\n+++ b/other.rs\n'
                         '@@ -0,0 +1,2 @@\n+a\n+b\n')
        with self.assertRaisesRegex(RatchetFailure, 'outside the measured inventory'):
            self.compare(report(4063, functional=12), diff=moved)

    def test_count_increases(self):
        for t, u, reason in ((501, 467, 'tested'), (500, 468, 'unverified')):
            with self.subTest(reason=reason), self.assertRaisesRegex(RatchetFailure, reason+' security lines increased'):
                self.compare(report(4063, t, u, 12))

    def test_whole_unit_requires_source_deletion(self):
        r = report(4063, functional=12)
        old = copy.deepcopy(r)
        old['units'].append(dict(r['units'][0], id='gone', line=8, end_line=9, sloc=2))
        old['summary'] = report()['summary']
        old['proof_ratio'] = report()['proof_ratio']
        self.assertEqual(self.compare(r, baseline(old)), [])
        with self.assertRaisesRegex(RatchetFailure, 'unmatched'):
            self.compare(r, baseline(old), '')

    def test_real_report_and_git_diff(self):
        # These synthetic receipts exercise the CLI's attribution and depth accounting.
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            shutil.copytree(ROOT / 'fixtures/hax', work, dirs_exist_ok=True)
            src = work / 'src/lib.rs'
            lines = '    let x = x;\n' * 11
            src.write_text('pub fn clean(x: u32) -> u32 {\n' + lines + '    x\n}\nfn other() {}\n')
            (work / 'formal-coverage.toml').write_text('security=["hax_fixture::*"]\n')
            (work / 'run.log').write_text('Verified module: Hax_fixture\n')
            gitenv.run(['cargo', 'generate-lockfile', '--offline'], cwd=work, check=True)
            def hashes(paths):
                return {p: hashlib.sha256((work / p).read_bytes()).hexdigest() for p in paths}

            def measure(contract):
                for sub in ('fresh', 'checked'):
                    dst = work / sub
                    dst.mkdir(exist_ok=True)
                    (dst / 'Hax_fixture.fst').write_text('module Hax_fixture\nlet clean (x: u32) : Tot u32 = x\n')
                    (dst / 'Hax_fixture.fsti').write_text(f'module Hax_fixture\nval clean (x: u32) : {contract}\n')
                module = 'checked/Hax_fixture.fst'
                receipt = dict(tool='hax-fstar', hax_version='fixture', hax_commit='1'*40,
                               fstar_version='fixture', extraction_command=['python3', 'hax-driver.py', 'extract-fstar'],
                               extraction_exit_code=0, files=hashes(['src/lib.rs', 'Cargo.toml', 'Cargo.lock', 'hax-driver.py']),
                               patches={}, log_sha256=hashes(['run.log'])['run.log'],
                               modules=[dict(file=module, command=['fstar.exe', module], exit_code=0, outcome='success')])
                for sub in ('fresh', 'checked'):
                    receipt[sub] = hashes([f'{sub}/Hax_fixture.{ext}' for ext in ('fst', 'fsti')])
                (work / 'run.log.evidence.json').write_text(json.dumps(receipt))
                gitenv.run([ROOT / 'target/debug/formal-coverage', 'report', '.', '--size-mode', 'whole-crate',
                            '--fstar-log', work / 'run.log', '--json', 'report.json'], cwd=work,
                           capture_output=True, check=True)
                return json.loads((work / 'report.json').read_text())

            gitenv.git(work, 'init', '-q')
            gitenv.git(work, 'add', 'src')
            gitenv.git(work, 'commit', '-qm', 'floor')
            initial = measure('Tot u32')
            self.assertTrue(initial['evidence_runs'][0]['accepted'], initial['evidence_runs'])
            floor = baseline(initial)
            src.write_text(src.read_text().replace('    let x = x;\n', '', 2))
            diff = gitenv.git(work, 'diff', '-U0', 'HEAD')
            r = measure('Tot u32')
            with self.assertRaisesRegex(RatchetFailure, '^security code changed.*functional security lines: 0 -> 0'):
                check(floor, baseline(r), r, diff)
            r = measure('Pure u32 True (ensures fun result -> result == x)')
            measured = baseline(r)
            self.assertEqual(measured['proven'], floor['proven'] - 2)
            self.assertEqual(measured['total'], floor['total'] - 2)
            self.assertEqual(measured['proof_depth']['functional'], 12)
            self.assertEqual(check(floor, measured, r, diff), [])
