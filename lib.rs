use scandir::{Walk, Toc};
use std::error::Error;
use regex::Regex;

pub fn find_files(pattern: &str, directory: &str) -> Result<(), Box<dyn Error>> {


    const PATH_PREFIX: &str = if cfg!(windows) { "" } else { "/" };
    

    let toc: Toc = Walk::new(directory, Some(true))?.collect()?;
    let re: Regex = Regex::new(pattern)?;

    toc.files()
        .iter()
        .map(|p: &String| format!("{}{}", PATH_PREFIX, p))
        .filter(|path: &String| re.is_match(path))
        .for_each(|path: String| println!("{}", path));

    Ok(())
}