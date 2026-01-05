use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::core::articles::metadata::ArticleMetadata;
use super::ExtractedLink;

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

/// Validation warning types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ValidationWarningType {
    UnusedTag,
    LowImportanceWithManyLinks,
    HighImportanceWithFewLinks,
    MissingBacklinks,
    InconsistentCasing,
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

/// Represents a validation warning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub warning_type: ValidationWarningType,
    pub source_article: String,
    pub target_reference: Option<String>,
    pub context: Option<String>,
    pub suggestion: Option<String>,
}

/// Summary statistics for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub total_articles: usize,
    pub total_links: usize,
    pub broken_links: usize,
    pub invalid_references: usize,
    pub orphaned_articles: usize,
    pub circular_references: usize,
    pub articles_with_errors: usize,
    pub articles_with_warnings: usize,
}

/// Complete validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    pub validation_date: String,
    pub summary: ValidationSummary,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub article_stats: HashMap<String, ArticleValidationStats>,
}

/// Per-article validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleValidationStats {
    pub outbound_links: usize,
    pub inbound_links: usize,
    pub broken_outbound_links: usize,
    pub invalid_related_articles: usize,
    pub has_errors: bool,
    pub has_warnings: bool,
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
/// Provides centralized validation of internal and external links
pub struct LinkValidator {
    existing_articles: HashSet<String>,
    article_map: HashMap<String, ProcessedArticleRef>,
}

