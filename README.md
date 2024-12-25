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

### [0.1.95]

#### Fixed

- Reduced redundant conversion errors when scanning restricted paths
- Improved error handling for kernel-protected directories
- Optimized path traversal logic

#### Changed

- Updated error messages to be more descriptive
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

scanit '\.png$' -C

## Find JavaScript test files

scanit '^/.*test.*\.js$'

## Find hidden git files

scanit -H '\.git'

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
| `-C, --cd` | Use current directory | - |
| `-H, --hidden` | Show hidden files | - |
| `-h, --help` | Show help | - |
| `-V, --version` | Show version | - |