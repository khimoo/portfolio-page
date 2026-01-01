# Interactive Mindmap Portfolio - Task Runner
# 
# This justfile provides organized recipes for development, building, and CI/CD operations.
# Run 'just --list' to see all available commands organized by category.

# Show all available recipes with descriptions
default:
    @just --list

# === DEVELOPMENT RECIPES ===

# Start complete development environment with article processing and live server
dev:
    @echo "🚀 Starting development environment..."
    @just process-articles
    @cd khimoo-portfolio && trunk serve

# Start development server with hot reload (requires pre-built data)
serve: build-data
    @echo "🌐 Starting development server..."
    @trunk serve --open

# Watch articles directory and rebuild data on changes
watch:
    @echo "👀 Watching articles for changes..."
    @watchexec -w articles -e md -- just build-data

# Complete development workflow: format, lint, test, build data, and serve
full-dev: fmt clippy test build-data
    @echo "🎯 Full development check complete"
    @trunk serve

# === DATA PROCESSING RECIPES ===

# Process markdown articles into JSON data structures
process-articles:
    @echo "📝 Processing articles..."
    @cd khimoo-portfolio && cargo run --bin process-articles --features cli-tools

# Validate internal and external links in articles
validate-links:
    @echo "🔗 Validating links..."
    @cargo run --bin validate-links

# Generate interactive link graph from article connections
generate-link-graph:
    @echo "🕸️  Generating link graph..."
    @cd khimoo-portfolio && cargo run --features cli-tools --bin generate-link-graph

# Build all data: process articles, validate links, and generate link graph
build-data: process-articles validate-links generate-link-graph
    @echo "✅ All data processed successfully"

# === BUILD AND TEST RECIPES ===

# Build WebAssembly application for production deployment
build: build-data
    @echo "🏗️  Building for production..."
    @trunk build --release

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
ci-setup:
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

# Optimize images using Python script with comprehensive verification
ci-optimize-images:
    @echo "🖼️ Optimizing images..."
    @python3 optimize_images.py
    @just _verify-images
    @echo "🎯 Image optimization complete"

# Process articles with validation and comprehensive output verification
ci-process-articles:
    @echo "📚 Processing articles..."
    @cd khimoo-portfolio && cargo run --features cli-tools --bin process-articles -- --articles-dir articles --output-dir data --verbose
    @just _verify-article-processing
    @echo "🎯 Article processing complete"

