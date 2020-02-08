use std::fs::OpenOptions;
use std::io::Write;

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

    // Log
    let mut log: Vec<(FileResult, String)> = Vec::new();
    for result in dir {
        match result {
            Err(error) => println!("{}", error),
            Ok(entry) => {
                let path = entry.path();
                let raw_path = String::from(path.to_str().unwrap());

                // Skipped current file and other non-file entries
                if path.eq(&current_exe) || !path.is_file() {
                    continue;
                }

                let res = accept(&path, &target, &options);

                // Print message
                match &res {
                    FileResult::Errored(error) => println!("{} {}", "✖ Errored:".red(), error),
                    FileResult::Moved => println!("{} {}", "♻ Moved:".green(), raw_path),
                    FileResult::Skipped => println!("{} {}", "⊙ Skipped:".yellow(), raw_path),
                    FileResult::Checked => println!("{} {}", "✔ Matched:".cyan(), raw_path)
                };

                // Push to logs
                log.push((res, raw_path));
            }
        };
    }

    // Append to log-file if enabled
    // Ignore it if a dry-run is being performed
    if app.has_arg("log") && !app.has_flag("dry") {
        let log_file_path = target.join(
            app.get_arg("log")
                .or(Option::Some(&String::from("cleanup.log")))
                .unwrap()
        );

        if !log_file_path.exists() {
            std::fs::write(&log_file_path, "")
                .ok()
                .expect(&format!("Failed to create log-file {:?}", log_file_path))
        }

        let mut log_file = OpenOptions::new()
            .append(true)
            .open(&log_file_path)
            .unwrap();

        for (res, path) in log {
            log_file
                .write(
                    (match res {
                        FileResult::Errored(error) => format!("[ERRORED] ({}) {}", error, path),
                        FileResult::Moved => format!("[MOVED] {}", path),
                        FileResult::Skipped => format!("[Skipped] {}", path),
                        FileResult::Checked => format!("[Checked] {}", path)
                    })
                    .as_bytes()
                )
                .and(log_file.write("\n".as_bytes()))
                .ok()
                .expect(&format!("Failed to update log-file {:?}", log_file_path));
        }

        println!("{} {:?}", "⚙ Log file updated:".cyan(), &log_file_path);
    }
}
