use crate::cli::result::CLIResult;
use crate::file::{accept, FileResult};
use crate::helper::resolve_directories;

pub fn start(app: CLIResult) {
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

                // Skipped current file and other non-file entries
                if path == current_exe && !path.is_file() {
                    return;
                }

                match accept(&path, &target, &app) {
                    Err(error) => println!("{}", error),
                    Ok(msg) => match msg {
                        FileResult::Moved => println!("[moved] {:?}", path),
                        FileResult::Skipped => println!("[skipped] {:?}", path),
                        FileResult::Checked => println!("[matched] {:?}", path)
                    }
                }
            }
            Err(error) => println!("{}", error)
        }
    }
}
