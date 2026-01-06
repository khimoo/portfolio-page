#![cfg(not(target_arch = "wasm32"))]

use khimoo_portfolio::core::articles::links::extractor::{LinkExtractor, LinkType};
use khimoo_portfolio::core::articles::metadata::MetadataExtractor;
use khimoo_portfolio::core::articles::processor::ArticleProcessor;

#[test]
fn test_parse_complete_front_matter() {
    let content = r#"---
title: "Test Article"
home_display: true
category: "programming"
importance: 4
related_articles: ["article1", "article2"]
tags: ["rust", "test"]
created_at: "2024-01-01T00:00:00Z"
updated_at: "2024-01-02T00:00:00Z"
---

# Test Content

This is the markdown content.
"#;

    let extractor = MetadataExtractor::new();
    let (metadata, markdown) = extractor.extract_frontmatter(content).unwrap();

    assert_eq!(metadata.title, "Test Article");
    assert_eq!(metadata.home_display, true);
    assert_eq!(metadata.category, Some("programming".to_string()));
    assert_eq!(metadata.importance, 4);
    assert_eq!(metadata.related_articles, vec!["article1", "article2"]);
    assert_eq!(metadata.tags, vec!["rust", "test"]);

    assert!(markdown.trim().starts_with("# Test Content"));
}

#[test]
fn test_parse_minimal_front_matter() {
    let content = r#"---
title: "Minimal Article"
---

# Minimal Content
"#;

    let extractor = MetadataExtractor::new();
    let (metadata, markdown) = extractor.extract_frontmatter(content).unwrap();

    assert_eq!(metadata.title, "Minimal Article");
    assert_eq!(metadata.home_display, false); // default
    assert_eq!(metadata.category, None);
    assert_eq!(metadata.importance, 3); // default
    assert!(metadata.related_articles.is_empty());
    assert!(metadata.tags.is_empty());

    assert!(markdown.trim().starts_with("# Minimal Content"));
}

#[test]
fn test_parse_no_front_matter() {
    let content = "# Just Markdown\n\nNo front matter here.";

    let extractor = MetadataExtractor::new();
    let (metadata, markdown) = extractor.extract_frontmatter(content).unwrap();

    assert_eq!(metadata.title, "Untitled"); // default
    assert_eq!(metadata.home_display, false);
    assert_eq!(metadata.importance, 3);

    assert_eq!(markdown, content);
}

#[test]
fn test_extract_markdown_links() {
    let extractor = LinkExtractor::new().unwrap();
    let content = "Check out [this article](article-slug) and [another one](second-slug).";

    let links = extractor.extract_internal_links(content);

    assert_eq!(links.len(), 2);

    assert_eq!(links[0].target_slug, "article-slug");
    assert_eq!(links[0].link_type, LinkType::MarkdownLink);

    assert_eq!(links[1].target_slug, "second-slug");
    assert_eq!(links[1].link_type, LinkType::MarkdownLink);
}

#[test]
fn test_ignore_external_links() {
    let extractor = LinkExtractor::new().unwrap();
    let content = r#"
        Internal: [article](internal-slug)
        External: [website](https://example.com)
        Email: [contact](mailto:test@example.com)
        "#;

    let links = extractor.extract_internal_links(content);

    // Should only extract the internal link
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].target_slug, "internal-slug");
    assert_eq!(links[0].link_type, LinkType::MarkdownLink);
}

#[test]
fn test_article_processor_creation() {
    let _processor = ArticleProcessor::new();
    // Just verify it can be created
    assert!(true);
}
