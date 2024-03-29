<h3 align="center">
    <img src="https://user-images.githubusercontent.com/30767528/73660959-80056800-4699-11ea-8516-4ec50f0e675b.png" width="250" alt="Logo">
</h3>

<h3 align="center">
    Quickly clean up chaos
</h3>

<p align="center">
  <a href="https://github.com/Simonwep/cleanup-files/actions?query=workflow%3ACI"><img
     alt="Build Status"
     src="https://github.com/Simonwep/cleanup-files/workflows/CI/badge.svg"></a>
  <img
     alt="Download count"
     src="https://img.shields.io/github/downloads/Simonwep/cleanup-files/latest/total.svg?color=1C77D8&style=popout-square"/>
  <img alt="Current version"
       src="https://img.shields.io/github/tag/Simonwep/cleanup-files.svg?color=2D8ECE&label=version&style=flat-square">
  <a href="https://github.com/sponsors/Simonwep"><img
     alt="Support me"
     src="https://img.shields.io/badge/github-support-3498DB.svg?style=popout-square"></a>
</p>

### Usage

> Disclaimer:  
> This project was intended to check out rust and learn its concepts and principles. The code is kept simple and I've re-invented a lot
> to learn how it works (I'm aware of libraries such as [`clap`](https://github.com/clap-rs/clap) and [`structop`](https://github.com/TeXitoi/structopt) to build CLIs)

You can find the latest release [here](releases), keep in mind that some commands may not exist in older versions.

```bash
$ ./cleanup -h
```

```
Usage: cleanup <source> <target> [options...]

Flags:
  -d, --dry, --dry-run           Performs a dry-run, e.g. nothing get's moved.
  -h, --help                     Prints this help text.
  -v, --version                  Prints the current version.

Arguments:
  -l, --log-file <file|boolean>  Creates (or disables) a log-file in the target folder. Default is 'c
leanup.log'.
  -i, --include <extensions...>  Move only files with one of the following extensions.
  -e, --exclude <extensions...>  Exclude certain files by their extension.

Values:
  <source>                       Source directory. Default is the current directory.
  <target>                       Target directory (Default is source + .archive).
```

### Examples

| Command | Explanation |
| ------- | ----------- |
| `./cleanup` | This will create a new `./.archive` in the current director and move all files (except these which start with a dot and the executable itself) into `.archive/[extension]/[file].[extension]`. |
| `./cleanup . etc` | It'll still use the current dir as source (`.`) but will move the files into `./etc` instead of `./.archive`.|
| `./cleanup -l --ext mp3,iso` | A `cleanup.log` file will be created inside of `./.archive` with information about what has been moved. Files with the exension `mp3` and `iso` are ignored and won't get moved. |
| `./cleanup ../ ./bam --log-file ../my-log.txt` | Grabs file from the parent-directory and moves them into `./bam` (the current directory). The log-file will be create in the _
current_ directory and is this time `my-log.txt`. |

### Building

This project is written in [rust](https://www.rust-lang.org), clone it via git:

```bash
$ git clone https://github.com/Simonwep/cleanup-files
```

Afterwards you can either run `cargo build` or `cargo test` to test the project.
