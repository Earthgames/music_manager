use directories::UserDirs;
use glob::glob;
use id3::{Tag, TagLike};
use std::fs; //instead of mv can I use this ??
use std::io;
use std::process::Command;

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

pub fn search<'a>(query: &str, contents: Vec<String>) -> Vec<String> {
    let mut results = Vec::new();

    for line in contents {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn download(webadress: &String, genre_type: &String) -> Result<(), &'static str> {
    // get user Music directory
    let music_dir = match get_dir_music() {
        Ok(dir) => dir,
        Err(e) => return Err(e),
    };

    // make String with path to a tmp dir
    // might need to check and or create this dir
    let tmp_music_dir = format!("{}/tmp", music_dir);

    // download from yt with yt-dlp
    let youtube_download = Command::new("yt-dlp")
        .arg(webadress)
        .current_dir(&tmp_music_dir)
        .status()
        .expect("Failed to execute yt-dlp");

    if !youtube_download.success() {
        return Err("Failed to download with yt-dlp");
    };

    // create path to all mp3 files that are in tmp dir
    let mp3_files = format!("{}{}", tmp_music_dir, "/*.mp3");
    let mp3_files = read_dir(&mp3_files).unwrap();

    // normalize mp3 files
    let mp3_normalizer = Command::new("mp3gain")
        .current_dir(&tmp_music_dir)
        .arg("-r")
        .args(&mp3_files)
        .status()
        .expect("failed to execute mp3gain");

    if !mp3_normalizer.success() {
        return Err("Failed to normalize audio with mp3gain");
    };

    let genre_type_dirs = match read_dir(format!("{}{}", music_dir, "/youtube/*").as_str()) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err("something went wrong while reading the genre/type directories");
        }
    };

    let genre_dir = &search(&genre_type, genre_type_dirs);

    if genre_dir.len() == 0 {
        return Err("genre_type not found");
    }

    match move_files(mp3_files, &genre_dir[0]) {
        Ok(_t) => (),
        Err(e) => return Err(e),
    };
    Ok(())
}

pub fn move_files(target_files: Vec<String>, target_dir: &str) -> Result<(), &'static str> {
    let move_files = Command::new("mv")
        .args(&target_files)
        .arg(target_dir)
        .status()
        .expect("mv failed to execute");

    if !move_files.success() {
        return Err("Failed to move files with mv");
    };
    // if target_files.len() > 3 {
    //     println!("moved {} files to {}", target_files.len(), target_dir);
    //     return Ok(());
    // }
    println!(
        "moved the files: {} to the folder:\n {}",
        target_files.join("\n"),
        target_dir
    );
    Ok(())
}

pub fn genres(genre: &Option<String>) -> Result<(), &'static str> {
    let music_dir = match get_dir_music() {
        Ok(dir) => dir,
        Err(e) => return Err(e),
    };

    let genre_dir = format!("{}{}", music_dir, "/youtube/");

    let genre_dirs = match read_dir(&format!("{genre_dir}*")) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err("something went wrong while reading the genre/type directories");
        }
    };

    if genre.is_none() {
        // print all genres and their discription
        for genre_dir in genre_dirs {
            let discription_path = format!("{}/discription", genre_dir);
            let contents = fs::read_to_string(discription_path)
                .expect("Something went wrong reading the file");
            for line in contents.lines() {
                let parts = line.split("=");
                let vec: Vec<&str> = parts.collect();
                if vec[0].trim() == "name" {
                    println!("{}", vec[1].trim());
                } else {
                    println!("{} \n", vec[1].trim());
                }
            }
        }
        return Ok(());
    } else {
        let genre = genre.as_ref().unwrap();
        let size = genre_dir.len();
        // println!("{:?}", genre_dirs);
        for genre_type in genre_dirs {
            // println!(
            //     "{:?}, {:?}, {:?}, {genre_dir}",
            //     genre_type,
            //     genre_type.len(),
            //     size
            // );

            let genre_type = &genre_type[(size)..];
            // println!("{}", genre_type);

            if &genre == &genre_type.trim() {
                let genre_path = format!("{}{}/", genre_dir, genre_type);
                let discription_path = format!("{}/discription", genre_path);
                let contents = fs::read_to_string(discription_path)
                    .expect("Something went wrong reading the file");
                let mut name: String = String::new();
                let mut discription: String = String::new();

                for line in contents.lines() {
                    let parts = line.split("=");
                    let vec: Vec<&str> = parts.collect();

                    if vec[0].trim() == "name" {
                        name.push_str(vec[1].trim());
                    } else {
                        discription.push_str(vec[1].trim());
                    }
                }
                let music_files = match read_dir(&format!("{genre_path}*.mp3")) {
                    Ok(dir) => dir,
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(
                            "something went wrong while reading the genre/type directories",
                        );
                    }
                };
                if music_files.len() > 15 {
                    //only print albums because output it will be to long
                    let mut albums: Vec<String> = Vec::new();
                    for music_file in music_files {
                        let tag = Tag::read_from_path(music_file).unwrap();

                        if let Some(album) = tag.album() {
                            if !albums.contains(&album.to_string()) {
                                albums.push(album.to_string());
                                if let Some(artist) = tag.artist() {
                                    println!("artist: {}", artist);
                                }
                                println!("album: {}\n", album);
                            }
                        }
                    }
                } else {
                    // print all songs
                    for music_file in music_files {
                        let tag = Tag::read_from_path(music_file).unwrap();

                        if let Some(title) = tag.title() {
                            println!("title: {}", title);
                        }

                        if let Some(artist) = tag.artist() {
                            println!("artist: {}\n", artist);
                        }
                    }
                }
                println!("name: {}\ndiscription: {}", name, discription);

                return Ok(());
            }
        }
        return Err("could not find genre/type, don't use any arguments to print all genres");
    }
    // check if there are arguments
    // like new genre which needs a titel and a discription
    // or if there is a specific genre and print their discription from that genre
    // Ok(())
}

pub fn modify() -> Result<(), Box<dyn std::error::Error>> {
    let tag = Tag::read_from_path("/home/earthgames/Music/youtube/genshin_impact/Jade Moon Upon a Sea of Clouds - Disc 1 - Glazed Moon Over the Tides｜Genshin Impact - 009 09. A Transparent Moon (Liuli Pavilion) [t1O7LpOTBfM].mp3")?;

    // Get a bunch of frames...
    if let Some(artist) = tag.artist() {
        println!("artist: {}", artist);
    }
    if let Some(title) = tag.title() {
        println!("title: {}", title);
    }
    if let Some(album) = tag.album() {
        println!("album: {}", album);
    }

    // Get frames before getting their content for more complex tags.
    if let Some(artist) = tag.get("TPE1").and_then(|frame| frame.content().text()) {
        println!("artist: {}", artist);
    }
    Ok(())
}
