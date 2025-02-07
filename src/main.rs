


mod printer;
use printer::write_paths;
mod convenience_functions;
use convenience_functions::{get_threads, resolve_directory,escape_regex_string};
use scanit::{find_files_iter, AVOID,START_PREFIX,ScanError};
use clap::Parser;

#[derive(Parser)]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    #[arg(required_unless_present = "version")]
    pattern: String,
    #[arg(required = false,help=format!("Path to search, defaults to {}\n",START_PREFIX))]
    directory: Option<String>,
    #[arg(
        short = 'c',
        long = "current-directory",
        conflicts_with = "directory",
        help = "Uses the current directory to load\n",
        default_value = "false"
    )]
    cd: bool,
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
    #[arg( short='n',long = "num-threads",default_value_t =  get_threads(),help = "Number of threads to use, defaults to available threads on your pc\n")]
    thread_num: usize,
    #[arg(
        short = 'i',
        long = "include-dirs",
        default_value_t = false,
        help = "Include directories\n"
    )]
    keep_dirs: bool,
    #[arg( short='s',long = "sys-paths",default_value_t = false,help = format!("Include system paths {:?}\n",AVOID))]
    keep_sys_paths: bool,
    #[arg(
        short = 'd',
        long = "depth",
        required = false,
        help = "Selects the max depth to go to \n"
    )]
    max_depth: Option<usize>,
    #[arg(
        short = 't',
        long = "top",
        required = false,
        help = "Retrieves the first t results, eg scanit rs$ -t 10  finds the first 10 results \nPLEASE NOTE THIS CANNOT DO FILE SORTING\n"
    )]
    top_n: Option<usize>,
    #[arg(
        short = 'r',
        long = "regex-escape",
        default_value_t=false,
        required = false,
        help = "Performs a literal search, not a regex\nFor example, if searching for something with a bracket in its name;\nYou may want to search for 'buildv(1.23)' then you can  -r to do this\nYou may need to wrap your expression in quotes/semi-quotes\n"
    )]
    regex_escape:bool
}



fn main() -> Result<(),ScanError> {
    let args: Args = Args::parse();
   

   let files = find_files_iter(
       &escape_regex_string(&args.pattern, args.regex_escape),
       &resolve_directory(args.cd, args.directory),
       args.hidden,
       args.case,
       args.thread_num,
       args.keep_dirs,
       args.keep_sys_paths,
       args.max_depth)?;

    write_paths(files, args.top_n)?;
   



Ok(())
    



}





