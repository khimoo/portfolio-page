use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use chrono::DateTime;
use yaml_front_matter::{Document, YamlFrontMatter};

/// Article metadata structure with default values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticleMetadata {
    pub title: String,
    #[serde(default)]
    pub home_display: bool,
    pub category: Option<String>,
    #[serde(default = "default_importance")]
    pub importance: u8,
    #[serde(default)]
    pub related_articles: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub author_image: Option<String>,
}

impl Default for ArticleMetadata {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            home_display: false,
            category: None,
            importance: default_importance(),
            related_articles: Vec::new(),
            tags: Vec::new(),
            created_at: None,
            updated_at: None,
            author_image: None,
        }
    }
}

fn default_importance() -> u8 {
    3
}

/// Metadata extractor for article processing
pub struct MetadataExtractor;

impl MetadataExtractor {
    /// Create a new metadata extractor
    pub fn new() -> Self {
        Self
    }

    /// Parse front matter from markdown content using yaml-front-matter library
    /// Returns (metadata, remaining_content)
    pub fn extract_frontmatter(&self, content: &str) -> Result<(ArticleMetadata, String)> {
        // Try to parse with yaml-front-matter
        match YamlFrontMatter::parse(content) {
            Ok(Document { metadata, content: markdown_content }) => {
                // Parse metadata into ArticleMetadata struct
                let metadata: ArticleMetadata = serde_yaml::from_value(metadata)
                    .context("Failed to deserialize front matter metadata")?;

                Ok((metadata, markdown_content))
            }
            Err(_) => {
                // No front matter found, return default metadata and full content
                Ok((ArticleMetadata::default(), content.to_string()))
            }
        }
    }

    /// Extract title from markdown content (first H1 heading if no frontmatter title)
    pub fn extract_title(&self, content: &str) -> Option<String> {
        // Look for first H1 heading
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("# ") {
                return Some(trimmed[2..].trim().to_string());
            }
        }
        None
    }

    /// Extract tags from content (hashtags or frontmatter)
    pub fn extract_tags(&self, content: &str) -> Vec<String> {
        let mut tags = Vec::new();
        
        // Extract hashtags from content
        for word in content.split_whitespace() {
            if word.starts_with('#') && word.len() > 1 {
                let tag = word[1..].trim_end_matches(|c: char| !c.is_alphanumeric());
                if !tag.is_empty() {
                    tags.push(tag.to_lowercase());
                }
            }
        }
        
        tags.sort();
        tags.dedup();
        tags
    }

    /// Extract date from filename or content
    pub fn extract_date(&self, content: &str) -> Option<chrono::DateTime<chrono::Utc>> {
        // Look for date patterns in content
        use regex::Regex;
        
        let date_regex = Regex::new(r"(\d{4}-\d{2}-\d{2})").ok()?;
        if let Some(captures) = date_regex.captures(content) {
            if let Some(date_str) = captures.get(1) {
                if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d") {
                    return Some(naive_date.and_hms_opt(0, 0, 0)?.and_utc());
                }
            }
        }
        
        None
    }

    /// Validate metadata fields
    pub fn validate_metadata(&self, metadata: &ArticleMetadata) -> Result<()> {
        // Validate importance range
        if metadata.importance < 1 || metadata.importance > 5 {
            return Err(anyhow::anyhow!(
                "Importance must be between 1 and 5, got: {}",
                metadata.importance
            ));
        }

        // Validate title is not empty
        if metadata.title.trim().is_empty() {
            return Err(anyhow::anyhow!("Title cannot be empty"));
        }

        // Validate datetime formats if present
        if let Some(created_at) = &metadata.created_at {
            DateTime::parse_from_rfc3339(created_at)
                .context("Invalid created_at datetime format")?;
        }

        if let Some(updated_at) = &metadata.updated_at {
            DateTime::parse_from_rfc3339(updated_at)
                .context("Invalid updated_at datetime format")?;
        }

        Ok(())
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Legacy alias for backward compatibility
pub type FrontMatterParser = MetadataExtractor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter_with_metadata() {
        let extractor = MetadataExtractor::new();
        let content = r#"---
title: "Test Article"
importance: 4
tags: ["rust", "web"]
---

# Content here"#;

        let result = extractor.extract_frontmatter(content).unwrap();
        assert_eq!(result.0.title, "Test Article");
        assert_eq!(result.0.importance, 4);
        assert_eq!(result.0.tags, vec!["rust", "web"]);
        assert_eq!(result.1.trim(), "# Content here");
    }

    #[test]
    fn test_extract_frontmatter_without_metadata() {
        let extractor = MetadataExtractor::new();
        let content = "# Just content";

        let result = extractor.extract_frontmatter(content).unwrap();
        assert_eq!(result.0.title, "Untitled");
        assert_eq!(result.0.importance, 3);
        assert_eq!(result.1, "# Just content");
    }

    #[test]
    fn test_extract_title() {
        let extractor = MetadataExtractor::new();
        let content = "Some text\n# Main Title\nMore content";
        
        let title = extractor.extract_title(content);
        assert_eq!(title, Some("Main Title".to_string()));
    }

    #[test]
    fn test_extract_tags() {
        let extractor = MetadataExtractor::new();
        let content = "This is about #rust and #webdev. Also #programming!";
        
        let tags = extractor.extract_tags(content);
        assert_eq!(tags, vec!["programming", "rust", "webdev"]);
    }

    #[test]
    fn test_validate_metadata() {
        let extractor = MetadataExtractor::new();
        let mut metadata = ArticleMetadata::default();
        
        // Valid metadata
        assert!(extractor.validate_metadata(&metadata).is_ok());
        
        // Invalid importance
        metadata.importance = 10;
        assert!(extractor.validate_metadata(&metadata).is_err());
        
        // Empty title
        metadata.importance = 3;
        metadata.title = "".to_string();
        assert!(extractor.validate_metadata(&metadata).is_err());
    }
}