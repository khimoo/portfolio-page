use serde::{Deserialize, Serialize};

use std::sync::OnceLock;

/// Node configuration structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeConfig {
    pub author_node_radius: i32,
    pub default_node_radius: i32,
    pub min_node_radius: i32,
    pub max_node_radius: i32,
    pub importance_multiplier: i32,
    pub inbound_link_multiplier: i32,
    pub default_importance: u8,
    pub physics_radius_multiplier_default: f32,
    pub physics_radius_multiplier_high_importance: f32,
    pub high_importance_threshold: u8,
}

impl NodeConfig {
}

impl NodeConfig {
    /// Get default node configuration values
    pub fn new() -> Self {
        Self {
            author_node_radius: 120,
            default_node_radius: 50,
            min_node_radius: 20,
            max_node_radius: 120,
            importance_multiplier: 10,
            inbound_link_multiplier: 20,
            default_importance: 3,
            physics_radius_multiplier_default: 1.2,
            physics_radius_multiplier_high_importance: 1.5,
            high_importance_threshold: 5,
        }
    }

    /// Get author node radius
    pub fn get_author_node_radius(&self) -> i32 {
        self.author_node_radius
    }

    /// Get default node radius
    pub fn get_default_node_radius(&self) -> i32 {
        self.default_node_radius
    }

    /// Get minimum node radius
    pub fn get_min_node_radius(&self) -> i32 {
        self.min_node_radius
    }

    /// Get maximum node radius
    pub fn get_max_node_radius(&self) -> i32 {
        self.max_node_radius
    }

    /// Get importance multiplier
    pub fn get_importance_multiplier(&self) -> i32 {
        self.importance_multiplier
    }

    /// Get inbound link multiplier
    pub fn get_inbound_link_multiplier(&self) -> i32 {
        self.inbound_link_multiplier
    }

    /// Get default importance
    pub fn get_default_importance(&self) -> u8 {
        self.default_importance
    }

    /// Get physics radius multiplier for default importance nodes
    pub fn get_physics_radius_multiplier_default(&self) -> f32 {
        self.physics_radius_multiplier_default
    }

    /// Get physics radius multiplier for high importance nodes
    pub fn get_physics_radius_multiplier_high_importance(&self) -> f32 {
        self.physics_radius_multiplier_high_importance
    }

    /// Get high importance threshold
    pub fn get_high_importance_threshold(&self) -> u8 {
        self.high_importance_threshold
    }

    /// Calculate node radius based on importance and inbound links
    pub fn calculate_node_radius(&self, importance: u8, inbound_links: usize) -> i32 {
        let base_radius = self.default_node_radius;
        let importance_bonus = (importance as i32) * self.importance_multiplier;
        let link_bonus = (inbound_links as i32) * self.inbound_link_multiplier;

        let calculated_radius = base_radius + importance_bonus + link_bonus;

        // Clamp to min/max bounds
        calculated_radius.clamp(self.min_node_radius, self.max_node_radius)
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Application configuration that handles environment-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub base_path: String,
    pub data_path: String,
    pub articles_path: String,
    pub assets_path: String,
    pub node_config: NodeConfig,
}

impl AppConfig {
    /// Create configuration based on current environment
    pub fn new() -> Self {
        let base_path = Self::detect_base_path();

        Self {
            data_path: format!("{base_path}/data"),
            articles_path: format!("{base_path}/articles"),
            assets_path: format!("{base_path}/assets"),
            base_path,
            node_config: NodeConfig::default(),
        }
    }

    /// Detect the correct base path based on environment
    fn detect_base_path() -> String {
        // Get deployment configuration from project.toml (only available in non-WASM environments)
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (github_pages_path, local_dev_path) = crate::config_loader::get_deployment_config();

            // Check if we're in debug mode (local development)
            if cfg!(debug_assertions) {
                return local_dev_path;
            }

            // Check window location for production
            if let Some(window) = web_sys::window() {
                // Check hostname for GitHub Pages
                if let Ok(hostname) = window.location().hostname() {
                    if hostname.contains("github.io") {
                        return github_pages_path;
                    }
                }

                if let Ok(pathname) = window.location().pathname() {
                    if pathname.starts_with(&format!("{}/", github_pages_path)) || pathname.contains(&github_pages_path)
                    {
                        return github_pages_path;
                    }
                }
            }

            // Default fallback
            local_dev_path
        }

        // WebAssembly fallback - use hardcoded values
        #[cfg(target_arch = "wasm32")]
        {
            // Check if we're in debug mode (local development)
            if cfg!(debug_assertions) {
                return String::new(); // Empty string for root path in dev
            }

            // Check window location for production
            if let Some(window) = web_sys::window() {
                // Check hostname for GitHub Pages
                if let Ok(hostname) = window.location().hostname() {
                    if hostname.contains("github.io") {
                        return "/portfolio-page".to_string();
                    }
                }

                if let Ok(pathname) = window.location().pathname() {
                    if pathname.starts_with("/portfolio-page/") || pathname.contains("/portfolio-page")
                    {
                        return "/portfolio-page".to_string();
                    }
                }
            }

            // Default fallback
            String::new()
        }
    }

    /// Get full URL for a resource path
    pub fn get_url(&self, resource_path: &str) -> String {
        let clean_path = resource_path.trim_start_matches('/');
        if self.base_path.is_empty() {
            format!("/{clean_path}")
        } else {
            format!("{}/{}", self.base_path, clean_path)
        }
    }

    /// Get data file URL
    pub fn data_url(&self, filename: &str) -> String {
        self.get_url(&format!("data/{filename}"))
    }

    /// Get article file URL
    pub fn article_url(&self, filepath: &str) -> String {
        // Remove any leading path components and keep only the filename
        let clean_path = filepath.trim_start_matches('/');

        // Extract just the filename from paths like "../content/articles/about-khimoo.md"
        let filename = if let Some(filename) = clean_path.split('/').next_back() {
            filename
        } else {
            clean_path
        };

        if self.base_path.is_empty() {
            format!("/articles/{filename}")
        } else {
            format!("{}/articles/{}", self.base_path, filename)
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Global configuration instance
static CONFIG: OnceLock<AppConfig> = OnceLock::new();

/// Get the global configuration instance
pub fn get_config() -> &'static AppConfig {
    CONFIG.get_or_init(AppConfig::new)
}
