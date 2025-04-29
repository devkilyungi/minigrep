//! Utility functions for the minigrep tool.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Recursively gathers all files in a directory.
///
/// # Arguments
///
/// * `dir_path` - Path to the directory to scan
///
/// # Returns
///
/// * `Result<Vec<PathBuf>, io::Error>` - Collection of file paths or an error
///
/// # Errors
///
/// Returns an error if:
/// - The directory doesn't exist
/// - The directory can't be read
/// - A subdirectory can't be accessed
pub fn get_all_files_in_directory(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let path = Path::new(dir_path);

    if path.is_file() {
        // Skip the file if it's a hidden file
        if !is_hidden_file(path) {
            files.push(path.to_path_buf());
        }
        return Ok(files);
    }

    // Stack-based approach to avoid deep recursion
    let mut dirs_to_process = vec![path.to_path_buf()];

    while let Some(current_dir) = dirs_to_process.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Skip if the file or directory is hidden
            if is_hidden_file(&path) {
                continue;
            }

            if path.is_dir() {
                dirs_to_process.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    Ok(files)
}

fn is_hidden_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with("."))
        .unwrap_or(false)
}
