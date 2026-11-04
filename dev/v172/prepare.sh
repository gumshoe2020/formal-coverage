#!/usr/bin/env bash
# usage: dev/v172/prepare.sh   (repository root)
# Exports HEAD's tool sources and copies the report inputs of the two v1.7.1 trees into .tmp/v1.7.2 (ignored).
# cargo-home is the v1.7.1 one (it holds the registry sources of the bertie-der dev-dependencies). Nothing under runs/v1.7.1 is modified; ws/, extract/ and the F* typecheck caches are not copied.
set -eu
out=.tmp/v1.7.2; src=.tmp/v1.7.2-sources/tool
rm -rf "$out" .tmp/v1.7.2-sources; mkdir -p "$out" "$src"
git archive HEAD | tar -x -C "$src"
cp -a runs/v1.7.1/cargo-home "$out/cargo-home"
git rev-parse HEAD > "$out/tool-source.commit"
git rev-parse HEAD:src > "$out/tool-source.src-tree"
for tree in upstream shipped; do
  mkdir -p "$out/$tree"
  for item in root check-target fresh libs proofs typecheck-extraction-2025.10.06.log typecheck-extraction-2025.10.06.log.evidence.json; do
    cp -a "runs/v1.7.1/$tree/$item" "$out/$tree/$item"
  done
done
