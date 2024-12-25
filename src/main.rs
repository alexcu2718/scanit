use clap::Parser;
use std::error::Error;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const START_PREFIX: &str = if cfg!(windows) { "C:/" } else { "/" };


#[derive(Parser)]
#[command(
    name = "scanit",
    author = "Alex Curtis alexcu@tutanota.com",
    about = "Fast file scanner with regex pattern matching",
    long_about = "A command line utility that recursively searches directories for files matching regex patterns. 
    
Examples:
    scanit \\.rs$ -d /usr         # Find Rust files in /usr
    scanit '\\.png$'              # Find PNG files from current directory
    scanit '^test.*\\.js$'        # Find JavaScript files starting with 'test',starting from root

    
Supports Unix paths and maybe Windows(not yet tested)."
)]


struct Args {
    #[arg(required_unless_present = "version")]
    pattern: Option<String>,
    #[arg(short, long, default_value = START_PREFIX)]
    directory: String,
    #[arg(short='c', long="current-directory", conflicts_with="directory", help="Uses the current directory to load")]
    cd: bool,
    #[arg(short='a', long="show-hidden", help="Shows hidden files eg .gitignore or .bashrc")]
    hval: bool,
    #[arg(short='v', long="version", help="Show version number", action=clap::ArgAction::SetTrue)]
    version: bool,
    #[arg(short='i', long="include-dirs", help="Include directories in regex")]
    dirs: bool
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args: Args = Args::parse();

    if args.version {
        println!("scanit {}", VERSION);
        std::process::exit(0);
    };

    if args.cd {
        args.directory = ".".into();
    };

    


    scanit::find_files(
        args.pattern.as_deref().unwrap(), &args.directory, args.hval,args.dirs)?;
    Ok(())
}