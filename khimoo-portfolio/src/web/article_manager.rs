use std::collections::HashMap;
use yew::prelude::*;

use crate::web::data_loader::{
    ArticlesData, DataLoadError, DataLoader, LightweightArticle, ProcessedArticle,
};
use crate::web::types::data_types::NodeRegistry;
use crate::web::types::node_types::{NodeId, AUTHOR_NODE_ID};

/// ArticleManager provides unified management of article data and node data
/// Integrates core module data with web-specific requirements
/// Requirements: 3.3 - Article data and node data unified management
#[derive(Debug, Clone)]
pub struct ArticleManager {
    /// Full article data (loaded on demand for performance)
    articles: HashMap<String, ProcessedArticle>,
    /// Lightweight article data (always loaded for performance)
    lightweight_articles: HashMap<String, LightweightArticle>,
    /// Articles to display on home screen
    home_articles: Vec<String>,
    /// Link graph for navigation (slug -> connected slugs)
    link_graph: HashMap<String, Vec<String>>,
    /// Mapping between article slugs and node IDs
    slug_to_node_id: HashMap<String, NodeId>,
    /// Reverse mapping for node ID to slug lookup
    node_id_to_slug: HashMap<NodeId, String>,
    /// Next available node ID
    next_node_id: u32,
    /// Content cache for full markdown content
    content_cache: HashMap<String, String>,
    /// Node registry for physics system integration
    node_registry: Option<NodeRegistry>,
}

impl ArticleManager {
    /// Create a new ArticleManager instance
    pub fn new() -> Self {
        Self {
            articles: HashMap::new(),
            lightweight_articles: HashMap::new(),
            home_articles: Vec::new(),
            link_graph: HashMap::new(),
            slug_to_node_id: HashMap::new(),
            node_id_to_slug: HashMap::new(),
            next_node_id: 1, // Reserve 0 for author node
            content_cache: HashMap::new(),
            node_registry: None,
        }
    }

    /// Load data from ArticlesData (from core module via CLI)
    /// Requirements: 3.3 - Unified management of article and node data
    pub fn load_from_data(&mut self, articles_data: ArticlesData) {
        // Clear existing data
        self.clear_data();

        // Build a set of all article slugs for link validation
        let all_slugs: std::collections::HashSet<String> = articles_data
            .articles
            .iter()
            .map(|a| a.slug.clone())
            .collect();

        // Load articles and create lightweight versions
        for article in articles_data.articles {
            let lightweight = LightweightArticle::from(article.clone());
            self.lightweight_articles
                .insert(article.slug.clone(), lightweight);

            // Only keep full articles for home articles in memory initially (performance optimization)
            if articles_data.home_articles.contains(&article.slug) {
                self.articles.insert(article.slug.clone(), article.clone());
            }

            // Build link graph from outbound_links
            let connections: Vec<String> = article
                .outbound_links
                .iter()
                .filter(|link| all_slugs.contains(&link.target_slug))
                .map(|link| link.target_slug.clone())
                .collect();
            if !connections.is_empty() {
                self.link_graph.insert(article.slug.clone(), connections);
            }
        }

        self.home_articles = articles_data.home_articles;
        self.assign_node_ids();
    }

    /// Load lightweight data only (for initial page load performance optimization)
    /// Requirements: 3.3 - Performance optimization implementation
    pub fn load_lightweight_data(&mut self, lightweight_articles: Vec<LightweightArticle>) {
        self.clear_data();

        // Build a set of all article slugs for link validation
        let all_slugs: std::collections::HashSet<String> = lightweight_articles
            .iter()
            .map(|a| a.slug.clone())
            .collect();

        // Load lightweight articles and build link graph
        for article in lightweight_articles {
            if article.metadata.home_display {
                self.home_articles.push(article.slug.clone());
            }

            // Build link graph from inbound_links (pre-calculated for performance)
            if !article.inbound_links.is_empty() {
                let connections: Vec<String> = article
                    .inbound_links
                    .iter()
                    .filter(|slug| all_slugs.contains(slug.as_str()))
                    .cloned()
                    .collect();
                if !connections.is_empty() {
                    self.link_graph.insert(article.slug.clone(), connections);
                }
            }

            self.lightweight_articles
                .insert(article.slug.clone(), article);
        }

        self.assign_node_ids();
    }

