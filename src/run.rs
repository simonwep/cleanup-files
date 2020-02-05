use std::path::PathBuf;

use path_absolutize::Absolutize;

use crate::cli::result::CLIResult;
use crate::utils::fs;

/**
 * Resolves source and target directory.
 */
fn resolve_directories(result: &CLIResult) -> (PathBuf, PathBuf) {
    let mut source = String::from(".");
    let mut target = String::from("./.archive");

    if result.has_value("source") {
        source = result.get_value("source").unwrap().clone();
        target = source.clone();
        target.push_str("/misc")
    }

    if result.has_value("target") {
        target = result.get_value("target").unwrap().clone();
    }

    let source_path = PathBuf::from(source).absolutize().unwrap();
    let target_path = PathBuf::from(target).absolutize().unwrap();

    // Create missing directories
    match fs::create_dir_tree(&target_path) {
        Ok(_) => (),
        Err(e) => panic!("Critical error: {}", e)
    };

    return (source_path, target_path);
}

pub fn move_files(app: CLIResult) {
    // Resolve current executable to prevent sorting it
    let current_exe = std::env::current_exe()
        .ok()
        .expect("Failed to resolve current executable.");

    // Source and target directory
    let (source, target) = resolve_directories(&app);

    // Parse arguments and read directory entries
    let dir = std::fs::read_dir(&source)
        .ok()
        .expect(&format!("Failed to read directory: {:?}", source));

    println!(
        "Using the following paths:\n Source: {:?}\n Target: {:?}",
        source, target
    );

    for result in dir {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // Path should point to a file and not be the current executable
                if path != current_exe && path.is_file() {
                    match handle_file(&path, &target, &app) {
                        Ok(msg) => println!("({}) {:?}", msg, path),
                        Err(error) => println!("{}", error)
                    }
                }
            }
            Err(error) => println!("{}", error)
        }
    }
}

/**
 * Moves a file to the corresponding destination directory
 */
fn handle_file(path: &PathBuf, destination: &PathBuf, app: &CLIResult) -> Result<String, String> {
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
                return Ok(String::from("Skipped by its extension"));
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
        return Ok(String::from("Dry run - ok"));
    }

    return match std::fs::rename(&path, &target) {
        Ok(_) => Ok(String::from("Ok")),
        Err(_) => Err(String::from("Failed to move file."))
    };
}
