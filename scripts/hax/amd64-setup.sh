set -eux
apt-get update
apt-get install -y --no-install-recommends libgmp10 libgomp1 make python3 patch
/out/fstar-bin/bin/fstar.exe --version
find /out/fstar-bin -name '*z3*' -type f
