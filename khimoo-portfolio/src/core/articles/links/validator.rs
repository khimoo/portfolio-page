use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::ExtractedLink;
use crate::core::articles::metadata::ArticleMetadata;

/// Validation error types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationErrorType {
    BrokenLink,
    InvalidRelatedArticle,
    MissingMetadata,
    InvalidMetadata,
    CircularReference,
    OrphanedArticle,
}

/// Represents a validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub error_type: ValidationErrorType,
    pub source_article: String,
    pub target_reference: String,
    pub context: Option<String>,
    pub line_number: Option<usize>,
    pub suggestion: Option<String>,
}

/// Summary statistics for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_articles: usize,
    pub total_links: usize,
    pub broken_links: usize,
    pub invalid_references: usize,
}

/// Complete validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub validation_date: String,
    pub summary: ValidationSummary,
    pub errors: Vec<ValidationError>,
}

/// Reference to a processed article for validation
#[derive(Debug, Clone)]
pub struct ProcessedArticleRef {
    pub slug: String,
    pub title: String,
    pub metadata: ArticleMetadata,
    pub outbound_links: Vec<ExtractedLink>,
    pub inbound_links: Vec<ExtractedLink>,
    pub file_path: String,
}

/// Link validation system
/// Provides centralized validation of internal links
pub struct LinkValidator {
    existing_articles: HashSet<String>,
    article_map: HashMap<String, ProcessedArticleRef>,
}

impl LinkValidator {
    /// Create a new link validator with article data
    pub fn new(articles: &[ProcessedArticleRef]) -> Self {
        let existing_articles: HashSet<String> = articles.iter().map(|a| a.slug.clone()).collect();

        let article_map: HashMap<String, ProcessedArticleRef> = articles
            .iter()
            .map(|a| (a.slug.clone(), a.clone()))
            .collect();

        Self {
            existing_articles,
            article_map,
        }
    }

    /// Validate all internal links across articles
    pub fn validate_internal_links(&self, links: &[ExtractedLink]) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for link in links {
            if !self.existing_articles.contains(&link.target_slug) {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::BrokenLink,
                    source_article: "unknown".to_string(), // Will be set by caller
                    target_reference: link.target_slug.clone(),
                    context: Some(format!("Link type: {:?}", link.link_type)),
                    line_number: None,
                    suggestion: None,
                });
            }
        }

        errors
    }

    /// Validate all articles and generate a comprehensive report
    pub fn validate_all(&self) -> Result<ValidationReport> {
        let mut errors = Vec::new();

        // Validate each article
        for article in self.article_map.values() {
            let article_errors = self.validate_article(article)?;
            errors.extend(article_errors);
        }

        // Generate summary statistics
        let summary = self.generate_summary(&errors);

        Ok(ValidationReport {
            validation_date: chrono::Utc::now().to_rfc3339(),
            summary,
            errors,
        })
    }

    /// Validate a single article
    fn validate_article(&self, article: &ProcessedArticleRef) -> Result<Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Validate outbound links
        for link in &article.outbound_links {
            if !self.existing_articles.contains(&link.target_slug) {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::BrokenLink,
                    source_article: article.slug.clone(),
                    target_reference: link.target_slug.clone(),
                    context: None,
                    line_number: None,
                    suggestion: None,
                });
            }
        }

        // Validate related_articles in metadata
        for related_slug in &article.metadata.related_articles {
            if !self.existing_articles.contains(related_slug) {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidRelatedArticle,
                    source_article: article.slug.clone(),
                    target_reference: related_slug.clone(),
                    context: Some("front matter related_articles".to_string()),
                    line_number: None,
                    suggestion: None,
                });
            }
        }

        Ok(errors)
    }

    /// Generate summary statistics
    fn generate_summary(&self, errors: &[ValidationError]) -> ValidationSummary {
        let total_articles = self.article_map.len();
        let total_links: usize = self
            .article_map
            .values()
            .map(|a| a.outbound_links.len())
            .sum();

        let broken_links = errors
            .iter()
            .filter(|e| matches!(e.error_type, ValidationErrorType::BrokenLink))
            .count();

        let invalid_references = errors
            .iter()
            .filter(|e| matches!(e.error_type, ValidationErrorType::InvalidRelatedArticle))
            .count();

        ValidationSummary {
            total_articles,
            total_links,
            broken_links,
            invalid_references,
        }
    }

    /// Get existing articles set
    pub fn existing_articles(&self) -> &HashSet<String> {
        &self.existing_articles
    }

    /// Get article map
    pub fn article_map(&self) -> &HashMap<String, ProcessedArticleRef> {
        &self.article_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::articles::metadata::ArticleMetadata;

    fn create_test_article(slug: &str, title: &str) -> ProcessedArticleRef {
        ProcessedArticleRef {
            slug: slug.to_string(),
            title: title.to_string(),
            metadata: ArticleMetadata::default(),
            outbound_links: Vec::new(),
            inbound_links: Vec::new(),
            file_path: format!("{}.md", slug),
        }
    }

    #[test]
    fn test_link_validator_creation() {
        let articles = vec![
            create_test_article("article1", "Article 1"),
            create_test_article("article2", "Article 2"),
        ];

        let validator = LinkValidator::new(&articles);
        assert_eq!(validator.existing_articles.len(), 2);
        assert!(validator.existing_articles.contains("article1"));
        assert!(validator.existing_articles.contains("article2"));
    }

    #[test]
    fn test_validate_internal_links() {
        let articles = vec![create_test_article("existing-article", "Existing Article")];

        let validator = LinkValidator::new(&articles);

        let links = vec![
            ExtractedLink {
                target_slug: "existing-article".to_string(),
                link_type: super::super::extractor::LinkType::MarkdownLink,
                original_text: "[existing article](existing-article)".to_string(),
                display_text: Some("existing article".to_string()),
            },
            ExtractedLink {
                target_slug: "missing-article".to_string(),
                link_type: super::super::extractor::LinkType::MarkdownLink,
                original_text: "[missing article](missing-article)".to_string(),
                display_text: Some("missing article".to_string()),
            },
        ];

        let errors = validator.validate_internal_links(&links);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target_reference, "missing-article");
    }
}
