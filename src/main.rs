use std::ffi::OsStr;
use std::path::PathBuf;

mod params;

fn main() {
    let op = params::parse_args(std::env::args());
    let dir = std::fs::read_dir(&op.source);

    // Check if read_dir was successful
    if dir.is_err() {
        println!("Unable to read source directory: {:?}", &op.source);
        return;
    }

    for entry in dir.unwrap() {
        if entry.is_err() {
            println!("Unable to read file: {}", entry.unwrap().path().to_str().unwrap());
            continue;
        }

        let path = entry.unwrap().path();
        if path.is_file() {
            handle_file(&path, &op.target);
        }
    }
}

/**
 * Resolves the destination directory for a specific file-extension.
 */
fn res_des_dir(ext: &OsStr, destination: &OsStr) -> PathBuf {
    PathBuf::from(destination).join(ext)
}

/**
 * Moves a file to the corresponding destination directory
 */
fn handle_file(path: &PathBuf, destination: &PathBuf) {
    let extension = path.extension().unwrap();
    let destination_directory = res_des_dir(extension, destination.as_os_str());

    if !destination_directory.exists() {
        let result = std::fs::create_dir(&destination_directory);

        if result.is_err() {
            println!("Failed to create directory \"{}\"", destination_directory.to_str().unwrap());
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