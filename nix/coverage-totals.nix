# A line-total receipt built in the sandbox: the package's own verification script with
# the pinned hax/F*/Z3, vendored Cargo dependencies, no network, then `report --totals-json`.
# The derivation hash covers source, tools, kit and features, so an unchanged input reuses
# the receipt; it is never substituted from a binary cache.
{ pkgs, formal-coverage, hax, haxPackages, z3 }:
{ src
, url                       # git URL the source comes from
, rev ? src.rev             # full commit of `src`
, package                   # the [ship] package the receipt describes
, features                  # the consumer's resolved feature set
, target ? pkgs.stdenv.hostPlatform.rust.rustcTarget
, verify                    # shell commands run in the source tree (FSTAR_JOBS is set)
, reportArgs ? [ ]          # evidence flags for `report`, relative to the source tree
, config ? null             # a formal-coverage.toml for revs without one
, cargoHash                 # fixed-output hash of the vendored Cargo.lock dependencies
, fstarJobs ? null          # default: the build's cores
}:
assert builtins.match "[0-9a-f]{40}" rev != null;
assert features != [ ];
let
  lib = pkgs.lib;
  vendor = pkgs.rustPlatform.fetchCargoVendor {
    inherit src;
    hash = cargoHash;
  };
  configArgs = lib.optionals (config != null) [ "--config" "${config}" ];
in
pkgs.stdenv.mkDerivation {
  pname = "coverage-totals-${package}";
  version = builtins.substring 0 12 rev;
  inherit src;
  nativeBuildInputs = [
    formal-coverage
    haxPackages.hax
    haxPackages.fstar
    z3
    pkgs.python3
    pkgs.git
    pkgs.cargo
    pkgs.rustc
  ];
  HAX_CORE_LIBS = "${hax}/proof-libs/fstar";
  HAX_LIB_PROOFS = "${hax}/hax-lib/proofs/fstar/extraction";
  CARGO_NET_OFFLINE = "true";
  allowSubstitutes = false;
  preferLocalBuild = true;
  dontConfigure = true;
  dontFixup = true;
  buildPhase = ''
    runHook preBuild
    export HOME=$TMPDIR/home CARGO_HOME=$TMPDIR/cargo-home
    mkdir -p $HOME $CARGO_HOME $out
    substitute ${vendor}/.cargo/config.toml $CARGO_HOME/config.toml --replace-fail @vendor@ ${vendor}
    jobs=${if fstarJobs == null then "\${NIX_BUILD_CORES:-0}" else toString fstarJobs}
    [ "$jobs" -gt 0 ] || jobs=$(getconf _NPROCESSORS_ONLN)
    export FSTAR_JOBS=$jobs
    # Verify scripts may list files with `git ls-files`; this index is never measured.
    git init -q
    git add -A
    git -c user.name=nix -c user.email=nix@invalid commit -q -m source
    set +e
    # The log (parallel completion order) stays in the build log, not the output.
    ( set -e; ${verify} )
    echo "$?" > $out/verify.exit
    set -e
    rm -rf .git
    formal-coverage report . ${lib.escapeShellArgs (configArgs ++ reportArgs)} \
      --ship-features ${lib.escapeShellArg (lib.concatStringsSep "," features)} \
      --target ${lib.escapeShellArg target} \
      --check-target-dir $TMPDIR/formal-coverage-target \
      --json $out/report.json --markdown $out/report.md \
      --totals-json $out/totals.json \
      --totals-source ${lib.escapeShellArg "git+${url}#${rev}"} > /dev/null
    python3 - ${lib.escapeShellArg package} $out/totals.json <<'PY'
    import json, sys
    name = json.load(open(sys.argv[2]))['package']['name']
    if name != sys.argv[1]:
        sys.exit(f'receipt describes {name}, not {sys.argv[1]}')
    PY
    runHook postBuild
  '';
  installPhase = "true";
}
