use colored::Colorize;
use directories::UserDirs;
use glob::glob;
use std::fs;
use std::path::Path;
use std::process::Command;
mod genre_description;
mod music_tag;
use music_tag::MusicTag;

// gives a string with all the files in that match a path pattern
pub fn read_dir(dir: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = Vec::new();
    for entry in glob(dir).expect("Failed to read glob pattern") {
        result.push(entry?.display().to_string());
    }
    Ok(result)
}

// could make this give a directory from a config. But where should the config be.
fn get_dir_music() -> Result<String, &'static str> {
    let user_dir = match UserDirs::new() {
        Some(dir) => dir,
        None => return Err("User directories not found"),
    };
    let music_dir = match UserDirs::audio_dir(&user_dir) {
        Some(dir) => dir,
        None => return Err("audio directory not found"),
    };
    let result = match music_dir.to_str() {
        Some(dir) => dir,
        None => return Err("could not convert audio directory in string"),
    };

    Ok(result.to_string())
}

fn search(query: &str, content: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in content {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn clean_tmp() -> Result<(), Box<dyn std::error::Error>> {
    let music_dir = get_dir_music().unwrap();
    let tmp_music_dir = format!("{}/tmp", music_dir);
    let tmp_dir_content = read_dir(&format!("{}/*", &tmp_music_dir))?;

    for file in tmp_dir_content {
        fs::remove_file(file)?;
    }
    Ok(())
}

pub fn download(webadress: &String, genre_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    // get user Music directory
    let music_dir = get_dir_music()?;

    let tmp_music_dir = format!("{}/tmp", music_dir);

    // checks if de temporary directory exists, makes it if it does not
    if !Path::new(&tmp_music_dir).is_dir() {
        println!(
            "there is no temporary directory in {}, trying to create it",
            &tmp_music_dir
        );
        fs::create_dir(&tmp_music_dir)?
    }

    let tmp_dir_content = read_dir(&format!("{}/*", &tmp_music_dir))?;

    // download from yt with yt-dlp
    let youtube_download = match Command::new("yt-dlp")
        .arg(webadress)
        .current_dir(&tmp_music_dir)
        .status(){
            Ok(e) => e,
            Err(err) => {
                eprintln!("could not use yt-dlp command \n is it installed?");
                return Err(err.into());
            }
        };

    if !youtube_download.success() {
        eprintln!("yt-dlp {}", youtube_download);
        println!("Failed to download with yt-dlp");
        return Ok(());
    };

    // creates a vector with only the newly created mp3 files
    let mut mp3_files: Vec<String> = Vec::new();
    let tmp_dir_content_after = read_dir(&format!("{}/*", &tmp_music_dir))?;

    if !tmp_dir_content.is_empty() {
        for content in tmp_dir_content_after.iter() {
            if !tmp_dir_content.contains(content) && content.ends_with(".mp3") {
                mp3_files.push(content.to_string());
            }
        }
    } else {
        for content in tmp_dir_content_after.iter() {
            if content.ends_with(".mp3") {
                mp3_files.push(content.to_string())
            }
        }
    }

    // normalize mp3 files
    let mp3_normalizer = match Command::new("mp3gain")
        .current_dir(&tmp_music_dir)
        .arg("-r")
        .args(&mp3_files)
        .status(){
            Ok(e) => e,
            Err(err) => {
                eprintln!("could not use mp3gain command \n is it installed?");
                return Err(err.into());
            }
        };

    if !mp3_normalizer.success() {
        eprintln!(
            "mp3gain {}\nFailed to normalize audio with mp3gain, please do it yourself",
            mp3_normalizer
        );
    };

    // search for dir so short names are possible. otherwise try to use the other directory
    let genre_dir = match search_genre(genre_type.to_string()) {
        Ok(dir) => dir,
        Err(_) => {
            // could this be diffrent ??
            println!("genre_type not found");
            let other_dir = format!("{}{}", music_dir, "/youtube/*");
            let genre_type_dirs = read_dir(format!("{}{}", music_dir, "/youtube/*").as_str())?;
            let genre_dir = search(other_dir.as_str(), genre_type_dirs)[0].clone();
            if genre_dir.is_empty() {
                println!(
                    "There is no other directory in {}, trying to create it",
                    other_dir
                );
                create_genre(
                    &"other".to_string(),
                    &"music that does not fit in another category".to_string(),
                )?;
            }
            genre_dir
        }
    };

    move_files(mp3_files, &genre_dir)?;
    Ok(())
}

fn search_genre(genre: String) -> Result<String, Box<dyn std::error::Error>> {
    let music_dir = get_dir_music()?;
    let genre_type_dirs = read_dir(format!("{}{}", music_dir, "/youtube/*").as_str())?;

    let genre_dir = search(&genre, genre_type_dirs);

    // Checking if the directory exists, otherwise it checks if the other directory,
    // if not it creates it
    if genre_dir.is_empty() {
        return Err("Not found".into());
    }
    Ok(genre_dir[0].clone())
}

pub fn move_files(
    target_files: Vec<String>,
    target_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in target_files {
        let file_name = match Path::new(&file).file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name,
                None => continue,
            },
            None => continue,
        };
        fs::copy(&file, format!("{}/{}", target_dir, file_name))?;
        match fs::remove_file(&file) {
            Ok(_) => println!("moved {} to {}", file, target_dir),
            Err(e) => {
                println!(
                    "copied {} to {}, could not remove the original",
                    file, target_dir
                );

                return Err(Box::new(e));
            }
        };
    }
    Ok(())
}

