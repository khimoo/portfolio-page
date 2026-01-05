//! Web styles module
//! 
//! This module contains styling definitions and utilities
//! for the web application components and layout.

pub mod components;
pub mod layout;
pub mod theme;
pub mod utils;

// Re-export style components
pub use components::*;
pub use layout::*;
pub use theme::*;
pub use utils::*;