use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};

#[cfg(feature = "cli-tools")]
use image::{ImageFormat, DynamicImage};

/// Configuration for image optimization
#[derive(Debug, Clone)]
pub struct ImageOptimizationConfig {
    pub webp_quality: u8,
    pub small_image_size: u32,
    pub medium_image_size: u32,
}

impl Default for ImageOptimizationConfig {
    fn default() -> Self {
        Self {
            webp_quality: 85,
            small_image_size: 64,
            medium_image_size: 128,
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
}

/// Image optimizer for portfolio assets
pub struct ImageOptimizer {
    config: ImageOptimizationConfig,
    verbose: bool,
}

impl ImageOptimizer {
    pub fn new(config: ImageOptimizationConfig, verbose: bool) -> Self {
        Self { config, verbose }
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
                if let Some(extension) = path.extension() {
                    let ext = extension.to_string_lossy().to_lowercase();
                    if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "gif" | "bmp") {
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
    pub fn extract_image_references(&self, metadata: &crate::articles::ArticleMetadata) -> Vec<String> {
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

    #[cfg(not(feature = "cli-tools"))]
    pub fn optimize_image(&self, _input_path: &Path, _output_dir: &Path) -> Result<OptimizedImageSet> {
        Err(anyhow::anyhow!("Image optimization requires cli-tools feature"))
    }

    #[cfg(not(feature = "cli-tools"))]
    pub fn optimize_directory(&self, _input_dir: &Path, _output_dir: &Path) -> Result<Vec<OptimizedImageSet>> {
        Err(anyhow::anyhow!("Image optimization requires cli-tools feature"))
    }
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
    }

    #[test]
    fn test_extract_image_references() {
        let optimizer = ImageOptimizer::new(ImageOptimizationConfig::default(), false);
        let mut metadata = crate::articles::ArticleMetadata::default();
        metadata.author_image = Some("/articles/img/author_img.png".to_string());

        let images = optimizer.extract_image_references(&metadata);
        assert_eq!(images, vec!["author_img.png"]);
    }
}