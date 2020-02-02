use std::path::PathBuf;

mod fs_utils;
mod params;

fn main() {
    let op = params::parse_args(std::env::args());
    let dir = std::fs::read_dir(&op.source)
        .ok().expect(&format!("Failed to read directory: {:?}", op.source));

    println!("Using the following paths:\n Source: {:?}\n Target: {:?}", op.source, op.target);

    // Create missing directories
    match fs_utils::create_dir_tree(&op.target) {
        Ok(_) => (),
        Err(e) => return println!("Critical error: {}", e)
    };

    for entry in dir {
        let path = entry.unwrap().path();

        if path.is_file() {
            match handle_file(&path, &op.target) {
                Ok(_) => println!("Successfully moved {:?}", path),
                Err(error) => println!("{}", error)
            }
        }
    }
}

/**
 * Moves a file to the corresponding destination directory
 */
fn handle_file(path: &PathBuf, destination: &PathBuf) -> Result<(), String> {
    let extension = match path.extension() {
        Some(os_str) => os_str,
        None => return Err(format!("Failed to resolve extension of {:?}", path)),
    };

    let destination_directory = PathBuf::from(destination).join(extension);
    if !destination_directory.exists() {
        match std::fs::create_dir(&destination_directory) {
            Ok(t) => t,
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
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to move file."))
    };
}