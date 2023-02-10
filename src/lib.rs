use colored::Colorize;
use config::get_config;
use glob::glob;
use std::fs;
use std::path::Path;
use std::process::Command;
mod config;
mod genre_description;
mod music_tag;
use music_tag::MusicTag;
use std::fs::File;
use std::io::ErrorKind;
use std::os::unix::fs::FileExt;

// gives a string with all the files in that match a path pattern
pub fn read_dir(dir: &Path, file_ext: Option<&str>) -> std::io::Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    let search = match file_ext {
        Some(ext) => dir.join(format!("*{}", ext)),
        None => dir.join("*"),
    };

    let dir = match search.to_str() {
        Some(dir) => dir,
        None => return Err(std::io::ErrorKind::NotFound.into()),
    };
    for entry in glob(dir).expect("Failed to read glob pattern") {
        match entry {
            Ok(entry) => result.push(entry.display().to_string()),
            Err(err) => return Err(err.into_error()),
        }
    }
    Ok(result)
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

// will be removed?
pub fn clean_tmp() -> Result<(), Box<dyn std::error::Error>> {
    let music_dir = get_config()?.music_dir;
    let tmp_music_dir = music_dir.join("tmp/*");
    let tmp_dir_content = read_dir(&tmp_music_dir, None)?;

    for file in tmp_dir_content {
        fs::remove_file(file)?;
    }
    Ok(())
}

pub fn download(webadress: &String, genre_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    // get user config directory
    let config = get_config()?;
    let music_dir = config.music_dir;

    let tmp_music_dir = music_dir.join("tmp/");

    // checks if de temporary directory exists, makes it if it does not
    if !Path::new(&tmp_music_dir).is_dir() {
        println!(
            "there is no temporary directory in {}, trying to make it",
            &tmp_music_dir.display()
        );
        fs::create_dir(&tmp_music_dir)?
    }

    let tmp_dir_content = read_dir(&tmp_music_dir, None)?;

    // download from yt with yt-dlp
    let youtube_download = match Command::new("yt-dlp")
        .arg(webadress)
        .current_dir(&tmp_music_dir)
        .status()
    {
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
    let tmp_dir_content_after = read_dir(&tmp_music_dir, None)?;
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
        .status()
    {
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
        Ok(dir) => Path::new(&dir).to_owned(),
        Err(_) => {
            // could this be diffrent ??
            println!("genre_type not found");
            let default_dir = config.default_dir;
            if !Path::new(&default_dir).is_dir() {
                println!("The default_dir is not in {}", default_dir.display());
                return Ok(());
            }
            default_dir
        }
    };

    move_files(mp3_files, &genre_dir.to_str().unwrap())?;
    Ok(())
}

// TODO look if it really does have to use clone
fn search_genre(genre: String) -> Result<String, std::io::Error> {
    // get config
    let config = get_config()?;
    let music_dir = config.music_dir;

    let genre_type_dirs = read_dir(&music_dir, None)?;

    let genre_dir = search(&genre, genre_type_dirs);

    // Checking if the directory exists, otherwise it checks if the other directory,
    // if not it creates it
    if genre_dir.is_empty() {
        return Err(std::io::ErrorKind::NotFound.into());
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
            Ok(path) => Path::new(&path).to_owned(),
            Err(_) => {
                println!("could not find genre/type, don't use any arguments to print all genres");
                return Ok(());
            }
        };

        let (name, description) = genre_description::get_genre_description(genre_path.as_path())?;

        let music_files = read_dir(&genre_path.as_path(), Some(".mp3"))?;
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

        if music_tags.is_empty() {
            println!("{}", "No music found".red().bold());
            return Ok(());
        }

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
        let music_dir = get_config()?.music_dir;
        let genre_dirs = read_dir(music_dir.as_path(), None)?;

        for genre_dir in genre_dirs {
            let (name, description) =
                match genre_description::get_genre_description(&Path::new(&genre_dir)) {
                    Ok(cont) => cont,
                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => println!(
                                "could not find description for folder {}, skipping",
                                genre_dir
                            ),
                            _ => println!("skipping because of error"),
                        }
                        continue;
                    }
                };
            println!("{}: {}", "Name".bold().purple(), name.bold());
            println!("{}: {}", "Description".bold().blue(), description);
            println!();
        }
        Ok(())
    }
}

pub fn create_genre(
    genre_name: &String,
    genre_description: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config()?;
    let music_dir = config.music_dir;

    let genre_dir = music_dir.join(&genre_name);

    // checks if de genre directory already exists, makes it if it does not
    if genre_dir.is_dir() {
        fs::create_dir(&genre_dir)?;
        println!("made genre directory {}", &genre_dir.display());
    }
    genre_description::create_genre_description(
        &genre_dir,
        Some(genre_name),
        Some(genre_description),
    )?;
    println!("made description file for genre {}", &genre_name);

    Ok(())
}

fn create_file(path: &Path, content: String) -> std::io::Result<()> {
    // create a file
    let description_file = File::create(path)?;

    // write to the file
    description_file.write_at(content.as_bytes(), 0)?;

    Ok(())
}
