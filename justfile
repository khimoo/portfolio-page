# Interactive Mindmap Portfolio - Task Runner
#
# This justfile provides organized recipes for development, building, and CI/CD operations.
# Run 'just --list' to see all available commands organized by category.

# === CONFIGURATION ===
# Get configuration values from project.toml
ARTICLES_DIR := `python3 scripts/config.py articles_dir --relative`
ASSETS_DIR := `python3 scripts/config.py assets_dir --relative`
IMAGES_DIR := `python3 scripts/config.py images_dir --relative`
DATA_DIR := `python3 scripts/config.py data_dir --relative`
APP_DIR := `python3 scripts/config.py app_dir --relative`

# Show all available recipes with descriptions
default:
    @just --list

# === DEVELOPMENT ===

# One-command dev: debug trunk serve + watchers that run CI-like steps on changes
dev:
	@echo "🚀 Starting dev: debug trunk serve + watchers (press Ctrl+C to stop all)..."
	@./scripts/dev.sh

# Run the CI-like pipeline but with dev (non-release) wasm build
dev-rebuild: ci-optimize-images ci-process-articles _copy-assets _dev-build-wasm _verify-build
    @echo "🎯 dev-rebuild complete"

# Small helper: build wasm in dev (no --release) to keep quick feedback during dev
_dev-build-wasm:
	@echo "🔧 Building WebAssembly (debug) for dev..."
	@cd {{APP_DIR}} && trunk build --public-url /
	@echo "✅ dev wasm build finished"

# Optional convenience: run only data pipeline (images + articles) without wasm
dev-data-only: ci-optimize-images ci-process-articles _copy-assets
    @echo "✅ Data pipeline complete (no wasm build)"

# === DATA PROCESSING RECIPES ===

# Process markdown articles into JSON data structures
process-articles:
    @echo "📝 Processing articles..."
    @cd {{APP_DIR}} && cargo test --features cli-tools test_process_articles_command -- --ignored --nocapture

# Process articles with image optimization
process-articles-with-images:
    @echo "📝 Processing articles with image optimization..."
    @cd {{APP_DIR}} && cargo test --features cli-tools test_process_articles_with_images_cli -- --ignored --nocapture

# Validate internal and external links in articles
validate-links:
    @echo "🔗 Validating links..."
    @cd {{APP_DIR}} && cargo test --features cli-tools test_validate_links_cli -- --ignored --nocapture

# Build all data: process articles and validate links
build-data: process-articles validate-links
    @echo "✅ All data processed successfully"

# === BUILD AND TEST RECIPES ===

# Build WebAssembly application for production deployment
build: build-data
    @echo "🏗️  Building for production..."
    @cd {{APP_DIR}} && trunk build --release --public-url /portfolio-page/

# Run all tests: unit tests and WebAssembly browser tests
test:
    @echo "🧪 Running tests..."
    @cargo test
    @wasm-pack test --headless --firefox

# Format all Rust code using rustfmt
fmt:
    @echo "🎨 Formatting code..."
    @cargo fmt

# Run clippy linter with strict warnings
clippy:
    @echo "📎 Running clippy..."
    @cargo clippy -- -D warnings

