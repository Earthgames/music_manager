use crate::{create_file, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::{fs, io::Error, path::Path};

// type to get the description
#[derive(Deserialize, Serialize)]
struct Description {
    name: String,
    description: String,
}

pub fn get_category_description(category_path: &Path) -> std::io::Result<(String, String)> {
    let description_path = category_path.join("description.toml");
    let contents = fs::read_to_string(description_path)?;
    let description: Description = match toml::from_str(contents.as_str()) {
        Ok(dis) => dis,
        Err(err) => return Err(Error::new(std::io::ErrorKind::Other, err)),
    };

    Ok((description.name, description.description))
}

pub fn create_category_description(
    category_path: &Path,
    category_name: Option<&str>,
    category_description: Option<&str>,
) -> Result<()> {
    // Create path form the category_path
    let description_path = category_path.join("description.toml");

    if description_path.is_file() {
        info!(
            "Description already exist in {}",
            description_path.display()
        );
        return Ok(());
    }

    let name = category_name.unwrap_or("default_name");

    let description = category_description
        .unwrap_or("This is a default description for a category, please add your own");

    let content = Description {
        name: name.to_string(),
        description: description.to_string(),
    };

    let toml = toml::to_string(&content)?;

    create_file(&description_path, toml)?;
    Ok(())
}
