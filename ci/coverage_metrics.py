# SPDX-License-Identifier: Apache-2.0
# Adapted from gumshoe2020/bertie-tls scripts at 26b54ae; see THIRD_PARTY_NOTICES.md.
"""Stable metric projection of a formal-coverage v1.8.1 schema-4 report."""
PROVEN = ('proven', 'proven-extracted')


def snapshot(report):
    if report['schema'] != 4:
        raise ValueError(f"unsupported report schema {report['schema']}: expected schema 4 (formal-coverage v1.8.1)")
    if not report['dependency_inventory_complete']:
        raise ValueError('incomplete inventory')
    if report['unparseable_files']:
        raise ValueError('unparseable files: security classification is incomplete')
    totals = report['summary']['first_party']['security_relevant']
    statuses = totals['statuses']
    total = totals['sloc']
    if total <= 0 or any(type(v) is not int or v < 0 for v in statuses.values()) or sum(statuses.values()) != total:
        raise ValueError('invalid security totals')
    proven = sum(statuses.get(k, 0) for k in PROVEN)
    tested = statuses.get('tested', 0)
    units = [u for u in report['units'] if u['first_party'] and u['security']
             and u['category'] == 'runtime']
    fields = ('file', 'line', 'end_line')
    evidence = ('id', 'sloc', 'status', 'inline_proof_sloc')
    complete = all(all(k in u for k in evidence) for u in units)
    if complete:
        fields += evidence
    inventory = sorted(({k: u[k] for k in fields} for u in units),
                       key=lambda u: tuple(u[k] for k in fields))
    return dict(schema=2 if complete else 1, scope='first-party security runtime', total=total,
                proven=proven, tested=tested, unverified=total-proven-tested, statuses=statuses,
                proof_sloc=report['proof_ratio']['proof_sloc'],
                proof_ratio=report['proof_ratio']['ratio'], inventory=inventory)


def proof_depth(report):
    """Proven security runtime lines by contract depth; None for reports older than v1.11.0.

    Kept out of snapshot(): recorded baselines are exact snapshots and must stay comparable.
    """
    depth = report['summary'].get('proof_depth')
    if depth is None:
        return None
    ratio = report['proof_ratio']
    proven = ratio['proven_runtime_sloc'] + ratio['proven_extracted_runtime_sloc']
    parts = (depth['safety_only_sloc'], depth['functional_own_code_only_sloc'], depth['functional_sloc'])
    if any(type(v) is not int or v < 0 for v in parts) or sum(parts) != proven:
        raise ValueError('proof depth does not add up to proven runtime SLoC')
    return dict(safety_only=parts[0], functional_own_code_only=parts[1], functional=parts[2])


def baseline(report):
    data = snapshot(report)
    depth = proof_depth(report)
    if depth is not None:
        data['proof_depth'] = depth
        functional_lines(data)
    return data


def functional_lines(data):
    if 'proof_depth' not in data:
        return None
    depth = data['proof_depth']
    if (not isinstance(depth, dict) or set(depth) != {'safety_only', 'functional_own_code_only', 'functional'}
            or any(type(v) is not int or v < 0 for v in depth.values())
            or sum(depth.values()) != data['proven']):
        raise ValueError('baseline proof depth does not add up to proven runtime SLoC')
    return depth['functional_own_code_only'] + depth['functional']
