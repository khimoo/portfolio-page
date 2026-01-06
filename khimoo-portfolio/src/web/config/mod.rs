//! Web configuration module
//!
//! This module contains configuration structures and settings
//! for the web application UI, themes, and physics simulation.

pub mod physics_config;
pub mod style_config;
pub mod theme_config;

// Re-export configuration components
pub use physics_config::*;
pub use style_config::*;
pub use theme_config::*;
