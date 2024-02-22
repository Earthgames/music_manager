use crate::{create_file, Result};
use directories::{BaseDirs, UserDirs};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub music_dir: PathBuf,
    pub default_dir: PathBuf,
}

pub fn get_config() -> Result<Config> {
    let base_dir = match BaseDirs::new() {
        Some(dir) => dir,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not find directories",
            )))
        }
    };
    let config_dir = BaseDirs::config_dir(&base_dir);
    let config_path = config_dir.join("music_manager/config.toml");

    // get content or create new content
    let config: Config = match fs::read_to_string(&config_path) {
        Ok(cont) => match toml::from_str(cont.as_str()) {
            Ok(cont) => cont,
            Err(err) => {
                error!("Could not read config");
                return Err(Box::new(err));
            }
        },
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                info!("Could not find config, making it");
                match fs::create_dir(config_dir.join("music_manager")) {
                    Ok(_) => (),
                    Err(err) => match err.kind() {
                        ErrorKind::AlreadyExists => warn!("Directory already exists"),
                        _ => {
                            error!("Could not make directory\n{err}");
                            return Err(Box::new(err));
                        }
                    },
                }
                let music_dir = Path::new(&get_dir_music()?).to_owned();
                let default_dir = music_dir.join("other");
                let config = Config {
                    music_dir,
                    default_dir,
                };
                make_config(&config, config_path.as_path())?;
                return Ok(config);
            }
            _ => return Err(Box::new(err)),
        },
    };

    Ok(config)
}

fn make_config(config: &Config, config_path: &Path) -> Result<()> {
    let content = match toml::to_string(&config) {
        Ok(cont) => cont,
        Err(err) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                err,
            )))
        }
    };

    create_file(config_path, content)
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
