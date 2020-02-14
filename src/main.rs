#![allow(dead_code)]

use crate::cli::flag::CLIFlag;
use crate::cli::value::CLIValue;
use crate::cli::CLIApp;

mod cli;
mod file;
mod lib;
mod run;

fn main() {
    let cli_app = CLIApp::new()
        .name("cleanup")
        .add_flag(
            CLIFlag::new("log")
                .description("Creates (or disables) a log-file in the target folder. Default is 'cleanup.log'.")
                .default(|_| String::from("cleanup.log"))
                .value_description("file|boolean")
                .abbr("-l")
                .abbr("--log-file")
        )
        .add_flag(
            CLIFlag::new("dry")
                .description("Performs a dry-run, e.g. nothing get's moved.")
                .abbr("-d")
                .abbr("--dry")
                .abbr("--dry-run")
        )
        .add_flag(
            CLIFlag::new("excluded")
                .description("Exclude certain files by their extension.")
                .expects_value(true)
                .value_description("extensions...")
                .abbr("-e")
                .abbr("--ext")
        )
        .add_flag(
            CLIFlag::new("help")
                .description("Prints this help text.")
                .abbr("-h")
                .abbr("--help")
        )
        .add_flag(
            CLIFlag::new("version")
                .description("Prints the current version.")
                .abbr("-v")
                .abbr("--version")
        )
        .add_value(
            CLIValue::new("source")
                .default(|_| ".".to_string())
                .description("Source directory. Default is the current directory.")
        )
        .add_value(
            CLIValue::new("target")
                .default(|list| {
                    let mut source = list.get("source").unwrap().clone();
                    source.push_str("/.archive");
                    source
                })
                .description("Target directory (Default is source + .archive).")
        );

    // Parse arguments
    let app = match cli_app.consume_args() {
        Err(e) => {
            println!("{}\n", e.as_str());
            cli_app.print_help();
            return;
        }
        Ok(v) => v,
    };

    // Check if version or help is requested
    if app.has_flag("help") {
        cli_app.print_help();
        return;
    } else if app.has_flag("version") {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    run::start(app);
}
