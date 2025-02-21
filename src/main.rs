#![allow(clippy::incompatible_msrv)]

mod printer;
use clap::{value_parser, ArgAction, ColorChoice, CommandFactory, Parser, ValueHint};
use clap_complete::aot::{generate, Shell};
use printer::{write_paths_coloured, write_paths_plain};
use regex::escape as RegexEscape;
use scanit::{find_files_iter, ScanError, SearchConfig};
use std::env::current_dir;
use std::io::stdout;
use std::path::Path;
use std::process::exit as process_exit;
mod constants;
use constants::{AVOID, DOT_PATTERN, START_PREFIX};
use std::io::IsTerminal;

///This is to avoid using the default . pattern, it doesnt show the full path, which considering this is written by a lazy
/// person like me, i dont like it.
#[allow(clippy::must_use_candidate)]
fn resolve_directory(args_cd: bool, args_directory: Option<String>) -> String {
    if args_cd || args_directory.as_ref().is_some_and(|x| x==DOT_PATTERN) {
        current_dir().map_or_else(
            |_| DOT_PATTERN.into(),
            |path_res| {
                path_res
                    .to_str()
                    .map_or_else(|| DOT_PATTERN.into(), Into::into)
            },
        )
    } else {
        let dir_to_use = args_directory.unwrap_or_else(|| START_PREFIX.into());
        let path_check = Path::new(&dir_to_use);
        if !path_check.is_dir() {
            eprintln!("{dir_to_use} is not a directory");
            process_exit(1)
        }
        dir_to_use
    }
}

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(next_line_help = true,term_width = 200,color=ColorChoice::Always)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    #[arg(
        value_name = "PATTERN",
        help = "Pattern to search for",
        required_unless_present = "generate",
        index = 1
    )]
    pattern: Option<String>,
    #[arg(
        value_name = "PATH",
        help = format!("Path to search (defaults to {START_PREFIX})\nUse -c to do current directory"),
        value_hint=ValueHint::DirPath,
        required=false,
        index=2
    )]
    directory: Option<String>,
    #[arg(
        short = 'c',
        long = "current-directory",
        conflicts_with = "directory",
        help = "Uses the current directory to load\n",
        default_value = "false"
    )]
    current_directory: bool,
    #[arg(
        short = 'a',
        long = "show-hidden",
        help = "Shows hidden files eg .gitignore or .bashrc\n"
    )]
    hidden: bool,
    #[arg(
        short = 'e',
        long = "case-insensitive",
        default_value_t = false,
        help = "Enable case-insensitive matching\n"
    )]
    case: bool,
    #[arg(
        short = 'n',
        long = "num-threads",
        default_value_t = env!("CPU_COUNT").parse::<usize>().unwrap_or(1),
        help = "Number of threads to use, defaults to available threads-1",
        value_name = "num-threads"
    )]
    thread_num: usize,
    #[arg(
        short = 'i',
        long = "include-dirs",
        default_value_t = false,
        help = "Include directories\n"
    )]
    keep_dirs: bool,
    #[arg(
        short = 's',
        long = "sys-paths",
        default_value_t = false,
        help = format!("Include system paths {:?}\n", AVOID)
    )]
    keep_sys_paths: bool,
    #[arg(
        short = 'd',
        long = "max-depth",
        required = false,
        help = "Selects the max depth to go to"
    )]
    max_depth: Option<usize>,
    #[arg(
        short = 't',
        long = "top",
        required = false,
        help = "Retrieves the first t results, scanit rs$ -t 10"
    )]
    top_n: Option<usize>,
    #[arg(
        short = 'r',
        long = "regex-escape",
        default_value_t = false,
        required = false,
        help = "Performs a literal search,use semi quotes to wrap your search"
    )]
    regex_escape: bool,
    #[arg(
        long = "generate",
        action = ArgAction::Set,
        value_parser = value_parser!(Shell),
        help = "Generate shell completions"
    )]
    generate: Option<Shell>,
    #[arg(
        short = 'g',
        long = "glob",
        required = false,
        default_value_t = false,
        help = "Use a glob pattern",
        conflicts_with = "regex_escape"
    )]
    glob: bool,
    #[arg(
        short = 'f',
        long = "full-path",
        required = false,
        default_value_t = false,
        help = "Use a full path for regex matching",
        conflicts_with = "glob"
    )]
    full_path: bool,
    #[arg(
        long = "colour",
        alias = "color",
        required = false,
        default_value_t = false,
        help = "Use custom colouring, this is WIP!"
    )]
    colour: bool,
}

fn escape_regex_string(input: &str, avoid_regex: bool, args_glob: bool) -> String {
    if !avoid_regex || args_glob {
        return input.into();
    }
    RegexEscape(input)
}

fn main() -> Result<(), ScanError> {
    let args: Args = Args::parse();

    if let Some(generator) = args.generate {
        let mut cmd = Args::command();
        let cmd_clone = cmd.clone();
        generate(
            generator,
            &mut cmd,
            cmd_clone.get_name().to_string(),
            &mut stdout(),
        );
        return Ok(());
    }

    let pattern = args.pattern.unwrap_or_else(|| {
        eprintln!("Error: Please provide a search pattern");
        process_exit(1)
    });

    let search_config = SearchConfig {
        pattern: &escape_regex_string(&pattern, args.regex_escape, args.glob),
        root: &resolve_directory(args.current_directory, args.directory),
        hide_hidden: args.hidden,
        case_sensitive: args.case,
        thread_count: args.thread_num,
        keep_dirs: args.keep_dirs,
        keep_sys_paths: args.keep_sys_paths,
        max_depth: args.max_depth,
        use_glob: args.glob,
        full_path: args.full_path,
    };

    let files_to_print = find_files_iter(&search_config)?;

    if args.colour && stdout().is_terminal() {
        write_paths_coloured(&files_to_print, args.top_n)?;
    } else {
        write_paths_plain(&files_to_print, args.top_n)?;
    }

    Ok(())
}
