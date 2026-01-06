//! Web application module
//!
//! This module contains all web UI components, pages, routing,
//! and web-specific functionality for the portfolio application.

pub mod app;
pub mod article_manager;
pub mod components;
pub mod config;
pub mod data_loader;
pub mod header;
pub mod pages;
pub mod physics_sim;
pub mod routes;
pub mod styles;
pub mod types;

// Re-export commonly used items
pub use app::*;
pub use routes::*;
