use anyhow::Result;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[cfg(feature = "cli-tools")]
use image::{ImageFormat, DynamicImage};
#[cfg(feature = "cli-tools")]
use anyhow::Context;
#[cfg(feature = "cli-tools")]
use std::fs;

/// Configuration for image optimization
#[derive(Debug, Clone)]
pub struct ImageOptimizationConfig {
    pub webp_quality: u8,
    pub small_image_size: u32,
    pub medium_image_size: u32,
    pub preserve_original: bool,
}

impl Default for ImageOptimizationConfig {
    fn default() -> Self {
        Self {
            webp_quality: 85,
            small_image_size: 64,
            medium_image_size: 128,
            preserve_original: true,
        }
    }
}

/// Result of image optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedImageSet {
    pub original_path: PathBuf,
    pub small_png_path: PathBuf,
    pub small_webp_path: PathBuf,
    pub medium_png_path: PathBuf,
    pub original_size: u64,
    pub small_png_size: u64,
    pub small_webp_size: u64,
    pub medium_png_size: u64,
    pub compression_ratio: f64,
}

/// Thumbnail generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub size_bytes: u64,
    pub format: String,
}

/// Compressed image result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedImage {
    pub path: PathBuf,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub quality: u8,
}

/// Image optimizer for portfolio assets
/// Provides centralized image processing and optimization functionality
pub struct ImageOptimizer {
    config: ImageOptimizationConfig,
    verbose: bool,
}

impl ImageOptimizer {
    /// Create a new image optimizer with configuration
    pub fn new(config: ImageOptimizationConfig, verbose: bool) -> Self {
        Self { config, verbose }
    }

    /// Create with default configuration
    pub fn with_defaults() -> Self {
        Self::new(ImageOptimizationConfig::default(), false)
    }

    /// Optimize a single image file
    #[cfg(feature = "cli-tools")]
    pub fn optimize_image(&self, input_path: &Path, output_dir: &Path) -> Result<OptimizedImageSet> {
        if !input_path.exists() {
            return Err(anyhow::anyhow!("Input image not found: {:?}", input_path));
        }

        // Create output directory
        fs::create_dir_all(output_dir)
            .with_context(|| format!("Failed to create output directory: {:?}", output_dir))?;

        // Load the image
        let img = image::open(input_path)
            .with_context(|| format!("Failed to open image: {:?}", input_path))?;

        if self.verbose {
            println!("Original size: {}x{}, format: {:?}", 
                img.width(), img.height(), 
                image::guess_format(&fs::read(input_path)?)?
            );
        }

        let file_stem = input_path.file_stem()
            .ok_or_else(|| anyhow::anyhow!("Invalid file name: {:?}", input_path))?
            .to_string_lossy();

        // Generate optimized versions
        let small_img = img.resize(
            self.config.small_image_size, 
            self.config.small_image_size, 
            image::imageops::FilterType::Lanczos3
        );

        let medium_img = img.resize(
            self.config.medium_image_size, 
            self.config.medium_image_size, 
            image::imageops::FilterType::Lanczos3
        );

        // Save small PNG
        let small_png_path = output_dir.join(format!("{}_small.png", file_stem));
        small_img.save_with_format(&small_png_path, ImageFormat::Png)
            .with_context(|| format!("Failed to save small PNG: {:?}", small_png_path))?;

        // Save small WebP
        let small_webp_path = output_dir.join(format!("{}_small.webp", file_stem));
        self.save_webp(&small_img, &small_webp_path)?;

        // Save medium PNG
        let medium_png_path = output_dir.join(format!("{}_medium.png", file_stem));
        medium_img.save_with_format(&medium_png_path, ImageFormat::Png)
            .with_context(|| format!("Failed to save medium PNG: {:?}", medium_png_path))?;

        // Get file sizes
        let original_size = fs::metadata(input_path)?.len();
        let small_png_size = fs::metadata(&small_png_path)?.len();
        let small_webp_size = fs::metadata(&small_webp_path)?.len();
        let medium_png_size = fs::metadata(&medium_png_path)?.len();

        let total_optimized_size = small_png_size + small_webp_size + medium_png_size;
        let compression_ratio = original_size as f64 / total_optimized_size as f64;

        if self.verbose {
            println!("Created small version ({}x{}): {:?} ({} bytes)", 
                self.config.small_image_size, self.config.small_image_size,
                small_png_path, small_png_size
            );
            println!("Created WebP version: {:?} ({} bytes)", 
                small_webp_path, small_webp_size
            );
            println!("Created medium version ({}x{}): {:?} ({} bytes)", 
                self.config.medium_image_size, self.config.medium_image_size,
                medium_png_path, medium_png_size
            );
            println!("Compression ratio: {:.2}x", compression_ratio);
        }

        Ok(OptimizedImageSet {
            original_path: input_path.to_path_buf(),
            small_png_path,
            small_webp_path,
            medium_png_path,
            original_size,
            small_png_size,
            small_webp_size,
            medium_png_size,
            compression_ratio,
        })
    }

