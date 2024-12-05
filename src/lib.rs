use std::{
    ffi::OsStr,
    fs::{self, File},
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
};

use glob::{glob_with, MatchOptions, Pattern};

use anyhow::{anyhow, Context, Result};

pub mod category;
pub mod commands;
pub mod config;
pub mod music_tag;
pub mod normalize;
pub mod tag;

/// Create a file with the given content
pub fn create_file(path: &Path, content: String) -> Result<()> {
    // create a file
    let description_file = File::create(path)?;

    // write to the file
    description_file.write_at(content.as_bytes(), 0)?;

    Ok(())
}

/// Gives all the files in that are in a directory,
/// with an option to only include files with a certain file extension
pub fn read_dir(dir: &Path, file_ext: Option<&OsStr>) -> Result<Vec<PathBuf>> {
    // check if it is a directory
    if !dir.is_dir() {
        return Err(anyhow!("Not a directory"));
    }

    // sanitize the directory form pattern matching characters
    let san_dir = PathBuf::from(Pattern::escape(dir.to_str().unwrap()));

    let search = match file_ext {
        Some(ext) => san_dir.join(format!("*{}", ext.to_str().unwrap_or_default())),
        None => san_dir.join("*"),
    };

    let dir = search.to_str().ok_or(anyhow!("Could not find directory"))?;
    read_pattern(dir, true)
}

/// Read a directory recursively to a max depth
/// with an option to only include files with a certain file extension.
/// This function will only give files, not directories
pub fn read_dir_recursive(
    dir: &Path,
    file_ext: Option<&OsStr>,
    max_depth: u8,
) -> Result<Vec<PathBuf>> {
    read_dir_recursive_intern(dir, file_ext, 0, max_depth).context("Failed recursive read")
}

fn read_dir_recursive_intern(
    dir: &Path,
    file_ext: Option<&OsStr>,
    depth: u8,
    max_depth: u8,
) -> Result<Vec<PathBuf>> {
    // check depth
    if depth > max_depth {
        return Ok(vec![]);
    }

    // search directory
    let in_dir = read_dir(dir, None)?;
    let mut result = vec![];

    for file in in_dir {
        if file.is_dir() {
            result.extend(read_dir_recursive_intern(
                &file,
                file_ext,
                depth + 1,
                max_depth,
            )?);
        } else if file_ext.is_none() || file.extension() == file_ext {
            result.push(file);
        }
    }
    Ok(result)
}

/// Gives a string with all the files in that match a path pattern
pub fn read_pattern(pattern: &str, case_sensitive: bool) -> Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = Vec::new();
    let match_options = MatchOptions {
        case_sensitive,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };
    for entry in glob_with(pattern, match_options).context("Glob problem")? {
        result.push(entry.context("Glob entry problem")?);
    }
    Ok(result)
}

/// Searches a vector for the given content
fn search(query: &str, content: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in content {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Move files to the target directory
pub fn move_files(target_files: &Vec<PathBuf>, target_dir: &Path) -> Result<()> {
    for file in target_files {
        if let Err(err) = move_file(&PathBuf::from(file), target_dir) {
            return Err(anyhow!(
                "could not move {} because of {err}",
                file.display()
            ));
        }
    }
    Ok(())
}

/// Move a file to the target directory
pub fn move_file(target_file: &Path, target_dir: &Path) -> Result<()> {
    let file_name = target_file
        .file_name()
        .ok_or(anyhow!("target_file is not a valid file"))?;

    // move the file
    fs::rename(target_file, target_dir.join(file_name)).context("Rename failed")
}
