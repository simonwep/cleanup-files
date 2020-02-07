use path_absolutize::Absolutize;
use std::io::Error;
use std::path::PathBuf;

/**
 * Resolves a path and panics if that fails.
 */
fn resolve_path(path: &String) -> PathBuf {
    match PathBuf::from(path).absolutize() {
        Ok(path) => path,
        Err(_) => panic!("Cannot resolve path: {:?}", path)
    }
}
