use std::collections::HashMap;
use std::env::Args;
use std::path::PathBuf;

use path_absolutize::Absolutize;

use crate::utils;

pub struct CLIArguments {
    pub source: PathBuf,
    pub target: PathBuf,
    pub flags: Vec<String>,
    pub args: HashMap<String, String>,
}

fn print_help() {
    println!("Usage: cleanup <source> <target> [options]");
    println!("  -h, --help This help text");
}

/**
 * Resolves a path and terminates the process if that fails.
 */
fn resolve_path(path: &String) -> PathBuf {
    match PathBuf::from(path).absolutize() {
        Ok(path) => {
            return path;
        }
        Err(_) => {
            println!("Cannot resolve path: {:?}", path);
            std::process::exit(-1);
        }
    }
}

/**
 * Parses the command-line-arguments for this utils-utility
 */
pub fn parse_args(raw_args: Args) -> CLIArguments {
    let parsed_args = utils::cli_utils::parse_raw_args(raw_args);
    let mut source = String::from(".");
    let mut target = String::from("./.archive");
    let values_length = parsed_args.values.len();

    // Check if help-text is requested
    if parsed_args.flags.contains(&String::from("-h")) ||
        parsed_args.flags.contains(&String::from("--help")) {
        print_help();
        std::process::exit(0);
    }

    if values_length > 2 {
        println!("Expected maximum of 2 paths, got {}.", values_length);
        print_help();
        std::process::exit(-1);
    }

    if values_length > 0 {
        source = parsed_args.values.get(0).unwrap().to_owned();
        target = source.clone();
        target.push_str("/misc")
    }

    if values_length > 1 {
        target = parsed_args.values.get(1).unwrap().to_owned();
    }

    return CLIArguments {
        source: resolve_path(&source),
        target: resolve_path(&target),
        args: parsed_args.args,
        flags: parsed_args.flags,
    };
}