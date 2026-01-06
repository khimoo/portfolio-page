//! Media processing module
//!
//! This module contains core logic for media processing, image optimization,
//! and asset management functionality.

pub mod image_optimizer;

// Re-export main components
pub use image_optimizer::{
    CompressedImage, ImageOptimizationConfig, ImageOptimizer, ImageProcessingError,
    OptimizedImageSet, Thumbnail,
};
