//! Web pages module
//! 
//! This module contains page-level components for different routes
//! in the web application.

pub mod home;
pub mod article_index;
pub mod article_view;

// Re-export page components
pub use home::HomePage;
pub use article_index::ArticleIndexPage;
pub use article_view::ArticleViewPage;