#!/usr/bin/env bash
# usage: dev/v172/run-m3.sh   (repository root; needs Docker and no running `dudley-rerun-m5`)
# v1.7.2 M3: rebuild the tool from HEAD, rerun the report step only on both v1.7.1 trees with the new
# binary and (control) the v1.7.1 binary, then diff. Output: .tmp/v1.7.2, diffs in logs/v1.7.2/m3-diff-*.txt.
set -eu
if pgrep -f dudley-rerun-m5 >/dev/null; then echo "dudley-rerun-m5 is running; not starting a container" >&2; exit 3; fi
mkdir -p logs/v1.7.2
bash dev/v172/prepare.sh
out="$PWD/.tmp/v1.7.2"
mkdir -p "$out/tool-old/release" && cp runs/v1.7.1/tool-target/release/formal-coverage "$out/tool-old/release/"
bash dev/v172/docker.sh scripts/hax/v171-toolbuild.sh > logs/v1.7.2/m3-toolbuild.log 2>&1
for tree in upstream shipped; do
  bash dev/v172/docker.sh dev/v172/report.sh $tree 2025.10.06 "$out/tool-target/release/formal-coverage" "" > logs/v1.7.2/m3-report-$tree.log 2>&1
  bash dev/v172/docker.sh dev/v172/report.sh $tree 2025.10.06 "$out/tool-old/release/formal-coverage" "-control" > logs/v1.7.2/m3-control-$tree.log 2>&1
  echo "$tree exits: new $(cat "$out/$tree/report.exit") control $(cat "$out/$tree/report-control.exit")"
  python3 dev/v172/compare.py runs/v1.7.1/$tree/report.json "$out/$tree/report-control.json" | tail -n 1 | sed "s/^/$tree control vs v1.7.1 original: /"
  python3 dev/v172/compare.py "$out/$tree/report-control.json" "$out/$tree/report.json" > logs/v1.7.2/m3-diff-$tree.txt
  tail -n 1 logs/v1.7.2/m3-diff-$tree.txt | sed "s/^/$tree new vs control: /"
done
