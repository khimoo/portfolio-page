pub mod extractor;
pub mod validator;

// Re-export types
pub use extractor::{ExtractedLink, LinkExtractor, LinkType};
pub use validator::{
    LinkValidator, ProcessedArticleRef, ValidationError, ValidationErrorType, ValidationReport,
    ValidationSummary,
};
