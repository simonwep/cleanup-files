use std::path::PathBuf;

pub struct Options {
    pub excluded: Vec<String>,
    pub dry_run: bool
}

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
    options: &Options
) -> Result<FileResult, String> {
    let extension = match path.extension() {
        Some(os_str) => os_str,
        None => return Err(format!("Failed to resolve extension of {:?}", path))
    };

    // User might want to exclude certain extension
    if options
        .excluded
        .contains(&extension.to_str().unwrap().to_string())
    {
        return Ok(FileResult::Skipped);
    }

    // Check if dry-run should be performed
    if options.dry_run {
        return Ok(FileResult::Checked);
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

    match std::fs::rename(&path, &target) {
        Ok(_) => Ok(FileResult::Moved),
        Err(_) => Err(String::from("Failed to move file."))
    }
}
