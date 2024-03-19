use std::path::{Path, PathBuf};

use log::{error, info, warn};

use crate::{
    category::get_category_config, commands::find_category, config, read_dir, read_pattern, Result,
};
use crate::commands::change_forbidden_chars;
use crate::music_tag::get_music_tag;

const MEDIA_EXTENSIONS: [&str; 82] = [
    "3gp", "3g2", "aa", "aac", "aax", "act", "aiff", "alac", "tak", "amr", "ape", "au", "awb",
    "dss", "dvf", "flac", "gsm", "iklax", "ivs", "m4a", "m4b", "m4p", "mmf", "movpkg", "mpg",
    "mp2", "mpeg", "mpe", "mpv", "m2v", "mp3", "mp4", "m4p", "m4v", "mpc", "msv", "nmf", "ogg",
    "oga", "mogg", "ogv", "opus", "ra", "rm", "rmvb", "raw", "rf64", "sln", "tta", "voc", "vox",
    "wav", "wma", "wv", "webm", "8svx", "cda", "mkv", "flv", "f4v", "f4p", "f4a", "f4b", "vob",
    "drc", "gif", "gifv", "mng", "avi", "MTS", "M2TS", "TS", "mov", "qt", "wmv", "yuv", "viv",
    "asf", "amv", "mxf", "roq", "nsv",
]; // from https://en.wikipedia.org/wiki/Audio_file_format and https://en.wikipedia.org/wiki/Video_file_format

/// Is going to check all the music
/// - if in right place
/// - if full album
/// - if all tags
/// -

pub fn check(opt_category: &Option<String>, check_tags: &bool) -> Result<()> {
    //TODO add auto rectify option

    // get config
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    // get directories
    let mut category_dirs: Vec<PathBuf> = vec![];
    if let Some(category) = opt_category {
        category_dirs.push(find_category(category)?)
    } else {
        // get all directories
        let mut category_type_dirs = read_dir(&music_dir, None)?;
        category_type_dirs.retain(|x| x.is_dir());
        category_dirs = category_type_dirs;
    }

    // the real checking
    for category_dir in category_dirs {
        // check config
        let category_config = match get_category_config(&category_dir) {
            Ok(con) => con,
            Err(err) => {
                error!(
                    "Failed to read config of \"{}\" because of {} ",
                    category_dir.display(),
                    err
                );
                continue;
            }
        };
        info!("Checking {}", category_config.name);

        // make vector with all files to check
        let mut album_patterns = vec![];
        if let Some(files) = &config.album_files {
            album_patterns.extend(files)
        };
        if let Some(files) = &category_config.album_files {
            album_patterns.extend(files)
        };

        // check for right place
        if category_config.artist_category.unwrap_or(false) {
            let mut album_dirs = read_dir(&category_dir, None)?;
            album_dirs.retain(|x| x.is_dir());

            for album_dir in album_dirs {
                check_album(None, &album_dir, &config.file_extensions, &album_patterns, check_tags)?;
            }
        } else {
            let mut artist_dirs = read_dir(&category_dir, None)?;
            artist_dirs.retain(|x| x.is_dir());

            for artist_dir in artist_dirs {
                let artist_name = artist_dir.file_name().unwrap().to_string_lossy();
                let mut album_dirs = read_dir(&artist_dir, None)?;
                album_dirs.retain(|x| x.is_dir());

                for album_dir in album_dirs {
                    check_album(
                        Some(&*artist_name),
                        &album_dir,
                        &config.file_extensions,
                        &album_patterns,
                        check_tags,
                    )?;
                }
            }
        }
    }
    Ok(())
}

/// Returns a bool whether there were errors
///
/// true means there were errors the location of the files
fn check_album(
    artist: Option<&str>,
    album_dir: &Path,
    file_extensions: &[String],
    album_patterns: &[&String],
    check_tags: &bool,
) -> Result<()> {
    //TODO add a option to check if some files exist in the album directory

    // get files
    let mut files = read_dir(album_dir, None)?;
    files.retain(|x| x.is_file());

    let mut error = false;

    if files.is_empty() {
        warn!("    No files found at \"{}\"", album_dir.display());
        return Ok(());
    }
    let album_name = album_dir.file_name().unwrap().to_str().unwrap();
    
    if *check_tags && check_path_and_tags(&files, file_extensions, artist, album_name) {
        error = true
    }
    // check files
    for file in files {
        // get extension
        let mut extension = file.extension().unwrap().to_string_lossy().to_string();
        // check extension
        extension.make_ascii_lowercase();
        if !file_extensions.contains(&extension) && MEDIA_EXTENSIONS.contains(&&*extension) {
            warn!(
                "    found media file at \"{}\" but it does not have the right extension",
                { file.display() }
            );
            error = true;
        }
    }
    for pattern in album_patterns {
        if read_pattern(album_dir.join(pattern).to_str().unwrap())?.is_empty() {
            warn!(
                "    Could not find files for pattern: \"{pattern}\" at \"{}\"",
                album_dir.display()
            );
            error = true
        }
    }
    if !error {
        info!("    {album_name} is ok");
    }
    Ok(())
}

/// Checks path based on tags, and check the tags themselves
fn check_path_and_tags(
    files: &[PathBuf],
    file_extensions: &[String],
    artist: Option<&str>,
    album_name: &str,
) -> bool {
    let mut error = false;
    for file in files {
        // get extension
        let extension = file.extension().unwrap().to_string_lossy().to_string();
        // check tags if music file
        if file_extensions.contains(&extension) {
            let tags = match get_music_tag(&file) {
                Ok(tags) => tags,
                Err(err) => {
                    error!(
                        "    Could not get music tags from \"{}\", because of {}",
                        file.display(),
                        err
                    );
                    error = true;
                    continue;
                }
            };
            // check artist
            if let Some(artist) = artist {
                if artist != change_forbidden_chars(&tags.album_artist) {
                    warn!("    {} is in the wrong artist folder", file.display());
                    error = true;
                }
                // check album
                else if album_name != change_forbidden_chars(&tags.album_title) {
                    warn!("    {} is in the wrong album folder", file.display());
                    error = true;
                }
            }
            // check album
            else if album_name != change_forbidden_chars(&tags.album_title) {
                warn!("    {} is in the wrong album folder", file.display());
                error = true;
            }
            // check for replaygain tags
            if !tags.replaygain {
                warn!("    found file with no replaygain tags");
                error = true;
            }
        }
    }
    error
}
