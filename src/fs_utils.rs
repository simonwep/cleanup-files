use std::io::Error;
use std::path::PathBuf;

/**
 * Creates missing directories in a path.
 */
pub fn create_dir_tree(path: &PathBuf) -> Result<(), Error> {
    let as_array: Vec<_> = path.iter().collect();

    for index in 1..(as_array.len() + 1) {
        let sub: PathBuf = (&as_array[0..index]).iter().collect();

        if !sub.exists() {
            match std::fs::create_dir(&sub) {
                Err(e) => return Err(e),
                Ok(t) => ()
            };
        }
    }

    return Ok(());
}