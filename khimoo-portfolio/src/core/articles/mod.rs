//! Articles processing module
//!
//! This module contains core logic for article processing, metadata extraction,
//! and content management functionality.

pub mod links;
pub mod metadata;
pub mod processor;

// Re-export main components
pub use links::{
    ExtractedLink, LinkExtractor, LinkType, LinkValidator, ProcessedArticleRef, ValidationReport,
};
pub use metadata::{ArticleMetadata, MetadataExtractor};
pub use processor::{ArticleProcessor, ProcessingError};
