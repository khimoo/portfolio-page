use anyhow::Result;
use std::path::Path;

use super::metadata::MetadataExtractor;
use super::links::{LinkExtractor, ProcessedArticleRef};

/// High-level article processing functionality
/// Provides UI-independent business logic for article processing
pub struct ArticleProcessor {
    metadata_extractor: MetadataExtractor,
    link_extractor: LinkExtractor,
}

impl ArticleProcessor {
    /// Create a new article processor
    pub fn new() -> Result<Self> {
        Ok(Self {
            metadata_extractor: MetadataExtractor::new(),
            link_extractor: LinkExtractor::new()?,
        })
    }

    /// Process a single article file and return processed article reference
    pub fn process_article(&self, file_path: &Path, content: &str) -> Result<ProcessedArticleRef> {
        // Parse front matter and content
        let (metadata, markdown_content) = self.metadata_extractor.extract_frontmatter(content)?;
        
        // Validate metadata
        self.metadata_extractor.validate_metadata(&metadata)?;
        
        // Extract links from content
        let outbound_links = self.link_extractor.extract_links(&markdown_content);
        
        // Generate slug from file path
        let slug = self.generate_slug_from_path(file_path);
        
        Ok(ProcessedArticleRef {
            slug,
            title: metadata.title.clone(),
            metadata,
            outbound_links,
            inbound_links: Vec::new(), // Will be populated later during validation
            file_path: file_path.to_string_lossy().to_string(),
        })
    }

    /// Process multiple articles from a directory
    pub fn process_all(&self, articles_dir: &Path) -> Result<Vec<ProcessedArticleRef>> {
        let mut articles = Vec::new();
        
        if !articles_dir.exists() {
            return Err(anyhow::anyhow!("Articles directory not found: {:?}", articles_dir));
        }

        // Read all markdown files in the directory
        for entry in std::fs::read_dir(articles_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let content = std::fs::read_to_string(&path)?;
                match self.process_article(&path, &content) {
                    Ok(article) => articles.push(article),
                    Err(e) => {
                        eprintln!("Warning: Failed to process {:?}: {}", path, e);
                    }
                }
            }
        }
        
        Ok(articles)
    }

    /// Validate a processed article
    pub fn validate_article(&self, article: &ProcessedArticleRef) -> Result<ValidationResult> {
        let mut issues = Vec::new();
        
        // Validate metadata
        if let Err(e) = self.metadata_extractor.validate_metadata(&article.metadata) {
            issues.push(format!("Metadata validation failed: {}", e));
        }
        
        // Check for empty content (basic validation)
        if article.title.trim().is_empty() {
            issues.push("Article has empty title".to_string());
        }
        
        // Check for broken internal links (basic check - full validation requires all articles)
        for link in &article.outbound_links {
            if link.target_slug.is_empty() {
                issues.push(format!("Empty link target: {}", link.original_text));
            }
        }
        
        Ok(ValidationResult {
            article_slug: article.slug.clone(),
            is_valid: issues.is_empty(),
            issues,
        })
    }

    /// Generate slug from file path
    fn generate_slug_from_path(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string()
    }

    /// Get metadata extractor for direct access
    pub fn metadata_extractor(&self) -> &MetadataExtractor {
        &self.metadata_extractor
    }

    /// Get link extractor for direct access
    pub fn link_extractor(&self) -> &LinkExtractor {
        &self.link_extractor
    }
}

impl Default for ArticleProcessor {
    fn default() -> Self {
        Self::new().expect("Failed to create default ArticleProcessor")
    }
}

/// Result of article validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub article_slug: String,
    pub is_valid: bool,
    pub issues: Vec<String>,
}

/// Error types for article processing
#[derive(Debug, thiserror::Error)]
pub enum ProcessingError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Metadata parsing error: {0}")]
    Metadata(#[from] anyhow::Error),
    #[error("Link extraction error: {0}")]
    LinkExtraction(String),
    #[error("Validation error: {0}")]
    Validation(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_process_article() {
        let processor = ArticleProcessor::new().unwrap();
        let content = r#"---
title: "Test Article"
importance: 4
---

# Test Article

This is a test article with a [[link-to-other]] and [markdown link](other-article).
"#;

        let path = PathBuf::from("test-article.md");
        let result = processor.process_article(&path, content).unwrap();
        
        assert_eq!(result.slug, "test-article");
        assert_eq!(result.title, "Test Article");
        assert_eq!(result.metadata.importance, 4);
        assert_eq!(result.outbound_links.len(), 2);
    }

    #[test]
    fn test_validate_article() {
        let processor = ArticleProcessor::new().unwrap();
        let content = r#"---
title: "Valid Article"
importance: 3
---

# Valid Article

Content here.
"#;

        let path = PathBuf::from("valid-article.md");
        let article = processor.process_article(&path, content).unwrap();
        let validation = processor.validate_article(&article).unwrap();
        
        assert!(validation.is_valid);
        assert!(validation.issues.is_empty());
    }

    #[test]
    fn test_generate_slug_from_path() {
        let processor = ArticleProcessor::new().unwrap();
        let path = PathBuf::from("/path/to/my-article.md");
        let slug = processor.generate_slug_from_path(&path);
        assert_eq!(slug, "my-article");
    }
}