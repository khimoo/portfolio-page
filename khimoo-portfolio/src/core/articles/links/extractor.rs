use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Types of links that can be extracted from markdown content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkType {
    MarkdownLink, // [text](slug) format
    ExternalLink, // [text](http://...) format
}

/// Represents a link found in markdown content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtractedLink {
    pub target_slug: String,
    pub link_type: LinkType,
    pub original_text: String,
    pub display_text: Option<String>,
}

/// Link extractor for markdown content
/// Provides centralized link extraction and processing functionality
pub struct LinkExtractor {
    markdown_regex: Regex,
}

impl LinkExtractor {
    /// Create a new link extractor with compiled regex patterns
    pub fn new() -> Result<Self> {
        let markdown_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)")
            .context("Failed to compile markdown link regex")?;

        Ok(Self { markdown_regex })
    }

    /// Extract all internal links from markdown content
    pub fn extract_internal_links(&self, content: &str) -> Vec<ExtractedLink> {
        let mut links = Vec::new();

        // Extract internal markdown-style links [text](slug)
        for cap in self.markdown_regex.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let text = cap.get(1).unwrap().as_str();
            let target = cap.get(2).unwrap().as_str();

            // Only process internal links (not starting with http/https)
            if !target.starts_with("http")
                && !target.starts_with("mailto:")
                && !target.starts_with("//")
            {
                links.push(ExtractedLink {
                    target_slug: target.to_string(),
                    link_type: LinkType::MarkdownLink,
                    original_text: full_match.as_str().to_string(),
                    display_text: Some(text.to_string()),
                });
            }
        }

        links
    }

    /// Extract all external links from markdown content
    pub fn extract_external_links(&self, content: &str) -> Vec<ExtractedLink> {
        let mut links = Vec::new();

        // Extract external markdown-style links [text](http://...)
        for cap in self.markdown_regex.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let text = cap.get(1).unwrap().as_str();
            let target = cap.get(2).unwrap().as_str();

            // Only process external links
            if target.starts_with("http")
                || target.starts_with("mailto:")
                || target.starts_with("//")
            {
                links.push(ExtractedLink {
                    target_slug: target.to_string(),
                    link_type: LinkType::ExternalLink,
                    original_text: full_match.as_str().to_string(),
                    display_text: Some(text.to_string()),
                });
            }
        }

        links
    }

    /// Extract all links from markdown content (both internal and external)
    pub fn extract_links(&self, content: &str) -> Vec<ExtractedLink> {
        let mut links = self.extract_internal_links(content);
        links.extend(self.extract_external_links(content));
        links
    }

    /// Validate link format
    pub fn validate_link_format(&self, link: &ExtractedLink) -> Result<()> {
        match link.link_type {
            LinkType::MarkdownLink => {
                if link.target_slug.is_empty() {
                    return Err(anyhow::anyhow!("Markdown link target cannot be empty"));
                }
            }
            LinkType::ExternalLink => {
                if !link.target_slug.starts_with("http")
                    && !link.target_slug.starts_with("mailto:")
                    && !link.target_slug.starts_with("//")
                {
                    return Err(anyhow::anyhow!(
                        "External link must start with http, mailto:, or //"
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Default for LinkExtractor {
    fn default() -> Self {
        Self::new().expect("Failed to create default LinkExtractor")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_markdown_links() {
        let extractor = LinkExtractor::new().unwrap();
        let content =
            "Check out [this article](other-article) and [external](https://example.com).";

        let internal_links = extractor.extract_internal_links(content);
        let external_links = extractor.extract_external_links(content);

        assert_eq!(internal_links.len(), 1);
        assert_eq!(internal_links[0].target_slug, "other-article");
        assert_eq!(internal_links[0].link_type, LinkType::MarkdownLink);

        assert_eq!(external_links.len(), 1);
        assert_eq!(external_links[0].target_slug, "https://example.com");
        assert_eq!(external_links[0].link_type, LinkType::ExternalLink);
    }

    #[test]
    fn test_validate_link_format() {
        let extractor = LinkExtractor::new().unwrap();

        let valid_markdown = ExtractedLink {
            target_slug: "valid-target".to_string(),
            link_type: LinkType::MarkdownLink,
            original_text: "[text](valid-target)".to_string(),
            display_text: Some("text".to_string()),
        };
        assert!(extractor.validate_link_format(&valid_markdown).is_ok());

        let invalid_markdown = ExtractedLink {
            target_slug: "".to_string(),
            link_type: LinkType::MarkdownLink,
            original_text: "[text]()".to_string(),
            display_text: Some("text".to_string()),
        };
        assert!(extractor.validate_link_format(&invalid_markdown).is_err());
    }
}
