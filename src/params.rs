use std::env::Args;
use std::path::PathBuf;

use path_absolutize::Absolutize;

pub struct Params {
    pub source: PathBuf,
    pub target: PathBuf,
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
 * Parses the command-line-arguments for this cli-utility
 */
pub fn parse_args(args: Args) -> Params {
    let mut source = String::from(".");
    let mut target = String::from("./misc");
    let arguments: Vec<_> = args.collect();
    let argument_count = arguments.len();

    if argument_count > 1 {
        source = arguments.get(1).unwrap().to_owned();
        target = source.clone();
        target.push_str("/misc")
    }

    if argument_count > 2 {
        target = arguments.get(2).unwrap().to_owned();
    }

    return Params {
        source: resolve_path(&source),
        target: resolve_path(&target),
    };
}