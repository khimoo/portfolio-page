{
  description = "Interactive Mindmap Portfolio";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        # CI-specific tools
        ciTools = with pkgs; [
          # Image processing
          python3
          python3Packages.pillow

          # File operations and utilities
          coreutils
          findutils
          gnugrep
          gawk
          
          # Verification tools
          file
          tree
          
          # Additional utilities for CI
          curl
          jq
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain

            # WebAssembly tools
            wasm-pack
            trunk

            # Development tools
            watchexec
            just

            # System dependencies
            pkg-config
            openssl
          ] ++ ciTools;

          shellHook = ''
            echo "🦀 Rust WebAssembly development environment"
            echo "📦 Available commands:"
            just --list
            
            # CI environment detection and setup
            if [ "$CI" = "true" ]; then
              echo "🔧 CI environment detected"
              echo "🐍 Python version: $(python3 --version)"
              echo "🖼️ Pillow available: $(python3 -c "import PIL; print('✅ PIL version:', PIL.__version__)" 2>/dev/null || echo "❌ PIL not available")"
              echo "📁 File utilities: $(file --version | head -1)"
              echo "🔍 Verification tools ready"
            else
              echo "💻 Local development environment"
            fi
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "khimoo-portfolio";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = with pkgs; [ pkg-config openssl ];

          # WebAssembly build
          buildPhase = ''
            cargo build --release
            wasm-pack build --target web --out-dir pkg
          '';
        };
      });
}
