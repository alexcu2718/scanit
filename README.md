# scanit
This is my first rust project, it's got rough edges!

A command line utility that recursively searches directories for files matching regex patterns.

Examples:
scanit \.rs$ -d /usr         # Find Rust files in /usr

scanit '\.png$' --C          # Find PNG files from current directory

scanit '^test.*\.js$'        # Find JavaScript files starting with 'test'

scanit -H '\.git'            # Find hidden git files

scanit -V                    # Show version number


Supports Unix and Windows paths automatically.

Usage: scanit [OPTIONS] <PATTERN>

Arguments:
  <PATTERN>
          Regex pattern to match files (e.g. \\.rs$)

Options:
  -d, --directory <DIRECTORY>
          Starting directory for search
          [default: /] if linux/mac
          [default: C:/] if windows

  -C, --cd
          Use current directory for search

  -H, --hidden
          Show hidden files (e.g. .gitignore, .bashrc)

  -h, --help
          Print help information

  -V, --version
          Print version information