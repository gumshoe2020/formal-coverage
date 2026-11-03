#!/usr/bin/env python3
"""Tests for the v1.5 Linux container driver (v15_replay.py --container) and v15_size_check.py."""
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import v15_replay  # noqa: E402
import v15_size_check  # noqa: E402

REPO = v15_replay.REPO
CASE = dict(source='target/release-survey/vest', target='aarch64-unknown-linux-gnu',
            cargo_home='runs/v1.4/vest/cargo-home', evidence=[['verus', 'docs/survey/vest.verus.log']])


class ContainerPaths(unittest.TestCase):
    def test_repository_paths_map_to_the_container_root(self):
        self.assertEqual(v15_replay.container_path(str(REPO / 'target/linux-tool/release/formal-coverage')),
                         '/fc/target/linux-tool/release/formal-coverage')
        self.assertEqual(v15_replay.container_path(str(REPO)), '/fc')

    def test_relative_flags_and_outside_paths_are_unchanged(self):
        for value in ['target/release-survey/vest', '--size-mode', 'linked-lines', '/usr/local/bin/cargo']:
            self.assertEqual(v15_replay.container_path(value), value)

    def test_host_command_is_unchanged(self):
        args, env = v15_replay.command_line('target/release/formal-coverage', CASE, Path('out'),
                                            ['--size-mode', 'whole-crate'])
        self.assertEqual(args[0], str((REPO / 'target/release/formal-coverage').resolve()))
        self.assertEqual(args[1:4], ['report', 'target/release-survey/vest', '--target'])
        self.assertEqual(args[-2:], ['--size-mode', 'whole-crate'])
        self.assertEqual(env['CARGO_HOME'], str(REPO.resolve() / 'runs/v1.4/vest/cargo-home'))

    def test_container_command_has_no_host_paths(self):
        binary = REPO / 'target/linux-tool/release/formal-coverage'
        args, env = v15_replay.command_line(binary, CASE, REPO / 'logs/x/vest',
                                            ['--binary', str(REPO / 'target/b/vest')], container='fcsurvey-x')
        self.assertEqual(env, {})
        self.assertEqual(args[:4], ['docker', 'exec', '-w', '/fc'])
        self.assertIn('CARGO_HOME=/fc/runs/v1.4/vest/cargo-home', args)
        self.assertIn('TMPDIR=/fc/.tmp', args)
        tool = args.index('fcsurvey-x') + 1
        self.assertEqual(args[tool], '/fc/target/linux-tool/release/formal-coverage')
        self.assertIn('/fc/logs/x/vest/report.json', args)
        self.assertEqual(args[-1], '/fc/target/b/vest')
        self.assertFalse([a for a in args if str(REPO.resolve()) in a])

    def test_portable_report_removes_the_container_prefix(self):
        text = 'reading /fc/runs/v1.4/vest/cargo-home/x.d and /fc/.tmp/y'
        self.assertEqual(v15_replay.portable_report(text, 'fcsurvey-x'), 'reading runs/v1.4/vest/cargo-home/x.d and .tmp/y')
        self.assertEqual(v15_replay.portable_report(text), text)


def report(deps, first=(1, 10), default=(1, 12), third=None):
    block = lambda proven, sloc: {'security_relevant': {'sloc': sloc, 'statuses': {'proven': proven}}}
    return {'target': 't', 'dependencies': [dict(name=n, version='1', sloc=s) for n, s in deps.items()],
            'size_fallbacks': [],
            'summary': {'first_party': block(*first), 'third_party': block(0, third if third is not None else sum(deps.values()))},
            'summary_default_rules': {'first_party': block(*default)}}


class SizeChecks(unittest.TestCase):
    def runs(self, whole, compiled, linked):
        return {'whole-crate': {'p': report(whole)}, 'compiled-files': {'p': report(compiled)},
                'linked-lines': {'p': report(linked)}}

    def test_ordered_sizes_pass(self):
        self.assertEqual(v15_size_check.order_exceptions(self.runs({'a': 10, 'b': 5}, {'a': 4, 'b': 5}, {'a': 0, 'b': 5})), [])

    def test_each_violation_is_reported(self):
        out = v15_size_check.order_exceptions(self.runs({'a': 10, 'b': 5}, {'a': 11, 'b': 5}, {'a': 0, 'b': 6}))
        self.assertEqual(out, ["p: a 1: compiled-files 11 > whole-crate 10", "p: b 1: linked-lines 6 > compiled-files 5"])

    def test_fallback_origin_is_labelled(self):
        runs = self.runs({'a': 10}, {'a': 4}, {'a': 10})
        runs['linked-lines']['p']['size_fallbacks'] = [dict(name='a', version='1', reason='r')]
        self.assertEqual(v15_size_check.order_exceptions(runs),
                         ['p: a 1: linked-lines 10 (whole-crate fallback) > compiled-files 4'])

    def test_dependency_set_mismatch_is_reported(self):
        out = v15_size_check.order_exceptions(self.runs({'a': 1}, {'a': 1, 'b': 1}, {'a': 1, 'b': 1}))
        self.assertEqual(len(out), 1)
        self.assertIn('list different dependencies', out[0])

    def test_reference_compares_first_party_and_whole_crate_totals(self):
        runs = {'whole-crate': {'p': report({'a': 10})}, 'compiled-files': {'p': report({'a': 4}, first=(1, 9))}}
        same = {'whole-crate': {'p': report({'a': 10})}, 'compiled-files': {'p': report({'a': 99}, first=(1, 9))}}
        self.assertEqual(v15_size_check.reference_exceptions(runs, same), [])
        moved = {'whole-crate': {'p': report({'a': 11})}, 'compiled-files': {'p': report({'a': 4})}}
        out = v15_size_check.reference_exceptions(runs, moved)
        self.assertEqual(len(out), 3, out)

    def test_cli_exit_code(self):
        with tempfile.TemporaryDirectory() as tmp:
            for mode, deps in [('whole-crate', {'a': 10}), ('compiled-files', {'a': 12})]:
                path = Path(tmp, mode, 'p', 'report.json')
                path.parent.mkdir(parents=True)
                path.write_text(json.dumps(report(deps)))
            self.assertEqual(v15_size_check.main(['--run', f'whole-crate={tmp}/whole-crate']), 0)
            self.assertEqual(v15_size_check.main(['--run', f'whole-crate={tmp}/whole-crate',
                                                  '--run', f'compiled-files={tmp}/compiled-files']), 1)


if __name__ == '__main__':
    unittest.main()
