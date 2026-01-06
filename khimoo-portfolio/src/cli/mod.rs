//! CLI module for command-line interface functionality
//!
//! This module provides command-line tools for processing articles,
//! validating links, and other maintenance tasks.

#[cfg(feature = "cli-tools")]
pub mod commands;
#[cfg(feature = "cli-tools")]
pub mod main;
#[cfg(feature = "cli-tools")]
pub mod utils;

// Re-export commonly used items
#[cfg(feature = "cli-tools")]
pub use commands::*;
#[cfg(feature = "cli-tools")]
pub use main::Cli;
