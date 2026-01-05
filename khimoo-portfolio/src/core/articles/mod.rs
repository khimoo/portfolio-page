//! Articles processing module
//! 
//! This module contains core logic for article processing, metadata extraction,
//! and content management functionality.

pub mod metadata;
pub mod processor;
pub mod links;

// Re-export main components
pub use metadata::{ArticleMetadata, MetadataExtractor};
pub use processor::{ArticleProcessor, ValidationResult, ProcessingError};
pub use links::{ExtractedLink, LinkType, LinkExtractor, LinkValidator, ValidationReport, ProcessedArticleRef};