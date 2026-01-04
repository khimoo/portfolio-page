/// Utility functions for path resolution and common operations

/// Helper function to resolve image paths based on current environment
/// This function uses the CI environment variable to determine the correct base path.
/// When CI is set (GitHub Pages), it uses /portfolio-page as the base.
/// When CI is not set (local development), it uses the root path.
pub fn resolve_image_path(image_path: &str) -> String {
    // Check if CI environment variable is set (indicates GitHub Pages deployment)
    if option_env!("CI").is_some() {
        // GitHub Pages environment - use /portfolio-page as base path
        format!("/portfolio-page{}", image_path)
    } else {
        // Local development environment - use root path (same as articles)
        image_path.to_string()
    }
}
