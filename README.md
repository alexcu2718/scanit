# scanit 🔍

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A command-line utility that recursively searches directories for files matching regex patterns.

## 🚀 Features

- Recursive directory scanning
- Regex pattern matching
- Hidden file support
- Cross-platform compatibility (Unix paths)
- Current directory search option

## ⚙️ Installation

```bash
cargo install scanit
```

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