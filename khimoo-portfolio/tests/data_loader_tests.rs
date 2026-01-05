#![cfg(target_arch = "wasm32")]

use khimoo_portfolio::web::data_loader::{DataLoadError, DataLoader};

#[test]
fn test_data_loader_creation() {
    let loader = DataLoader::new();
    assert_eq!(loader.base_url, "/data");
}

#[test]
fn test_data_loader_with_custom_base_url() {
    let loader = DataLoader::with_base_url("/custom/path".to_string());
    assert_eq!(loader.base_url, "/custom/path");
}

#[test]
fn test_error_display() {
    let error = DataLoadError::NetworkError("Test error".to_string());
    assert_eq!(error.to_string(), "Network error: Test error");
}

// Placeholder test for non-wasm32 targets
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn test_placeholder() {
    // Data loader tests are only available for wasm32 target
    assert!(true);
} 