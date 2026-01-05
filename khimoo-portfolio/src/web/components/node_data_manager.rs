use crate::web::data_loader::{ArticlesData, ProcessedArticle};
use crate::web::types::*;
use crate::config::get_config;
use std::collections::HashMap;

/// データ処理を担当するモジュール
pub struct NodeDataManager;

impl NodeDataManager {
    /// 記事の内容に基づいてNodeContentを決定
    pub fn determine_node_content(article: &ProcessedArticle) -> NodeContent {
        if let Some(image_url) = &article.metadata.author_image {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(
                &format!(
                    "Creating author node for article: '{}' with image: '{}'",
                    article.title, image_url
                )
                .into(),
            );

            // 最適化された中サイズ画像を使用
            let optimized_image_url = if image_url.starts_with("articles/") || image_url.starts_with("/articles/") {
                let optimized_path = image_url
                    .replace("articles/img/author_img.png", "articles/img/author_img_medium.png")
                    .replace("/articles/img/author_img.png", "/articles/img/author_img_medium.png");
                get_config().get_url(&optimized_path)
            } else {
                get_config().get_url(image_url)
            };

            NodeContent::Author {
                name: article.title.clone(),
                image_url: optimized_image_url,
                bio: None,
            }
        } else {
            NodeContent::Text(article.title.clone())
        }
    }

    /// ArticlesDataからNodeRegistryを生成
    pub fn create_node_registry_from_articles(
        articles_data: &ArticlesData,
        container_bound: &ContainerBound,
    ) -> (NodeRegistry, HashMap<NodeId, String>) {
        let mut reg = NodeRegistry::new();
        let mut slug_to_id = HashMap::new();
        let mut id_to_slug = HashMap::new();
        let mut next_id = 1u32;

        // コンテナの中心を計算
        let center_x = container_bound.width / 2.0;
        let center_y = container_bound.height / 2.0;

        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(
                &format!("Container bound in create_node_registry: {:?}", container_bound).into(),
            );
            web_sys::console::log_1(&format!("Calculated center: ({}, {})", center_x, center_y).into());
        }

        // home_display=trueの記事のみをノードとして追加
        let home_articles: Vec<_> = articles_data
            .articles
            .iter()
            .filter(|article| article.metadata.home_display)
            .collect();

        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(&format!("Total articles: {}", articles_data.articles.len()).into());
            web_sys::console::log_1(&format!("Home articles count: {}", home_articles.len()).into());
        }

        // home_articlesが空の場合はフォールバック
        if home_articles.is_empty() {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(&"No home articles found! Creating fallback author node".into());

            reg.add_node(
                NodeId(next_id),
                Position { x: center_x, y: center_y },
                40,
                NodeContent::Text("Author".to_string()),
            );
            slug_to_id.insert("author".to_string(), NodeId(next_id));
            id_to_slug.insert(NodeId(next_id), "author".to_string());
            return (reg, id_to_slug);
        }

        // 円形配置の計算
        let radius = (container_bound.width.min(container_bound.height) * 0.3).max(150.0);
        let angle_step = 2.0 * std::f32::consts::PI / home_articles.len() as f32;

        for (index, article) in home_articles.iter().enumerate() {
            let node_id = NodeId(next_id);
            let content = Self::determine_node_content(article);

            // 作者記事の場合は中央に配置
            let (position, base_radius) = if article.metadata.author_image.is_some() {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(
                    &format!("Placing author article '{}' at center", article.title).into(),
                );
                (Position { x: center_x, y: center_y }, 60)
            } else {
                let angle = index as f32 * angle_step;
                let x = center_x + radius * angle.cos();
                let y = center_y + radius * angle.sin();
                (Position { x, y }, 30)
            };

            reg.add_node(node_id, position, base_radius, content);
            reg.set_node_importance(node_id, article.metadata.importance);
            reg.set_node_inbound_count(node_id, article.inbound_links.len());

            slug_to_id.insert(article.slug.clone(), node_id);
            id_to_slug.insert(node_id, article.slug.clone());
            next_id += 1;
        }

        // 記事間のリンクを追加
        for article in &home_articles {
            if let Some(&from_id) = slug_to_id.get(&article.slug) {
                for link in &article.outbound_links {
                    if let Some(&to_id) = slug_to_id.get(&link.target_slug) {
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::log_1(
                            &format!(
                                "Adding edge: {} -> {} (IDs: {} -> {})",
                                article.slug, link.target_slug, from_id.0, to_id.0
                            )
                            .into(),
                        );
                        reg.add_edge(from_id, to_id);
                    }
                }
            }
        }

        (reg, id_to_slug)
    }
}