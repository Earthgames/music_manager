use crate::create_file;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// type to get the discription
#[derive(Deserialize, Serialize)]
struct Description {
    name: String,
    description: String,
}

pub fn get_genre_description(
    genre_path: &Path,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let description_path = genre_path.join("/discription.toml");
    let contents = fs::read_to_string(description_path)?;
    let description: Description = toml::from_str(contents.as_str())?;

    Ok((description.name, description.description))
}

pub fn create_genre_description(
    genre_path: &Path,
    genre_name: Option<&str>,
    genre_description: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // implement Path everywhere, makes things easier
    // create path form the genre_path, for now
    let path_str = genre_path.join("/description.toml");
    let path = Path::new(&path_str);

    let name = genre_name.unwrap_or("default_name");

    let description = genre_description
        .unwrap_or("This is a default description for a genre. Please add your own");

    let content = Description {
        name: name.to_string(),
        description: description.to_string(),
    };

    let toml = toml::to_string(&content)?;

    create_file(path, toml)?;
    Ok(())
}
