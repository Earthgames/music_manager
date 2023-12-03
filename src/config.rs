use crate::create_file;
use directories::{BaseDirs, UserDirs};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{ErrorKind, Result};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub music_dir: PathBuf,
    pub default_dir: PathBuf,
}

pub fn get_config() -> Result<Config> {
    let base_dir = match BaseDirs::new() {
        Some(dir) => dir,
        None => return Err(ErrorKind::NotFound.into()),
    };
    let config_dir = BaseDirs::config_dir(&base_dir);
    let config_path = config_dir.join("music_manager/config.toml");
    // get content or create new content
    let config: Config = match fs::read_to_string(&config_path) {
        Ok(cont) => match toml::from_str(cont.as_str()) {
            Ok(cont) => cont,
            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
        },
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("Could not find config, making it");
                fs::create_dir(config_dir.join("music_manager"))?;
                let music_dir = Path::new(&get_dir_music()?).to_owned();
                let default_dir = music_dir.join("other");
                let config = Config {
                    music_dir,
                    default_dir,
                };
                mk_config(&config, config_path.as_path())?;
                return Ok(config);
            }
            _ => return Err(err),
        },
    };

    Ok(config)
}

fn mk_config(config: &Config, config_path: &Path) -> Result<()> {
    let content = match toml::to_string(&config) {
        Ok(cont) => cont,
        Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err)),
    };

    create_file(config_path, content)
}

fn get_dir_music() -> Result<String> {
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
