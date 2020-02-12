<h3 align="center">
    <img src="https://user-images.githubusercontent.com/30767528/73660959-80056800-4699-11ea-8516-4ec50f0e675b.png" width="250" alt="Logo">
</h3>

<h3 align="center">
    Quickly clean up chaos
</h3>


### Usage
> Disclaimer:  
> This project was intended to check out rust and learn its concepts and principles. The code is kept simple and I've re-invented a lot 
> to learn how it works (I'm aware of libraries such as [`clap`](https://github.com/clap-rs/clap) and [`structop`](https://github.com/TeXitoi/structopt) to build CLIs)

You can find the latest release [here](releases), keep in mind that some commands may not exist
in older versions.

```bash
$ ./cleanup -h
```

```
Usage: cleanup <source> <target> [options...]

Flags:
  -d, --dry, --dry-run       Performs a dry-run, e.g. nothing get's moved.
  -h, --help                 Prints this help text.
  -v, --version              Prints the current version.

Arguments:
  -l, --log-file <file>      Creates / updates a log-file in the target folder.
  -e, --ext <extensions...>  Exclude certain files by their extension.

Values:
  <source>                   Source directory (Default is the current one).
  <target>                   Target directory (Default is source + ./misc).
```
> Use `./clearup.exe -h` or `./clearup.exe --help` to see the content above.

### Building
This project is written in [rust](https://www.rust-lang.org), clone it via git: 
```bash
$ git clone https://github.com/Simonwep/cleanup-files
```

Afterwards you can either run `cargo build` or use [`./release.sh`](release.sh) to build a production-ready version of it.

> [`release.sh`](release.sh) will automatically create a `checksums.txt` file in `/target/release` with several checksums for the executable. If you're on a
windows-machine the script will additionally download [rcedit](https://github.com/electron/rcedit) to set the [icon](icon.ico) of the `.exe` file.