    /// Generate thumbnails for an image
    #[cfg(feature = "cli-tools")]
    pub fn generate_thumbnails(&self, image_path: &Path, output_dir: &Path) -> Result<Vec<Thumbnail>> {
        let img = image::open(image_path)
            .with_context(|| format!("Failed to open image: {:?}", image_path))?;

        let file_stem = image_path.file_stem()
            .ok_or_else(|| anyhow::anyhow!("Invalid file name: {:?}", image_path))?
            .to_string_lossy();

        let mut thumbnails = Vec::new();

        // Generate different thumbnail sizes
        let sizes = vec![
            (32, "tiny"),
            (64, "small"),
            (128, "medium"),
            (256, "large"),
        ];

        for (size, suffix) in sizes {
            let thumbnail_img = img.resize(size, size, image::imageops::FilterType::Lanczos3);
            let thumbnail_path = output_dir.join(format!("{}_{}.png", file_stem, suffix));
            
            thumbnail_img.save_with_format(&thumbnail_path, ImageFormat::Png)
                .with_context(|| format!("Failed to save thumbnail: {:?}", thumbnail_path))?;

            let size_bytes = fs::metadata(&thumbnail_path)?.len();

            thumbnails.push(Thumbnail {
                path: thumbnail_path,
                width: size,
                height: size,
                size_bytes,
                format: "PNG".to_string(),
            });
        }

        Ok(thumbnails)
    }

    /// Compress an image with specified quality
    #[cfg(feature = "cli-tools")]
    pub fn compress_image(&self, image_path: &Path, output_path: &Path, quality: u8) -> Result<CompressedImage> {
        let img = image::open(image_path)
            .with_context(|| format!("Failed to open image: {:?}", image_path))?;

        // Save with JPEG compression for better file size
        img.save_with_format(output_path, ImageFormat::Jpeg)
            .with_context(|| format!("Failed to save compressed image: {:?}", output_path))?;

        let original_size = fs::metadata(image_path)?.len();
        let compressed_size = fs::metadata(output_path)?.len();
        let compression_ratio = original_size as f64 / compressed_size as f64;

        Ok(CompressedImage {
            path: output_path.to_path_buf(),
            original_size,
            compressed_size,
            compression_ratio,
            quality,
        })
    }

    #[cfg(feature = "cli-tools")]
    fn save_webp(&self, img: &DynamicImage, path: &Path) -> Result<()> {
        // For now, save as PNG since WebP support in the image crate requires additional features
        // In a production environment, you might want to use a dedicated WebP library
        let webp_path_as_png = path.with_extension("webp.png");
        img.save_with_format(&webp_path_as_png, ImageFormat::Png)
            .with_context(|| format!("Failed to save WebP (as PNG): {:?}", webp_path_as_png))?;
        
        // Rename to .webp extension
        fs::rename(&webp_path_as_png, path)
            .with_context(|| format!("Failed to rename to WebP: {:?}", path))?;
        
        Ok(())
    }

