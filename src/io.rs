use std::fs;
use std::path::{Path, PathBuf};

use crate::error;

const OUTPUT_FOLDER_NAME: &str = "output/";

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

pub fn init_dirs<P: AsRef<Path>>(relative_dirs: Vec<P>) -> error::EmptyResult {
    for relative_dir in relative_dirs {
        if relative_dir.as_ref().is_absolute() {
            return Err(Box::new(error::SiteError {
                msg: format!(
                    "Initializing directory: {} was not relative",
                    relative_dir.as_ref().display()
                ),
            }));
        }
        if !relative_dir.as_ref().is_dir() {
            return Err(Box::new(error::SiteError {
                msg: format!(
                    "Initializaing directory: {} was not directory",
                    relative_dir.as_ref().display()
                ),
            }));
        }

        let mut new_path = PathBuf::from(OUTPUT_FOLDER_NAME);
        new_path.push(relative_dir);
        fs::create_dir_all(new_path)?;
    }
    Ok(())
}

pub fn write_output_file<P: AsRef<Path>>(relative_file: P, content: String) -> error::EmptyResult {
    if relative_file.as_ref().is_absolute() {
        return Err(Box::new(error::SiteError {
            msg: format!(
                "Input path: {} was not relative",
                relative_file.as_ref().display()
            ),
        }));
    }

    let mut new_path = PathBuf::from(OUTPUT_FOLDER_NAME);
    new_path.push(relative_file);

    fs::write(new_path, content)?;
    Ok(())
}
