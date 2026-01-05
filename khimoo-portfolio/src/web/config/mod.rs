//! Web configuration module
//! 
//! This module contains configuration structures and settings
//! for the web application UI, themes, and physics simulation.

pub mod theme_config;
pub mod style_config;
pub mod physics_config;

// Re-export configuration components
pub use theme_config::*;
pub use style_config::*;
pub use physics_config::*;