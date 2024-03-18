use std::{fs, io::Error, path::Path};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};

use crate::{create_file, Result};

/// Config for the category
#[derive(Deserialize, Serialize)]
pub struct CategoryConfig {
    pub name: String,
    pub description: String,
    /// If a category is dedicated to one artist
    pub artist_category: Option<bool>,
}

/// Get the config for a category
pub fn get_category_config(category_path: &Path) -> Result<CategoryConfig> {
    let description_path = category_path.join("config.toml");

    if description_path.is_file() {
        debug!(
            "Found category config at \"{}\"",
            description_path.display()
        )
    } else {
        error!(
            "Could not find category config at \"{}\"",
            description_path.display()
        );
        create_category_config(category_path, None, None)?;
    }

    let contents = fs::read_to_string(description_path)?;
    let description: CategoryConfig = match toml::from_str(contents.as_str()) {
        Ok(dis) => dis,
        Err(err) => return Err(Box::new(Error::new(std::io::ErrorKind::Other, err))),
    };

    Ok(description)
}

//TODO add for every category https://xspf.org/applications
pub fn create_category_config(
    category_path: &Path,
    category_name: Option<&str>,
    category_description: Option<&str>,
) -> Result<()> {
    // Create path form the category_path
    let config_path = category_path.join("config.toml");

    if config_path.is_file() {
        info!(
            "Category config already exist in \"{}\"",
            config_path.display()
        );
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
        artist_category: None,
    };

    let toml = toml::to_string(&content)?;
    create_file(&config_path, toml)?;

    info!(
        "Created category config for {} at \"{}\"",
        name,
        config_path.display()
    );

    Ok(())
}
