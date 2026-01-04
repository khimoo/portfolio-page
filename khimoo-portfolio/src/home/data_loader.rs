use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;
use crate::config::{get_config, AppConfig};

// Data structures matching the generated JSON format
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessedArticle {
    pub slug: String,
    pub title: String,
    pub metadata: ProcessedMetadata,
    pub file_path: String,
    pub outbound_links: Vec<ProcessedLink>,
    pub inbound_links: Vec<ProcessedLink>,
    pub processed_at: String,
}

// Lightweight article data for list display (without full content)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightweightArticle {
    pub slug: String,
    pub title: String,
    pub summary: Option<String>, // First paragraph or excerpt
    pub metadata: ProcessedMetadata,
    pub file_path: String,
    pub outbound_links: Vec<ProcessedLink>,
    pub inbound_links: Vec<ProcessedLink>,
    pub processed_at: String,
}

impl From<ProcessedArticle> for LightweightArticle {
    fn from(article: ProcessedArticle) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            summary: None, // Summary will be loaded from file when needed
            metadata: article.metadata,
            file_path: article.file_path,
            outbound_links: article.outbound_links,
            inbound_links: article.inbound_links,
            processed_at: article.processed_at,
        }
    }
}

// Extract summary from article content
#[allow(dead_code)]
fn extract_summary(content: &str) -> String {
    // Remove markdown headers and get first paragraph
    let lines: Vec<&str> = content.lines().collect();
    let mut summary_lines = Vec::new();
    let mut found_content = false;

    for line in lines {
        let trimmed = line.trim();

        // Skip empty lines and headers at the beginning
        if trimmed.is_empty() || trimmed.starts_with('#') {
            if found_content {
                break; // Stop at first empty line or header after content
            }
            continue;
        }

        found_content = true;
        summary_lines.push(trimmed);

        // Stop after first paragraph or when we have enough content
        if summary_lines.join(" ").len() > 200 {
            break;
        }
    }

    let summary = summary_lines.join(" ");
    if summary.len() > 200 {
        format!("{}...", &summary[..197])
    } else {
        summary
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessedMetadata {
    pub title: String,
    pub home_display: bool,
    pub category: Option<String>,
    pub importance: Option<u8>,
    pub related_articles: Vec<String>,
    pub tags: Vec<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub author_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessedLink {
    pub target_slug: String,
    pub link_type: LinkType,
    pub original_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LinkType {
    WikiLink,
    MarkdownLink,
    TagReference,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ArticlesData {
    pub articles: Vec<ProcessedArticle>,
    pub generated_at: String,
    pub total_count: usize,
    pub home_articles: Vec<String>,
}


// Error types for data loading
#[derive(Debug, Clone, PartialEq)]
pub enum DataLoadError {
    NetworkError(String),
    ParseError(String),
    NotFound(String),
}

impl std::fmt::Display for DataLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataLoadError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            DataLoadError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DataLoadError::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for DataLoadError {}

// Simple front matter parser for WASM environment
struct SimpleFrontMatterParser;

impl SimpleFrontMatterParser {
    /// Parse front matter from markdown content
    /// Returns (remaining_content) - metadata is stripped
    fn parse_content_only(content: &str) -> String {
        let content = content.trim();
        
        // Check if content starts with front matter delimiter
        if content.starts_with("---") {
            let lines: Vec<&str> = content.lines().collect();
            
            // Find the closing delimiter
            let mut end_index = None;
            for (i, line) in lines.iter().enumerate().skip(1) {
                if line.trim() == "---" {
                    end_index = Some(i);
                    break;
                }
            }
            
            if let Some(end_idx) = end_index {
                // Return content after the closing delimiter
                let remaining_lines = &lines[end_idx + 1..];
                return remaining_lines.join("\n").trim_start().to_string();
            }
        }
        
        // No front matter found, return original content
        content.to_string()
    }
}

// DataLoader structure for loading static JSON files
#[derive(Debug, Clone)]
pub struct DataLoader {
    config: &'static AppConfig,
}

impl DataLoader {
    pub fn new() -> Self {
        Self { 
            config: get_config(),
        }
    }

    // Load articles data with error handling and fallback
    pub async fn load_articles(&self) -> Result<ArticlesData, DataLoadError> {
        let url = self.config.data_url("articles.json");

        // Debug logging
        web_sys::console::log_1(&format!("DataLoader: Loading articles from: {}", url).into());

        match self.fetch_json::<ArticlesData>(&url).await {
            Ok(data) => {
                web_sys::console::log_1(&format!("DataLoader: Successfully loaded {} articles", data.articles.len()).into());
                Ok(data)
            },
            Err(e) => {
                web_sys::console::warn_1(&format!("Failed to load articles data: {}", e).into());
                // Fallback to empty data structure
                Ok(ArticlesData {
                    articles: Vec::new(),
                    generated_at: "1970-01-01T00:00:00Z".to_string(),
                    total_count: 0,
                    home_articles: Vec::new(),
                })
            }
        }
    }


    // Load lightweight articles data (without full content)
    pub async fn load_lightweight_articles(&self) -> Result<Vec<LightweightArticle>, DataLoadError> {
        let articles_data = self.load_articles().await?;
        let lightweight_articles = articles_data.articles
            .into_iter()
            .map(LightweightArticle::from)
            .collect();
        Ok(lightweight_articles)
    }


    // Load full article content from file path
    pub async fn load_article_content(&self, file_path: &str) -> Result<String, DataLoadError> {
        let url = self.config.article_url(file_path);
        web_sys::console::log_1(&format!("DataLoader: Loading article content from: {}", url).into());

        // Fetch the markdown file
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts)
            .map_err(|e| DataLoadError::NetworkError(format!("Failed to create request: {:?}", e)))?;

        let window = web_sys::window()
            .ok_or_else(|| DataLoadError::NetworkError("No window object".to_string()))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| DataLoadError::NetworkError(format!("Fetch failed: {:?}", e)))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| DataLoadError::NetworkError(format!("Invalid response: {:?}", e)))?;

        web_sys::console::log_1(&format!("DataLoader: Response status: {} {}", resp.status(), resp.status_text()).into());

        if !resp.ok() {
            return Err(DataLoadError::NotFound(format!(
                "HTTP {}: {}",
                resp.status(),
                resp.status_text()
            )));
        }

        let text = JsFuture::from(resp.text().map_err(|e| {
            DataLoadError::ParseError(format!("Failed to get text: {:?}", e))
        })?)
        .await
        .map_err(|e| DataLoadError::ParseError(format!("Failed to parse text: {:?}", e)))?;

        let content = text.as_string()
            .ok_or_else(|| DataLoadError::ParseError("Response is not a string".to_string()))?;

        Ok(content)
    }

    // Load article content without front matter metadata (content only)
    pub async fn load_article_content_only(&self, file_path: &str) -> Result<String, DataLoadError> {
        let full_content = self.load_article_content(file_path).await?;
        
        // Parse and extract only the markdown content (without metadata)
        let content_only = SimpleFrontMatterParser::parse_content_only(&full_content);
        
        web_sys::console::log_1(&"DataLoader: Successfully separated content from metadata".into());
        Ok(content_only)
    }


    // Load article by slug (metadata only, content must be loaded separately)
    pub async fn load_article_by_slug(&self, slug: &str) -> Result<ProcessedArticle, DataLoadError> {
        web_sys::console::log_1(&format!("DataLoader: Looking for article with slug: {}", slug).into());

        let articles_data = self.load_articles().await?;

        web_sys::console::log_1(&format!("DataLoader: Loaded {} articles", articles_data.articles.len()).into());

        // Debug: log all available slugs
        for article in &articles_data.articles {
            web_sys::console::log_1(&format!("DataLoader: Available slug: '{}'", article.slug).into());
        }

        let found_article = articles_data.articles
            .into_iter()
            .find(|article| {
                web_sys::console::log_1(&format!("DataLoader: Comparing '{}' with '{}'", article.slug, slug).into());
                article.slug == slug
            });

        match found_article {
            Some(article) => {
                web_sys::console::log_1(&format!("DataLoader: Found article: {}", article.title).into());
                Ok(article)
            }
            None => {
                web_sys::console::log_1(&format!("DataLoader: Article not found: {}", slug).into());
                Err(DataLoadError::NotFound(format!("Article not found: {}", slug)))
            }
        }
    }

    // Generic JSON fetching method
    async fn fetch_json<T>(&self, url: &str) -> Result<T, DataLoadError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(url, &opts)
            .map_err(|e| DataLoadError::NetworkError(format!("Failed to create request: {:?}", e)))?;

        let window = web_sys::window()
            .ok_or_else(|| DataLoadError::NetworkError("No window object".to_string()))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| DataLoadError::NetworkError(format!("Fetch failed: {:?}", e)))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| DataLoadError::NetworkError(format!("Invalid response: {:?}", e)))?;

        if !resp.ok() {
            return Err(DataLoadError::NotFound(format!(
                "HTTP {}: {}",
                resp.status(),
                resp.status_text()
            )));
        }

        let json = JsFuture::from(resp.json().map_err(|e| {
            DataLoadError::ParseError(format!("Failed to get JSON: {:?}", e))
        })?)
        .await
        .map_err(|e| DataLoadError::ParseError(format!("Failed to parse JSON: {:?}", e)))?;

        let data: T = serde_wasm_bindgen::from_value(json)
            .map_err(|e| DataLoadError::ParseError(format!("Failed to deserialize: {:?}", e)))?;

        Ok(data)
    }
}

