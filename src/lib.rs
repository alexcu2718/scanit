

use std::error::Error;
use regex::Regex;
use jwalk::WalkDir;

pub fn find_files(pattern: &str, directory: &str,show_hidden:bool) -> Result<(), Box<dyn Error + 'static>> {
   
    let mut matches = Vec::new();
    let re = Regex::new(pattern)?;
    for entry in WalkDir::new(directory).skip_hidden(!show_hidden) {
        match entry {
            Ok(e) => {
                if let Some(path) = e.path().to_str() {
                    if re.is_match(path) {
                        matches.push(path.to_owned());
                    }
                }
            },
           
            Err(_) => {continue}
        }
    }

    println!("{}", matches.join("\n"));
   
    
    Ok(())
}