    /// Set node registry for physics system integration
    /// Requirements: 3.3 - Article data and node data unified management
    pub fn set_node_registry(&mut self, registry: NodeRegistry) {
        self.node_registry = Some(registry);
    }

    /// Get node registry (if available)
    pub fn get_node_registry(&self) -> Option<&NodeRegistry> {
        self.node_registry.as_ref()
    }

    /// Get mutable node registry (if available)
    pub fn get_node_registry_mut(&mut self) -> Option<&mut NodeRegistry> {
        self.node_registry.as_mut()
    }

    /// Get home articles (articles that should be displayed as nodes)
    pub fn get_home_articles(&self) -> Vec<&ProcessedArticle> {
        self.home_articles
            .iter()
            .filter_map(|slug| self.articles.get(slug))
            .collect()
    }

    /// Get home article slugs
    pub fn get_home_article_slugs(&self) -> Vec<String> {
        self.home_articles.clone()
    }

    /// Get home articles as lightweight data
    pub fn get_home_articles_lightweight(&self) -> Vec<&LightweightArticle> {
        self.home_articles
            .iter()
            .filter_map(|slug| self.lightweight_articles.get(slug))
            .collect()
    }

    /// Get related articles for a given article slug
    pub fn get_related_articles(&self, slug: &str) -> Vec<&ProcessedArticle> {
        let mut related = Vec::new();

        // Get directly linked articles from link graph
        if let Some(connections) = self.link_graph.get(slug) {
            for connected_slug in connections {
                if let Some(article) = self.articles.get(connected_slug) {
                    related.push(article);
                }
            }
        }

        // Also include articles from metadata related_articles
        if let Some(article) = self.articles.get(slug) {
            for related_slug in &article.metadata.related_articles {
                if let Some(related_article) = self.articles.get(related_slug) {
                    if !related.iter().any(|a| a.slug == related_article.slug) {
                        related.push(related_article);
                    }
                }
            }
        }

        related
    }

    /// Get related articles as lightweight data
    pub fn get_related_articles_lightweight(&self, slug: &str) -> Vec<&LightweightArticle> {
        let mut related = Vec::new();

        // Get directly linked articles from link graph
        if let Some(connections) = self.link_graph.get(slug) {
            for connected_slug in connections {
                if let Some(article) = self.lightweight_articles.get(connected_slug) {
                    related.push(article);
                }
            }
        }

        // Also include articles from metadata related_articles
        if let Some(article) = self.lightweight_articles.get(slug) {
            for related_slug in &article.metadata.related_articles {
                if let Some(related_article) = self.lightweight_articles.get(related_slug) {
                    if !related.iter().any(|a| a.slug == related_article.slug) {
                        related.push(related_article);
                    }
                }
            }
        }

        related
    }

    /// Get article by slug (returns full article if cached, otherwise None)
    pub fn get_article(&self, slug: &str) -> Option<&ProcessedArticle> {
        self.articles.get(slug)
    }

    /// Get lightweight article by slug
    pub fn get_lightweight_article(&self, slug: &str) -> Option<&LightweightArticle> {
        self.lightweight_articles.get(slug)
    }

    /// Check if full article content is loaded
    pub fn is_article_loaded(&self, slug: &str) -> bool {
        self.articles.contains_key(slug)
    }

    /// Cache full article content (performance optimization)
    pub fn cache_article(&mut self, article: ProcessedArticle) {
        self.articles.insert(article.slug.clone(), article);
    }

    /// Get cached content or None if not cached
    pub fn get_cached_content(&self, slug: &str) -> Option<&String> {
        self.content_cache.get(slug)
    }

    /// Cache content separately (for memory efficiency)
    pub fn cache_content(&mut self, slug: String, content: String) {
        self.content_cache.insert(slug, content);
    }

    /// Get all articles (only loaded ones)
    pub fn get_all_articles(&self) -> Vec<&ProcessedArticle> {
        self.articles.values().collect()
    }

    /// Get all lightweight articles
    pub fn get_all_lightweight_articles(&self) -> Vec<&LightweightArticle> {
        self.lightweight_articles.values().collect()
    }

    /// Get node ID for a given article slug
    pub fn get_node_id(&self, slug: &str) -> Option<NodeId> {
        self.slug_to_node_id.get(slug).copied()
    }

