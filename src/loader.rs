//! # Task definitions loader

use super::errors::{Result, err_definitions_not_found};
use std::fs::{read_dir, read_to_string};

/// Returns the content of the task definitions file.
///
/// Error handling is intentionally minimal. File access and filesystem errors
/// are not reported precisely because such failures are expected to be rare.
/// This function will just panic if the task definitions file is not accessible.
///
/// # Errors
///
/// (tbd)
///
/// # Panics
///
/// TODO Get rid of panics
pub fn load_definitions() -> Result<String> {
    let mut files = vec![];
    let paths = read_dir(".").expect("Failed to access current directory");
    for entry in paths.flatten() {
        let file_type = entry.file_type().expect("Failed to get file type");
        if file_type.is_file() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if file_name.ends_with(".taskcmd") {
                files.push(file_name);
            }
        }
    }
    files.sort();
    files.reverse();
    files.first().map_or_else(
        || Err(err_definitions_not_found()),
        |file_name| Ok(read_to_string(file_name).expect("Failed to read task definitions file")),
    )
}
