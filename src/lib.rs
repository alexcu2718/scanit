use scandir::{Walk, Toc};
use std::error::Error;
use regex::Regex;

pub fn find_files(pattern: &str, directory: &str,show_hidden:bool) -> Result<(), Box<dyn Error>> {
    const PATH_PREFIX: &str = if cfg!(windows) { "" } else { "/" };
    let re: Regex = Regex::new(pattern)?;

    
  
    let  mut walk: Walk = Walk::new(directory, Some(true))?
    .skip_hidden(!show_hidden);
    let toc: Toc = walk.collect()?;

    for file in toc.files() {
        if re.is_match(&file) {
            let full_path: String = format!("{}{}", PATH_PREFIX, file);
            println!("{}", full_path);
        }
    }

    Ok(())
}