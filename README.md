# scanit 🔍

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A command-line utility that recursively searches directories for files matching regex patterns.

## 🚀 Features

- Recursive directory scanning
- Regex pattern matching
- Hidden file support
- Cross-platform compatibility (Unix paths) (Possibly windows but not tested')
- Current directory search option

## ⚙️ Installation

```bash
cargo install scanit
```

## 📝 Changelog

### [0.1.98]

#### Fixed

- Reduced redundant conversion errors when scanning restricted paths
- Improved error handling for kernel-protected directories
- Optimized path traversal logic
- Added new CLI argument for including dirs in searching, defaults to off

#### Changed

- Improved performance when handling permission denied errors

## 🔧 Dependencies

| Dependency | Version | Description |
|:-----------|:--------|:------------|
| [regex](https://crates.io/crates/regex) | 1.11.1 | Regular expression pattern matching |
| [clap](https://crates.io/crates/clap) | 4.4 | Command line argument parsing |
| [jwalk](https://crates.io/crates/jwalk) | 0.8.1 | Fast parallel directory traversal |

Special thanks to:

jwalk - For the excellent parallel directory traversal

regex - For the powerful regex engine


clap - For the robust CLI argument parsing


## Examples

## Find Rust files in /usr

scanit '\.rs$' -d /usr

## Find PNG files from current directory

scanit '\.png$' -c

(or equivalently, scanit '\.png$' --current-directory)

## Find JavaScript test files

scanit '^/.*test.*\.js$'

## Find hidden git files

scanit -a '\.git'

## Search all dirs and file names for a pattern eg zshrc

scanit zshrc -h -i

(or equivalently, scanit zshrc --show-hidden --include-dirs)

##

Supports Unix and in theory windows, not tested yet! VM's are buggy!



Usage: scanit [OPTIONS] <PATTERN>


Arguments:
  <PATTERN>
          Regex pattern to match files (e.g. \\.rs$)
### Options

| Option | Description | Default |
|:-------|:-----------|:--------|
| `-d, --directory <DIR>` | Starting directory | `/` (Unix), `C:/` (Windows) |
| `-c, --cd` | Use current directory | - |
| `-a, --show-hidden` | Show hidden files | - |
| `-h, --help` | Show help | - |
| `-V, --version` | Show version | - |
| `-i  --include-dirs | Include directories in search pattern | Not including dirs|
