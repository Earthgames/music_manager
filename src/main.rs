use music_manager_helper::{get_dir_music, read_dir, search, Config};

use std::env;
use std::process;
use std::process::Command;

fn main() {
    // get config from user
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // get user Music directory
    let music_dir = get_dir_music().unwrap_or_else(|err| {
        eprintln!("Problem getting music directory: {}", err);
        process::exit(1);
    });

    // make String with path to a tmp dir
    // might need to check and or create this dir
    let tmp_music_dir = format!("{}/tmp", music_dir);

    // create commands
    let mut mp3_normalizer = Command::new("mp3gain");
    let mut youtube_download = Command::new("yt-dlp");
    let mut move_files = Command::new("mv");

    // try to download yt video to the tmp dir
    // might need to chenk if it succeded
    youtube_download
        .arg(config.webadress)
        .current_dir(&tmp_music_dir)
        .status()
        .expect("yt-dlp failed to execute");

    // create path to all mp3 files that are in tmp dir
    // let files = &tmp_music_dir;
    // files.push_str("/*.mp3");
    let mp3_files = format!("{}{}", tmp_music_dir, "/*.mp3");
    let mp3_files = read_dir(&mp3_files).unwrap();

    // normalize mp3 files
    mp3_normalizer
        .current_dir(&tmp_music_dir)
        .arg("-r")
        .args(&mp3_files)
        .status()
        .expect("mp3gain failed to execute");

    let genre_type_dirs = read_dir(format!("{}{}", music_dir, "/youtube/*").as_str())
        .unwrap_or_else(|err| {
            eprintln!("Problem converting dir list to str: {}", err);
            process::exit(1);
        });

    let genre_dir = &search(&config.genre_type, genre_type_dirs);

    if genre_dir.len() == 0 {
        eprintln!("genre_type not found");
        process::exit(1);
    }

    move_files
        .current_dir(&tmp_music_dir)
        .args(mp3_files)
        .arg(&genre_dir[0])
        .status()
        .expect("mv failed to execute");
}
