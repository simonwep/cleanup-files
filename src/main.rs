use std::path::PathBuf;

use crate::cli::CLIArguments;

mod utils;
mod cli;

fn main() {

    // Resolve current executable to prevent sorting it
    let current_exe = std::env::current_exe()
        .ok().expect("Failed to resolve current executable.");

    // Parse arguments and read directory entries
    let cli = cli::parse_args(std::env::args());
    let dir = std::fs::read_dir(&cli.source)
        .ok().expect(&format!("Failed to read directory: {:?}", cli.source));

    println!("Using the following paths:\n Source: {:?}\n Target: {:?}", cli.source, cli.target);

    // Create missing directories
    match utils::fs_utils::create_dir_tree(&cli.target) {
        Ok(_) => (),
        Err(e) => return println!("Critical error: {}", e)
    };

    for result in dir {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // Path should point to a file and not be the current executable
                if path != current_exe && path.is_file() {
                    match handle_file(&path, &cli.target, &cli) {
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
fn handle_file(path: &PathBuf, destination: &PathBuf, cli: &CLIArguments) -> Result<String, String> {
    let extension = match path.extension() {
        Some(os_str) => os_str,
        None => return Err(format!("Failed to resolve extension of {:?}", path)),
    };

    // User might want to exclude certain extension
    match cli.get_arg_value("--exclude", "-e") {
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
            Err(e) => return Err(
                format!("Failed to create directory: {:?} ({})",
                        destination_directory,
                        e.to_string()
                )
            )
        }
    }

    let target = PathBuf::from(&destination_directory)
        .join(path.file_name().unwrap());

    return match std::fs::rename(&path, &target) {
        Ok(_) => Ok(String::from("Successfully moved")),
        Err(_) => Err(String::from("Failed to move file."))
    };
}