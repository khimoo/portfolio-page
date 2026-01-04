use khimoo_portfolio::articles::{
    FrontMatterParser, ArticleMetadata, LinkExtractor, ExtractedLink
};
use khimoo_portfolio::config_loader::{get_default_articles_dir, get_image_optimization_config, get_images_dir};
use anyhow::{Context, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::Utc;

#[cfg(feature = "cli-tools")]
use khimoo_portfolio::articles::image_optimizer::{ImageOptimizer, OptimizedImageSet};

#[derive(Parser)]
#[command(name = "process-articles")]
#[command(about = "Process articles and generate static data")]
struct Args {
    /// Articles directory path
    #[arg(short, long)]
    articles_dir: Option<PathBuf>,
    
    /// Output directory for generated data
    #[arg(short, long, default_value = "data")]
    output_dir: PathBuf,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// Enable parallel processing
    #[arg(short, long)]
    parallel: bool,
    
    /// Enable image optimization
    #[arg(long)]
    optimize_images: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedArticle {
    pub slug: String,
    pub title: String,
    pub metadata: ArticleMetadata,
    pub file_path: String,
    pub outbound_links: Vec<ExtractedLink>,
    pub inbound_links: Vec<ExtractedLink>,
    pub processed_at: String,
    #[cfg(feature = "cli-tools")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub optimized_images: Vec<OptimizedImageSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticlesData {
    pub articles: Vec<ProcessedArticle>,
    pub generated_at: String,
    pub total_count: usize,
    pub home_articles: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Use configuration default if not provided
    let articles_dir = args.articles_dir.unwrap_or_else(|| get_default_articles_dir());
    
    if args.verbose {
        println!("🔄 Processing articles from {:?}", articles_dir);
        println!("📁 Output directory: {:?}", args.output_dir);
        println!("⚡ Parallel processing: {}", args.parallel);
        println!("🖼️  Image optimization: {}", args.optimize_images);
    }
    
    let processor = ArticleProcessor::new(articles_dir, args.output_dir, args.verbose);
    
    if args.parallel {
        processor.process_all_articles_parallel()
    } else if args.optimize_images {
        processor.process_all_articles_with_images()
    } else {
        processor.process_all_articles_sequential()
    }
}

pub struct ArticleProcessor {
    articles_dir: PathBuf,
    output_dir: PathBuf,
    verbose: bool,
    link_extractor: LinkExtractor,
    #[cfg(feature = "cli-tools")]
    image_optimizer: Option<ImageOptimizer>,
    #[cfg(feature = "cli-tools")]
    images_dir: PathBuf,
}

impl ArticleProcessor {
    pub fn new(articles_dir: PathBuf, output_dir: PathBuf, verbose: bool) -> Self {
        #[cfg(feature = "cli-tools")]
        let image_optimizer = {
            let config = get_image_optimization_config();
            Some(ImageOptimizer::new(config, verbose))
        };
        
        #[cfg(feature = "cli-tools")]
        let images_dir = get_images_dir();
        
        Self {
            articles_dir,
            output_dir,
            verbose,
            link_extractor: LinkExtractor::new().expect("Failed to create LinkExtractor"),
            #[cfg(feature = "cli-tools")]
            image_optimizer,
            #[cfg(feature = "cli-tools")]
            images_dir,
        }
    }

    pub fn process_all_articles_sequential(&self) -> Result<()> {
        self.process_all_articles_sequential_impl(false)
    }

    pub fn process_all_articles_with_images(&self) -> Result<()> {
        self.process_all_articles_sequential_impl(true)
    }

    fn process_all_articles_sequential_impl(&self, optimize_images: bool) -> Result<()> {
        // Create output directory
        std::fs::create_dir_all(&self.output_dir)
            .context("Failed to create output directory")?;
        
        // Load and parse all articles
        let articles = self.load_and_parse_articles(optimize_images)
            .context("Failed to load articles")?;
        
        if self.verbose {
            println!("📚 Found {} articles", articles.len());
        }
        
        // Calculate inbound links
        let articles_with_links = self.calculate_inbound_links(articles)
            .context("Failed to calculate inbound links")?;
        
        // Write output files
        self.write_articles_data(&articles_with_links)
            .context("Failed to write articles data")?;
        
        println!("✅ Successfully processed {} articles", articles_with_links.len());
        
        // Display summary
        self.display_summary(&articles_with_links);
        
        Ok(())
    }

    pub fn process_all_articles_parallel(&self) -> Result<()> {
        // For now, fall back to sequential processing
        // TODO: Implement parallel processing with rayon
        if self.verbose {
            println!("⚠️  Parallel processing not yet implemented, using sequential");
        }
        self.process_all_articles_sequential()
    }

    fn load_and_parse_articles(&self, optimize_images: bool) -> Result<Vec<ProcessedArticle>> {
        let mut articles = Vec::new();
        
        for entry in WalkDir::new(&self.articles_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                // Exclude Templates directory and its subdirectories
                !e.path().components().any(|component| {
                    component.as_os_str().to_string_lossy() == "Templates"
                })
            })
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        {
            match self.process_article_file(entry.path(), optimize_images) {
                Ok(article) => {
                    if self.verbose {
                        println!("✅ Processed: {} - '{}'", 
                            entry.path().display(), 
                            article.title
                        );
                    }
                    articles.push(article);
                }
                Err(e) => {
                    eprintln!("❌ Error processing {}: {}", entry.path().display(), e);
                    return Err(e);
                }
            }
        }
        
        Ok(articles)
    }

    fn process_article_file(&self, file_path: &Path, optimize_images: bool) -> Result<ProcessedArticle> {
        // Read file content
        let content = std::fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;

        // Parse front matter
        let (metadata, markdown_content) = FrontMatterParser::parse(&content)
            .with_context(|| format!("Failed to parse front matter in: {:?}", file_path))?;

        // Validate metadata
        FrontMatterParser::validate_metadata(&metadata)
            .with_context(|| format!("Invalid metadata in: {:?}", file_path))?;

        // Extract links from content
        let outbound_links = self.link_extractor.extract_links(&markdown_content);

        // Generate slug from file path
        let slug = self.generate_slug(file_path);

        // Process images if requested
        #[cfg(feature = "cli-tools")]
        let optimized_images = if optimize_images {
            self.process_article_images(&metadata)?
        } else {
            Vec::new()
        };

        if self.verbose {
            println!("   � TitlOe: {}", metadata.title);
            println!("   🆔 Slug: {}", slug);
            println!("   🏠 Home display: {}", metadata.home_display);
            if let Some(category) = &metadata.category {
                println!("   📂 Category: {}", category);
            }
            println!("   ⭐ Importance: {}", metadata.importance);
            if !outbound_links.is_empty() {
                println!("   🔗 Outbound links: {}", outbound_links.len());
                for link in &outbound_links {
                    println!("      → {} ({})", link.target_slug, 
                        match link.link_type {
                            khimoo_portfolio::articles::LinkType::WikiLink => "wiki",
                            khimoo_portfolio::articles::LinkType::MarkdownLink => "markdown",
                        }
                    );
                }
            }
            if !metadata.tags.is_empty() {
                println!("   🏷️  Tags: {:?}", metadata.tags);
            }
            #[cfg(feature = "cli-tools")]
            if !optimized_images.is_empty() {
                println!("   🖼️  Optimized images: {}", optimized_images.len());
                for img in &optimized_images {
                    println!("      → {} ({} bytes → {} bytes WebP)", 
                        img.original_path.file_name().unwrap().to_string_lossy(),
                        img.original_size,
                        img.small_webp_size
                    );
                }
            }
        }

        Ok(ProcessedArticle {
            slug,
            title: metadata.title.clone(),
            metadata,
            file_path: file_path.to_string_lossy().to_string(),
            outbound_links,
            inbound_links: Vec::new(), // Will be calculated later
            processed_at: Utc::now().to_rfc3339(),
            #[cfg(feature = "cli-tools")]
            optimized_images,
        })
    }

    fn generate_slug(&self, file_path: &Path) -> String {
        file_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string()
            .to_lowercase()
            .replace(' ', "-")
    }

    /// Process images referenced in article metadata
    #[cfg(feature = "cli-tools")]
    fn process_article_images(&self, metadata: &ArticleMetadata) -> Result<Vec<OptimizedImageSet>> {
        let mut optimized_images = Vec::new();

        if let Some(image_optimizer) = &self.image_optimizer {
            let image_refs = image_optimizer.extract_image_references(metadata);
            
            for image_filename in image_refs {
                let input_path = self.images_dir.join(&image_filename);
                
                if input_path.exists() {
                    match image_optimizer.optimize_image(&input_path, &self.images_dir) {
                        Ok(optimized) => {
                            if self.verbose {
                                println!("      🖼️  Optimized: {}", image_filename);
                            }
                            optimized_images.push(optimized);
                        }
                        Err(e) => {
                            eprintln!("      ⚠️  Failed to optimize {}: {}", image_filename, e);
                        }
                    }
                } else if self.verbose {
                    println!("      ⚠️  Image not found: {}", input_path.display());
                }
            }
        }

        Ok(optimized_images)
    }

    fn calculate_inbound_links(&self, mut articles: Vec<ProcessedArticle>) -> Result<Vec<ProcessedArticle>> {
        // Create a map of slug -> article index for quick lookup
        let slug_to_index: HashMap<String, usize> = articles
            .iter()
            .enumerate()
            .map(|(i, article)| (article.slug.clone(), i))
            .collect();

        // Collect inbound links for each article
        let mut inbound_links_map: HashMap<String, Vec<ExtractedLink>> = HashMap::new();
        
        for article in &articles {
            for link in &article.outbound_links {
                if slug_to_index.contains_key(&link.target_slug) {
                    // Create an inbound link (reverse direction)
                    let inbound_link = ExtractedLink {
                        target_slug: article.slug.clone(),
                        link_type: link.link_type.clone(),
                        original_text: link.original_text.clone(),
                    };
                    inbound_links_map
                        .entry(link.target_slug.clone())
                        .or_insert_with(Vec::new)
                        .push(inbound_link);
                }
            }
        }

        // Apply the inbound links
        for article in &mut articles {
            article.inbound_links = inbound_links_map
                .remove(&article.slug)
                .unwrap_or_default();
        }

        Ok(articles)
    }

    fn write_articles_data(&self, articles: &[ProcessedArticle]) -> Result<()> {
        let articles_data = ArticlesData {
            articles: articles.to_vec(),
            generated_at: Utc::now().to_rfc3339(),
            total_count: articles.len(),
            home_articles: articles
                .iter()
                .filter(|a| a.metadata.home_display)
                .map(|a| a.slug.clone())
                .collect(),
        };
        
        let output_path = self.output_dir.join("articles.json");
        let json = serde_json::to_string_pretty(&articles_data)
            .context("Failed to serialize articles data")?;
        
        std::fs::write(&output_path, json)
            .with_context(|| format!("Failed to write articles data to {:?}", output_path))?;
        
        if self.verbose {
            println!("📄 Written articles data to: {:?}", output_path);
        }
        
        Ok(())
    }

    fn display_summary(&self, articles: &[ProcessedArticle]) {
        println!("\n📊 Processing Summary:");
        println!("   📚 Total articles: {}", articles.len());
        
        let home_articles = articles.iter().filter(|a| a.metadata.home_display).count();
        println!("   🏠 Home display articles: {}", home_articles);
        
        let total_links: usize = articles.iter().map(|a| a.outbound_links.len()).sum();
        println!("   🔗 Total outbound links: {}", total_links);
        
        let articles_with_inbound: usize = articles.iter().filter(|a| !a.inbound_links.is_empty()).count();
        println!("   📥 Articles with inbound links: {}", articles_with_inbound);
        
        // Category breakdown
        let mut categories: HashMap<String, usize> = HashMap::new();
        for article in articles {
            let category = article.metadata.category.as_deref().unwrap_or("uncategorized");
            *categories.entry(category.to_string()).or_insert(0) += 1;
        }
        
        if !categories.is_empty() {
            println!("   📂 Categories:");
            for (category, count) in categories {
                println!("      {}: {}", category, count);
            }
        }
        
        // Tag statistics
        let mut all_tags: HashMap<String, usize> = HashMap::new();
        for article in articles {
            for tag in &article.metadata.tags {
                *all_tags.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        
        if !all_tags.is_empty() {
            println!("   🏷️  Most common tags:");
            let mut tag_counts: Vec<_> = all_tags.into_iter().collect();
            tag_counts.sort_by(|a, b| b.1.cmp(&a.1));
            for (tag, count) in tag_counts.into_iter().take(5) {
                println!("      {}: {}", tag, count);
            }
        }
        
        // Link validation preview
        let existing_slugs: std::collections::HashSet<_> = articles.iter().map(|a| &a.slug).collect();
        let mut broken_links = 0;
        
        for article in articles {
            for link in &article.outbound_links {
                if !existing_slugs.contains(&link.target_slug) {
                    broken_links += 1;
                }
            }
        }
        
        if broken_links > 0 {
            println!("   ⚠️  Broken links detected: {} (run validate-links for details)", broken_links);
        } else {
            println!("   ✅ All links valid");
        }
    }
}