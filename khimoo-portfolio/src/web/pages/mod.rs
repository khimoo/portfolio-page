//! Web pages module
//!
//! This module contains page-level components for different routes
//! in the web application.

pub mod article_index;
pub mod article_view;
pub mod home;

// Re-export page components
pub use article_index::ArticleIndexPage;
pub use article_view::ArticleViewPage;
pub use home::HomePage;
