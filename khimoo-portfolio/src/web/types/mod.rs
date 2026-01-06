//! Web types module
//!
//! This module contains type definitions specific to the web application,
//! including UI state, data structures, and component properties.

pub mod data_types;
pub mod node_types;
pub mod physics_types;
pub mod ui_types;

// Re-export type definitions
pub use data_types::*;
pub use node_types::*;
pub use physics_types::*;
pub use ui_types::*;