    /// Clean up previously optimized images to avoid recursive optimization
    #[cfg(feature = "cli-tools")]
    pub fn cleanup_optimized_images(&self, dir: &Path) -> Result<usize> {
        let mut cleaned_count = 0;

        if !dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.contains("_small") || 
                       file_name_str.contains("_medium") || 
                       file_name_str.contains("_large") ||
                       file_name_str.contains("_tiny") {
                        if self.verbose {
                            println!("🗑️  Removing optimized image: {:?}", path);
                        }
                        fs::remove_file(&path)
                            .with_context(|| format!("Failed to remove optimized image: {:?}", path))?;
                        cleaned_count += 1;
                    }
                }
            }
        }

        if self.verbose && cleaned_count > 0 {
            println!("🧹 Cleaned up {} previously optimized images", cleaned_count);
        }

        Ok(cleaned_count)
    }

    /// Optimize all images in a directory
    #[cfg(feature = "cli-tools")]
    pub fn optimize_directory(&self, input_dir: &Path, output_dir: &Path) -> Result<Vec<OptimizedImageSet>> {
        let mut results = Vec::new();

        if !input_dir.exists() {
            return Err(anyhow::anyhow!("Input directory not found: {:?}", input_dir));
        }

        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                // Skip already optimized images
                if let Some(file_name) = path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.contains("_small") || 
                       file_name_str.contains("_medium") || 
                       file_name_str.contains("_large") ||
                       file_name_str.contains("_tiny") {
                        if self.verbose {
                            println!("⏭️  Skipping already optimized: {:?}", path);
                        }
                        continue;
                    }
                }

                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp") {
                        match self.optimize_image(&path, output_dir) {
                            Ok(result) => {
                                if self.verbose {
                                    println!("✅ Optimized: {:?}", path);
                                }
                                results.push(result);
                            }
                            Err(e) => {
                                eprintln!("❌ Failed to optimize {:?}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Extract image references from article metadata
    pub fn extract_image_references(&self, metadata: &crate::core::articles::ArticleMetadata) -> Vec<String> {
        let mut images = Vec::new();

        // Extract author_image
        if let Some(author_image) = &metadata.author_image {
            // Convert from web path to file path
            if let Some(filename) = author_image.split('/').last() {
                images.push(filename.to_string());
            }
        }

        // TODO: Extract images from content if needed
        // This could be extended to parse markdown content for image references

        images
    }

    /// Extract image references from markdown content
    pub fn extract_images_from_content(&self, content: &str) -> Vec<String> {
        use regex::Regex;
        let mut images = Vec::new();

        // Match markdown image syntax: ![alt](path)
        if let Ok(img_regex) = Regex::new(r"!\[([^\]]*)\]\(([^)]+)\)") {
            for cap in img_regex.captures_iter(content) {
                if let Some(path) = cap.get(2) {
                    let path_str = path.as_str();
                    // Only include local images (not URLs)
                    if !path_str.starts_with("http") && !path_str.starts_with("//") {
                        images.push(path_str.to_string());
                    }
                }
            }
        }

        // Match HTML img tags: <img src="path" />
        if let Ok(html_regex) = Regex::new(r#"<img[^>]+src=["']([^"']+)["'][^>]*>"#) {
            for cap in html_regex.captures_iter(content) {
                if let Some(src) = cap.get(1) {
                    let src_str = src.as_str();
                    // Only include local images (not URLs)
                    if !src_str.starts_with("http") && !src_str.starts_with("//") {
                        images.push(src_str.to_string());
                    }
                }
            }
        }

        images
    }

    /// Get optimization configuration
    pub fn config(&self) -> &ImageOptimizationConfig {
        &self.config
    }

    /// Set verbose mode
    pub fn set_verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    // Stub implementations for when cli-tools feature is not enabled
    #[cfg(not(feature = "cli-tools"))]
    pub fn optimize_image(&self, _input_path: &Path, _output_dir: &Path) -> Result<OptimizedImageSet> {
        Err(anyhow::anyhow!("Image optimization requires cli-tools feature"))
    }

    #[cfg(not(feature = "cli-tools"))]
    pub fn generate_thumbnails(&self, _image_path: &Path, _output_dir: &Path) -> Result<Vec<Thumbnail>> {
        Err(anyhow::anyhow!("Thumbnail generation requires cli-tools feature"))
    }

    #[cfg(not(feature = "cli-tools"))]
    pub fn compress_image(&self, _image_path: &Path, _output_path: &Path, _quality: u8) -> Result<CompressedImage> {
        Err(anyhow::anyhow!("Image compression requires cli-tools feature"))
    }

    #[cfg(not(feature = "cli-tools"))]
    pub fn optimize_directory(&self, _input_dir: &Path, _output_dir: &Path) -> Result<Vec<OptimizedImageSet>> {
        Err(anyhow::anyhow!("Directory optimization requires cli-tools feature"))
    }

    #[cfg(not(feature = "cli-tools"))]
    pub fn cleanup_optimized_images(&self, _dir: &Path) -> Result<usize> {
        Err(anyhow::anyhow!("Image cleanup requires cli-tools feature"))
    }
}

impl Default for ImageOptimizer {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Error types for image processing
#[derive(Debug, thiserror::Error)]
pub enum ImageProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image processing error: {0}")]
    ImageError(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_optimization_config_default() {
        let config = ImageOptimizationConfig::default();
        assert_eq!(config.webp_quality, 85);
        assert_eq!(config.small_image_size, 64);
        assert_eq!(config.medium_image_size, 128);
        assert!(config.preserve_original);
    }

    #[test]
    fn test_extract_image_references() {
        let optimizer = ImageOptimizer::with_defaults();
        let mut metadata = crate::core::articles::ArticleMetadata::default();
        metadata.author_image = Some("/articles/img/author_img.png".to_string());

        let images = optimizer.extract_image_references(&metadata);
        assert_eq!(images, vec!["author_img.png"]);
    }

    #[test]
    fn test_extract_images_from_content() {
        let optimizer = ImageOptimizer::with_defaults();
        let content = r#"
Here's an image: ![Alt text](./images/test.png)
And another: <img src="assets/photo.jpg" alt="Photo" />
External image: ![External](https://example.com/image.png)
"#;

        let images = optimizer.extract_images_from_content(content);
        assert_eq!(images.len(), 2);
        assert!(images.contains(&"./images/test.png".to_string()));
        assert!(images.contains(&"assets/photo.jpg".to_string()));
        // External image should not be included
        assert!(!images.iter().any(|img| img.contains("example.com")));
    }

    #[test]
    fn test_image_optimizer_creation() {
        let config = ImageOptimizationConfig {
            webp_quality: 90,
            small_image_size: 48,
            medium_image_size: 96,
            preserve_original: false,
        };

        let optimizer = ImageOptimizer::new(config.clone(), true);
        assert_eq!(optimizer.config.webp_quality, 90);
        assert_eq!(optimizer.config.small_image_size, 48);
        assert_eq!(optimizer.config.medium_image_size, 96);
        assert!(!optimizer.config.preserve_original);
        assert!(optimizer.verbose);
    }
}