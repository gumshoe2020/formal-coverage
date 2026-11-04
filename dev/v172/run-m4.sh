#!/usr/bin/env bash
# usage: dev/v172/run-m4.sh   (repository root; run dev/v172/run-m3.sh first: it builds the tools and copies upstream/shipped)
# v1.7.2 M4: report-only rerun on the bertie-der M5 `fixed` tree (fix/ecdsa-der f694480) with the new and the v1.7.1 binary,
# then the derived figures (scripts/hax/v171_numbers.py) for upstream (ef65f76) and fixed under the new tool.
set -eu
if pgrep -f dudley-rerun-m5 >/dev/null; then echo "dudley-rerun-m5 is running; not starting a container" >&2; exit 3; fi
out="$PWD/.tmp/v1.7.2"; tree=fixed
[ -x "$out/tool-target/release/formal-coverage" ] || { echo "run dev/v172/run-m3.sh first" >&2; exit 2; }
rm -rf "$out/$tree"; mkdir -p "$out/$tree" logs/v1.7.2
for item in root check-target fresh libs proofs typecheck-extraction-2025.10.06.log typecheck-extraction-2025.10.06.log.evidence.json; do
  cp -a "runs/v1.7.1/$tree/$item" "$out/$tree/$item"
done
bash dev/v172/docker.sh dev/v172/report.sh $tree 2025.10.06 "$out/tool-target/release/formal-coverage" "" > logs/v1.7.2/m4-report-$tree.log 2>&1
bash dev/v172/docker.sh dev/v172/report.sh $tree 2025.10.06 "$out/tool-old/release/formal-coverage" "-control" > logs/v1.7.2/m4-control-$tree.log 2>&1
echo "$tree exits: new $(cat "$out/$tree/report.exit") control $(cat "$out/$tree/report-control.exit")"
python3 dev/v172/compare.py runs/v1.7.1/$tree/report.json "$out/$tree/report-control.json" | tail -n 1 | sed "s/^/$tree control vs v1.7.1 original: /"
python3 dev/v172/compare.py "$out/$tree/report-control.json" "$out/$tree/report.json" > logs/v1.7.2/m4-diff-$tree.txt
tail -n 1 logs/v1.7.2/m4-diff-$tree.txt | sed "s/^/$tree new vs control: /"
for t in upstream fixed; do python3 scripts/hax/v171_numbers.py "$out/$t" > logs/v1.7.2/m4-numbers-$t.log 2>&1; echo "numbers $t exit $?"; done
