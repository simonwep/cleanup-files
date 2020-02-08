<h3 align="center">
    <img src="https://user-images.githubusercontent.com/30767528/73660959-80056800-4699-11ea-8516-4ec50f0e675b.png" width="250" alt="Logo">
</h3>

<h3 align="center">
    Quickly clean up chaos
</h3>


### Usage
> Disclaimer:  
> This project was intended to checkout rust and learn how it works. The code is kept simple and I've re-invented a lot to get a feel
> for it. I'm aware of libraries to manage CLI Commands and design-patterns such as the builder-pattern were intentionally implemented
> that way. The're probabaly ways to solve the problems I've faced, way better as I chose.

You can find the latest release [here](releases), keep in mind that some commands may not exist
in older versions.

```
Usage: cleanup <source> <target> [options...]
  -l, --log-file <file>      Creates / updates a log-file in the target folder.
  -d, --dry, --dry-run       Performs a dry-run, e.g. nothing get's moved.
  -e, --ext <extensions...>  Exclude certain files by their extension.
  -h, --help                 Prints this help text.
  -v, --version              Prints version.
```
> Use `./clearup.exe -h` or `./clearup.exe --help` to see the content above.

### Building
This project is written in [rust](https://www.rust-lang.org), clone it via git: 
```bash
$ git clone https://github.com/Simonwep/cleanup-files
```

Afterwards you can either run `cargo build` or use [`./release.sh`](release.sh).

[`release.sh`](release.sh) will automatically create a `checksums.txt` file in `/target/release` with several checksums for the executable. If you're on a
windows-machine the script will additionally download [rcedit](https://github.com/electron/rcedit) to set the [icon](icon.ico) of the `.exe` file.

There are no tests written for the tool _itself_ but for tools (like the [cli-parser](src/cli)) which can be run by `cargo test`.