{
  description = "The flake that is used add Rust and it's packages the dev shell.";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs: with inputs; flake-utils.lib.eachDefaultSystem (system:
      let

        pkgs = import nixpkgs {
          inherit system;
          overlays = [ cargo2nix.overlays.default ];
        };
        
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.75.0";
          packageFun = import ./Cargo.nix;
        };

        overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));

      in rec {

        packages = {
          advent_of_code_2024 = (rustPkgs.workspace.advent_of_code_2024 {});
          default = packages.advent_of_code_2024;
        };

        devShell = pkgs.mkShell {
          
          RUSTC_VERSION = overrides.toolchain.channel;
          RUST_BACKTRACE = 1;

          buildInputs = with pkgs; [ cargo rustc rustup rust-analyzer pkg-config openssl ];
          shellHook = (''
            # if the terminal supports color.
            if [[ -n "$(tput colors)" && "$(tput colors)" -gt 2 ]]; then
              echo -e "\033[1;32mStarted\033[0m a \033[1;31mRust\033[0m Development Shell powered by \033[1;34mNix\033[0m."
              echo -e "Using \033[1;33m$(cargo --version)\033[00m together with \033[1;33m$(rustc --version)\033[00m."
              export PS1="(\033[1m\033[35mDev-Shell\033[0m) $PS1"
            else 
              echo "Started a Rust Development Shell powered by Nix."
              echo "Using $(cargo --version) together with $(rustc --version)."
              export PS1="(Dev-Shell) $PS1"
            fi''
          );
        };
      }
    );
}