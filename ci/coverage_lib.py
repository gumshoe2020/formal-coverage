# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
"""Versioned, first-party security ratchet. hax credit stays explicitly labelled."""
from collections import Counter, defaultdict
from fractions import Fraction
import re

from coverage_metrics import PROVEN, baseline, functional_lines, snapshot


def share(data):
    return Fraction(data['proven'], data['total'])


def touched(diff, inventories):
    """Conservative U0 diff mapping against both inventories, including deletions.

    A whole security file is charged if diff syntax cannot bind its changed lines.
    Non-Rust manifests are covered by policy independently of this line classifier.
    """
    paths = {}
    for inv in inventories:
        for unit in inv:
            paths.setdefault(unit['file'], []).append((unit['line'], unit['end_line']))
    old = new = None
    changed = False
    for line in diff.splitlines():
        if line.startswith('diff --git '):
            old = new = None
            changed = False
        elif line.startswith('--- '):
            old = line[4:].removeprefix('a/')
        elif line.startswith('+++ '):
            new = line[4:].removeprefix('b/')
        elif line.startswith('@@ '):
            match = re.match(r'@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@', line)
            if not match:
                if old in paths or new in paths:
                    return True
                continue
            a, ac, b, bc = match.groups()
            for path, start, count in [(old, int(a), int(ac or '1')), (new, int(b), int(bc or '1'))]:
                # Zero-width additions/deletions charge the adjacent source boundary.
                end = start + max(count, 1) - 1
                if any(start <= hi and end >= lo for lo, hi in paths.get(path, [])):
                    return True
            changed = True
        elif (line.startswith('Binary files ') or line.startswith('GIT binary patch')) and (old in paths or new in paths):
            return True
    # Renames/moves of classified source receive no coverage credit automatically.
    for match in re.finditer(r'^rename (?:from|to) (.+)$', diff, re.M):
        if match.group(1) in paths:
            return True
    return False


def exception(markdown, commits):
    """An exact commit in the exceptions table, with a named approver and reason.

    Approval authenticity belongs to human review; text cannot attest identity.
    """
    if '## Exceptions' not in markdown:
        return False
    table = markdown.split('## Exceptions', 1)[1].split('\n## ', 1)[0]
    for line in table.splitlines():
        cells = [c.strip().strip('`') for c in line.strip().strip('|').split('|')]
        if len(cells) != 4:
            continue
        commit, approver, reason, restore = cells
        if commit in commits and re.fullmatch(r'[0-9a-f]{40}', commit) and approver and reason and restore:
            return True
    return False


class RatchetFailure(ValueError):
    """A valid measurement violates the coverage policy."""


def validate_floor(floor):
    # Every value participating in rational comparisons must be a nonnegative integer.
    for key in ('total', 'proven', 'tested', 'unverified'):
        if type(floor[key]) is not int or floor[key] < 0:
            raise ValueError('invalid merge-base baseline')
    if floor['total'] <= 0 or sum(floor[k] for k in ('proven', 'tested', 'unverified')) != floor['total']:
        raise ValueError('invalid merge-base totals')
    return functional_lines(floor)


def changed_spans(diff):
    """Old and new intervals of zero-context hunks, keyed by unquoted path."""
    removed, added = defaultdict(list), defaultdict(list)
    for patch in re.split(r'^diff --git ', diff, flags=re.M)[1:]:
        header, *hunks = re.split(r'^@@ ', patch, flags=re.M)
        old = re.search(r'^--- a/([^\t\n]+)\t?$', header, re.M)
        new = re.search(r'^\+\+\+ b/([^\t\n]+)\t?$', header, re.M)
        for hunk in hunks:
            m = re.match(r'-(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@', hunk)
            if m:
                a, ac, b, bc = (int(v or 1) for v in m.groups())
                if old and ac:
                    removed[old[1]].append((a, a + ac - 1, bc))
                if new and bc:
                    added[new[1]].append((b, b + bc - 1))
    return removed, added


