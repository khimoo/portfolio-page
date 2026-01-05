//! Web application module
//! 
//! This module contains all web UI components, pages, routing,
//! and web-specific functionality for the portfolio application.

pub mod app;
pub mod routes;
pub mod header;
pub mod data_loader;
pub mod article_manager;
pub mod physics_sim;
pub mod pages;
pub mod components;
pub mod config;
pub mod types;
pub mod styles;

// Re-export commonly used items
pub use app::*;
pub use routes::*;