    /// Get article slug for a given node ID
    pub fn get_article_slug(&self, node_id: NodeId) -> Option<&String> {
        self.node_id_to_slug.get(&node_id)
    }

    /// Check if an article should be displayed on home screen
    pub fn is_home_article(&self, slug: &str) -> bool {
        self.home_articles.contains(&slug.to_string())
    }

    /// Get connections for a given article slug
    pub fn get_connections(&self, slug: &str) -> Vec<String> {
        self.link_graph.get(slug).cloned().unwrap_or_default()
    }

    /// Update node sizes based on current data (for dynamic updates)
    /// Requirements: 3.3 - Performance optimization implementation
    pub fn update_node_sizes(&self) {
        if let Some(registry) = &self.node_registry {
            // Update author node (always fixed size)
            let mut registry = registry.clone();
            registry.update_node_radius(AUTHOR_NODE_ID, registry.node_config.author_node_radius);

            // Update article nodes
            for (slug, node_id) in &self.slug_to_node_id {
                if let Some(article) = self.lightweight_articles.get(slug) {
                    let new_radius = registry.calculate_dynamic_radius(
                        *node_id,
                        Some(article.metadata.importance),
                        article.inbound_links.len(),
                    );
                    registry.update_node_radius(*node_id, new_radius);
                }
            }
        }
    }

    /// Get lightweight articles by category
    pub fn get_lightweight_articles_by_category(&self, category: &str) -> Vec<&LightweightArticle> {
        self.lightweight_articles
            .values()
            .filter(|article| {
                article
                    .metadata
                    .category
                    .as_ref()
                    .map_or(false, |cat| cat == category)
            })
            .collect()
    }

    /// Get articles by category (full articles only)
    pub fn get_articles_by_category(&self, category: &str) -> Vec<&ProcessedArticle> {
        self.articles
            .values()
            .filter(|article| {
                article
                    .metadata
                    .category
                    .as_ref()
                    .map_or(false, |cat| cat == category)
            })
            .collect()
    }

    /// Get all categories
    pub fn get_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self
            .lightweight_articles
            .values()
            .filter_map(|article| article.metadata.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        categories.sort();
        categories
    }