# Build WebAssembly application with asset copying and verification
ci-build-wasm:
    @echo "🚀 Building WebAssembly application..."
    @cd khimoo-portfolio && trunk build --release --public-url /portfolio-page/
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
    @if [ -f "khimoo-portfolio/articles/img/author_img.png" ]; then \
        echo "✅ Original PNG found: $(ls -lh khimoo-portfolio/articles/img/author_img.png | awk '{print $5}')"; \
    else \
        echo "⚠️ Original PNG not found"; \
    fi
    @# Check for optimized WebP files
    @if [ -f "khimoo-portfolio/articles/img/author_img.webp" ]; then \
        echo "✅ Optimized WebP found: $(ls -lh khimoo-portfolio/articles/img/author_img.webp | awk '{print $5}')"; \
    else \
        echo "⚠️ Optimized WebP not found"; \
    fi
    @# Verify image directory exists and list all image files
    @if [ -d "khimoo-portfolio/articles/img" ]; then \
        echo "📁 Image directory contents:"; \
        ls -lah khimoo-portfolio/articles/img/ | grep -E '\.(png|webp|jpg|jpeg|gif)$' || echo "  No image files found"; \
    else \
        echo "❌ Image directory not found!" && exit 1; \
    fi
    @# Check file formats and sizes
    @for img in khimoo-portfolio/articles/img/*.{png,webp,jpg,jpeg} 2>/dev/null; do \
        if [ -f "$img" ]; then \
            echo "🔍 $(basename $img): $(file $img | cut -d: -f2 | xargs) - $(ls -lh $img | awk '{print $5}')"; \
        fi; \
    done

# Verify article processing results including JSON validation and content checks
_verify-article-processing:
    @echo "🔍 Verifying article processing..."
    @# Check for articles.json
    @if [ -f "khimoo-portfolio/data/articles.json" ]; then \
        echo "✅ articles.json generated successfully: $(ls -lh khimoo-portfolio/data/articles.json | awk '{print $5}')"; \
        echo "📄 Article count: $(cat khimoo-portfolio/data/articles.json | python3 -c "import sys, json; data=json.load(sys.stdin); print(len(data.get('articles', [])) if isinstance(data, dict) else len(data))" 2>/dev/null || echo "Unable to parse")"; \
    else \
        echo "❌ articles.json not found!" && exit 1; \
    fi
    @# Check for validation report
    @if [ -f "khimoo-portfolio/data/validation-report.json" ]; then \
        echo "✅ validation-report.json found: $(ls -lh khimoo-portfolio/data/validation-report.json | awk '{print $5}')"; \
    else \
        echo "⚠️ validation-report.json not found"; \
    fi
    @# Verify data directory structure
    @if [ -d "khimoo-portfolio/data" ]; then \
        echo "📁 Data directory contents:"; \
        ls -lah khimoo-portfolio/data/; \
    else \
        echo "❌ Data directory not found!" && exit 1; \
    fi
    @# Validate JSON structure
    @if [ -f "khimoo-portfolio/data/articles.json" ]; then \
        echo "🔍 Validating JSON structure..."; \
        python3 -c "import json; json.load(open('khimoo-portfolio/data/articles.json')); print('✅ Valid JSON structure')" 2>/dev/null || echo "❌ Invalid JSON structure"; \
    fi

# Copy assets to build directory including images and data files
_copy-assets:
    @echo "📸 Copying assets..."
    @# Create target directories
    @mkdir -p khimoo-portfolio/dist/articles/img
    @mkdir -p khimoo-portfolio/dist/data
    @# Copy image assets
    @if [ -d "khimoo-portfolio/articles/img" ]; then \
        if ls khimoo-portfolio/articles/img/* >/dev/null 2>&1; then \
            cp -v khimoo-portfolio/articles/img/* khimoo-portfolio/dist/articles/img/ && \
            echo "✅ Images copied successfully"; \
        else \
            echo "⚠️ No images found to copy"; \
        fi; \
    else \
        echo "⚠️ Image source directory not found"; \
    fi
    @# Copy data files
    @if [ -d "khimoo-portfolio/data" ]; then \
        if ls khimoo-portfolio/data/*.json >/dev/null 2>&1; then \
            cp -v khimoo-portfolio/data/*.json khimoo-portfolio/dist/data/ && \
            echo "✅ Data files copied successfully"; \
        else \
            echo "⚠️ No data files found to copy"; \
        fi; \
    else \
        echo "⚠️ Data source directory not found"; \
    fi
    @# Verify copied assets
    @echo "🔍 Verifying copied assets:"
    @if [ -d "khimoo-portfolio/dist/articles/img" ]; then \
        echo "  📁 Images: $(ls khimoo-portfolio/dist/articles/img/ 2>/dev/null | wc -l) files"; \
    fi
    @if [ -d "khimoo-portfolio/dist/data" ]; then \
        echo "  📁 Data: $(ls khimoo-portfolio/dist/data/ 2>/dev/null | wc -l) files"; \
    fi

# Verify build artifacts including WebAssembly and JavaScript files
_verify-build:
    @echo "🔍 Verifying build artifacts..."
    @# Check dist directory exists
    @if [ -d "khimoo-portfolio/dist" ]; then \
        echo "✅ dist directory found"; \
        echo "📁 Build size: $(du -sh khimoo-portfolio/dist | awk '{print $1}')"; \
    else \
        echo "❌ dist directory not found!" && exit 1; \
    fi
    @# Check for essential files
    @if [ -f "khimoo-portfolio/dist/index.html" ]; then \
        echo "✅ index.html found: $(ls -lh khimoo-portfolio/dist/index.html | awk '{print $5}')"; \
    else \
        echo "❌ index.html not found!" && exit 1; \
    fi
    @# Check for WebAssembly files
    @if ls khimoo-portfolio/dist/*.wasm >/dev/null 2>&1; then \
        echo "✅ WebAssembly files found:"; \
        ls -lh khimoo-portfolio/dist/*.wasm | awk '{print "  " $9 ": " $5}'; \
    else \
        echo "❌ No WebAssembly files found!" && exit 1; \
    fi
    @# Check for JavaScript files
    @if ls khimoo-portfolio/dist/*.js >/dev/null 2>&1; then \
        echo "✅ JavaScript files found:"; \
        ls -lh khimoo-portfolio/dist/*.js | awk '{print "  " $9 ": " $5}'; \
    else \
        echo "⚠️ No JavaScript files found"; \
    fi
    @# List all files in dist
    @echo "📄 Complete dist directory structure:"; \
    ls -lah khimoo-portfolio/dist/

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
    @if [ -d "khimoo-portfolio/dist" ]; then \
        cp -r khimoo-portfolio/dist/* public/ && \
        echo "✅ Copied dist contents to public/"; \
    else \
        echo "❌ dist directory not found!" && exit 1; \
    fi
    @# Copy data contents to public/data
    @if [ -d "khimoo-portfolio/data" ]; then \
        cp -r khimoo-portfolio/data/* public/data/ && \
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
    @# Copy test data loader if it exists
    @if [ -f "test-data-loader.html" ]; then \
        cp test-data-loader.html public/ && \
        echo "✅ Copied test-data-loader.html"; \
    else \
        echo "⚠️ test-data-loader.html not found"; \
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