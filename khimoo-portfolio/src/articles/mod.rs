pub mod metadata;
pub mod links;

#[cfg(feature = "cli-tools")]
pub mod image_optimizer;
pub mod processing;

// Re-export commonly used types
pub use metadata::{ArticleMetadata, FrontMatterParser};
pub use links::{ExtractedLink, LinkType, LinkExtractor, LinkValidator, ValidationReport, ProcessedArticleRef};
pub use processing::ArticleProcessor;