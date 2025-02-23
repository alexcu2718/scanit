# scanit 🔍

A command-line utility that recursively searches directories for files matching regex patterns.

Notably this is my first rust cli. It's probably redundant in favour of fd. It is faster in certain cases(if you don't want to search directories), and probably easier to use!

## Features

- Unix and Windows Compatible
- Recursive directory scanning
- Regex pattern matching
- Hidden file support
- Cross-platform compatibility (Unix and Windows)
- Current directory search option

## Installation

```bash
cargo install scanit
```

## Changelog

### Version=[0.3.9]

Increased efficiency of matching algorithm for colours.
Added colours to display as an environment variable.

You can force colours by  by using 'export SCANIT_COLOUR=true' (add it to your shell environment)

Changed memory allocator choices for Linux, kept mimalloc for Windows
Fixed minor bugs. Unnecessary conversions

### Version=[0.3.8]

Dramatically increased colour selection, sped up colour matching DRAMATICALLY(thanks memchr!).
Fixed some slight inefficiency in the full path options.
!!! NOTE I HAVE NOT TESTED THE COLOUR MATCHING ON WINDOWS, THIS IS ON THE LIST TO DO.
SO COLOUR MATCHING IS NOT AVAILABLE, WILL BE IN NEXT RELEASE.
The speed is on par or better than fd, with a lot better colours!

### Version=[0.3.7]

#### Breaking Change

Changed the structure of the code to work on bytes.
The convenience function 'find_files' now returns an OsString (because I prefer to leave this option to the caller)

#### New Features

##### Shell Completions

- Added shell completion support
- Setup command:

Put the following into your respective shell config

  ```bash
  eval "$(scanit --generate $SHELL_NAME)"
  ```

- Supported shells: `bash`, `elvish`, `fish`, `powershell`, `zsh`

##### Glob Pattern Search

- Added `-g` flag for glob-style pattern matching
- Searches entire path names
- Examples:

  ```bash
  scanit "**py3**.py" -g    # Find Python files with 'py3' in path
  ```

##### Full/Short path file matching

Added an option to search full paths/file ends only
NOTE: glob works on full paths by default.

#### Experimental WIP feature

I've added an option --colour option (also aliased to --color for yanks, basically it colours depending on file extension endings), I've not tested it too much, I still have a lot to work on.
Such as implementing a file config option for it.
Don't expect much, it carries a MINOR bit of overhead.

#### Bug Fixes

- Fixed regex matching to only work on filenames(not on glob though)
- Improved output handling for non-UTF8 characters
  - Switched to byte-based regex implementation
  - Enhanced output handling for better compatibility

#### Future Plans

- **Planned Features:**
  - File type matching support
- **Under Consideration:**
  - Colored LS-style output
  - File type search based on extensions
  - Note: Performance-optimised version using SIMD exists but requires nightly Rust

### Version=[0.3.6]

Simplified code base a bit, added thiserror for better error handling(it shouldn't return errors for missed file paths due to permissions errors, because this will happen a lot!)

Added path checking to check if valid path.

Started to use arcstr crate due to avoiding less memory allocations.

### Version=[0.3.5]

Added a different memory allocator for Linux, this should speed up processing.

It is a lot faster, also rewrote the printing function I used.

Added configuration options:

- Regex escape option, -r, you may need to wrap your expression in quotes/semi-quotes.
- Top n results, show the first n results (These cannot be sorted within rewriting a bit, TODO?)

Better error handling yet again(I will probably use thiserror/anyhow in my next iteration)

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
|:-------|:------------|:---------|
| `-c, --current-directory` | Uses the current directory to load | - |
| `-a, --show-hidden` | Shows hidden files (e.g. .gitignore, .bashrc) | - |
| `-e, --case-insensitive` | Enable case-insensitive matching | - |
| `-n, --num-threads <THREAD_NUM>` | Number of threads to use | Available CPU threads |
| `-i, --include-dirs` | Include directories in search results | - |
| `-s, --sys-paths` | Include system paths (/proc, /sys, /tmp, /run, /dev, /sbin) | - |
| `-d, --depth <MAX_DEPTH>` | Maximum search depth in directories | - |
| `-t, --top <TOP_N>` | Retrieve first N results (no sorting supported) | - |
| `-r, --regex-escape` | Perform literal search (conflicts with `--glob`) | - |
| `--generate` | Generate completions [bash, elvish, fish, powershell, zsh] | - |
| `-g, --glob` | Use glob pattern matching (conflicts with `--regex-escape` and `--full-path`) | - |
| `--colour` | Colour output depending on file extension, it's not extensive yet. WIP | - |
| `-f, --full-path` | Match regex against full path (conflicts with `--glob`) | - |
| `-h, --help` | Print help information | - |
| `-V, --version` | Show version number | - |
