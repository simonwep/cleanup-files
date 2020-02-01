use std::path::PathBuf;
use std::ffi::OsStr;

const SOURCE: &str = "...";
const DESTINATION: &str = "...";

fn main() {
    let path = std::env::current_dir();
    let dir = std::fs::read_dir(SOURCE);

    // Check if read_dir was successful
    if dir.is_err() {
        println!("Unable to read source directory: {}", SOURCE);
        return;
    }

    for entry in dir.unwrap() {
        if entry.is_err() {
            println!("Unable to read file: {}", entry.unwrap().path().to_str().unwrap());
            continue;
        }

        let path = entry.unwrap().path();
        if path.is_file() {
            handle_file(path);
        }
    }
}

/**
 * Resolves the destination directory for a specific file-extension.
 */
fn res_des_dir(ext: &OsStr) -> PathBuf {
    PathBuf::from(DESTINATION).join(ext)
}

/**
 * Moves a file to the corresponding destination directory
 */
fn handle_file(path: PathBuf) {
    let extension = path.extension().unwrap();
    let destination_directory = res_des_dir(&extension);

    if !destination_directory.exists() {
        let result = std::fs::create_dir(&destination_directory);

        if result.is_err() {
            println!("Failed to create directory");
            return;
        }
    }

    let target = PathBuf::from(&destination_directory)
        .join(path.file_name().unwrap());

    print!(
        "Moving {source} to {dest} ... ",
        source = path.to_str().unwrap(),
        dest = target.to_str().unwrap()
    );

    let cpy = std::fs::rename(&path, &target);
    if cpy.is_ok() {
        println!(" Success!")
    } else {
        println!(" Failed!")
    }
}