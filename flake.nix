{
  description = "lnmai-core-ffi";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    lnmai-core = {
      url = "github:Neuron-Group/lnmai-core?ref=main";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs = { self, nixpkgs, flake-utils, lnmai-core }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        libs = [ pkgs.zlib ];
        devLibs = map pkgs.lib.getDev libs;
        commonEnv = ''
          export CARGO="${pkgs.cargo}/bin/cargo"
          export RUSTC="${pkgs.rustc}/bin/rustc"
          export CC="${pkgs.stdenv.cc}/bin/cc"
          export CXX="${pkgs.stdenv.cc}/bin/c++"
          export AR="${pkgs.binutils}/bin/ar"
          export RANLIB="${pkgs.binutils}/bin/ranlib"
          export PATH="${pkgs.lib.makeBinPath [ pkgs.binutils pkgs.cargo pkgs.coreutils pkgs.elan pkgs.git pkgs.pkg-config pkgs.rustc pkgs.stdenv.cc ]}:$PATH"
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath libs}''${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
          export PKG_CONFIG_PATH="${pkgs.lib.makeSearchPath "lib/pkgconfig" devLibs}:${pkgs.lib.makeSearchPath "share/pkgconfig" devLibs}''${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
        '';
        buildScript = pkgs.writeShellApplication {
          name = "lnmai-core-ffi-build";
          runtimeInputs = with pkgs; [
            binutils
            cargo
            coreutils
            elan
            git
            pkg-config
            rustc
            stdenv.cc
            zlib
          ];
          text = ''
            ${commonEnv}
            repo_root="''${LNMAI_CORE_FFI_REPO_ROOT:-$PWD}"
            workspace_root="$(mktemp -d "''${TMPDIR:-/tmp}/lnmai-core-ffi.XXXXXX")"
            source_root="$workspace_root/source"

            trap 'rm -rf "$workspace_root"' EXIT

            mkdir -p "$source_root"

            cp -R --no-preserve=mode,ownership ${self}/. "$source_root/"
            chmod -R u+w "$source_root"

            rm -rf "$source_root/lnmai-core"
            mkdir -p "$source_root/lnmai-core"
            cp -R --no-preserve=mode,ownership ${lnmai-core}/. "$source_root/lnmai-core/"
            chmod -R u+w "$source_root/lnmai-core"

            export CARGO_TARGET_DIR="$repo_root/target/nix"
            cd "$source_root"
            if [ "$#" -eq 0 ]; then
              set -- --lib
            fi
            exec "$CARGO" build "$@"
          '';
        };
      in {
        packages.default = buildScript;

        apps.default = {
          type = "app";
          program = "${buildScript}/bin/lnmai-core-ffi-build";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            binutils
            cargo
            coreutils
            elan
            git
            pkg-config
            rustc
            rustfmt
            stdenv.cc
            zlib
          ];

          buildInputs = libs ++ devLibs;

          shellHook = commonEnv;
        };
      });
}
