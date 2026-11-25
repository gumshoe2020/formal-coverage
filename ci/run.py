#!/usr/bin/env python3
"""Measure, check committed outputs, ratchet against git, and stage badges.

Runs locally or inside the composite action. All subprocess arguments are arrays;
report-args is JSON, never shell code. Prover receipts are caller-owned inputs.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

from config import load
from coverage_lib import check
from coverage_metrics import snapshot
from coverage_section import END, START, measured_section

HERE = Path(__file__).resolve().parent


def git(workspace, *args):
    return subprocess.check_output(['git', '-C', str(workspace), *args], text=True).strip()


def floor_and_diff(workspace, baseline, base):
    if not base:
        raise ValueError('no base commit: install and review an initial baseline before enabling the ratchet')
    base = git(workspace, 'rev-parse', '--verify', base + '^{commit}')
    floor = json.loads(git(workspace, 'show', f'{base}:{baseline}'))
    diff = git(workspace, 'diff', '--no-ext-diff', '--no-textconv', '-U0', base, 'HEAD')
    commits = git(workspace, 'rev-list', f'{base}..HEAD').splitlines()
    return floor, diff, commits


def show(workspace, spec):
    result = subprocess.run(['git', '-C', str(workspace), 'show', spec], capture_output=True, text=True)
    return result.stdout if result.returncode == 0 else None


def data_refs(branch):
    return [f'refs/heads/{branch}', f'refs/remotes/origin/{branch}']


def recorded_floor(workspace, base, config):
    """Floor from the data branch's baselines/<full-sha>.json; fail closed otherwise."""
    branch = config['data_branch']
    text = next((t for ref in data_refs(branch) if (t := show(workspace, f'{ref}:baselines/{base}.json'))), None)
    if text is None:
        text = bootstrap_floor(workspace, base, config)
    floor = json.loads(text)
    if not isinstance(floor, dict) or set(floor) != set(snapshot_keys()):
        raise ValueError(f'unreadable floor for {base}: not a schema-1 baseline')
    return floor


def snapshot_keys():
    return ('schema', 'scope', 'total', 'proven', 'tested', 'unverified', 'statuses',
            'proof_sloc', 'proof_ratio', 'inventory')


def has_recorded_baselines(workspace, branch):
    for ref in data_refs(branch):
        listing = show(workspace, f'{ref}:baselines')
        if listing and any(line.endswith('.json') for line in listing.splitlines()):
            return True
    return False


def bootstrap_floor(workspace, base, config):
    branch, path = config['data_branch'], config['bootstrap_baseline']
    missing = (f'no recorded baseline for merge-base {base} on branch {branch} '
               f'(baselines/{base}.json): that main commit has not been measured, or predates adoption')
    if not path:
        raise ValueError(missing)
    if has_recorded_baselines(workspace, branch):
        raise ValueError(missing + '; bootstrap_baseline applies only before the first recording')
    text = show(workspace, f'{base}:{path}')
    if text is None:
        raise ValueError(missing + f'; bootstrap_baseline {path} is not in that commit')
    return text


def report_arguments(encoded):
    args = json.loads(encoded)
    if not isinstance(args, list) or any(not isinstance(v, str) for v in args):
        raise ValueError('report-args must be a JSON array of strings')
    # Outputs and subcommand are controlled by the runner. Evidence/config/target
    # flags pass straight through to the pinned tool's own CLI parser.
    if any(v.split('=')[0] in ('--json', '--markdown') for v in args):
        raise ValueError('report output paths are controlled by the kit')
    return args


def measure(tool, workspace, extra, output):
    command = [str(tool), 'report', '.', *extra,
               '--json', str(output / 'report.json'), '--markdown', str(output / 'report.md')]
    subprocess.run(command, cwd=workspace, check=True)
    # `report` does not gate the ratio. Never catch/retry arbitrary tool failures.


def run(args):
    workspace = args.workspace.resolve()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=True)
    config_path = (workspace / args.config).resolve() if args.config else None
    config = load(config_path)
    extra = report_arguments(args.report_args)
    measure(args.tool.resolve(), workspace, extra, output)
    report_path = output / 'report.json'
    report = json.loads(report_path.read_text())
    if config['floor'] == 'recorded':
        errors, baseline_path, measured = ratchet_recorded(args, config, workspace, output, report)
    else:
        errors, baseline_path, measured = ratchet_committed(args, workspace, output, report_path, report)
    conclude(config, config_path, output, report, errors, baseline_path, measured)


