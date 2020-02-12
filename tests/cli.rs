use assert_cmd::prelude::*;

mod lib;

#[test]
fn simple_sort() {
    lib::test_command(|cmd| {
        cmd.arg(".").unwrap();

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
        cmd.arg(".").arg("sorted").unwrap();

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
