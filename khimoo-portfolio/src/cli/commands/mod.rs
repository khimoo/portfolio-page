//! CLI commands module
//! 
//! This module contains individual command implementations for
//! article processing, link validation, and other CLI operations.

#[cfg(feature = "cli-tools")]
pub mod process_articles;
#[cfg(feature = "cli-tools")]
pub mod validate_links;

// Re-export command implementations
#[cfg(feature = "cli-tools")]
pub use process_articles::{ProcessArticlesCommand, ProcessArticlesArgs, ProcessedArticle, ArticlesData};
#[cfg(feature = "cli-tools")]
pub use validate_links::{ValidateLinksCommand, ValidateLinksArgs};