# Clean all generated files and build artifacts
clean:
    @echo "🧹 Cleaning up..."
    @rm -rf dist data/*.json target pkg

# === CI/CD RECIPES ===

# Verify CI environment and display tool versions and configuration
ci-verify-setup:
    @echo "🔧 Setting up CI environment..."
    @echo "Verifying required tools are available:"
    @echo "✅ Nix: $(nix --version 2>/dev/null || echo 'NOT FOUND')"
    @echo "✅ Rust: $(rustc --version 2>/dev/null || echo 'NOT FOUND')"
    @echo "✅ Trunk: $(trunk --version 2>/dev/null || echo 'NOT FOUND')"
    @echo "✅ Just: $(just --version 2>/dev/null || echo 'NOT FOUND')"
    @echo "✅ Python3: $(python3 --version 2>/dev/null || echo 'NOT FOUND')"
    @echo "✅ Cargo: $(cargo --version 2>/dev/null || echo 'NOT FOUND')"
    @echo "Environment variables:"
    @echo "  CI: ${CI:-not set}"
    @echo "  CARGO_TERM_COLOR: ${CARGO_TERM_COLOR:-not set}"
    @echo "  RUST_BACKTRACE: ${RUST_BACKTRACE:-not set}"
    @echo "🎯 CI environment setup complete"

# Optimize images using integrated Rust image processor
ci-optimize-images:
    @echo "🖼️ Optimizing images..."
    @just process-articles-with-images
    @echo "🎯 Image optimization complete"

# Process articles with validation and comprehensive output verification
ci-process-articles:
    @echo "📚 Processing articles..."
    @cd {{APP_DIR}} && cargo test --features cli-tools test_process_articles_with_images_cli -- --ignored --nocapture
    @just _verify-article-processing
    @echo "🎯 Article processing complete"

# Build WebAssembly application with asset copying and verification
ci-build-wasm:
    @echo "🚀 Building WebAssembly application..."
    @cd {{APP_DIR}} && trunk build --release --public-url /portfolio-page/
    @just _copy-assets
    @just _verify-build
    @echo "🎯 WebAssembly build complete"

# Prepare deployment directory with proper file structure and verification
ci-prepare-deploy:
    @echo "📁 Preparing deployment..."
    @just _setup-deploy-dir
    @just _copy-deployment-files
    @just _verify-deployment
    @echo "🎯 Deployment preparation complete"

# Run comprehensive verification of all build artifacts and deployment readiness
ci-verify:
    @echo "🔍 Final verification..."
    @just _verify-all-artifacts
    @echo ""
    @echo "🎯 All verification checks complete - ready for deployment!"

# === INTERNAL HELPER RECIPES ===
# These recipes are prefixed with _ and are used internally by CI recipes

# Verify image optimization results including file sizes and formats
_verify-images:
    @echo "📊 Verifying image optimization..."
    @# Check for original PNG files
    @if [ -f "{{IMAGES_DIR}}/author_img.png" ]; then \
        echo "✅ Original PNG found: $(ls -lh {{IMAGES_DIR}}/author_img.png | awk '{print $5}')"; \
    else \
        echo "⚠️ Original PNG not found"; \
    fi
    @# Check for optimized WebP files
    @if [ -f "{{IMAGES_DIR}}/author_img.webp" ]; then \
        echo "✅ Optimized WebP found: $(ls -lh {{IMAGES_DIR}}/author_img.webp | awk '{print $5}')"; \
    else \
        echo "⚠️ Optimized WebP not found"; \
    fi
    @# Verify image directory exists and list all image files
    @if [ -d "{{IMAGES_DIR}}" ]; then \
        echo "📁 Image directory contents:"; \
        ls -lah {{IMAGES_DIR}}/ | grep -E '\.(png|webp|jpg|jpeg|gif)$' || echo "  No image files found"; \
    else \
        echo "❌ Image directory not found!" && exit 1; \
    fi
    @# Check file formats and sizes
    @for ext in png webp jpg jpeg; do \
        for img in {{IMAGES_DIR}}/*.$${ext}; do \
            if [ -f "$${img}" ]; then \
                echo "🔍 $$(basename $${img}): $$(file $${img} | cut -d: -f2 | xargs) - $$(ls -lh $${img} | awk '{print $$5}')"; \
            fi; \
        done; \
    done 2>/dev/null || true

# Verify article processing results including JSON validation and content checks
_verify-article-processing:
    @echo "🔍 Verifying article processing..."
    @# Check for articles.json
    @if [ -f "{{DATA_DIR}}/articles.json" ]; then \
        echo "✅ articles.json generated successfully: $(ls -lh {{DATA_DIR}}/articles.json | awk '{print $5}')"; \
        echo "📄 Article count: $(cat {{DATA_DIR}}/articles.json | python3 -c "import sys, json; data=json.load(sys.stdin); print(len(data.get('articles', [])) if isinstance(data, dict) else len(data))" 2>/dev/null || echo "Unable to parse")"; \
    else \
        echo "❌ articles.json not found!" && exit 1; \
    fi
    @# Check for validation report
    @if [ -f "{{DATA_DIR}}/validation-report.json" ]; then \
        echo "✅ validation-report.json found: $(ls -lh {{DATA_DIR}}/validation-report.json | awk '{print $5}')"; \
    else \
        echo "⚠️ validation-report.json not found"; \
    fi
    @# Verify data directory structure
    @if [ -d "{{DATA_DIR}}" ]; then \
        echo "📁 Data directory contents:"; \
        ls -lah {{DATA_DIR}}/; \
    else \
        echo "❌ Data directory not found!" && exit 1; \
    fi
    @# Validate JSON structure
    @if [ -f "{{DATA_DIR}}/articles.json" ]; then \
        echo "🔍 Validating JSON structure..."; \
        python3 -c "import json; json.load(open('{{DATA_DIR}}/articles.json')); print('✅ Valid JSON structure')" 2>/dev/null || echo "❌ Invalid JSON structure"; \
    fi

# Copy assets to build directory including images and data files
_copy-assets:
    @echo "📸 Copying assets..."
    @# Create target directories
    @mkdir -p {{APP_DIR}}/dist/articles/img
    @mkdir -p {{APP_DIR}}/dist/data
    @# Copy image assets
    @if [ -d "{{IMAGES_DIR}}" ]; then \
        if ls {{IMAGES_DIR}}/* >/dev/null 2>&1; then \
            cp -v {{IMAGES_DIR}}/* {{APP_DIR}}/dist/articles/img/ && \
            echo "✅ Images copied successfully"; \
        else \
            echo "⚠️ No images found to copy"; \
        fi; \
    else \
        echo "⚠️ Image source directory not found"; \
    fi
    @# Copy data files
    @if [ -d "{{DATA_DIR}}" ]; then \
        if ls {{DATA_DIR}}/*.json >/dev/null 2>&1; then \
            cp -v {{DATA_DIR}}/*.json {{APP_DIR}}/dist/data/ && \
            echo "✅ Data files copied successfully"; \
        else \
            echo "⚠️ No data files found to copy"; \
        fi; \
    else \
        echo "⚠️ Data source directory not found"; \
    fi
    @# Verify copied assets
    @echo "🔍 Verifying copied assets:"
    @if [ -d "{{APP_DIR}}/dist/articles/img" ]; then \
        echo "  📁 Images: $(ls {{APP_DIR}}/dist/articles/img/ 2>/dev/null | wc -l) files"; \
    fi
    @if [ -d "{{APP_DIR}}/dist/data" ]; then \
        echo "  📁 Data: $(ls {{APP_DIR}}/dist/data/ 2>/dev/null | wc -l) files"; \
    fi

# Verify build artifacts including WebAssembly and JavaScript files
_verify-build:
    @echo "🔍 Verifying build artifacts..."
    @# Check dist directory exists
    @if [ -d "{{APP_DIR}}/dist" ]; then \
        echo "✅ dist directory found"; \
        echo "📁 Build size: $(du -sh {{APP_DIR}}/dist | awk '{print $1}')"; \
    else \
        echo "❌ dist directory not found!" && exit 1; \
    fi
    @# Check for essential files
    @if [ -f "{{APP_DIR}}/dist/index.html" ]; then \
        echo "✅ index.html found: $(ls -lh {{APP_DIR}}/dist/index.html | awk '{print $5}')"; \
    else \
        echo "❌ index.html not found!" && exit 1; \
    fi
    @# Check for WebAssembly files
    @if ls {{APP_DIR}}/dist/*.wasm >/dev/null 2>&1; then \
        echo "✅ WebAssembly files found:"; \
        ls -lh {{APP_DIR}}/dist/*.wasm | awk '{print "  " $9 ": " $5}'; \
    else \
        echo "❌ No WebAssembly files found!" && exit 1; \
    fi
    @# Check for JavaScript files
    @if ls {{APP_DIR}}/dist/*.js >/dev/null 2>&1; then \
        echo "✅ JavaScript files found:"; \
        ls -lh {{APP_DIR}}/dist/*.js | awk '{print "  " $9 ": " $5}'; \
    else \
        echo "⚠️ No JavaScript files found"; \
    fi
    @# List all files in dist
    @echo "📄 Complete dist directory structure:"; \
    ls -lah {{APP_DIR}}/dist/

# Setup deployment directory structure
_setup-deploy-dir:
    @echo "🏗️ Setting up deployment directory..."
    @mkdir -p public
    @mkdir -p public/data
    @echo "✅ Deployment directories created"

# Copy files to deployment directory with proper structure
_copy-deployment-files:
    @echo "📋 Copying deployment files..."
    @# Copy dist contents to public
    @if [ -d "{{APP_DIR}}/dist" ]; then \
        cp -r {{APP_DIR}}/dist/* public/ && \
        echo "✅ Copied dist contents to public/"; \
    else \
        echo "❌ dist directory not found!" && exit 1; \
    fi
    @# Copy data contents to public/data
    @if [ -d "{{DATA_DIR}}" ]; then \
        cp -r {{DATA_DIR}}/* public/data/ && \
        echo "✅ Copied data contents to public/data/"; \
    else \
        echo "⚠️ data directory not found"; \
    fi
    @# Create 404.html from index.html
    @if [ -f "public/index.html" ]; then \
        cp public/index.html public/404.html && \
        echo "✅ Created 404.html from index.html"; \
    else \
        echo "⚠️ index.html not found, cannot create 404.html"; \
    fi

# Verify deployment directory structure and essential files
_verify-deployment:
    @echo "🔍 Verifying deployment structure..."
    @# Check public directory exists
    @if [ -d "public" ]; then \
        echo "✅ public directory created"; \
        echo "📁 Deployment size: $(du -sh public | awk '{print $1}')"; \
    else \
        echo "❌ public directory not found!" && exit 1; \
    fi
    @# Check essential files
    @if [ -f "public/index.html" ]; then \
        echo "✅ public/index.html found: $(ls -lh public/index.html | awk '{print $5}')"; \
    else \
        echo "❌ public/index.html not found!" && exit 1; \
    fi
    @# Check data directory
    @if [ -d "public/data" ]; then \
        echo "✅ public/data directory found: $(du -sh public/data | awk '{print $1}')"; \
        if ls public/data/*.json >/dev/null 2>&1; then \
            echo "  📄 JSON files: $(ls public/data/*.json | wc -l)"; \
        fi; \
    else \
        echo "⚠️ public/data directory missing"; \
    fi
    @# List deployment structure
    @echo "📄 Deployment structure:"; \
    ls -lah public/ | head -20

# Verify all artifacts and deployment readiness with summary output
_verify-all-artifacts:
    @echo "📊 Checking all artifacts and deployment readiness:"
    @echo ""
    @echo "🖼️ Image artifacts:"
    @just _verify-images | grep -E "(✅|❌|⚠️)" | sed 's/^/  /'
    @echo ""
    @echo "📚 Article processing artifacts:"
    @just _verify-article-processing | grep -E "(✅|❌|⚠️)" | sed 's/^/  /'
    @echo ""
    @echo "🚀 Build artifacts:"
    @just _verify-build | grep -E "(✅|❌|⚠️)" | sed 's/^/  /'
    @echo ""
    @echo "📁 Deployment artifacts:"
    @just _verify-deployment | grep -E "(✅|❌|⚠️)" | sed 's/^/  /'

# === 内部用: Watcherから呼び出されるレシピ ===

# Articles変更時に実行: データ再構築 -> index.htmlの更新(reloadトリガー)
_on-article-change:
    @echo "[watcher:articles] change detected: running pipeline..."
    @just dev-data-only
    @# index.htmlをtouchすることでtrunk serveにリロードさせる
    @touch khimoo-portfolio/index.html
