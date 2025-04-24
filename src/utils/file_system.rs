use std::{fs, path::{Path, PathBuf}, io};

pub fn get_all_files_in_directory(dir_path: &str) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let path = Path::new(dir_path);
    
    if path.is_file() {
        files.push(path.to_path_buf());
        return Ok(files);
    }
    
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Recursively get files from subdirectory
            let sub_files = get_all_files_in_directory(path.to_str().unwrap_or(""))?;
            files.extend(sub_files);
        } else if path.is_file() {
            files.push(path);
        }
    }
    
    Ok(files)
}