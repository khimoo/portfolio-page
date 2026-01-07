use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(feature = "cli-tools")]
use crate::core::media::image_optimizer::ImageOptimizationConfig;
use crate::config::NodeConfig;

/// Load configuration from project.toml
pub fn load_project_config() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?
        .parent()
        .ok_or("Cannot find project root")?
        .to_path_buf();

    let config_path = project_root.join("project.toml");

    if !config_path.exists() {
        return Err("project.toml not found".into());
    }

    let config_content = std::fs::read_to_string(config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;

    let mut paths = HashMap::new();

    if let Some(paths_table) = config.get("paths").and_then(|v| v.as_table()) {
        for (key, value) in paths_table {
            if let Some(path_str) = value.as_str() {
                paths.insert(key.clone(), path_str.to_string());
            }
        }
    }

    Ok(paths)
}

/// Load full TOML configuration
#[cfg(feature = "cli-tools")]
pub fn load_full_config() -> Result<toml::Value, Box<dyn std::error::Error>> {
    let project_root = std::env::current_dir()?
        .parent()
        .ok_or("Cannot find project root")?
        .to_path_buf();

    let config_path = project_root.join("project.toml");

    if !config_path.exists() {
        return Err("project.toml not found".into());
    }

    let config_content = std::fs::read_to_string(config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;

    Ok(config)
}

/// Get image optimization configuration from project.toml
#[cfg(feature = "cli-tools")]
pub fn get_image_optimization_config() -> ImageOptimizationConfig {
    match load_full_config() {
        Ok(config) => {
            let mut opt_config = ImageOptimizationConfig::default();

            if let Some(optimization) = config.get("optimization").and_then(|v| v.as_table()) {
                if let Some(quality) = optimization
                    .get("webp_quality")
                    .and_then(|v| v.as_integer())
                {
                    opt_config.webp_quality = quality as u8;
                }
                if let Some(small_size) = optimization
                    .get("small_image_size")
                    .and_then(|v| v.as_integer())
                {
                    opt_config.small_image_size = small_size as u32;
                }
                if let Some(medium_size) = optimization
                    .get("medium_image_size")
                    .and_then(|v| v.as_integer())
                {
                    opt_config.medium_image_size = medium_size as u32;
                }
            }

            opt_config
        }
        Err(_) => ImageOptimizationConfig::default(),
    }
}

/// Get node configuration from project.toml
pub fn get_node_config() -> NodeConfig {
    match load_full_config() {
        Ok(config) => {
            let mut node_config = NodeConfig::default();

            if let Some(nodes) = config.get("nodes").and_then(|v| v.as_table()) {
                if let Some(author_radius) = nodes
                    .get("author_node_radius")
                    .and_then(|v| v.as_integer())
                {
                    node_config.author_node_radius = author_radius as i32;
                }
                if let Some(default_radius) = nodes
                    .get("default_node_radius")
                    .and_then(|v| v.as_integer())
                {
                    node_config.default_node_radius = default_radius as i32;
                }
                if let Some(min_radius) = nodes
                    .get("min_node_radius")
                    .and_then(|v| v.as_integer())
                {
                    node_config.min_node_radius = min_radius as i32;
                }
                if let Some(max_radius) = nodes
                    .get("max_node_radius")
                    .and_then(|v| v.as_integer())
                {
                    node_config.max_node_radius = max_radius as i32;
                }
                if let Some(importance_mult) = nodes
                    .get("importance_multiplier")
                    .and_then(|v| v.as_integer())
                {
                    node_config.importance_multiplier = importance_mult as i32;
                }
                if let Some(inbound_mult) = nodes
                    .get("inbound_link_multiplier")
                    .and_then(|v| v.as_integer())
                {
                    node_config.inbound_link_multiplier = inbound_mult as i32;
                }
                if let Some(default_imp) = nodes
                    .get("default_importance")
                    .and_then(|v| v.as_integer())
                {
                    node_config.default_importance = default_imp as u8;
                }
            }

            node_config
        }
        Err(_) => NodeConfig::default(),
    }
}

/// Get default articles directory from configuration
pub fn get_default_articles_dir() -> PathBuf {
    match load_project_config() {
        Ok(config) => {
            if let Some(articles_dir) = config.get("articles_dir") {
                PathBuf::from(format!("../{articles_dir}"))
            } else {
                PathBuf::from("../content/articles")
            }
        }
        Err(_) => PathBuf::from("../content/articles"),
    }
}

/// Get images directory from configuration
#[cfg(feature = "cli-tools")]
pub fn get_images_dir() -> PathBuf {
    match load_project_config() {
        Ok(config) => {
            if let Some(images_dir) = config.get("images_dir") {
                PathBuf::from(format!("../{}", images_dir))
            } else {
                PathBuf::from("../content/assets/img")
            }
        }
        Err(_) => PathBuf::from("../content/assets/img"),
    }
}
