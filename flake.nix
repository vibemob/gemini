{
  description = "A flake for Dioxus development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let

        overlays = [ rust-overlay.overlays.default];
        pkgs = import nixpkgs { inherit system overlays; };

        # Pin the Rust toolchain.
        # This will use the toolchain specified in a `rust-toolchain.toml` file
        # in your project root. If one doesn't exist, it falls back to stable.
        # The function expects the path directly, not in a set.
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustBuildInputs = [
            pkgs.pkg-config
            pkgs.openssl
            pkgs.libiconv
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          # The `buildInputs` are available in the shell, but not propagated
          # to the build of packages that might use this shell.
          # `packages` is an alias for `buildInputs` in mkShell.
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            wasm-bindgen-cli_0_2_99
            dioxus-cli

          ] ++ rustBuildInputs;

          RUST_BACKTRACE = 1;
        };
      });
}
