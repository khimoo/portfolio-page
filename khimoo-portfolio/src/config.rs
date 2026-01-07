use serde::{Deserialize, Serialize};

use std::sync::OnceLock;

/// Node configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub author_node_radius: i32,
    pub default_node_radius: i32,
    pub min_node_radius: i32,
    pub max_node_radius: i32,
    pub importance_multiplier: i32,
    pub inbound_link_multiplier: i32,
    pub default_importance: u8,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            author_node_radius: 60,
            default_node_radius: 30,
            min_node_radius: 20,
            max_node_radius: 80,
            importance_multiplier: 8,
            inbound_link_multiplier: 4,
            default_importance: 3,
        }
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
