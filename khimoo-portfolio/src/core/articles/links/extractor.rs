use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use regex::Regex;

/// Types of links that can be extracted from markdown content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkType {
    WikiLink,      // [[article-name]] format
    MarkdownLink,  // [text](slug) format
    ExternalLink,  // [text](http://...) format
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
    wiki_regex: Regex,
    markdown_regex: Regex,
}

impl LinkExtractor {
    /// Create a new link extractor with compiled regex patterns
    pub fn new() -> Result<Self> {
        let wiki_regex = Regex::new(r"\[\[([^\]]+)\]\]")
            .context("Failed to compile wiki link regex")?;
        let markdown_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)")
            .context("Failed to compile markdown link regex")?;

        Ok(Self {
            wiki_regex,
            markdown_regex,
        })
    }

    /// Extract all internal links from markdown content
    pub fn extract_internal_links(&self, content: &str) -> Vec<ExtractedLink> {
        let mut links = Vec::new();

        // Extract wiki-style links [[article-name]] or [[target|display]]
        for cap in self.wiki_regex.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let inner = cap.get(1).unwrap().as_str();

            // '|' があれば左をリンクターゲット、右を表示テキストとして扱う
            let parts: Vec<&str> = inner.splitn(2, '|').collect();
            let link_target = parts[0].trim();
            let display_text = if parts.len() == 2 { 
                Some(parts[1].trim().to_string()) 
            } else { 
                None 
            };

            links.push(ExtractedLink {
                target_slug: self.generate_slug_from_title(link_target),
                link_type: LinkType::WikiLink,
                original_text: full_match.as_str().to_string(),
                display_text,
            });
        }

        // Extract internal markdown-style links [text](slug)
        for cap in self.markdown_regex.captures_iter(content) {
            let full_match = cap.get(0).unwrap();
            let text = cap.get(1).unwrap().as_str();
            let target = cap.get(2).unwrap().as_str();

            // Only process internal links (not starting with http/https)
            if !target.starts_with("http") && !target.starts_with("mailto:") && !target.starts_with("//") {
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
            if target.starts_with("http") || target.starts_with("mailto:") || target.starts_with("//") {
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

    /// Generate a slug from article title (for wiki links)
    fn generate_slug_from_title(&self, title: &str) -> String {
        let slug = title
            .to_lowercase()
            .trim()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .collect::<String>()
            .trim_matches('-')
            .to_string();

        // Replace multiple consecutive dashes with single dash
        let re = Regex::new(r"-+").unwrap();
        re.replace_all(&slug, "-").to_string()
    }

    /// Validate link format
    pub fn validate_link_format(&self, link: &ExtractedLink) -> Result<()> {
        match link.link_type {
            LinkType::WikiLink => {
                if link.target_slug.is_empty() {
                    return Err(anyhow::anyhow!("Wiki link target cannot be empty"));
                }
            }
            LinkType::MarkdownLink => {
                if link.target_slug.is_empty() {
                    return Err(anyhow::anyhow!("Markdown link target cannot be empty"));
                }
            }
            LinkType::ExternalLink => {
                if !link.target_slug.starts_with("http") && 
                   !link.target_slug.starts_with("mailto:") && 
                   !link.target_slug.starts_with("//") {
                    return Err(anyhow::anyhow!("External link must start with http, mailto:, or //"));
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
    fn test_extract_wiki_links() {
        let extractor = LinkExtractor::new().unwrap();
        let content = "This has [[simple-link]] and [[target|display text]].";
        
        let links = extractor.extract_internal_links(content);
        assert_eq!(links.len(), 2);
        
        assert_eq!(links[0].target_slug, "simple-link");
        assert_eq!(links[0].link_type, LinkType::WikiLink);
        assert_eq!(links[0].display_text, None);
        
        assert_eq!(links[1].target_slug, "target");
        assert_eq!(links[1].link_type, LinkType::WikiLink);
        assert_eq!(links[1].display_text, Some("display text".to_string()));
    }

    #[test]
    fn test_extract_markdown_links() {
        let extractor = LinkExtractor::new().unwrap();
        let content = "Check out [this article](other-article) and [external](https://example.com).";
        
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
    fn test_generate_slug_from_title() {
        let extractor = LinkExtractor::new().unwrap();
        
        assert_eq!(extractor.generate_slug_from_title("Simple Title"), "simple-title");
        assert_eq!(extractor.generate_slug_from_title("Title with Numbers 123"), "title-with-numbers-123");
        assert_eq!(extractor.generate_slug_from_title("Special!@#$%Characters"), "specialcharacters");
    }

    #[test]
    fn test_validate_link_format() {
        let extractor = LinkExtractor::new().unwrap();
        
        let valid_wiki = ExtractedLink {
            target_slug: "valid-target".to_string(),
            link_type: LinkType::WikiLink,
            original_text: "[[valid-target]]".to_string(),
            display_text: None,
        };
        assert!(extractor.validate_link_format(&valid_wiki).is_ok());
        
        let invalid_wiki = ExtractedLink {
            target_slug: "".to_string(),
            link_type: LinkType::WikiLink,
            original_text: "[[]]".to_string(),
            display_text: None,
        };
        assert!(extractor.validate_link_format(&invalid_wiki).is_err());
    }
}