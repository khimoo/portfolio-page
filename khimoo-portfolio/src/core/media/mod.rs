//! Media processing module
//! 
//! This module contains core logic for media processing, image optimization,
//! and asset management functionality.

pub mod image_optimizer;

// Re-export main components
pub use image_optimizer::{
    ImageOptimizer, ImageOptimizationConfig, OptimizedImageSet, 
    Thumbnail, CompressedImage, ImageProcessingError
};