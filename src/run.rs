use std::fs::OpenOptions;
use std::io::Write;

use chrono::Utc;
use colored::Colorize;

use crate::cli::result::CLIResult;
use crate::file::{accept, FileResult, Options};
use crate::lib::resolve_directories;

fn resolve_cs_list(val: Option<&String>) -> Option<Vec<String>> {
    if val.is_none() {
        return Option::None;
    }

    Option::Some(val.unwrap().split(",").map(|s| s.to_string()).collect())
}

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
        excluded: resolve_cs_list(app.get_arg("excluded")),
        included: resolve_cs_list(app.get_arg("included")),
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
                    FileResult::Moved(_) => println!("{} {}", "♻ Moved:".green(), raw_path),
                    FileResult::Skipped => println!("{} {}", "⊙ Skipped:".yellow(), raw_path),
                    FileResult::Checked => println!("{} {}", "✔ Matched:".cyan(), raw_path),
                };

                // Push to logs
                log.push((res, raw_path));
            }
        };
    }

    // Don't create a log-file if a dry-run is being performed
    if !app.has_flag("dry") {
        let default_log_file = &String::from("cleanup.log");
        let log_file = app
            .get_arg("log")
            .or(Option::Some(default_log_file))
            .unwrap();

        if log_file.eq("false") {
            return;
        }

        let log_file_path = target.join(log_file);
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
            let mut content = Utc::now().format("%Y-%m-%d %H:%M:%S ").to_string();
            content.push_str(
                (match res {
                    FileResult::Errored(error) => format!("[ERRORED] {} ({})", path, error),
                    FileResult::Moved(dest) => {
                        format!("[MOVED] {} -> {}", path, dest.to_str().unwrap())
                    }
                    FileResult::Skipped => format!("[SKIPPED] {}", path),
                    FileResult::Checked => format!("[CHECKED] {}", path),
                })
                .as_str(),
            );

            log_file
                .write(content.as_bytes())
                .and(log_file.write("\n".as_bytes()))
                .ok()
                .expect(&format!("Failed to update log-file {:?}", log_file_path));
        }

        println!("{} {:?}", "⚙ Log file updated:".cyan(), &log_file_path);
    }
}
