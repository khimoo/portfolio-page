use anyhow::{Context, Result};
use chrono::Utc;
use clap::Parser;
use pulldown_cmark::{Event, Parser as MarkdownParser};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config_loader::{get_default_articles_dir, get_images_dir};
use crate::core::articles::links::{ExtractedLink, ProcessedArticleRef};
use crate::core::articles::metadata::ArticleMetadata;
use crate::core::articles::processor::ArticleProcessor;
#[cfg(feature = "cli-tools")]
use crate::core::media::image_optimizer::ImageOptimizer;

/// CLI arguments for the process articles command
#[derive(Parser, Debug, Clone)]
#[command(name = "process-articles")]
#[command(about = "Process articles and generate static data")]
pub struct ProcessArticlesArgs {
    /// Directory containing markdown articles
    #[arg(short, long)]
    pub articles_dir: Option<PathBuf>,

    /// Output directory for processed data
    #[arg(short, long, default_value = "data")]
    pub output_dir: PathBuf,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Enable parallel processing
    #[arg(short, long)]
    pub parallel: bool,

    /// Optimize images during processing
    #[arg(long)]
    pub optimize_images: bool,
}

/// Processed article data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedArticle {
    pub slug: String,
    pub title: String,
    pub metadata: ArticleMetadata,
    pub file_path: String,
    pub summary: Option<String>,
    pub outbound_links: Vec<ExtractedLink>,
    pub inbound_links: Vec<ExtractedLink>,
    pub processed_at: String,
}

impl ProcessedArticle {
    /// Create from ProcessedArticleRef and file path
    pub fn from_ref_and_file_path(
        article_ref: ProcessedArticleRef,
        file_path: String,
        summary: Option<String>,
    ) -> Self {
        use chrono::Utc;

        Self {
            slug: article_ref.slug,
            title: article_ref.title,
            metadata: article_ref.metadata,
            file_path,
            summary,
            outbound_links: article_ref.outbound_links,
            inbound_links: article_ref.inbound_links,
            processed_at: Utc::now().to_rfc3339(),
        }
    }
}

/// Articles data structure for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlesData {
    pub articles: Vec<ProcessedArticle>,
    pub generated_at: String,
    pub total_count: usize,
    pub home_articles: Vec<String>,
}

/// Command implementation for processing articles
pub struct ProcessArticlesCommand {
    processor: ArticleProcessor,
    #[cfg(feature = "cli-tools")]
    image_optimizer: Option<ImageOptimizer>,
}

impl ProcessArticlesCommand {
    pub fn new() -> Result<Self> {
        let processor = ArticleProcessor::new()?;

        #[cfg(feature = "cli-tools")]
        let image_optimizer = Some(ImageOptimizer::with_defaults());
        #[cfg(not(feature = "cli-tools"))]
        let image_optimizer = None;

        Ok(Self {
            processor,
            #[cfg(feature = "cli-tools")]
            image_optimizer,
        })
    }

    pub fn execute(&self, args: ProcessArticlesArgs) -> Result<()> {
        let articles_dir = args
            .articles_dir
            .clone()
            .unwrap_or_else(|| get_default_articles_dir());

        if args.verbose {
            println!("Processing articles from: {}", articles_dir.display());
            println!("Output directory: {}", args.output_dir.display());
        }

        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&args.output_dir).context("Failed to create output directory")?;

        // Process articles
        let articles = self.process_articles(&articles_dir, &args)?;

        // Create articles data structure
        let home_articles = articles
            .iter()
            .filter(|a| a.metadata.home_display)
            .map(|a| a.slug.clone())
            .collect();

        let articles_data = ArticlesData {
            total_count: articles.len(),
            articles,
            generated_at: Utc::now().to_rfc3339(),
            home_articles,
        };

        // Write JSON output
        let output_path = args.output_dir.join("articles.json");
        let json = serde_json::to_string_pretty(&articles_data)?;
        std::fs::write(&output_path, json).context("Failed to write articles.json")?;

        if args.verbose {
            println!("✅ Processed {} articles", articles_data.total_count);
            println!("📄 Output written to: {}", output_path.display());
        }

