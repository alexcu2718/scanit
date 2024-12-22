use std::error::Error;
use regex::Regex;
use jwalk::WalkDir;

pub fn find_files(pattern: &str, directory: &str,show_hidden:bool) -> Result<(), Box<dyn Error + 'static>> {
   
    let mut matches = Vec::new();
    let re = Regex::new(pattern)?;

    for entry in WalkDir::new(directory).skip_hidden(!show_hidden) {
        match entry {
            Ok(e) => {
                let path = e.path().to_string_lossy().into_owned();
                if re.is_match(&path) {
                    matches.push(path);
                }
            },
            Err(e) if e.to_string().contains("Permission denied") => {
                //skipped += 1;
                continue
            },
            Err(e) => return Err(Box::new(e))
        }
    }

    for path in &matches {
        println!("{}", path);
    }
   
    
    Ok(())
}
