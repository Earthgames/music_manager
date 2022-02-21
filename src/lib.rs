use directories::UserDirs;
use glob::glob;
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

    let genre_dirs = match read_dir(format!("{}{}", music_dir, "/youtube/*").as_str()) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{:?}", e);
            return Err("something went wrong while reading the genre/type directories");
        }
    };

    if genre == &None {
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
                // println!("{}", line);
            }
        }
        return Ok(());
        //print all genres and their discriptions
    };

    // check if there are arguments
    // like new genre which needs a titel and a discription
    // or if there is a specific genre and print their discription form that genre
    Ok(())
}
