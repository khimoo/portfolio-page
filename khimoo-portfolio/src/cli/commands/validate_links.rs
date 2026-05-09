use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::config_loader::get_default_articles_dir;
use crate::core::articles::links::{LinkValidator, ProcessedArticleRef};
use crate::core::articles::processor::ArticleProcessor;

/// CLI arguments for the validate links command
#[derive(Parser, Debug, Clone)]
#[command(name = "validate-links")]
#[command(about = "Validate links in markdown articles")]
pub struct ValidateLinksArgs {
    /// Directory containing markdown articles
    #[arg(short, long)]
    pub articles_dir: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

/// Command implementation for validating links
pub struct ValidateLinksCommand {
    processor: ArticleProcessor,
}

impl ValidateLinksCommand {
    pub fn new() -> Result<Self> {
        let processor = ArticleProcessor::new()?;

        Ok(Self { processor })
    }

    pub fn execute(&self, args: ValidateLinksArgs) -> Result<()> {
        let articles_dir = args
            .articles_dir
            .clone()
            .unwrap_or_else(get_default_articles_dir);

        if args.verbose {
            println!(
                "Validating links in articles from: {}",
                articles_dir.display()
            );
        }

        // Process articles and extract links
        let processed_articles = self.process_articles(&articles_dir, &args)?;

        // Create validator with processed articles
        let validator = LinkValidator::new(&processed_articles);

        // Validate links
        let validation_results = validator.validate_all()?;

        // Output to console
        println!("🔍 Link Validation Report");
        println!("📅 Generated: {}", chrono::Utc::now().to_rfc3339());
        println!();
        println!("📊 Summary:");
        println!(
            "   📚 Total articles: {}",
            validation_results.summary.total_articles
        );
        println!(
            "   🔗 Total links: {}",
            validation_results.summary.total_links
        );

        if validation_results.summary.broken_links > 0 {
            println!(
                "   ❌ Broken links: {}",
                validation_results.summary.broken_links
            );
            println!();
            println!("❌ Errors:");
            for (i, error) in validation_results.errors.iter().enumerate() {
                let error_type_str = match error.error_type {
                    crate::core::articles::links::ValidationErrorType::BrokenLink => {
                        "🔗 Broken Link"
                    }
                    crate::core::articles::links::ValidationErrorType::InvalidRelatedArticle => {
                        "📋 Invalid Related Article"
                    }
                    crate::core::articles::links::ValidationErrorType::MissingMetadata => {
                        "📝 Missing Metadata"
                    }
                    crate::core::articles::links::ValidationErrorType::InvalidMetadata => {
                        "❌ Invalid Metadata"
                    }
                    crate::core::articles::links::ValidationErrorType::CircularReference => {
                        "🔄 Circular Reference"
                    }
                    crate::core::articles::links::ValidationErrorType::OrphanedArticle => {
                        "🏝️  Orphaned Article"
                    }
                };

                let mut formatted = format!(
                    "{}. {}: {} → {}",
                    i + 1,
                    error_type_str,
                    error.source_article,
                    error.target_reference
                );

                if let Some(context) = &error.context {
                    formatted.push_str(&format!(" ({context})"));
                }

                println!("{formatted}");
            }
        } else {
            println!("   ✅ All links valid");
        }

        if args.verbose {
            println!(
                "✅ Validated links in {} articles",
                processed_articles.len()
            );
        }

        Ok(())
    }

    fn process_articles(
        &self,
        articles_dir: &std::path::Path,
        args: &ValidateLinksArgs,
    ) -> Result<Vec<ProcessedArticleRef>> {
        let mut processed_articles = Vec::new();

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

                let content = std::fs::read_to_string(path)?;
                let processed = self.processor.process_article(path, &content)?;
                processed_articles.push(processed);
            }
        }

        Ok(processed_articles)
    }
}

impl Default for ValidateLinksCommand {
    fn default() -> Self {
        Self::new().expect("Failed to create ValidateLinksCommand")
    }
}
