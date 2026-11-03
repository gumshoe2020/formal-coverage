# Sourced inside the container. Toolchain read-only at /out; all writes under $V171_OUT.
export RUSTUP_TOOLCHAIN=nightly-2025-11-08
export RUSTUP_HOME="$V171_TOOLCHAIN/rustup"
export HOME="$V171_OUT/home" CARGO_HOME="$V171_OUT/cargo-home"
export OPAMROOT="$V171_TOOLCHAIN/home/.opam" OPAMNOENVNOTICE=1
mkdir -p "$HOME"
if [ ! -d "$CARGO_HOME/bin" ]; then cp -a "$V171_TOOLCHAIN/cargo-home" "$CARGO_HOME"; fi
export PATH="$CARGO_HOME/bin:$PATH"
if command -v opam >/dev/null 2>&1; then eval "$(opam env --switch=hax --readonly)"; fi
