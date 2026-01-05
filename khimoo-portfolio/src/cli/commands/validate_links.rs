use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;

use crate::core::articles::processor::ArticleProcessor;
use crate::core::articles::links::{LinkValidator, ProcessedArticleRef};
use crate::core::articles::links::report_formatter::ValidationReportFormatter;
use crate::config_loader::get_default_articles_dir;

/// CLI arguments for the validate links command
#[derive(Parser, Debug, Clone)]
#[command(name = "validate-links")]
#[command(about = "Validate links in markdown articles")]
pub struct ValidateLinksArgs {
    /// Directory containing markdown articles
    #[arg(short, long)]
    pub articles_dir: Option<PathBuf>,
    
    /// Output directory for validation reports
    #[arg(short, long, default_value = "validation_reports")]
    pub output_dir: PathBuf,
    
    /// Output format (json, text)
    #[arg(short, long, default_value = "json")]
    pub format: String,
    
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
        
        Ok(Self {
            processor,
        })
    }
    
    pub fn execute(&self, args: ValidateLinksArgs) -> Result<()> {
        let articles_dir = args.articles_dir.clone().unwrap_or_else(|| get_default_articles_dir());
        
        if args.verbose {
            println!("Validating links in articles from: {}", articles_dir.display());
            println!("Output directory: {}", args.output_dir.display());
        }
        
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(&args.output_dir)?;
        
        // Process articles and extract links
        let processed_articles = self.process_articles(&articles_dir, &args)?;
        
        // Create validator with processed articles
        let validator = LinkValidator::new(&processed_articles);
        
        // Validate links
        let validation_results = validator.validate_all()?;
        
        // Format and save report based on requested format
        match args.format.as_str() {
            "json" => {
                let report = ValidationReportFormatter::format_json(&validation_results)?;
                let output_path = args.output_dir.join("validation-report.json");
                std::fs::write(&output_path, report)?;
                
                if args.verbose {
                    println!("📄 JSON report written to: {}", output_path.display());
                }
            }
            "text" | "console" => {
                let report = ValidationReportFormatter::format_console(&validation_results);
                let output_path = args.output_dir.join("validation-report.txt");
                std::fs::write(&output_path, report)?;
                
                if args.verbose {
                    println!("📄 Text report written to: {}", output_path.display());
                }
            }
            "markdown" | "md" => {
                let report = ValidationReportFormatter::format_markdown(&validation_results);
                let output_path = args.output_dir.join("validation-report.md");
                std::fs::write(&output_path, report)?;
                
                if args.verbose {
                    println!("📄 Markdown report written to: {}", output_path.display());
                }
            }
            _ => {
                // Default to writing all formats
                ValidationReportFormatter::write_report_files(&validation_results, &args.output_dir)?;
                
                if args.verbose {
                    println!("📄 All report formats written to: {}", args.output_dir.display());
                }
            }
        }
        
        if args.verbose {
            println!("✅ Validated links in {} articles", processed_articles.len());
        }
        
        Ok(())
    }
    
    fn process_articles(&self, articles_dir: &std::path::Path, args: &ValidateLinksArgs) -> Result<Vec<ProcessedArticleRef>> {
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