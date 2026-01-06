// Article manager tests are only available for wasm32 target
// since ArticleManager is part of the web module
#[cfg(target_arch = "wasm32")]
mod tests {
    use khimoo_portfolio::web::article_manager::ArticleManager;
    use khimoo_portfolio::web::data_loader::{ArticlesData, ProcessedArticle, ProcessedMetadata};

    fn create_test_article(
        slug: &str,
        title: &str,
        home_display: bool,
        importance: u8,
    ) -> ProcessedArticle {
        ProcessedArticle {
            slug: slug.to_string(),
            title: title.to_string(),
            metadata: ProcessedMetadata {
                title: title.to_string(),
                home_display,
                category: Some("test".to_string()),
                importance: Some(importance),
                related_articles: Vec::new(),
                tags: vec!["test".to_string()],
                created_at: None,
                updated_at: None,
                author_image: None,
            },
            file_path: format!("articles/{}.md", slug),
            outbound_links: Vec::new(),
            inbound_links: Vec::new(),
            processed_at: "2024-01-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_article_manager_creation() {
        let manager = ArticleManager::new();
        assert_eq!(manager.get_all_articles().len(), 0);
        assert_eq!(manager.get_home_article_slugs().len(), 0);
    }

    #[test]
    fn test_load_from_data() {
        let mut manager = ArticleManager::new();

        let articles_data = ArticlesData {
            articles: vec![
                create_test_article("test1", "Test Article 1", true, 4),
                create_test_article("test2", "Test Article 2", false, 2),
            ],
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            total_count: 2,
            home_articles: vec!["test1".to_string()],
        };

        manager.load_from_data(articles_data);

        assert_eq!(manager.get_all_articles().len(), 2);
        assert_eq!(manager.get_home_article_slugs().len(), 1);
        assert_eq!(manager.get_home_article_slugs()[0], "test1");
    }

    #[test]
    fn test_get_home_articles() {
        let mut manager = ArticleManager::new();

        let articles_data = ArticlesData {
            articles: vec![
                create_test_article("test1", "Test Article 1", true, 4),
                create_test_article("test2", "Test Article 2", false, 2),
            ],
            generated_at: "2024-01-01T00:00:00Z".to_string(),
            total_count: 2,
            home_articles: vec!["test1".to_string()],
        };

        manager.load_from_data(articles_data);

        let home_articles = manager.get_home_articles();
        assert_eq!(home_articles.len(), 1);
        assert_eq!(home_articles[0].slug, "test1");
    }
}

// Placeholder test for non-wasm32 targets
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_placeholder() {
    // Article manager tests are only available for wasm32 target
    assert!(true);
}
