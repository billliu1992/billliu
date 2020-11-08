use std::path::Path;
use std::fs;

use crate::error;

pub fn recursively_read_directory(
    root_dir: &Path,
    handler: &mut dyn FnMut(String, String) -> error::EmptyResult,
) -> error::EmptyResult {
    let mut view_dir_files = fs::read_dir(root_dir)?;
    while let Some(dir_entry) = view_dir_files.next() {
        let path = dir_entry?.path();
        if path.is_dir() {
            recursively_read_directory(path.as_path(), handler)?;
        } else {
            let file_stem = path
                .file_stem()
                .map(|f| f.to_str())
                .flatten()
                .map(|s| s.to_string());

            if let Some(stem) = file_stem {
                handler(stem, fs::read_to_string(path)?)?;
            }
        }
    }
    Ok(())
}