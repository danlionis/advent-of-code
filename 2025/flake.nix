{
  description = "Rust nightly dev shell";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    {
      devShells.x86_64-linux.default =
        let
          pkgs = import nixpkgs {
            system = "x86_64-linux";
            overlays = [ rust-overlay.overlays.default ];
          };
          rust = pkgs.rust-bin.nightly."2025-11-09".default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          };
          # rust = pkgs.rust-bin.nightly."2024-04-01".default;
          # rust = pkgs.rust-bin.nightly."2024-10-10".default;
        in
        pkgs.mkShell {
          buildInputs = with pkgs; [
            rust
            cargo-flamegraph
            aoc-cli
          ];
        };
    };
}
