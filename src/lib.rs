

use std::error::Error;
use regex::Regex;
use jwalk::WalkDir;
use std::io::{self, Write,Stdout,stdout};
pub fn find_files(pattern: &str, directory: &str, show_hidden: bool, include_dirs: bool) 
    -> Result<(), Box<dyn Error + 'static>> {
    let re: Regex = Regex::new(pattern)?;
    let stdout: Stdout = stdout();
    let mut handle: io::StdoutLock<'_> = stdout.lock();

    for entry in WalkDir::new(directory).skip_hidden(!show_hidden) {
        match entry {
            Ok(e) => {
                if !include_dirs {
                    if let Some(path) = e.file_name().to_str() {
                        if re.is_match(path) {
                            let path_buf = e.path().to_path_buf();
                            let Some(final_path) = path_buf.to_str() else { continue; };
                            let _ = writeln!(handle, "{}", final_path);
                        }
                    }
                } else {
                    if let Some(path) = e.path().to_str() {
                        if re.is_match(path) {
                            let _ = writeln!(handle, "{}", path);
                        }
                    }
                }
            },
            Err(_) => continue,
        }
    }
    Ok(())
}
