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
    scanit '^test.*\\.js$'        # Find JavaScript files starting with 'test'

    
Supports Unix and Windows paths automatically."
)]


struct Args {
    #[arg(required_unless_present = "version")]
    pattern: Option<String>,
    #[arg(short, long, default_value = START_PREFIX)]
    directory: String,
    #[arg(short='C', long="cd", conflicts_with="directory", help="Uses the current directory to load")]
    cd: bool,
    #[arg(short='H', long="show-hidden", help="Shows hidden files eg .gitignore or .bashrc")]
    hval: bool,
    #[arg(short='V', long="version", help="Show version number", action=clap::ArgAction::SetTrue)]
    version: bool
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

    


    scanit::find_files(args.pattern.as_deref().unwrap(), &args.directory, args.hval)?;
    Ok(())
}