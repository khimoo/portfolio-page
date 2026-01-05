//! Web components module
//! 
//! This module contains reusable UI components for the web application,
//! including node graph, physics rendering, and article display components.

pub mod node_graph_container;
pub mod physics_renderer;
pub mod node_renderer;
pub mod debug_panel;
pub mod node_data_manager;
pub mod article_header;
pub mod article_content;
pub mod article_state_renderer;

// Re-export commonly used components
pub use node_graph_container::*;
pub use physics_renderer::*;
pub use node_renderer::*;
pub use debug_panel::*;
pub use node_data_manager::*;
pub use article_header::*;
pub use article_content::*;
pub use article_state_renderer::*;