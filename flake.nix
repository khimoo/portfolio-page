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

        ciTools = with pkgs; [
          python3
          python3Packages.pillow
          coreutils
          findutils
          gnugrep
          gawk
          file
          tree
          curl
          jq
        ];

        # devShellを変数として定義してnix run(apps.default)で使えるようにしたい
        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            wasm-pack
            trunk
            watchexec
            just
            pkg-config
            openssl
          ] ++ ciTools;

          shellHook = ''
            echo "🦀 Rust WebAssembly development environment"
            echo "📦 Available commands:"
            just --list

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
      in
      {
        devShells.default = devShell;

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "khimoo-portfolio";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = with pkgs; [ pkg-config openssl ];

          buildPhase = ''
            cargo build --release
            wasm-pack build --target web --out-dir pkg
          '';
        };

        apps.default = {
          type = "app";
          program = "${pkgs.writeShellApplication {
            name = "dev";
            runtimeInputs = devShell.buildInputs;
            text = ''
              exec just dev
            '';
          }}/bin/dev";
        };
      });
}
