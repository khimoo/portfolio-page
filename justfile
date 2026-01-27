# Interactive Mindmap Portfolio - Task Runner
# Simple, maintainable recipes following DRY, KISS, and ETC principles

# === CONFIGURATION ===
ARTICLES_DIR := `python3 scripts/config.py articles_dir --relative`
DATA_DIR := `python3 scripts/config.py data_dir --relative`
APP_DIR := `python3 scripts/config.py app_dir --relative`
GITHUB_PAGES_PATH := `python3 scripts/config.py github_pages_path --section deployment`
LOCAL_DEV_PATH := `python3 scripts/config.py local_dev_path --section deployment`

# Show all available recipes
default:
    @just --list

# === SETUP ===

# Initialize project and submodules
setup:
    @echo "🛠️ Setting up project..."
    @git submodule update --init --recursive
    @echo "✅ Setup complete"

# === DEVELOPMENT ===

# Start development server with file watchers
dev:
    @echo "🚀 Starting development environment..."
    @./scripts/dev.sh

# Development with GitHub Pages path for testing
dev-gh-pages:
    @echo "🚀 Starting dev with GitHub Pages path..."
    @GITHUB_PAGES_MODE=1 ./scripts/dev.sh

# Rebuild everything for development
dev-rebuild: process-data build-wasm-dev copy-assets
    @echo "✅ Development rebuild complete"

# === DATA PROCESSING ===

# Process articles and optimize images
process-data:
    @echo "📝 Processing data..."
    @cd {{APP_DIR}} && cargo run --bin khimoo-portfolio --features cli-tools -- process-articles --optimize-images

# Validate links in articles
validate-links:
    @echo "🔗 Validating links..."
    @cd {{APP_DIR}} && cargo run --bin khimoo-portfolio --features cli-tools -- validate-links

# === BUILD ===

# Build WebAssembly for development
build-wasm-dev:
    @echo "🔧 Building WebAssembly (debug)..."
    @cd {{APP_DIR}} && trunk build --public-url {{LOCAL_DEV_PATH}}

# Build WebAssembly for production
build-wasm-prod:
    @echo "🔧 Building WebAssembly (release)..."
    @cd {{APP_DIR}} && trunk build --release --public-url {{GITHUB_PAGES_PATH}}

# Copy assets to dist directory
copy-assets:
    @echo "📸 Copying assets..."
    @mkdir -p {{APP_DIR}}/dist/articles/img {{APP_DIR}}/dist/data
    @cp -r content/assets/img/* {{APP_DIR}}/dist/articles/img/ 2>/dev/null || true
    @cp -r {{DATA_DIR}}/*.json {{APP_DIR}}/dist/data/ 2>/dev/null || true

# Full production build
build: process-data build-wasm-prod copy-assets
    @echo "🏗️ Production build complete"

# === TESTING ===

# Run all tests
test:
    @echo "🧪 Running tests..."
    @cargo test
    @wasm-pack test --headless --firefox

# Format code
fmt:
    @echo "🎨 Formatting code..."
    @cd {{APP_DIR}} && cargo fmt

# Run linter
clippy:
    @echo "📎 Running clippy..."
    @cd {{APP_DIR}} && cargo clippy -- -D warnings

# Clean build artifacts
clean:
    @echo "🧹 Cleaning up..."
    @rm -rf {{APP_DIR}}/dist {{DATA_DIR}}/*.json {{APP_DIR}}/target {{APP_DIR}}/pkg public
    @rm -rf scripts/__pycache__
    @rm -rf ~/.cache/trunk
    @echo "✅ Cleanup complete"

# === CI/CD ===

# Setup CI environment
ci-setup:
    @echo "🔧 CI environment setup..."
    @echo "Tools: $(rustc --version), $(trunk --version), $(just --version)"

# Full CI pipeline
ci-build: process-data build-wasm-prod copy-assets prepare-deploy
    @echo "🎯 CI build complete"

# Prepare deployment directory
prepare-deploy:
    @echo "📁 Preparing deployment..."
    @mkdir -p public
    @cp -r {{APP_DIR}}/dist/* public/
    @cp -r {{DATA_DIR}}/* public/data/ 2>/dev/null || true
    @cp public/index.html public/404.html

# Verify build artifacts
verify:
    @echo "🔍 Verifying build..."
    @test -f {{APP_DIR}}/dist/index.html || (echo "❌ Missing index.html" && exit 1)
    @test -f {{APP_DIR}}/dist/*.wasm || (echo "❌ Missing WebAssembly files" && exit 1)
    @test -d public || (echo "❌ Missing public directory" && exit 1)
    @echo "✅ All checks passed"

# === INTERNAL ===

# Called by file watcher on article changes
_on-article-change:
    @echo "[watcher] Article change detected, rebuilding data..."
    @just process-data
    @touch {{APP_DIR}}/index.html
