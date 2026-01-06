// Configuration module (shared between CLI and Web)
pub mod config;

// Core business logic modules (shared between CLI and Web)
pub mod core;

// Web application module (only for WASM targets)
#[cfg(target_arch = "wasm32")]
pub mod web;

// CLI module (only for non-WASM targets)
#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

// Only include config_loader for non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub mod config_loader;
