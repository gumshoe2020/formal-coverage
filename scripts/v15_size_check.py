#!/usr/bin/env python3
"""v1.5 size-mode sanity checks over replay directories written by scripts/v15_replay.py.

Each --run MODE=DIR names one size mode's output directory (DIR/CASE/report.json). Checks, per
case and dependency (name, version):
  - compiled-files <= whole-crate, and linked-lines <= compiled-files (the linked-lines unit is
    linked lines, not SLoC; the bound is the plan's expectation, not an identity);
  - all modes list the same runtime dependencies.
With --reference MODE=DIR (an earlier run of the same cases, e.g. the macOS v1.5 replay):
  - whole-crate third-party and first-party summaries are unchanged;
  - first-party summaries are unchanged in every mode given in both.
Prints a summary line per case and mode, then every exception (labelled when the narrower mode
fell back to whole-crate); exits 1 if there is any.
"""
import argparse
import json
from pathlib import Path

MODES = ['whole-crate', 'compiled-files', 'linked-lines']


def load(directory):
    return {p.parent.name: json.loads(p.read_text()) for p in sorted(Path(directory).glob('*/report.json'))}


def sizes(report):
    return {(d['name'], d['version']): d['sloc'] for d in report['dependencies']}


def fallen_back(report):
    return {(f['name'], f['version']) for f in report.get('size_fallbacks', [])}


def security(block):
    return block['security_relevant']['statuses']['proven'], block['security_relevant']['sloc']


def first_party(report):
    """(proven, SLoC) with inputs and under default rules only."""
    default = report.get('summary_default_rules', report['summary'])
    return security(report['summary']['first_party']), security(default['first_party'])


def third_party(report):
    return report['summary']['third_party']['security_relevant']['sloc']


def order_exceptions(runs):
    """Per-dependency ordering violations between consecutive modes, for every case."""
    out = []
    modes = [m for m in MODES if m in runs]
    cases = sorted(set().union(*(runs[m] for m in modes)))
    for case in cases:
        present = [m for m in modes if case in runs[m]]
        for wider, narrower in zip(present, present[1:]):
            big, small = sizes(runs[wider][case]), sizes(runs[narrower][case])
            fallback = fallen_back(runs[narrower][case])
            if big.keys() != small.keys():
                out.append(f'{case}: {wider} and {narrower} list different dependencies: '
                           f'only {wider} {sorted(big.keys() - small.keys())}, '
                           f'only {narrower} {sorted(small.keys() - big.keys())}')
            for key in sorted(big.keys() & small.keys()):
                if small[key] > big[key]:
                    why = ' (whole-crate fallback)' if key in fallback else ''
                    out.append(f'{case}: {key[0]} {key[1]}: {narrower} {small[key]}{why} > {wider} {big[key]}')
    return out


def reference_exceptions(runs, reference):
    out = []
    for mode in [m for m in MODES if m in runs and m in reference]:
        for case in sorted(runs[mode].keys() & reference[mode].keys()):
            now, then = runs[mode][case], reference[mode][case]
            if first_party(now) != first_party(then):
                out.append(f'{case}: {mode} first-party changed: {first_party(then)} -> {first_party(now)}')
            if mode == 'whole-crate' and third_party(now) != third_party(then):
                out.append(f'{case}: whole-crate third-party changed: {third_party(then)} -> {third_party(now)}')
            if mode == 'whole-crate' and sizes(now) != sizes(then):
                out.append(f'{case}: whole-crate dependency sizes changed')
    return out


def summary(mode, case, report):
    (proven, sloc), (d_proven, d_sloc) = first_party(report)
    fallbacks = len(report.get('size_fallbacks', []))
    return (f'{case} {mode}: first-party {proven}/{sloc} (default rules {d_proven}/{d_sloc}); '
            f'third-party {third_party(report)}; deps {len(report["dependencies"])}; '
            f'fallbacks {fallbacks}; target {report["target"]}')


def pairs(values):
    out = {}
    for value in values:
        mode, _, directory = value.partition('=')
        if mode not in MODES or not directory:
            raise SystemExit(f'expected MODE=DIR with MODE in {MODES}: {value!r}')
        out[mode] = load(directory)
    return out


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument('--run', action='append', default=[], required=True)
    parser.add_argument('--reference', action='append', default=[])
    args = parser.parse_args(argv)
    runs, reference = pairs(args.run), pairs(args.reference)
    for mode in [m for m in MODES if m in runs]:
        for case, report in runs[mode].items():
            print(summary(mode, case, report))
    exceptions = order_exceptions(runs) + reference_exceptions(runs, reference)
    print(f'exceptions: {len(exceptions)}')
    for line in exceptions:
        print(f'  {line}')
    return 1 if exceptions else 0


if __name__ == '__main__':
    raise SystemExit(main())