impl LinkValidator {
    /// Create a new link validator with article data
    pub fn new(articles: &[ProcessedArticleRef]) -> Self {
        let existing_articles: HashSet<String> = articles
            .iter()
            .map(|a| a.slug.clone())
            .collect();

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
                    suggestion: self.suggest_similar_article(&link.target_slug),
                });
            }
        }

        errors
    }

    /// Validate external links (basic format validation)
    pub fn validate_external_links(&self, links: &[ExtractedLink]) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        for link in links {
            // Basic URL format validation
            if !link.target_slug.starts_with("http") && 
               !link.target_slug.starts_with("mailto:") && 
               !link.target_slug.starts_with("//") {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::BrokenLink,
                    source_article: "unknown".to_string(),
                    target_reference: link.target_slug.clone(),
                    context: Some("Invalid external link format".to_string()),
                    line_number: None,
                    suggestion: Some("External links should start with http://, https://, mailto:, or //".to_string()),
                });
            }
        }

        errors
    }

    /// Validate all articles and generate a comprehensive report
    pub fn validate_all(&self) -> Result<ValidationReport> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let mut article_stats = HashMap::new();

        // Validate each article
        for article in self.article_map.values() {
            let (article_errors, article_warnings, stats) = self.validate_article(article)?;

            errors.extend(article_errors);
            warnings.extend(article_warnings);
            article_stats.insert(article.slug.clone(), stats);
        }

        // Generate summary statistics
        let summary = self.generate_summary(&errors, &warnings, &article_stats);

        Ok(ValidationReport {
            validation_date: chrono::Utc::now().to_rfc3339(),
            summary,
            errors,
            warnings,
            article_stats,
        })
    }

    /// Validate a single article
    fn validate_article(&self, article: &ProcessedArticleRef) -> Result<(Vec<ValidationError>, Vec<ValidationWarning>, ArticleValidationStats)> {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Validate outbound links
        let mut broken_outbound_links = 0;
        for link in &article.outbound_links {
            if !self.existing_articles.contains(&link.target_slug) {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::BrokenLink,
                    source_article: article.slug.clone(),
                    target_reference: link.target_slug.clone(),
                    context: None,
                    line_number: None,
                    suggestion: self.suggest_similar_article(&link.target_slug),
                });
                broken_outbound_links += 1;
            }
        }

        // Validate related_articles in metadata
        let mut invalid_related_articles = 0;
        for related_slug in &article.metadata.related_articles {
            if !self.existing_articles.contains(related_slug) {
                errors.push(ValidationError {
                    error_type: ValidationErrorType::InvalidRelatedArticle,
                    source_article: article.slug.clone(),
                    target_reference: related_slug.clone(),
                    context: Some("front matter related_articles".to_string()),
                    line_number: None,
                    suggestion: self.suggest_similar_article(related_slug),
                });
                invalid_related_articles += 1;
            }
        }

        // Check for circular references in related_articles
        for related_slug in &article.metadata.related_articles {
            if let Some(related_article) = self.article_map.get(related_slug) {
                if related_article.metadata.related_articles.contains(&article.slug) {
                    // This is actually a good thing (bidirectional relationship)
                    // But we could warn if there are too many circular references
                    continue;
                }
            }
        }

        // Calculate inbound links for this article
        let inbound_links = self.count_inbound_links(&article.slug);

        // Generate warnings based on article characteristics
        self.generate_article_warnings(article, inbound_links, &mut warnings);

        let stats = ArticleValidationStats {
            outbound_links: article.outbound_links.len(),
            inbound_links,
            broken_outbound_links,
            invalid_related_articles,
            has_errors: !errors.is_empty(),
            has_warnings: !warnings.is_empty(),
        };

        Ok((errors, warnings, stats))
    }

    /// Count inbound links to a specific article
    fn count_inbound_links(&self, target_slug: &str) -> usize {
        self.article_map
            .values()
            .map(|article| {
                article.outbound_links
                    .iter()
                    .filter(|link| link.target_slug == target_slug)
                    .count()
            })
            .sum()
    }

    /// Generate warnings for an article based on its characteristics
    fn generate_article_warnings(&self, article: &ProcessedArticleRef, inbound_links: usize, warnings: &mut Vec<ValidationWarning>) {
        // Warning: High importance but few inbound links
        if article.metadata.importance >= 4 && inbound_links < 2 {
            warnings.push(ValidationWarning {
                warning_type: ValidationWarningType::HighImportanceWithFewLinks,
                source_article: article.slug.clone(),
                target_reference: None,
                context: Some(format!("Importance: {}, Inbound links: {}", article.metadata.importance, inbound_links)),
                suggestion: Some("Consider adding more links to this important article or reducing its importance level".to_string()),
            });
        }

        // Warning: Low importance but many inbound links
        if article.metadata.importance <= 2 && inbound_links >= 5 {
            warnings.push(ValidationWarning {
                warning_type: ValidationWarningType::LowImportanceWithManyLinks,
                source_article: article.slug.clone(),
                target_reference: None,
                context: Some(format!("Importance: {}, Inbound links: {}", article.metadata.importance, inbound_links)),
                suggestion: Some("Consider increasing the importance level of this well-connected article".to_string()),
            });
        }

        // Warning: Article has no inbound or outbound links (orphaned)
        if article.outbound_links.is_empty() && inbound_links == 0 {
            warnings.push(ValidationWarning {
                warning_type: ValidationWarningType::MissingBacklinks,
                source_article: article.slug.clone(),
                target_reference: None,
                context: Some("No inbound or outbound links".to_string()),
                suggestion: Some("Consider adding links to/from this article to integrate it better".to_string()),
            });
        }
    }

    /// Suggest similar articles for broken links using simple string similarity
    fn suggest_similar_article(&self, broken_slug: &str) -> Option<String> {
        let mut best_match = None;
        let mut best_score = 0.0;

        for existing_slug in &self.existing_articles {
            let score = self.calculate_similarity(broken_slug, existing_slug);
            if score > best_score && score > 0.5 { // Threshold for suggestions
                best_score = score;
                best_match = Some(existing_slug.clone());
            }
        }

        best_match.map(|slug| format!("Did you mean '{}'?", slug))
    }

    /// Calculate simple string similarity (Jaccard similarity on character bigrams)
    fn calculate_similarity(&self, s1: &str, s2: &str) -> f64 {
        let bigrams1: HashSet<String> = s1.chars()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|w| format!("{}{}", w[0], w[1]))
            .collect();

        let bigrams2: HashSet<String> = s2.chars()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|w| format!("{}{}", w[0], w[1]))
            .collect();

        if bigrams1.is_empty() && bigrams2.is_empty() {
            return 1.0;
        }

        let intersection = bigrams1.intersection(&bigrams2).count();
        let union = bigrams1.union(&bigrams2).count();

        intersection as f64 / union as f64
    }

    /// Generate summary statistics
    fn generate_summary(&self, errors: &[ValidationError], warnings: &[ValidationWarning], article_stats: &HashMap<String, ArticleValidationStats>) -> ValidationSummary {
        let total_articles = self.article_map.len();
        let total_links: usize = article_stats.values().map(|s| s.outbound_links).sum();

        let broken_links = errors.iter()
            .filter(|e| matches!(e.error_type, ValidationErrorType::BrokenLink))
            .count();

        let invalid_references = errors.iter()
            .filter(|e| matches!(e.error_type, ValidationErrorType::InvalidRelatedArticle))
            .count();

        let orphaned_articles = warnings.iter()
            .filter(|w| matches!(w.warning_type, ValidationWarningType::MissingBacklinks))
            .count();

        let articles_with_errors = article_stats.values()
            .filter(|s| s.has_errors)
            .count();

        let articles_with_warnings = article_stats.values()
            .filter(|s| s.has_warnings)
            .count();

        ValidationSummary {
            total_articles,
            total_links,
            broken_links,
            invalid_references,
            orphaned_articles,
            circular_references: 0, // TODO: Implement circular reference detection
            articles_with_errors,
            articles_with_warnings,
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
        let articles = vec![
            create_test_article("existing-article", "Existing Article"),
        ];

        let validator = LinkValidator::new(&articles);
        
        let links = vec![
            ExtractedLink {
                target_slug: "existing-article".to_string(),
                link_type: super::super::extractor::LinkType::WikiLink,
                original_text: "[[existing-article]]".to_string(),
                display_text: None,
            },
            ExtractedLink {
                target_slug: "missing-article".to_string(),
                link_type: super::super::extractor::LinkType::WikiLink,
                original_text: "[[missing-article]]".to_string(),
                display_text: None,
            },
        ];

        let errors = validator.validate_internal_links(&links);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].target_reference, "missing-article");
    }

    #[test]
    fn test_calculate_similarity() {
        let articles = vec![];
        let validator = LinkValidator::new(&articles);
        
        let similarity = validator.calculate_similarity("test-article", "test-article");
        assert!((similarity - 1.0).abs() < 0.001);
        
        let similarity = validator.calculate_similarity("test-article", "test-other");
        assert!(similarity > 0.0 && similarity < 1.0);
    }
}