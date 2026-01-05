//! CLI integration tests
//! 
//! These tests can be used to execute CLI commands during development and CI.

#[cfg(all(not(target_arch = "wasm32"), feature = "cli-tools"))]
mod tests {
    use khimoo_portfolio::cli::commands::process_articles::{ProcessArticlesCommand, ProcessArticlesArgs};
    use khimoo_portfolio::cli::commands::validate_links::{ValidateLinksCommand, ValidateLinksArgs};
    use std::path::PathBuf;

    #[test]
    #[ignore] // Use --ignored to run these tests
    fn test_process_articles_cli() {
        let command = ProcessArticlesCommand::new().expect("Failed to create command");
        let args = ProcessArticlesArgs {
            articles_dir: Some(PathBuf::from("../content/articles")),
            output_dir: PathBuf::from("data"),
            verbose: true,
            parallel: false,
            optimize_images: false,
        };
        
        command.execute(args).expect("Failed to process articles");
    }

    #[test]
    #[ignore] // Use --ignored to run these tests
    fn test_process_articles_with_images_cli() {
        let command = ProcessArticlesCommand::new().expect("Failed to create command");
        let args = ProcessArticlesArgs {
            articles_dir: Some(PathBuf::from("../content/articles")),
            output_dir: PathBuf::from("data"),
            verbose: true,
            parallel: false,
            optimize_images: true,
        };
        
        command.execute(args).expect("Failed to process articles with images");
    }

    #[test]
    #[ignore] // Use --ignored to run these tests
    fn test_validate_links_cli() {
        let command = ValidateLinksCommand::new().expect("Failed to create command");
        let args = ValidateLinksArgs {
            articles_dir: Some(PathBuf::from("../content/articles")),
            output_dir: PathBuf::from("validation_reports"),
            format: "json".to_string(),
            verbose: true,
        };
        
        command.execute(args).expect("Failed to validate links");
    }
}

// Placeholder test for when CLI tools are not available
#[cfg(not(all(not(target_arch = "wasm32"), feature = "cli-tools")))]
#[test]
fn test_placeholder() {
    // CLI tests require the cli-tools feature and non-wasm32 target
    assert!(true);
}