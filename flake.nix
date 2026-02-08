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

        overlays = [
          rust-overlay.overlays.default
        ];

        pkgs = import nixpkgs { inherit system overlays; };

        # Pin the Rust toolchain.
        # This will use the toolchain specified in a `rust-toolchain.toml` file
        # in your project root. If one doesn't exist, it falls back to stable.
        # The function expects the path directly, not in a set.
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        rustBuildInputs = with pkgs;
          [
            pkg-config
            openssl
          ] ++ lib.optionals stdenv.isLinux [
            # GTK and related dependencies for Linux
            cairo
            gdk-pixbuf
            gtk3
            libsoup_3
            webkitgtk_4_1
            xdotool # For window manipulation
            librsvg # For SVG support
            libayatana-appindicator # For system tray support
          ] ++ lib.optionals stdenv.isDarwin [
            apple-sdk_14
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
            tailwindcss_4
          ] ++ rustBuildInputs;

          RUST_BACKTRACE = 1;

          # This hook runs when you enter the shell.
          # It ensures that SDKROOT is set, which is needed by some build scripts on macOS.
          shellHook = ''
            ${pkgs.lib.optionalString pkgs.stdenv.isLinux "export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath rustBuildInputs}:$LD_LIBRARY_PATH"}
            ${pkgs.lib.optionalString pkgs.stdenv.isDarwin "export SDKROOT=$(xcrun --sdk macosx --show-sdk-path)"}
          '';
        };
      });
}
