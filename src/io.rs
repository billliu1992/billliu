use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error;

const OUTPUT: &str = "output/";
const INPUT: &str = "input/";

pub fn read_dir<P: AsRef<Path>>(
    relative_dir: P,
    handler: &mut dyn FnMut(String, String) -> error::EmptyResult,
) -> error::EmptyResult {
    recursively_read_directory(create_path(INPUT, relative_dir)?, handler)
}

pub fn init_dirs<P: AsRef<Path>>(relative_dirs: Vec<P>) -> error::EmptyResult {
    for relative_dir in relative_dirs {
        let path = create_path(OUTPUT, relative_dir)?;

        if path.as_ref().exists() && !path.as_ref().is_dir() {
            return Err(Box::new(error::SiteError {
                msg: format!(
                    "Initializaing directory: {} is not a dir",
                    path.as_ref().display()
                ),
            }));
        }

        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn write_output_file<P: AsRef<Path>>(relative_file: P, content: String) -> error::EmptyResult {
    let path = create_path(OUTPUT, relative_file)?;

    fs::write(path, content)?;
    Ok(())
}

pub fn copy_to_output<P: AsRef<Path> + Copy>(relative_file: P) -> error::EmptyResult {
    fs::copy(
        create_path(INPUT, relative_file)?,
        create_path(OUTPUT, relative_file)?,
    )?;
    Ok(())
}

fn recursively_read_directory<P: AsRef<Path>>(
    root_dir: P,
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

fn create_path<P: AsRef<Path>>(
    base_path: &str,
    relative_path: P,
) -> Result<Box<Path>, Box<dyn Error>> {
    if relative_path.as_ref().is_absolute() {
        return Err(Box::new(error::SiteError {
            msg: format!(
                "Input path: {} was not relative",
                relative_path.as_ref().display()
            ),
        }));
    }

    let mut new_path = PathBuf::from(base_path);
    new_path.push(relative_path);

    Ok(new_path.into_boxed_path())
}
