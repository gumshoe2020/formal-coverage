#!/usr/bin/env python3
"""Derive the M4 figures for one tree from its report.json and the report root's sources.

usage: v171_numbers.py TREE_DIR   -> writes TREE_DIR/numbers.json and prints it
Out-of-proof-scope is a syntactic classification of first-party runtime security units, exclusive, in this order:
non_hax (module path contains non_hax), stream (src/stream/**), feature-gated (an enclosing item or the unit itself
has a #[cfg(...)] that is inactive under the proof configuration: features defensive,std and --cfg hax), else in scope.
Feature-gated units are split into "shipped-only" (active under the shipped features api,std,defensive without --cfg hax)
and "inactive in both builds".
"""
import collections
import json
import re
import sys
from pathlib import Path

PROOF = {"features": {"defensive", "std"}, "flags": {"hax"}}
SHIPPED = {"features": {"api", "std", "defensive"}, "flags": set()}
ATTRIBUTE = re.compile(r"^\s*#\[(.*)\]\s*$")
ITEM_HEADER = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:impl|mod|trait)\b")


def split_arguments(text):
    depth, current, parts = 0, "", []
    for ch in text:
        depth += ch == "("
        depth -= ch == ")"
        if ch == "," and depth == 0:
            parts.append(current.strip())
            current = ""
        else:
            current += ch
    if current.strip():
        parts.append(current.strip())
    return parts


def evaluate(expression, config):
    expression = expression.strip()
    call = re.fullmatch(r"(all|any|not)\((.*)\)", expression, re.DOTALL)
    if call:
        values = [evaluate(a, config) for a in split_arguments(call.group(2))]
        return {"all": all, "any": any}[call.group(1)](values) if call.group(1) != "not" else not values[0]
    feature = re.fullmatch(r'feature\s*=\s*"([^"]+)"', expression)
    if feature:
        return feature.group(1) in config["features"]
    return expression in config["flags"]


def attributes_above(lines, index):
    found = []
    while index > 0 and (ATTRIBUTE.match(lines[index - 1]) or lines[index - 1].strip().startswith("///")):
        match = ATTRIBUTE.match(lines[index - 1])
        if match:
            found.append(match.group(1))
        index -= 1
    return found


def cfg_expressions(attrs):
    return [m.group(1) for a in attrs for m in [re.fullmatch(r"cfg\((.*)\)", a.strip(), re.DOTALL)] if m]


def enclosing_headers(lines, line_index):
    """Indices of enclosing impl/mod/trait header lines (syntactic, by indentation)."""
    headers, indent = [], len(lines[line_index]) - len(lines[line_index].lstrip())
    for i in range(line_index - 1, -1, -1):
        text = lines[i]
        if not text.strip():
            continue
        here = len(text) - len(text.lstrip())
        if here < indent and ITEM_HEADER.match(text):
            headers.append(i)
            indent = here
        elif here < indent and not text.strip().startswith(("#", "//")):
            indent = here
    return headers


def unit_cfgs(lines, unit_line):
    index = unit_line - 1
    expressions = cfg_expressions(attributes_above(lines, index))
    for header in enclosing_headers(lines, index):
        expressions += cfg_expressions(attributes_above(lines, header))
    return expressions


def scope_reason(unit, lines):
    if "non_hax" in unit["id"].split("::"):
        return "non_hax"
    if unit["file"].startswith("src/stream/"):
        return "stream"
    expressions = unit_cfgs(lines, unit["line"])
    if unit["file"] == "src/test_utils.rs":
        expressions.append('feature = "test_utils"')
    if all(evaluate(e, PROOF) for e in expressions):
        return None
    return "feature-gated (shipped-only)" if all(evaluate(e, SHIPPED) for e in expressions) else "feature-gated (inactive in both builds)"


def origin(path):
    for prefix, name in (("proofs/fstar/extraction/", "bertie-extraction"), ("libs/", "hax-and-model-libraries")):
        if path.startswith(prefix):
            return name
    return "first-party-rust" if path.endswith(".rs") else "other"


def first_party_runtime(report):
    return [u for u in report["units"] if u["first_party"] and u["security"] and u["category"] == "runtime"]


