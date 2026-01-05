//! Core business logic module
//! 
//! This module contains UI-independent business logic for article processing,
//! metadata extraction, link management, and media optimization.

pub mod articles;
pub mod media;

// Re-export commonly used items from articles
pub use articles::{
    ArticleMetadata, MetadataExtractor, ArticleProcessor, ValidationResult, ProcessingError,
    ExtractedLink, LinkType, LinkExtractor, LinkValidator, ValidationReport, ProcessedArticleRef
};

// Re-export commonly used items from media
pub use media::{
    ImageOptimizer, ImageOptimizationConfig, OptimizedImageSet, 
    Thumbnail, CompressedImage, ImageProcessingError
};