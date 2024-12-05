use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use directories::{BaseDirs, UserDirs};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Context, Result};

use crate::create_file;

#[derive(Deserialize, Serialize)]
pub struct Config {
    /// The root directory of all the music
    pub music_dir: PathBuf,
    /// The default directory
    pub default_dir: PathBuf,
    /// File extensions that are allowed in the music folders
    pub file_extensions: Vec<String>,
    /// Files to check if they are in the album directory, as a glob pattern
    pub album_files: Option<Vec<String>>,
}

pub fn get_config() -> Result<Config> {
    let base_dir = BaseDirs::new().ok_or(anyhow!("Could not find directories"))?;
    let config_dir = BaseDirs::config_dir(&base_dir);
    let config_path = config_dir.join("music_manager/config.toml");

    // get content or create new content
    let config: Config = match fs::read_to_string(config_path) {
        Ok(cont) => toml::from_str(cont.as_str()).context("Could not read config")?,
        Err(err) => {
            return match err.kind() {
                ErrorKind::NotFound => {
                    info!("Could not find config, making it");
                    let config = make_config()?;
                    Ok(config)
                }
                _ => Err(anyhow!("Could not get config because of {err}")),
            }
        }
    };

    Ok(config)
}

fn make_config() -> Result<Config> {
    let base_dir = BaseDirs::new().ok_or(anyhow!("Could not find directories"))?;
    let config_dir = BaseDirs::config_dir(&base_dir);
    if let Err(err) = fs::create_dir(config_dir.join("music_manager")) {
        match err.kind() {
            ErrorKind::AlreadyExists => warn!("Directory already exists"),
            _ => {
                return Err(anyhow!("Could not make directory\n{err}"));
            }
        }
    }
    let music_dir = Path::new(&get_dir_music()?).to_owned();
    let default_dir = music_dir.join("other");
    let config = Config {
        music_dir,
        default_dir,
        file_extensions: vec!["opus".to_string()],
        album_files: None,
    };

    let content = toml::to_string(&config).context("deserialize config")?;
    create_file(&config_dir.join("music_manager/config.toml"), content)?;
    Ok(config)
}

fn get_dir_music() -> std::io::Result<String> {
    let user_dir = match UserDirs::new() {
        Some(dir) => dir,
        None => return Err(ErrorKind::NotFound.into()),
    };
    let music_dir = match UserDirs::audio_dir(&user_dir) {
        Some(dir) => dir,
        None => return Err(ErrorKind::NotFound.into()),
    };
    let result = match music_dir.to_str() {
        Some(dir) => dir,
        None => return Err(ErrorKind::NotFound.into()),
    };

    Ok(result.to_string())
}