def scope_table(report, root):
    cache, table, members = {}, collections.Counter(), collections.defaultdict(list)
    for unit in first_party_runtime(report):
        lines = cache.setdefault(unit["file"], (root / unit["file"]).read_text().split("\n"))
        reason = scope_reason(unit, lines) or "in proof scope"
        table[reason] += unit["sloc"]
        members[reason].append("%s:%d %s (%d SLoC, %s)" % (unit["file"], unit["line"], unit["id"], unit["sloc"], unit["status"]))
    return dict(table), dict(members)


def worked_example(report):
    names = ("certificate_verify", "ecdsa_signature", "parse_certificate_verify", "parse_ecdsa_signature")
    definitions = [
        {k: x[k] for k in ("name", "file", "line", "classification", "contract_kind")}
        for x in report["fstar_definitions"] if x["name"].rsplit(".", 1)[-1] in names
    ]
    units = [
        {k: u[k] for k in ("id", "file", "line", "sloc", "status")}
        for u in report["units"] if u["name"] in names and u["category"] == "runtime"
    ]
    return {"definitions": definitions, "units": units}


def figures(report, root):
    units = first_party_runtime(report)
    total = sum(u["sloc"] for u in units)
    by_status = collections.Counter()
    for u in units:
        by_status[u["status"]] += u["sloc"]
    extraction = [x for x in report["fstar_definitions"] if origin(x["file"]) == "bertie-extraction"]
    credited = [u for u in units if u["status"] == "proven-extracted"]
    kinds = collections.Counter(u["proof_scope"].split("contract ")[1].split(";")[0] for u in credited)
    kind_sloc = collections.Counter()
    for u in credited:
        kind_sloc[u["proof_scope"].split("contract ")[1].split(";")[0]] += u["sloc"]
    scope, members = scope_table(report, root)
    libcrux = sorted((d["name"], d["version"], d["sloc"]) for d in report["dependencies"] if d["name"].startswith("libcrux"))
    return {
        "first_party_security_sloc": total,
        "status_sloc": dict(by_status),
        "proven_extracted_sloc": by_status["proven-extracted"],
        "proven_extracted_percent": round(100 * by_status["proven-extracted"] / total, 2),
        "credited_units": len(credited),
        "credited_contract_kinds_units": dict(kinds),
        "credited_contract_kinds_sloc": dict(kind_sloc),
        "extraction_definitions": dict(collections.Counter(x["classification"] for x in extraction)),
        "extraction_contract_kinds_proven_extracted": dict(collections.Counter(x["contract_kind"] for x in extraction if x["classification"] == "proven-extracted")),
        "trusted_items_by_origin": dict(collections.Counter(origin(t["file"]) if not t["file"].startswith("/") else "hax-record" for t in report["trusted_items"])),
        "trusted_sloc_by_origin": trusted_sloc(report),
        "escapes_by_origin_and_kind": escapes(report),
        "out_of_proof_scope_sloc": scope,
        "out_of_proof_scope_units": members,
        "libcrux": {"packages": libcrux, "sloc": sum(x[2] for x in libcrux)},
        "worked_example": worked_example(report),
    }


def trusted_sloc(report):
    totals = collections.Counter()
    for t in report["trusted_items"]:
        totals[origin(t["file"]) if not t["file"].startswith("/") else "hax-record"] += t["sloc"]
    return dict(totals)


def escapes(report):
    table = collections.Counter()
    for e in report["escape_hatches"]:
        path = e["file"]
        key = origin(path)
        if path.startswith("/"):
            key = "hax-record" if path.endswith(".log") else ("first-party-rust" if e.get("first_party") else "third-party-rust")
        table["%s | %s" % (key, e["kind"])] += 1
    return dict(sorted(table.items()))


def main(argv):
    tree = Path(argv[1])
    report = json.loads((tree / "report.json").read_text())
    result = figures(report, tree / "root")
    (tree / "numbers.json").write_text(json.dumps(result, indent=2) + "\n")
    summary = {k: v for k, v in result.items() if k != "out_of_proof_scope_units"}
    print(json.dumps(summary, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
