# Khimoo Portfolio

Interactive portfolio website built with Rust and WebAssembly.

## Project Structure

```
khimoo.io/
├── project.toml               # 🎯 Central configuration (DRY, ETC, KISS)
├── content/                   # 📚 Content directory (articles & assets)
│   ├── articles/             # Markdown articles
│   └── assets/img/           # Images and media
├── khimoo-portfolio/         # 🦀 Rust/WASM application
├── scripts/                  # 🔧 Build and development scripts
│   ├── config.py            # Configuration loader
│   ├── dev.sh               # Development environment
│   └── optimize_images.py   # Image optimization
├── public/                   # 📦 Deployment output
└── justfile                  # 🚀 Task runner
```

## Configuration System

This project follows **DRY**, **ETC**, and **KISS** principles through centralized configuration:

### Central Configuration (`project.toml`)

All paths and settings are managed in one place:

```toml
[paths]
articles_dir = "content/articles"
assets_dir = "content/assets"
images_dir = "content/assets/img"
data_dir = "khimoo-portfolio/data"
# ... more paths

[build]
debounce_ms = 300
parallel_processing = true

[optimization]
webp_quality = 85
small_image_size = 64
medium_image_size = 128
```

### Benefits

- **DRY**: No hardcoded paths across the codebase
- **ETC**: Easy to change paths and settings
- **KISS**: Single source of truth for configuration

### Usage

```bash
# Get configuration values
python3 scripts/config.py articles_dir          # Absolute path
python3 scripts/config.py articles_dir --relative  # Relative path

# All build commands use the configuration automatically
just process-articles    # Uses configured articles_dir
just ci-optimize-images  # Uses configured images_dir
just dev                 # Uses all configured paths
```

## Development

```bash
# Start development environment (uses configuration)
just dev

# Process articles (uses configured paths)
just process-articles

# Process articles with image optimization
just process-articles-with-images

# Optimize images (integrated with article processing)
just ci-optimize-images

# Build for production
just build
```

## Future Content Management

The content directory is designed for future migration to a separate repository:

1. **Current**: `content/` directory in same repo
2. **Future**: Independent content repository
3. **Migration**: Update `project.toml` paths only
4. **Integration**: Git submodule/subtree with CI/CD sync

The configuration system ensures this migration will be transparent to the build process.

## Architecture

- **Frontend**: Yew (Rust WebAssembly framework)
- **Physics**: Rapier2D for interactive elements
- **Content**: Markdown with YAML frontmatter
- **Build**: Trunk for WASM bundling
- **Task Runner**: Just for development workflows
- **Configuration**: TOML-based centralized config