#![cfg(not(target_arch = "wasm32"))]

use khimoo_portfolio::core::articles::metadata::MetadataExtractor;

#[test]
fn test_front_matter_tag_integration() {
    let content = r#"---
:title: "Rustでの非同期プログラミング"
:home_display: true
:category: "programming"
:importance: 4
:related_articles: ["tokio-basics", "async-patterns"]
:tags: ["rust", "async", "programming"]
:created_at: "2024-01-02T00:00:00Z"
:updated_at: "2024-01-02T00:00:00Z"
:---
:
:# Rustでの非同期プログラミング
:
:Rustにおける非同期プログラミングの基礎について説明します。
:
:## 基本概念
:
:非同期プログラミングを理解するには、まず[[tokio-basics]]を理解することから始めましょう。
:実用的な[パターン集](async-patterns)も参考になります。
:
:## 主要な特徴
:
:- Future trait
:- async/await構文
:- 非同期ランタイム
:
:[[hello]]の記事でも触れましたが、非同期処理は現代のWebアプリケーション開発において重要な概念です。
:"#;

    let content = content.replace("\n:", "\n");

    let extractor = MetadataExtractor::new();
    let (metadata, _markdown_content) = extractor.extract_frontmatter(&content).unwrap();
    assert_eq!(metadata.tags, vec!["rust", "async", "programming"]);
    assert_eq!(metadata.title, "Rustでの非同期プログラミング");
    assert_eq!(metadata.home_display, true);
    assert_eq!(metadata.category, Some("programming".to_string()));
    assert_eq!(metadata.importance, 4);
    assert_eq!(
        metadata.related_articles,
        vec!["tokio-basics", "async-patterns"]
    );
}

#[test]
fn test_front_matter_only_tags() {
    let content = r#"---
:title: "Test Article"
:tags: ["rust", "web", "programming"]
:---
:
:# Test Article
:
:This article covers various topics but tags are only managed in front matter.
:Content may contain words like rust and programming but they are not extracted as tags.
:"#;

    let content = content.replace("\n:", "\n");

    let extractor = MetadataExtractor::new();
    let (metadata, _) = extractor.extract_frontmatter(&content).unwrap();
    assert_eq!(metadata.tags, vec!["rust", "web", "programming"]);
    assert_eq!(metadata.title, "Test Article");
}

#[test]
fn test_metadata_extraction() {
    let content = r#"---
title: "Test Article"
home_display: true
category: "test"
importance: 3
tags: ["test"]
---

# Test Article

This is a test article."#;

    let extractor = MetadataExtractor::new();
    let (metadata, _) = extractor.extract_frontmatter(content).unwrap();

    assert_eq!(metadata.title, "Test Article");
    assert_eq!(metadata.home_display, true);
    assert_eq!(metadata.category, Some("test".to_string()));
    assert_eq!(metadata.importance, 3);
    assert_eq!(metadata.tags, vec!["test"]);
}