def ratchet_committed(args, workspace, output, report_path, report):
    subprocess.run([sys.executable, str(HERE / 'write-coverage.py'), str(report_path),
                    '--baseline', args.baseline, '--coverage', args.coverage, '--check'],
                   cwd=workspace, check=True)
    floor, diff, commits = floor_and_diff(workspace, args.baseline, args.base)
    page = (workspace / args.coverage).read_text()
    errors = check(floor, json.loads((workspace / args.baseline).read_text()), report, diff, page, commits)
    (output / 'floor.json').write_text(json.dumps(floor, indent=2) + '\n')
    (output / 'diff.patch').write_text(diff + '\n')
    measured = page.split(START, 1)[1].split(END, 1)[0]
    return errors, workspace / args.baseline, measured


def ratchet_recorded(args, config, workspace, output, report):
    """Floor = main's recorded measurement of the merge base; nothing numeric is committed."""
    snap = snapshot(report)
    record = output / 'record'
    record.mkdir(exist_ok=True)
    section = measured_section(snap, report)
    (record / 'baseline.json').write_text(json.dumps(snap, indent=2, sort_keys=True) + '\n')
    (record / 'COVERAGE-MEASURED.md').write_text(section + '\n')
    (record / 'sha').write_text(git(workspace, 'rev-parse', 'HEAD') + '\n')
    (record / 'branch').write_text(config['data_branch'] + '\n')
    if not args.base:
        raise ValueError('no base commit: cannot find the recorded floor')
    base = git(workspace, 'rev-parse', '--verify', args.base + '^{commit}')
    floor = recorded_floor(workspace, base, config)
    diff = git(workspace, 'diff', '--no-ext-diff', '--no-textconv', '-U0', base, 'HEAD')
    commits = git(workspace, 'rev-list', f'{base}..HEAD').splitlines()
    coverage = workspace / args.coverage
    page = coverage.read_text() if coverage.exists() else ''
    errors = check(floor, snap, report, diff, page, commits)
    (output / 'floor.json').write_text(json.dumps(floor, indent=2) + '\n')
    (output / 'diff.patch').write_text(diff + '\n')
    return errors, record / 'baseline.json', section.removeprefix(START).removesuffix(END)


def conclude(config, config_path, output, report, errors, baseline_path, measured):
    message = 'ratchet PASS' + (' (maintainer exception: ' + '; '.join(errors) + ')' if errors else '')
    bounds = config['proof_ratio'][config['proof_ratio']['stage']]
    ratio = report['proof_ratio']['ratio']
    warnings = []
    if ratio is not None and not bounds[0] <= ratio <= bounds[1]:
        warnings.append(f'Proof/code ratio {ratio:.2f} outside {bounds}: informational only. ' + config['proof_ratio']['reason'])
    if config['proof_ratio']['stage'] == 'established':
        for module in report['proof_ratio']['modules']:
            if module['ratio'] is not None and module['ratio'] > config['proof_ratio']['established'][1]:
                warnings.append(f"Review proof cost: {module['crate_name']}::{module['module']} ratio {module['ratio']:.2f}")
    summary = message + '\n\n' + measured
    summary += '\n' + '\n'.join(warnings) + '\n'
    (output / 'summary.md').write_text(summary)
    if os.environ.get('GITHUB_STEP_SUMMARY'):
        with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as stream:
            stream.write(summary)
    for warning in warnings:
        print('::warning::' + warning)
    badge_command = [sys.executable, str(HERE / 'write-badges.py'),
                     str(baseline_path), str(output / 'badges'), 'pass']
    if config_path:
        badge_command += ['--config', str(config_path)]
    subprocess.run(badge_command, check=True)
    print(message)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--tool', type=Path, required=True)
    parser.add_argument('--workspace', type=Path, default=Path('.'))
    parser.add_argument('--report-args', default='[]')
    parser.add_argument('--baseline', default='coverage/baseline.json')
    parser.add_argument('--coverage', default='COVERAGE.md')
    parser.add_argument('--config', default='')
    parser.add_argument('--base', required=True, help='merge-base SHA (PR) or before SHA (push)')
    parser.add_argument('--output', type=Path, default=Path('target/coverage-ci'))
    args = parser.parse_args()
    try:
        run(args)
    except (ValueError, KeyError, TypeError, OSError, subprocess.CalledProcessError) as exc:
        parser.exit(1, f'coverage FAIL: {exc}\n')


if __name__ == '__main__':
    main()
