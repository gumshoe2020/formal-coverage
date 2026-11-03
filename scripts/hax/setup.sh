set -eux
export HOME=/out/home CARGO_HOME=/out/cargo-home RUSTUP_HOME=/out/rustup
export PATH="$CARGO_HOME/bin:$PATH"
apt-get update
apt-get install -y --no-install-recommends opam ocaml build-essential pkg-config libgmp-dev libffi-dev bubblewrap unzip jq python3 nodejs rsync z3
cd /out/hax
git rev-parse HEAD > /out/hax.commit
cat rust-toolchain.toml
curl -fLsS https://api.github.com/repos/FStarLang/FStar/releases/tags/v2025.03.25 > /out/fstar-release.json
jq -r '.assets[].browser_download_url' /out/fstar-release.json
opam init --disable-sandboxing --bare -y
opam switch create hax 5.4.1 -y
eval "$(opam env --switch=hax)"
./setup.sh