    /// Get lightweight articles by tag
    pub fn get_lightweight_articles_by_tag(&self, tag: &str) -> Vec<&LightweightArticle> {
        self.lightweight_articles
            .values()
            .filter(|article| article.metadata.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Get articles by tag (full articles only)
    pub fn get_articles_by_tag(&self, tag: &str) -> Vec<&ProcessedArticle> {
        self.articles
            .values()
            .filter(|article| article.metadata.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Get all tags
    pub fn get_all_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = self
            .lightweight_articles
            .values()
            .flat_map(|article| article.metadata.tags.iter().cloned())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        tags.sort();
        tags
    }

    /// Search lightweight articles by title
    pub fn search_lightweight_articles(&self, query: &str) -> Vec<&LightweightArticle> {
        let query_lower = query.to_lowercase();
        self.lightweight_articles
            .values()
            .filter(|article| article.title.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Search articles by title (full articles only)
    pub fn search_articles(&self, query: &str) -> Vec<&ProcessedArticle> {
        let query_lower = query.to_lowercase();
        self.articles
            .values()
            .filter(|article| article.title.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Get statistics
    pub fn get_stats(&self) -> ArticleStats {
        let total_articles = self.lightweight_articles.len();
        let home_articles_count = self.home_articles.len();
        let total_connections = self.link_graph.values().map(|v| v.len()).sum();
        let categories_count = self.get_categories().len();
        let tags_count = self.get_all_tags().len();

        ArticleStats {
            total_articles,
            home_articles_count,
            total_connections,
            categories_count,
            tags_count,
        }
    }

    /// Clear all data
    fn clear_data(&mut self) {
        self.articles.clear();
        self.lightweight_articles.clear();
        self.home_articles.clear();
        self.link_graph.clear();
        self.slug_to_node_id.clear();
        self.node_id_to_slug.clear();
        self.next_node_id = 1;
        self.content_cache.clear();
        self.node_registry = None;
    }

    /// Assign node IDs to home articles
    fn assign_node_ids(&mut self) {
        self.next_node_id = 1; // Reserve 0 for author node
        for slug in &self.home_articles {
            if self.lightweight_articles.contains_key(slug) {
                let node_id = NodeId(self.next_node_id);
                self.slug_to_node_id.insert(slug.clone(), node_id);
                self.node_id_to_slug.insert(node_id, slug.clone());
                self.next_node_id += 1;
            }
        }
    }
}

impl Default for ArticleManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about articles
#[derive(Debug, Clone, PartialEq)]
pub struct ArticleStats {
    pub total_articles: usize,
    pub home_articles_count: usize,
    pub total_connections: usize,
    pub categories_count: usize,
    pub tags_count: usize,
}

/// Hook for using ArticleManager with data loading
/// Requirements: 3.3 - Unified management integration
#[hook]
pub fn use_article_manager() -> (
    UseStateHandle<Option<ArticleManager>>,
    UseStateHandle<bool>,
    UseStateHandle<Option<DataLoadError>>,
) {
    let manager = use_state(|| None);
    let loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let manager = manager.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let manager = manager.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let loader = DataLoader::new();
                match loader.load_articles().await {
                    Ok(articles_data) => {
                        let mut article_manager = ArticleManager::new();
                        article_manager.load_from_data(articles_data);

                        // Also load node registry for unified management
                        match loader.build_node_registry().await {
                            Ok(node_registry) => {
                                article_manager.set_node_registry(node_registry);
                            }
                            Err(e) => {
                                web_sys::console::warn_1(
                                    &format!("Failed to load node registry: {}", e).into(),
                                );
                            }
                        }

                        manager.set(Some(article_manager));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            });

            || {}
        });
    }

    (manager, loading, error)
}

/// Hook for using ArticleManager with lightweight data loading (faster initial load)
/// Requirements: 3.3 - Performance optimization implementation
#[hook]
pub fn use_lightweight_article_manager() -> (
    UseStateHandle<Option<ArticleManager>>,
    UseStateHandle<bool>,
    UseStateHandle<Option<DataLoadError>>,
) {
    let manager = use_state(|| None);
    let loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let manager = manager.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let manager = manager.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let loader = DataLoader::new();
                match loader.load_articles().await {
                    Ok(articles_data) => {
                        let lightweight_articles: Vec<LightweightArticle> = articles_data
                            .articles
                            .into_iter()
                            .map(LightweightArticle::from)
                            .collect();
                        let mut article_manager = ArticleManager::new();
                        article_manager.load_lightweight_data(lightweight_articles);

                        // Also load node registry for unified management
                        match loader.build_node_registry().await {
                            Ok(node_registry) => {
                                article_manager.set_node_registry(node_registry);
                            }
                            Err(e) => {
                                web_sys::console::warn_1(
                                    &format!("Failed to load node registry: {}", e).into(),
                                );
                            }
                        }

                        manager.set(Some(article_manager));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            });

            || {}
        });
    }

    (manager, loading, error)
}

/// Hook for lazy loading a specific article into the manager
/// Requirements: 3.3 - Performance optimization implementation
#[hook]
pub fn use_lazy_article_loader(
    manager: UseStateHandle<Option<ArticleManager>>,
    slug: Option<String>,
) -> (UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let loading = use_state(|| false);
    let error = use_state(|| None);

    {
        let manager = manager.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(slug.clone(), move |slug| {
            if let (Some(slug), Some(current_manager)) = (slug, (*manager).clone()) {
                // Check if article is already loaded
                if !current_manager.is_article_loaded(slug) {
                    let manager = manager.clone();
                    let loading = loading.clone();
                    let error = error.clone();
                    let slug = slug.clone();

                    loading.set(true);
                    error.set(None);

                    wasm_bindgen_futures::spawn_local(async move {
                        let loader = DataLoader::new();
                        match loader.load_article_by_slug(&slug).await {
                            Ok(article) => {
                                if let Some(mut current_manager) = (*manager).clone() {
                                    current_manager.cache_article(article);
                                    manager.set(Some(current_manager));
                                }
                                error.set(None);
                            }
                            Err(e) => {
                                error.set(Some(e));
                            }
                        }
                        loading.set(false);
                    });
                }
            }

            || {}
        });
    }

    (loading, error)
}
