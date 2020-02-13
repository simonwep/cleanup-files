use assert_cmd::prelude::*;

mod lib;

#[test]
fn simple_sort() {
    lib::test_command(|cmd| {
        cmd.arg(".").assert().success();

        vec![
            "misc/txt/t1.txt",
            "misc/txt/t2.txt",
            "misc/mp4/m1.mp4",
            "misc/mp4/m2.mp4",
            "misc/psd/f1.psd",
            "misc/psd/f2.psd",
            ".ignored-file",
        ]
    });
}

#[test]
fn custom_target() {
    lib::test_command(|cmd| {
        cmd.arg(".").arg("sorted").assert().success();

        vec![
            "sorted/txt/t1.txt",
            "sorted/txt/t2.txt",
            "sorted/mp4/m1.mp4",
            "sorted/mp4/m2.mp4",
            "sorted/psd/f1.psd",
            "sorted/psd/f2.psd",
            ".ignored-file",
        ]
    });
}

#[test]
fn exclude_extensions() {
    lib::test_command(|cmd| {
        cmd.arg(".").arg("--ext").arg("txt,mp4").assert().success();

        vec![
            "misc/psd/f1.psd",
            "misc/psd/f2.psd",
            "t1.txt",
            "t2.txt",
            "m1.mp4",
            "m2.mp4",
            ".ignored-file",
        ]
    });
}

#[test]
fn dry_run() {
    lib::test_command(|cmd| {
        cmd.arg(".")
            .arg("--ext")
            .arg("txt,mp4")
            .arg("-d")
            .assert()
            .success();

        vec![
            "f1.psd",
            "f2.psd",
            "t1.txt",
            "t2.txt",
            "m1.mp4",
            "m2.mp4",
            ".ignored-file",
        ]
    });
}

#[test]
fn version_help() {
    lib::test_command(|cmd| {
        cmd.arg(".")
            .arg("-v")
            .assert()
            .success()
            .stdout(predicates::str::contains(format!(
                "v{}",
                env!("CARGO_PKG_VERSION")
            )));

        // Nothing should change
        vec![
            "f1.psd",
            "f2.psd",
            "t1.txt",
            "t2.txt",
            "m1.mp4",
            "m2.mp4",
            ".ignored-file",
        ]
    });
}

#[test]
fn invalid_source() {
    lib::test_command(|cmd| {
        cmd.arg("./bar/foo").assert().failure();

        vec![]
    });
}

#[test]
fn log_file() {
    lib::test_command(|cmd| {
        cmd.arg(".").arg("-l").assert().success();

        vec!["misc/cleanup.log"]
    });
}

#[test]
fn custom_log_file() {
    lib::test_command(|cmd| {
        cmd.arg(".")
            .arg("--log-file")
            .arg("hello.txt")
            .assert()
            .success();

        vec!["misc/hello.txt"]
    });
}
