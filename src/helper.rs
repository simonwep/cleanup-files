use std::path::PathBuf;

use path_absolutize::Absolutize;

use crate::cli::result::CLIResult;
use crate::utils::fs;

/**
* Resolves source and target directory.
*/
pub fn resolve_directories(result: &CLIResult) -> (PathBuf, PathBuf) {
    let source_path = PathBuf::from(result.get_value("source").unwrap().clone())
        .absolutize()
        .unwrap();

    let target_path = PathBuf::from(result.get_value("target").unwrap().clone())
        .absolutize()
        .unwrap();

    // Create missing directories
    match fs::create_dir_tree(&target_path) {
        Ok(_) => (),
        Err(e) => panic!("Critical error: {}", e)
    };

    return (source_path, target_path);
}
