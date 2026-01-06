use serde::{Deserialize, Serialize};

use std::sync::OnceLock;

/// Application configuration that handles environment-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub base_path: String,
    pub data_path: String,
    pub articles_path: String,
    pub assets_path: String,
}

impl AppConfig {
    /// Create configuration based on current environment
    pub fn new() -> Self {
        let base_path = Self::detect_base_path();

        Self {
            data_path: format!("{}/data", base_path),
            articles_path: format!("{}/articles", base_path),
            assets_path: format!("{}/assets", base_path),
            base_path,
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
            format!("/{}", clean_path)
        } else {
            format!("{}/{}", self.base_path, clean_path)
        }
    }

    /// Get data file URL
    pub fn data_url(&self, filename: &str) -> String {
        self.get_url(&format!("data/{}", filename))
    }

    /// Get article file URL
    pub fn article_url(&self, filepath: &str) -> String {
        // Remove any leading path components and keep only the filename
        let clean_path = filepath.trim_start_matches('/');

        // Extract just the filename from paths like "../content/articles/about-khimoo.md"
        let filename = if let Some(filename) = clean_path.split('/').last() {
            filename
        } else {
            clean_path
        };

        if self.base_path.is_empty() {
            format!("/articles/{}", filename)
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
    CONFIG.get_or_init(|| AppConfig::new())
}
