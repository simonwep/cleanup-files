use colored::Colorize;

use crate::cli::result::CLIResult;
use crate::file::{accept, FileResult, Options};
use crate::helper::resolve_directories;

pub fn start(app: CLIResult) {
    // Resolve current executable to prevent sorting it
    let current_exe = std::env::current_exe()
        .ok()
        .expect("Failed to resolve current executable.");

    // Source and target directory
    let (source, target) = resolve_directories(&app);

    println!(
        "Using the following paths:\n | Source: {:?}\n | Target: {:?}\n",
        source, target
    );

    let options = Options {
        dry_run: app.has_flag("dry"),
        excluded: app
            .get_arg("excluded")
            .or(Option::Some(&String::default()))
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .collect()
    };

    // Parse arguments and read directory entries
    let dir = std::fs::read_dir(&source)
        .ok()
        .expect(&format!("Failed to read directory: {:?}", source));

    for result in dir {
        match result {
            Err(error) => println!("{}", error),
            Ok(entry) => {
                let path = entry.path();

                // Skipped current file and other non-file entries
                if path.eq(&current_exe) || !path.is_file() {
                    continue;
                }

                match accept(&path, &target, &options) {
                    Err(error) => println!("{} {:?}", "✖ Errored:".red(), error),
                    Ok(msg) => match msg {
                        FileResult::Moved => println!("{} {:?}", "♻ Moved:".green(), path),
                        FileResult::Skipped => println!("{} {:?}", "⊙ Skipped:".yellow(), path),
                        FileResult::Checked => println!("{} {:?}", "✔ Matched:".cyan(), path)
                    }
                };
            }
        };
    }
}
