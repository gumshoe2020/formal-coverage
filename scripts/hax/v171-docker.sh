#!/usr/bin/env bash
# usage: v171-docker.sh STEP [ARGS...]   (run from the repository root)
# Runs scripts/hax/v171-STEP.sh in the v1.7 hax image under the v1.7 mount rules:
# sources read-only, the v1.7 toolchain read-only (as /out), runs/v1.7.1 writable.
# Not mounted: the repository, host home, credentials, Docker socket.
set -eu
step="$1"; shift
root="$PWD"
sources="$root/.tmp/v1.7.1-sources" toolchain="$root/runs/v1.7/bertie" out="$root/runs/v1.7.1"
mkdir -p "$out/container"
cp scripts/hax/v171-env.sh scripts/hax/v171-"$step".sh "$out/container/"
platform=()
if [ "${V171_PLATFORM:-}" != "" ]; then platform=(--platform "$V171_PLATFORM"); fi
image="${V171_IMAGE:-fc-verify-hax:v17}"
network=(--network none)
if [ "$step" = fetch ]; then network=(); fi
docker run --rm --cpus 6 --memory 5g ${network[@]+"${network[@]}"} ${platform[@]+"${platform[@]}"} \
  --mount type=bind,src="$sources",dst="$sources",readonly \
  --mount type=bind,src="$toolchain",dst="$toolchain",readonly \
  --mount type=bind,src="$out",dst="$out" \
  -e V171_OUT="$out" -e V171_TOOLCHAIN="$toolchain" -e V171_SOURCES="$sources" \
  "$image" bash -c 'rm -rf /out && ln -s "$V171_TOOLCHAIN" /out && source "$V171_OUT/container/v171-env.sh" && bash "$V171_OUT/container/v171-$0.sh" "$@"' "$step" "$@"
