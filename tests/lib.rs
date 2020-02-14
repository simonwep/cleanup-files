use std::path::PathBuf;
use std::process::Command;

use assert_cmd::prelude::*;
use rand::Rng;
use remove_dir_all::*;

/// Verifies a file-tree based on a list of entries
pub fn verify_file_tree(paths: Vec<String>, expected: bool) {
    for path_str in paths {
        let path = PathBuf::from(path_str);

        if path.exists() != expected {
            panic!(format!("Path not found: {:?}", path));
        };
    }
}

/// Tests a command and compares the file-structure with the result.
pub fn test_command(test: fn(&mut Command, &dyn Fn(Vec<&str>, bool))) {
    let hash: String = (0..10)
        .map(|_| rand::thread_rng().gen_range(97 as u8, 122 as u8) as char)
        .collect();

    let dir = format!("tests/{}/", hash);
    std::fs::create_dir_all(&dir).unwrap();

    // Create test files
    for file in vec![
        "t1.txt",
        "t2.txt",
        "m1.mp4",
        "m2.mp4",
        "f1.psd",
        "f2.psd",
        ".ignored-file",
    ] {
        let mut file_path = String::default();
        file_path.push_str(&dir);
        file_path.push_str(&file);
        std::fs::write(&file_path, "").unwrap();
    }

    // Execute test commands
    test(
        &mut Command::cargo_bin("cleanup").unwrap().current_dir(&dir),
        &|mut vec, expected| {
            verify_file_tree(
                vec.iter_mut()
                    .map(|s| {
                        let mut string = String::default();
                        string.push_str(&dir);
                        string.push_str(s);
                        string
                    })
                    .collect(),
                expected
            )
        }
    );

    // Clean up
    remove_dir_all(&dir).unwrap();
}