        Ok(())
    }

    fn process_articles(
        &self,
        articles_dir: &Path,
        args: &ProcessArticlesArgs,
    ) -> Result<Vec<ProcessedArticle>> {
        let mut articles = Vec::new();

        // Find all markdown files
        for entry in WalkDir::new(articles_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                if args.verbose {
                    println!("Processing: {}", path.display());
                }

                let content = std::fs::read_to_string(path)
                    .with_context(|| format!("Failed to read file: {}", path.display()))?;

                let content_only = self.parse_content_only(&content);
                let summary = self.extract_summary_from_content(&content_only);
                let processed_ref = self.processor.process_article(path, &content)?;
                let file_path = path.to_string_lossy().to_string();
                let processed = ProcessedArticle::from_ref_and_file_path(
                    processed_ref,
                    file_path,
                    Some(summary),
                );
                articles.push(processed);
            }
        }

        // Optimize images if requested
        #[cfg(feature = "cli-tools")]
        if args.optimize_images {
            if let Some(ref optimizer) = self.image_optimizer {
                self.optimize_images(optimizer, args)?;
            }
        }

        Ok(articles)
    }

    fn extract_summary_from_content(&self, content: &str) -> String {
        let plain_text = self.extract_plain_text_from_markdown(content);
        let content = plain_text.trim();

        if let Some(first_paragraph) = content.split("\n\n").next() {
            let cleaned = first_paragraph
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join(" ");

            if !cleaned.is_empty() {
                return self.truncate_string_safely(&cleaned, 150);
            }
        }

        let cleaned = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ");

        if cleaned.is_empty() {
            "記事の概要はありません。".to_string()
        } else {
            self.truncate_string_safely(&cleaned, 150)
        }
    }

    fn truncate_string_safely(&self, text: &str, max_len: usize) -> String {
        if text.len() <= max_len {
            return text.to_string();
        }

        let mut truncate_at = max_len.saturating_sub(3);
        while truncate_at > 0 && !text.is_char_boundary(truncate_at) {
            truncate_at -= 1;
        }

        format!("{}...", &text[..truncate_at])
    }

    fn extract_plain_text_from_markdown(&self, content: &str) -> String {
        let parser = MarkdownParser::new(content);
        let mut plain_text = String::new();

        for event in parser {
            if let Event::Text(text) = event {
                plain_text.push_str(&text);
            }
        }

        plain_text
    }

    fn parse_content_only(&self, content: &str) -> String {
        let content = content.trim();

        if content.starts_with("---") {
            let lines: Vec<&str> = content.lines().collect();

            let mut end_index = None;
            for (i, line) in lines.iter().enumerate().skip(1) {
                if line.trim() == "---" {
                    end_index = Some(i);
                    break;
                }
            }

            if let Some(end_idx) = end_index {
                let remaining_lines = &lines[end_idx + 1..];
                return remaining_lines.join("\n").trim_start().to_string();
            }
        }

        content.to_string()
    }

    #[cfg(feature = "cli-tools")]
    fn optimize_images(
        &self,
        optimizer: &ImageOptimizer,
        args: &ProcessArticlesArgs,
    ) -> Result<()> {
        let images_dir = get_images_dir();
        let output_dir = images_dir.clone(); // Optimize in place

        if args.verbose {
            println!("Optimizing images from: {}", images_dir.display());
        }

        // First, clean up any previously optimized images to avoid recursive optimization
        let cleaned_count = optimizer.cleanup_optimized_images(&images_dir)?;
        if args.verbose && cleaned_count > 0 {
            println!(
                "🧹 Cleaned up {} previously optimized images",
                cleaned_count
            );
        }

        for entry in WalkDir::new(&images_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Skip already optimized images
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.contains("_small")
                    || file_name_str.contains("_medium")
                    || file_name_str.contains("_large")
                    || file_name_str.contains("_tiny")
                {
                    if args.verbose {
                        println!("⏭️  Skipping already optimized: {}", path.display());
                    }
                    continue;
                }
            }

            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                if matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg") {
                    if args.verbose {
                        println!("Optimizing image: {}", path.display());
                    }

                    let _optimized = optimizer.optimize_image(path, &output_dir)?;
                    // The optimizer handles saving the optimized images
                }
            }
        }

        Ok(())
    }
}

impl Default for ProcessArticlesCommand {
    fn default() -> Self {
        Self::new().expect("Failed to create ProcessArticlesCommand")
    }
}
