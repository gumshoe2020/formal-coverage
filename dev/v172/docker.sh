#!/usr/bin/env bash
# usage: dev/v172/docker.sh SCRIPT [ARGS...]   (repository root; SCRIPT is a scripts/hax/v171-*.sh step script)
# Same mount rules as scripts/hax/v171-docker.sh: sources and the v1.7 toolchain read-only, only .tmp/v1.7.2 writable, no network.
set -eu
script="$1"; shift
root="$PWD"
sources="$root/.tmp/v1.7.2-sources" toolchain="$root/runs/v1.7/bertie" out="$root/.tmp/v1.7.2"
mkdir -p "$out/container"
cp scripts/hax/v171-env.sh "$script" "$out/container/"
docker run --rm --cpus 6 --memory 5g --network none \
  --mount type=bind,src="$sources",dst="$sources",readonly \
  --mount type=bind,src="$toolchain",dst="$toolchain",readonly \
  --mount type=bind,src="$out",dst="$out" \
  -e V171_OUT="$out" -e V171_TOOLCHAIN="$toolchain" -e V171_SOURCES="$sources" \
  "${V171_IMAGE:-fc-verify-hax:v17}" bash -c 'rm -rf /out && ln -s "$V171_TOOLCHAIN" /out && source "$V171_OUT/container/v171-env.sh" && bash "$V171_OUT/container/$(basename "$0")" "$@"' "$script" "$@"
