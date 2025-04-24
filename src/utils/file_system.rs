use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn get_all_files_in_directory(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let path = Path::new(dir_path);

    if path.is_file() {
        files.push(path.to_path_buf());
        return Ok(files);
    }

    // Stack-based approach to avoid deep recursion
    let mut dirs_to_process = vec![path.to_path_buf()];

    while let Some(current_dir) = dirs_to_process.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                dirs_to_process.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }

    Ok(files)
}
