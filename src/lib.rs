use directories::UserDirs;
use glob::glob;
use std::env;
use std::io;

// gives a string with all the files in that match a path pattern
pub fn read_dir(dir: &str) -> Result<Vec<String>, io::Error> {
    let mut result: Vec<String> = Vec::new();
    for entry in glob(dir).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => result.push(path.to_str().unwrap().to_string()),
            Err(e) => return Err(e.into_error()),
        };
    }
    return Ok(result);
}

pub fn get_dir_music() -> Result<String, &'static str> {
    let user_dir = match UserDirs::new() {
        Some(dir) => dir,
        None => return Err("user directory not found"),
    };
    let music_dir = match UserDirs::audio_dir(&user_dir) {
        Some(dir) => dir,
        None => return Err("audio directory not found"),
    };
    let result = match music_dir.to_str() {
        Some(dir) => dir,
        None => return Err("could not convert audio directory in string"),
    };

    return Ok(result.to_string());
}

pub struct Config {
    pub webadress: String,
    pub genre_type: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let webadress = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a webadress"),
        };
        let genre_type = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a genre_type"),
        };

        Ok(Config {
            webadress,
            genre_type,
        })
    }
}

pub fn search<'a>(query: &str, contents: Vec<String>) -> Vec<String> {
    let mut results = Vec::new();

    for line in contents {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
