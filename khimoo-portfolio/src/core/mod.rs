//! Core business logic module
//!
//! This module contains UI-independent business logic for article processing,
//! metadata extraction, link management, and media optimization.

pub mod articles;
pub mod media;

// Re-export commonly used items from articles
pub use articles::{
    ArticleMetadata, ArticleProcessor, ExtractedLink, LinkExtractor, LinkType, LinkValidator,
    MetadataExtractor, ProcessedArticleRef, ProcessingError, ValidationReport,
};

// Re-export commonly used items from media
pub use media::{
    CompressedImage, ImageOptimizationConfig, ImageOptimizer, ImageProcessingError,
    OptimizedImageSet, Thumbnail,
};
