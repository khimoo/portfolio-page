pub mod extractor;
pub mod validator;
pub mod report_formatter;

// Re-export types
pub use extractor::{ExtractedLink, LinkType, LinkExtractor};
pub use validator::{
    LinkValidator, ValidationReport, ValidationError, ValidationWarning,
    ValidationErrorType, ValidationWarningType, ValidationSummary,
    ArticleValidationStats, ProcessedArticleRef
};
pub use report_formatter::ValidationReportFormatter;