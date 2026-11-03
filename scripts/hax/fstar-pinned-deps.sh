set -eux
export HOME=/out/home CARGO_HOME=/out/cargo-home RUSTUP_HOME=/out/rustup OPAMJOBS=3
export PATH="$CARGO_HOME/bin:$PATH"
eval "$(opam env --switch=fstar)"
opam pin add sedlex 3.3 --no-action -y
opam pin add ppxlib 0.35.0 --no-action -y
cd /out/fstar-src/fstar
opam install --deps-only -y .
make minimal -j3
out/bin/fstar.exe --version
cd /out
git clone --branch z3-4.13.3 --depth 1 https://github.com/Z3Prover/z3 z3-src
git -C z3-src rev-parse HEAD > z3.commit
cd z3-src
python3 scripts/mk_make.py --prefix=/out/z3
make -C build -j3
make -C build install
/out/z3/bin/z3 --version
opam switch export /out/opam-fstar-switch.export