impl Default for DataLoader {
    fn default() -> Self {
        Self::new()
    }
}

// Hook for using DataLoader in Yew components
#[hook]
pub fn use_data_loader() -> UseStateHandle<Option<DataLoader>> {
    use_state(|| Some(DataLoader::new()))
}

// Hook for loading articles data
#[hook]
pub fn use_articles_data() -> (UseStateHandle<Option<ArticlesData>>, UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let data = use_state(|| None);
    let loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let data = data.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let loader = DataLoader::new();
                match loader.load_articles().await {
                    Ok(articles_data) => {
                        data.set(Some(articles_data));
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

    (data, loading, error)
}


// Hook for loading lightweight articles (for list display)
#[hook]
pub fn use_lightweight_articles() -> (UseStateHandle<Option<Vec<LightweightArticle>>>, UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let data = use_state(|| None);
    let loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let data = data.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let loader = DataLoader::new();
                match loader.load_lightweight_articles().await {
                    Ok(lightweight_articles) => {
                        data.set(Some(lightweight_articles));
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

    (data, loading, error)
}

// Hook for loading a specific article by slug (with caching)
#[hook]
pub fn use_article_content(slug: Option<String>) -> (UseStateHandle<Option<ProcessedArticle>>, UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let data = use_state(|| None);
    let loading = use_state(|| false);
    let error = use_state(|| None);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(slug.clone(), move |slug| {
            if let Some(slug) = slug {
                web_sys::console::log_1(&format!("use_article_content: Loading article with slug: {}", slug).into());

                let data = data.clone();
                let loading = loading.clone();
                let error = error.clone();
                let slug = slug.clone();

                loading.set(true);
                error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let loader = DataLoader::new();
                    web_sys::console::log_1(&format!("use_article_content: Created DataLoader, calling load_article_by_slug").into());

                    match loader.load_article_by_slug(&slug).await {
                        Ok(article) => {
                            web_sys::console::log_1(&format!("use_article_content: Successfully loaded article: {}", article.title).into());
                            data.set(Some(article));
                            error.set(None);
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("use_article_content: Failed to load article: {}", e).into());
                            error.set(Some(e));
                        }
                    }
                    loading.set(false);
                });
            } else {
                web_sys::console::log_1(&"use_article_content: No slug provided".into());
                data.set(None);
                loading.set(false);
                error.set(None);
            }

            || {}
        });
    }

    (data, loading, error)
}
