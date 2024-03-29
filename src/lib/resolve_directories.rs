use std::path::PathBuf;

use path_absolutize::Absolutize;

use crate::cli::result::CLIResult;

/**
* Resolves source and target directory.
*/
pub fn resolve_directories(app: &CLIResult) -> (PathBuf, PathBuf) {
    let source_path = PathBuf::from(app.get_value("source").unwrap().clone())
        .absolutize()
        .unwrap();

    let target_path = PathBuf::from(app.get_value("target").unwrap().clone())
        .absolutize()
        .unwrap();

    // Check if source-dir exists
    if !source_path.exists() {
        panic!("Source not found: {:?}", source_path);
    }

    // Create missing directories
    if !app.has_flag("dry") {
        match std::fs::create_dir_all(&target_path) {
            Ok(_) => (),
            Err(e) => panic!("Critical error: {}", e),
        };
    }

    return (source_path, target_path);
}
