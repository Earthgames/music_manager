use crate::{create_file, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::{fs, io::Error, path::Path};

/// Config for the category
#[derive(Deserialize, Serialize)]
struct CategoryConfig {
    name: String,
    description: String,
}

/// Get the config for a category
pub fn get_category_config(category_path: &Path) -> std::io::Result<(String, String)> {
    let description_path = category_path.join("config.toml");
    let contents = fs::read_to_string(description_path)?;
    let description: CategoryConfig = match toml::from_str(contents.as_str()) {
        Ok(dis) => dis,
        Err(err) => return Err(Error::new(std::io::ErrorKind::Other, err)),
    };

    Ok((description.name, description.description))
}

pub fn create_category_config(
    category_path: &Path,
    category_name: Option<&str>,
    category_description: Option<&str>,
) -> Result<()> {
    // Create path form the category_path
    let config_path = category_path.join("config.toml");

    if config_path.is_file() {
        info!("Category config already exist in {}", config_path.display());
        return Ok(());
    }

    info!("Creating category config at {}", config_path.display());

    let name = match category_name {
        Some(name) => name,
        None => category_path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("default name"),
    };

    let description = category_description
        .unwrap_or("This is a default description for a category, please add your own");

    let content = CategoryConfig {
        name: name.to_string(),
        description: description.to_string(),
    };

    let toml = toml::to_string(&content)?;
    create_file(&config_path, toml)?;

    info!(
        "Created category config for {} at {}",
        name,
        config_path.display()
    );

    Ok(())
}
