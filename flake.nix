{
  description = "ctx-analyzer dev shell — Rust toolchain + tooling";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustToolchain
            cargo-nextest
            cargo-deny
            cargo-edit
            cargo-watch
            prettier
            lefthook
            just
            sqlite
            jq
          ];

          shellHook = ''
            echo "ctx-analyzer dev shell ready."
            echo "Try: just check"
          '';
        };

        formatter = pkgs.nixfmt-rfc-style;
      });
}
