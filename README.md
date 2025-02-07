# scanit 🔍

A command-line utility that recursively searches directories for files matching regex patterns.

Notably this is my first rust cli. It's probably redundant in favour of fd. It is faster in certain cases(if you don't want to search directories), and probably easier to use!

## 🚀 Features

- Unix and Windows Compatible
- Recursive directory scanning
- Regex pattern matching
- Hidden file support
- Cross-platform compatibility (Unix and Windows)
- Current directory search option

## ⚙️ Installation

```bash
cargo install scanit
```

## 📝 Changelog

### Version=[0.3.5] 🚀

Added a different memory allocator for Linux, this should speed up processing.

It is a lot faster, also rewrote the printing function I used.

Added configuration options:

- Regex escape option, -r, you may need to wrap your expression in quotes/semi-quotes.
- Top n results, show the first n results (These cannot be sorted within rewriting a bit, TODO?)

Better error handling yet again(I will probably use thiserror/anyhow in my next iteration)

### Version=[0.3.3] 🚀

Changed underlying structure to use parallel processing
Added configuration options:

- Thread count control (-n)
- Depth limit (-d)
- Result limit (-t)

Better error handling

### Version=[0.3.0]

It should be a lot faster now!
Changed main directory traversing library to ignore.

Simplified CLI arguments for directory to eg scanit \.py$ [DIRECTORY] (defaults to root)

I have not investigate how it works on Windows for this release yet!

### version=[0.2.5]

Changed underlying structure to better enable use as a library.

Mostly, this means that now the find files returns an Iterator, which allows the collect method(or iteration)

Added a case-insensitivity option.

Increased performance again, roughly about 30% on my limited tests

Lowered Rust Version required to 1.74.1 for people on older Rust versions.

## 🔧 Dependencies

| Dependency | Version | Description |
|:-----------|:--------|:------------|
| [regex](https://crates.io/crates/regex) | 1.11.1 | Regular expression pattern matching |
| [clap](https://crates.io/crates/clap) | 4.5.27 | Command line argument parsing |
| [ignore](https://crates.io/crates/ignore) | 0.4.23 | Fast parallel directory traversal |
| [jemallocator](https://crates.io/crates/jemallocator) | 0.5.4| Memory allocation optimisation |
| [arcstr](https://crates.io/crates/arcstr) | 1.2.0| Better reference counted string types |

## Examples

## Escaping regex and finding .py files from root

scanit '.py' / -r
(This was added as an easier way to search for certain extensions etc.)

## Show the first 10 results for llvm in /usr

scanit llvm /usr -t 10

## Find Rust files in /usr

scanit '\.rs$' /usr

## Find everything on your PC

scanit . -a -i -s      # . = pattern, -a = --show-hidden, -i = --include-dirs, -s = --sys-paths

## Find PNG files from current directory

scanit '\.png$' -c     # -c = --current-directory

## Find JavaScript test files starting from root

scanit '^/.*test.*\.js$'

## Find hidden git files

scanit -a '\.git'

## Search all dirs and file names for a pattern

scanit zshrc -a -i     # -i = --include-directories

## Search case insensitively

scanit PYTHON -e       # -e = --case-insensitive   (e is short for everything and I didn't want to change my current directory argument!)

## Usage Instructions

Usage: scanit [OPTIONS] [PATTERN] [DIRECTORY]
Note: Options can go before or after arguments.

### Arguments

| Argument | Description |
|:---------|:------------|
| PATTERN | Regex pattern to match against filenames |
| DIRECTORY | Path to search (defaults to / on Unix systems or C:/ on Windows) |

### Options

| Option | Description | Default |
|:-------|:-----------|:---------|
| `-c, --current-directory` | Uses the current directory to load | - |
| `-a, --show-hidden` | Shows hidden files (e.g. .gitignore, .bashrc) | - |
| `-e, --case-insensitive` | Enable case-insensitive matching | - |
| `-n, --num-threads <THREAD_NUM>` | Number of threads to use | Available CPU threads minus 1|
| `-i, --include-dirs` | Include directories in search results | - |
| `-s, --sys-paths` | Include system paths (/proc, /sys, /tmp, /run, /dev, /sbin) | - |
| `-d, --depth <MAX_DEPTH>` | Maximum search depth in directories | - |
| `-t, --top <TOP_N>` | Retrieve first N results (no sorting supported) | - |
| `-r, --regex-escape` | Perform literal search instead of regex | - |
| `-h, --help` | Print help information | - |
| `-V, --version` | Show version number | - |
