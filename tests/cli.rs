use assert_cmd::prelude::*;

mod lib;

#[test]
fn simple_sort() {
    lib::test_command(|cmd, test| {
        cmd.arg(".").assert().success();

        test(
            vec![
                ".archive/txt/t1.txt",
                ".archive/txt/t2.txt",
                ".archive/mp4/m1.mp4",
                ".archive/mp4/m2.mp4",
                ".archive/psd/f1.psd",
                ".archive/psd/f2.psd",
                ".archive/cleanup.log",
                ".ignored-file",
            ],
            true
        );
    });
}

#[test]
fn custom_target() {
    lib::test_command(|cmd, test| {
        cmd.arg(".").arg("sorted").assert().success();

        test(
            vec![
                "sorted/txt/t1.txt",
                "sorted/txt/t2.txt",
                "sorted/mp4/m1.mp4",
                "sorted/mp4/m2.mp4",
                "sorted/psd/f1.psd",
                "sorted/psd/f2.psd",
                ".ignored-file",
            ],
            true
        );
    });
}

#[test]
fn exclude_extensions() {
    lib::test_command(|cmd, test| {
        cmd.arg(".").arg("--ext").arg("txt,mp4").assert().success();

        test(
            vec![
                ".archive/psd/f1.psd",
                ".archive/psd/f2.psd",
                "t1.txt",
                "t2.txt",
                "m1.mp4",
                "m2.mp4",
                ".ignored-file",
            ],
            true
        );
    });
}

#[test]
fn dry_run() {
    lib::test_command(|cmd, test| {
        cmd.arg(".")
            .arg("--ext")
            .arg("txt,mp4")
            .arg("-d")
            .assert()
            .success();

        test(
            vec![
                "f1.psd",
                "f2.psd",
                "t1.txt",
                "t2.txt",
                "m1.mp4",
                "m2.mp4",
                ".ignored-file",
            ],
            true
        );
    });
}

#[test]
fn version_help() {
    lib::test_command(|cmd, test| {
        cmd.arg(".")
            .arg("-v")
            .assert()
            .success()
            .stdout(predicates::str::contains(format!(
                "v{}",
                env!("CARGO_PKG_VERSION")
            )));

        // Nothing should change
        test(
            vec![
                "f1.psd",
                "f2.psd",
                "t1.txt",
                "t2.txt",
                "m1.mp4",
                "m2.mp4",
                ".ignored-file",
            ],
            true
        );
    });
}

#[test]
fn invalid_source() {
    lib::test_command(|cmd, _| {
        cmd.arg("./bar/foo").assert().failure();
    });
}

#[test]
fn log_file() {
    lib::test_command(|cmd, test| {
        cmd.arg(".").arg("-l").assert().success();

        test(vec![".archive/cleanup.log"], true);
    });
}

#[test]
fn custom_log_file() {
    lib::test_command(|cmd, test| {
        cmd.arg(".")
            .arg("--log-file")
            .arg("hello.txt")
            .assert()
            .success();

        test(vec![".archive/hello.txt"], true);
    });
}

#[test]
fn disable_log_file() {
    lib::test_command(|cmd, test| {
        cmd.arg(".")
            .arg("--log-file")
            .arg("false")
            .assert()
            .success();

        test(vec![".archive/cleanup.log"], false);
    });
}