// print details about genres
pub fn genres(genre: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(genre) = genre {
        let genre_path = match search_genre(genre.clone()) {
            Ok(path) => path,
            Err(_) => {
                println!(
                    "could not find genre/type, don't use any arguments to print all genres"
                );
                return Ok(());
            }
        };

        let (name, description) = genre_description::get_genre_description(&genre_path)?;

        let music_files = read_dir(&format!("{}/*.mp3", genre_path))?;
        let mut music_tags: Vec<MusicTag> = music_tag::get_music_tags(music_files)?;

        let big_tags: bool = if music_tags.len() > 15 {
            music_tags.sort_by(|a, b| a.album_title.cmp(&b.album_title));
            music_tags.dedup_by(|a, b| a.album_title == b.album_title);
            music_tags.sort_by(|a, b| a.artist_name.cmp(&b.artist_name));
            true
        } else {
            music_tags.sort_by(|a, b| a.album_title.cmp(&b.album_title));
            music_tags.sort_by(|a, b| a.artist_name.cmp(&b.artist_name));
            false
        };

        println!("{}: {}", "Name".bold().purple(), name.bold());
        println!("{}: {}", "Description".bold().blue(), description);
        println!();

        for music_tag in music_tags {
            println!("{}: {}", "Artist".bold().magenta(), music_tag.artist_name);
            println!("{}: {}", "Album".bold().magenta(), music_tag.album_title);
            if !big_tags {
                println!("{}: {}", "Song".bold().blue(), music_tag.song_title)
            }
            println!();
        }
        Ok(())
    } else {
                // print all genres and their description
                let music_dir = get_dir_music()?;
                let genre_dir = format!("{}{}", music_dir, "/youtube/");
                let genre_dirs = read_dir(&format!("{}*", genre_dir))?;

                for genre_dir in genre_dirs {
                    let (name, description) = genre_description::get_genre_description(&genre_dir)?;
                    println!("name: {}\ndescription: {}\n", name, description);
                }
                Ok(())
            }
}

pub fn create_genre(
    genre_name: &String,
    genre_description: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    // get user Music directory
    let music_dir = get_dir_music()?;

    let genre_dir = format!("{}/youtube/{}", music_dir, genre_name);

    // checks if de genre directory already exists, makes it if it does not
    if !Path::new(&genre_dir).is_dir() {
        fs::create_dir(&genre_dir)?;
        println!("created genre directory {}", &genre_dir);
    }
    genre_description::create_genre_description(
        &genre_dir,
        Some(genre_name),
        Some(genre_description),
    )?;
    println!("created description file for genre {}", &genre_name);

    Ok(())
}