def deletion_obstacle(floor, measured, report, diff):
    if floor['schema'] != 2:
        return 'merge-base floor lacks per-unit statuses'
    if measured['schema'] != 2:
        return 'measurement lacks per-unit statuses'
    for status in ('unverified', 'tested'):
        if measured[status] > floor[status]:
            return f'{status} security lines increased'

    def key(unit):
        return unit['file'], unit['id']

    for data in (floor, measured):
        counts = Counter()
        for u in data['inventory']:
            if type(u['sloc']) is not int or u['sloc'] < 0:
                raise ValueError('invalid inventory SLoC')
            counts['proven' if u['status'] in PROVEN else
                   'tested' if u['status'] == 'tested' else 'unverified'] += u['sloc']
        if any(counts[k] != data[k] for k in ('proven', 'tested', 'unverified')):
            raise ValueError('inventory does not explain security totals')

    old = Counter(map(key, floor['inventory']))
    current = defaultdict(list)
    for u in report['units']:
        current[key(u)].append(u)
    if re.search(r'^(?:---|\+\+\+) "', diff, flags=re.M):
        return 'unmatched quoted path in source diff'
    removed, added = changed_spans(diff)
    missing = False
    for u in floor['inventory']:
        if u['status'] not in PROVEN:
            continue
        k = key(u)
        candidates = current.get(k, [])
        if old[k] != 1 or len(candidates) > 1:
            return f'ambiguous unit match for {k}'
        hunks = removed[u['file']]
        if not candidates:
            if not any(lo <= u['line'] and u['end_line'] <= hi and replacements == 0
                       for lo, hi, replacements in hunks):
                return f'unmatched proven unit {k} without a complete source deletion'
            missing = True
            continue
        v = candidates[0]
        if not (v['first_party'] and v['security'] and v['category'] == 'runtime'):
            return f'proven unit {k} left security runtime scope'
        if v['status'] not in PROVEN:
            return f'proven unit {k} demoted to {v["status"]}'
        loss = u['sloc'] - v['sloc']
        if loss > 0:
            deleted = sum(max(0, min(hi, u['end_line']) - max(lo, u['line']) + 1)
                          for lo, hi, _ in hunks)
            if deleted < loss:
                return f'shorter proven unit {k} lacks sufficient source deletion evidence'

    # A removed function plus a newly named/moved function is ambiguous.
    for k, units in current.items():
        if k in old:
            continue
        for v in units:
            if v.get('test_evidence') in ('builtin-attribute', 'cfg-test', 'test-target'):
                continue
            proven = (v['first_party'] and v['security'] and v['category'] == 'runtime'
                      and v['status'] in PROVEN)
            if (missing or not proven) and any(lo <= v['end_line'] and hi >= v['line']
                                               for lo, hi in added.get(v['file'], ())):
                return f'unmatched replacement unit {k}'
    reported_files = {u['file'] for u in report['units']}
    for path in added:
        if path.endswith('.rs') and path not in reported_files:
            return f'unmatched Rust source outside the measured inventory: {path}'
    return None


def check(floor, committed, report, diff, markdown='', commits=()):
    measured = baseline(report)
    if committed != measured:
        raise ValueError('committed baseline does not match measured report (stale or hand-edited)')
    functional_lines(committed)  # Python equality alone accepts True as 1.
    before = validate_floor(floor)
    after = functional_lines(measured)
    errors = []
    if measured['unverified'] > floor['unverified']:
        errors.append('unverified security lines increased')
    previous_share, current_share = share(floor), share(measured)
    deletion_only = False
    if current_share < previous_share:
        reason = deletion_obstacle(floor, measured, report, diff)
        if reason:
            errors.append(f'proven share decreased ({reason})')
        else:
            deletion_only = True
    if deletion_only or (current_share <= previous_share and touched(diff, [floor['inventory'], measured['inventory']])):
        if before is None or after is None:
            errors.append('security code changed without increasing proven share')
        elif (current_share < previous_share and not deletion_only) or after <= before:
            errors.append(f'security code changed without increasing proven share or deepening proofs '
                          f'at equal share (functional security lines: {before} -> {after})')
    if errors and not exception(markdown, commits):
        raise RatchetFailure('; '.join(errors))
    return errors
