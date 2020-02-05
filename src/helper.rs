use std::path::PathBuf;

use path_absolutize::Absolutize;

use crate::cli::result::CLIResult;
use crate::utils::fs;

/**
* Resolves source and target directory.
*/
pub fn resolve_directories(result: &CLIResult) -> (PathBuf, PathBuf) {
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
