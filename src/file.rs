use std::path::PathBuf;

use crate::cli::result::CLIResult;

pub enum FileResult {
    Moved,
    Skipped,
    Checked
}

/**
 * Moves a file to the corresponding destination directory
 */
pub fn accept(
    path: &PathBuf,
    destination: &PathBuf,
    app: &CLIResult
) -> Result<FileResult, String> {
    let extension = match path.extension() {
        Some(os_str) => os_str,
        None => return Err(format!("Failed to resolve extension of {:?}", path))
    };

    // User might want to exclude certain extension
    match app.get_arg("exclude") {
        None => (),
        Some(value) => {
            let list: Vec<&str> = value.split(",").collect();

            // Check if extension shall be skipped
            if list.contains(&extension.to_str().unwrap()) {
                return Ok(FileResult::Skipped);
            }
        }
    }

    let destination_directory = PathBuf::from(destination).join(extension);
    if !destination_directory.exists() {
        match std::fs::create_dir(&destination_directory) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!(
                    "Failed to create directory: {:?} ({})",
                    destination_directory,
                    e.to_string()
                ));
            }
        }
    }

    let target = PathBuf::from(&destination_directory).join(path.file_name().unwrap());

    // Check if dry-run should be performed
    if app.has_flag("dry") {
        return Ok(FileResult::Checked);
    }

    match std::fs::rename(&path, &target) {
        Ok(_) => Ok(FileResult::Moved),
        Err(_) => Err(String::from("Failed to move file."))
    }
}
