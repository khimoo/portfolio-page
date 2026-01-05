// Component tests are only available for wasm32 target
// since components are part of the web module
#[cfg(target_arch = "wasm32")]
mod tests {
    use khimoo_portfolio::web::article_manager::ArticleManager;
    use khimoo_portfolio::web::data_loader::{ArticlesData, ProcessedArticle, ProcessedMetadata};

    fn create_test_article(slug: &str, title: &str, author_image: Option<String>) -> ProcessedArticle {
        ProcessedArticle {
            slug: slug.to_string(),
            title: title.to_string(),
            metadata: ProcessedMetadata {
                title: title.to_string(),
                home_display: true,
                category: None,
                importance: Some(3),
                related_articles: vec![],
                tags: vec![],
                created_at: None,
                updated_at: None,
                author_image,
            },
            file_path: format!("{}.md", slug),
            outbound_links: vec![],
            inbound_links: vec![],
            processed_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_article_manager_with_author_article() {
        let articles_data = ArticlesData {
            articles: vec![
                create_test_article("article1", "Article 1", None),
                create_test_article("author", "About Me", Some("/images/profile.jpg".to_string())),
                create_test_article("article2", "Article 2", None),
            ],
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            total_count: 3,
            home_articles: vec!["article1".to_string(), "author".to_string(), "article2".to_string()],
        };

        let mut manager = ArticleManager::new();
        manager.load_from_data(articles_data);

        // Verify articles are loaded
        assert_eq!(manager.get_all_lightweight_articles().len(), 3);
        assert_eq!(manager.get_home_article_slugs().len(), 3);
        
        // Verify author article is accessible
        let author_article = manager.get_lightweight_article("author");
        assert!(author_article.is_some());
        let author = author_article.unwrap();
        assert_eq!(author.title, "About Me");
        assert_eq!(author.metadata.author_image, Some("/images/profile.jpg".to_string()));
    }

    #[test]
    fn test_article_manager_without_author_article() {
        let articles_data = ArticlesData {
            articles: vec![
                create_test_article("article1", "Article 1", None),
                create_test_article("article2", "Article 2", None),
            ],
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            total_count: 2,
            home_articles: vec!["article1".to_string(), "article2".to_string()],
        };

        let mut manager = ArticleManager::new();
        manager.load_from_data(articles_data);

        // Verify articles are loaded
        assert_eq!(manager.get_all_lightweight_articles().len(), 2);
        assert_eq!(manager.get_home_article_slugs().len(), 2);
    }

    #[test]
    fn test_article_manager_empty() {
        let articles_data = ArticlesData {
            articles: vec![],
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            total_count: 0,
            home_articles: vec![],
        };

        let mut manager = ArticleManager::new();
        manager.load_from_data(articles_data);

        assert_eq!(manager.get_all_lightweight_articles().len(), 0);
        assert_eq!(manager.get_home_article_slugs().len(), 0);
    }

    #[test]
    fn test_article_manager_get_article_by_slug() {
        let articles_data = ArticlesData {
            articles: vec![
                create_test_article("test1", "Test 1", None),
                create_test_article("test2", "Test 2", None),
            ],
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            total_count: 2,
            home_articles: vec!["test1".to_string(), "test2".to_string()],
        };

        let mut manager = ArticleManager::new();
        manager.load_from_data(articles_data);

        let article = manager.get_lightweight_article("test1");
        assert!(article.is_some());
        assert_eq!(article.unwrap().title, "Test 1");

        let missing = manager.get_lightweight_article("nonexistent");
        assert!(missing.is_none());
    }
}

// Placeholder test for non-wasm32 targets
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_placeholder() {
    // Component tests are only available for wasm32 target
    assert!(true);
}
