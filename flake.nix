{
  description = "formal-coverage: formal verification coverage reports";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  inputs.hax.url = "github:hacspec/hax/5ccf89dd2e996c9f88f2267eaa2a64739c4ca41a";

  outputs = { self, nixpkgs, hax }:
    let
      systems = [ "aarch64-darwin" "x86_64-linux" "aarch64-linux" "x86_64-darwin" ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
      # The very solver bundled by the pinned F* flake, also as `z3`.
      fstarZ3 = system:
        let
          pkgs = import nixpkgs { inherit system; };
          fstarPackages = hax.inputs.fstar.packages.${system};
        in pkgs.runCommand "fstar-z3-4.13.3" { } ''
          mkdir -p $out/bin
          ln -s ${fstarPackages.z3}/bin/z3-4.13.3 $out/bin/z3
          ln -s ${fstarPackages.z3}/bin/z3-4.13.3 $out/bin/z3-4.13.3
        '';
    in {
      packages = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          formal-coverage = pkgs.rustPlatform.buildRustPackage {
            pname = "formal-coverage";
            version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
            src = pkgs.lib.cleanSource self;
            cargoLock.lockFile = ./Cargo.lock;
            doCheck = true;
            nativeCheckInputs = [ pkgs.git ];
            # The default check hook sets CARGO_BUILD_TARGET on the test process,
            # redirecting nested fixture builds away from their native layout.
            checkPhase = ''
              runHook preCheck
              env -u CARGO_BUILD_TARGET cargo test --release --offline --locked \
                --target ${pkgs.stdenv.hostPlatform.rust.rustcTarget} -- --include-ignored
              runHook postCheck
            '';
            meta = {
              description = "Measure formal verification coverage in Rust projects";
              license = pkgs.lib.licenses.mit;
              mainProgram = "formal-coverage";
              platforms = systems;
            };
          };
        in { inherit formal-coverage; default = formal-coverage; });

      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.formal-coverage}/bin/formal-coverage";
        };
      });

      devShells = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
          haxPackages = hax.packages.${system};
          z3 = fstarZ3 system;
        in {
          hax-fstar = assert hax.rev == "5ccf89dd2e996c9f88f2267eaa2a64739c4ca41a";
            assert hax.inputs.fstar.rev == "7b347386330d0e5a331a220535b6f15288903234";
            pkgs.mkShell {
              packages = [
                self.packages.${system}.formal-coverage
                haxPackages.hax
                haxPackages.fstar
                z3
                pkgs.python3
                haxPackages.rustc
              ];
              HAX_CORE_LIBS = "${hax}/proof-libs/fstar";
              HAX_LIB_PROOFS = "${hax}/hax-lib/proofs/fstar/extraction";
              shellHook = ''
                fstar_version=$(fstar.exe --version) || exit 1
                case "$fstar_version" in
                  *2025.10.06*) ;;
                  *) printf 'Expected F* 2025.10.06, got: %s\n' "$fstar_version" >&2; exit 1 ;;
                esac
                z3_version=$(z3 --version) || exit 1
                case "$z3_version" in
                  "Z3 version 4.13.3 "*) ;;
                  *) printf 'Expected Z3 4.13.3, got: %s\n' "$z3_version" >&2; exit 1 ;;
                esac
                printf 'hax ${hax.rev} / F* 2025.10.06 / Z3 4.13.3 / %s\n' "$(formal-coverage --version)"
                unset fstar_version z3_version
              '';
            };
        });

      lib = forAllSystems (system:
        let
          pkgs = import nixpkgs { inherit system; };
        in {
          coverageTotals = import ./nix/coverage-totals.nix {
            inherit pkgs hax;
            formal-coverage = self.packages.${system}.formal-coverage;
            haxPackages = hax.packages.${system};
            z3 = fstarZ3 system;
          };
        });

      checks = forAllSystems (system: {
        inherit (self.packages.${system}) formal-coverage;
      });
    };
}
