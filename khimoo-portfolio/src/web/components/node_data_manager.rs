use crate::config::get_config;
use crate::web::data_loader::{ArticlesData, ProcessedArticle};
use crate::web::types::*;
use crate::web::types::node_types::NodeNavigation;
use std::collections::HashMap;

/// 簡単な疑似乱数生成器（線形合同法）
struct SimpleRng {
    seed: u32,
}

impl SimpleRng {
    fn new(seed: u32) -> Self {
        Self { seed }
    }

    fn next(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed
    }

    fn next_f32(&mut self) -> f32 {
        (self.next() as f32) / (u32::MAX as f32)
    }

    fn next_range(&mut self, min: f32, max: f32) -> f32 {
        min + self.next_f32() * (max - min)
    }
}

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
            let optimized_image_url =
                if image_url.starts_with("articles/") || image_url.starts_with("/articles/") {
                    let optimized_path = image_url
                        .replace(
                            "articles/img/author_img.png",
                            "articles/img/author_img_medium.png",
                        )
                        .replace(
                            "/articles/img/author_img.png",
                            "/articles/img/author_img_medium.png",
                        );
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

    /// 記事情報からナビゲーション動作を決定
    fn determine_navigation(article: &ProcessedArticle) -> NodeNavigation {
        NodeNavigation::ShowArticle(article.slug.clone())
    }

    /// ArticlesDataからNodeRegistryを生成
    pub fn create_node_registry_from_articles(
        articles_data: &ArticlesData,
        container_bound: &ContainerBound,
    ) -> (NodeRegistry, HashMap<NodeId, NodeNavigation>) {
        let mut reg = NodeRegistry::new_with_config(get_config().node_config.clone());
        let mut slug_to_id = HashMap::new();
        // ID -> NodeNavigation のマップに変更
        let mut id_to_action = HashMap::new();
        let mut next_id = 1u32;

        // コンテナの中心を計算
        let center_x = container_bound.width / 2.0;
        let center_y = container_bound.height / 2.0;

        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(
                &format!(
                    "Container bound in create_node_registry: {:?}",
                    container_bound
                )
                .into(),
            );
            web_sys::console::log_1(
                &format!("Calculated center: ({}, {})", center_x, center_y).into(),
            );
        }

        // home_display=trueの記事のみをノードとして追加
        let home_articles: Vec<_> = articles_data
            .articles
            .iter()
            .filter(|article| article.metadata.home_display)
            .collect();

        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(
                &format!("Total articles: {}", articles_data.articles.len()).into(),
            );
            web_sys::console::log_1(
                &format!("Home articles count: {}", home_articles.len()).into(),
            );
        }

        // home_articlesが空の場合はフォールバック
        if home_articles.is_empty() {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(
                &"No home articles found! Creating fallback author node".into(),
            );

            reg.add_node(
                NodeId(next_id),
                Position {
                    x: center_x,
                    y: center_y,
                },
                get_config().node_config.default_node_radius,
                NodeContent::Text("Author".to_string()),
            );
            slug_to_id.insert("author".to_string(), NodeId(next_id));
            id_to_action.insert(NodeId(next_id), NodeNavigation::StayOnHome);
            return (reg, id_to_action);
        }

        // 疑似乱数生成器を初期化（記事数をシードに使用して再現性を保つ）
        let mut rng = SimpleRng::new(home_articles.len() as u32 * 42);
        let scatter_radius = 80.0; // 中心からの最大散らばり距離

        // 円形配置の計算を削除し、疑似乱数で少しバラけさせて配置
        for (index, article) in home_articles.iter().enumerate() {
            let node_id = NodeId(next_id);
            let content = Self::determine_node_content(article);

            // 疑似乱数で中心からの位置をずらす
            let (position, base_radius) = if article.metadata.author_image.is_some() {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(
                    &format!("Placing author article '{}' at center", article.title).into(),
                );
                // 作者ノードは中心に配置
                (
                    Position {
                        x: center_x,
                        y: center_y,
                    },
                    get_config().node_config.author_node_radius,
                )
            } else {
                // 記事ノードは中心から少しずらして配置
                let offset_x = rng.next_range(-scatter_radius, scatter_radius);
                let offset_y = rng.next_range(-scatter_radius, scatter_radius);
                (
                    Position {
                        x: center_x + offset_x,
                        y: center_y + offset_y,
                    },
                    get_config().node_config.default_node_radius,
                )
            };

            reg.add_node(node_id, position, base_radius, content);
            reg.set_node_importance(node_id, article.metadata.importance);
            reg.set_node_inbound_count(node_id, article.inbound_links.len());
            
            // 重要度とインバウンドリンク数に基づいて動的にサイズを計算・更新
            let dynamic_radius = reg.calculate_dynamic_radius(
                node_id,
                Some(article.metadata.importance),
                article.inbound_links.len(),
            );
            reg.update_node_radius(node_id, dynamic_radius);

            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(
                &format!(
                    "Node '{}': importance={}, inbound_links={}, base_radius={}, dynamic_radius={}",
                    article.title, article.metadata.importance, article.inbound_links.len(), base_radius, dynamic_radius
                )
                .into(),
            );

            slug_to_id.insert(article.slug.clone(), node_id);
            // ナビゲーションアクションを設定
            let action = Self::determine_navigation(article);
            id_to_action.insert(node_id, action);
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

        (reg, id_to_action)
    }
}
