# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
"""Versioned, first-party security ratchet. hax credit stays explicitly labelled."""
from fractions import Fraction
import re

from coverage_metrics import baseline, functional_lines, snapshot


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
    if share(measured) < share(floor):
        errors.append('proven share decreased')
    if touched(diff, [floor['inventory'], measured['inventory']]) and share(measured) <= share(floor):
        if before is None or after is None:
            errors.append('security code changed without increasing proven share')
        elif share(measured) < share(floor) or after <= before:
            errors.append(f'security code changed without increasing proven share or deepening proofs '
                          f'at equal share (functional security lines: {before} -> {after})')
    if errors and not exception(markdown, commits):
        raise RatchetFailure('; '.join(errors))
    return errors
