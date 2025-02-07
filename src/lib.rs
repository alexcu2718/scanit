
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::single_char_lifetime_names)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::fn_params_excessive_bools)]
#![allow(clippy::struct_excessive_bools)]


//!CLI TOOL FOR SEARCHING FILE PATHS
//! A CLI tool for efficiently searching file paths in parallel
//! 
//! This crate provides functionality to:
//! - Search files using regex patterns
//! - Filter system paths
//! - Handle both Unix and Windows paths
//! - Process directories in parallel

//redundant checks on this 
#[cfg(all(
    not(windows),
    not(target_os = "android"),
    not(target_os = "macos"),
    not(target_os = "freebsd"),
    target_os="linux"
))]
#[global_allocator]
pub static ALLOCATOR: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub use std::path::Path;
pub use ignore::{DirEntry, ParallelVisitor, ParallelVisitorBuilder, WalkBuilder, WalkState,Error as WalkError};
pub use regex::{Regex, RegexBuilder,Error as RegexError};
pub use std::io;
pub use std::process::exit as process_exit;
pub use std::sync::mpsc::{channel as unbounded,IntoIter,SendError,Sender};
pub use std::env::current_dir;
pub use arcstr::ArcStr;

use std::io::Error as IoError;
use thiserror::Error;









/// System paths to avoid during file scanning
#[cfg(unix)]
pub const AVOID: [&str; 6] = ["/proc", "/sys", "/tmp", "/run", "/dev", "/sbin"];

/// System paths to avoid during file scanning
#[cfg(windows)]
pub const AVOID: [&str; 4] = [r"C:\Windows\System32",r"C:\Windows\SysWOW64",r"C:\Windows\Temp",r"C:\$Recycle.Bin"];


///Default Start Prefix to use, defaults to "/" on Unix, "C:/" on Windows.
pub const START_PREFIX: &str =if cfg!(unix){"/"}else{r"C:/"};


/// INTERNAL HEURISTIC USED FOR AVOIDING SYSPATHS
const DEPTH_CHECK: usize = if cfg!(unix) { 1 } else { 3 };





#[derive(Error, Debug)]
pub enum ScanError {
    #[error("I/O error: {0}")]
    Io(#[from] IoError),
    #[error("Regex error, consider using -r to escape the Regex: {0:?}")]
    Regex(#[from] RegexError),
    #[error("Directory traversal error: {0}")]
    Walk(#[from] WalkError),
    #[error("Channel send error: {0}")]
    Send(#[from] SendError<ArcStr>),
}

/// Checks if a given path should be excluded from system paths
/// 
/// # Arguments
/// * `filepath` - The path to check
/// 
/// # Returns
/// * `true` if the path should be included
/// * `false` if the path should be excluded
#[allow(clippy::inline_always)]
#[inline(always)]
fn avoid_sys_paths(filepath: &Path) -> bool {
    !AVOID.iter().any(|pathname| filepath.starts_with(pathname) )
}
    

#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
fn path_to_str(pathname: &DirEntry, keep_dirs: bool) -> Option<&str> {
    if !keep_dirs && pathname.file_type()?.is_dir() {
        return None;
    }
    pathname.path().to_str()
}


#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
fn process_file(filename: &str,re: Option<&Regex>,tx: &Sender<ArcStr>,is_dot: bool,) -> WalkState {
    
if is_dot || re.is_some_and(|search| search.is_match(filename)) {
    match tx.send(ArcStr::from(filename)) {
        Ok(()) => WalkState::Continue,
        Err(_) => WalkState::Skip
    }
} else {
    WalkState::Continue
}
}



fn build_regex(pattern: &str,case_sensitive: bool)->Result<Regex,ScanError>{
match RegexBuilder::new(pattern)
    .case_insensitive(case_sensitive)
    .build() {
        Ok(regex_good) => Ok(regex_good),
        Err(error) => {
            eprintln!("Invalid Regex, consider using -r or --regex-escape to avoid this\n");
            Err(ScanError::Regex(error))
        }
    }
}




/// Creates an iterator over files matching the given pattern
///
/// # Arguments
/// * `pattern` - Regex pattern to match against file paths
/// * `path` - Root directory to start search from
/// * `hide_hidden` - Whether to skip hidden files/directories
/// * `case_sensitive` - Whether regex matching should be case sensitive
/// * `thread_count` - Number of parallel threads to use
/// * `keep_dirs` - Whether to include directory paths
/// * `keep_sys_paths` - Whether to include system paths
/// * `max_depth` - Maximum directory depth to traverse
///
/// # Errors
/// Returns `io::Error` if:
/// * The regex pattern is invalid
/// * Directory traversal fails
/// * File system access is denied
///
/// # Returns
/// * `Result<IntoIter<StaticStr>, io::Error>` - Iterator of matched paths
///
/// # Examples
/// ```
/// use scanit::find_files_iter;
/// use std::io;
///
/// fn main() -> Result<(), io::Error> {
///     let iter = find_files_iter(
///         r".*\.rs$",  // Find Rust files using proper regex
///         ".",        // Start from current directory
///         true,       // Hide hidden files
///         false,      // Case insensitive
///         4,          // Use 4 threads
///         false,      // Skip directories
///         false,      // Skip system paths
///         Some(5)     // Max depth of 5
///     )?;
///
///     for path in iter {
///         println!("{}", &*path);
///     }
///     Ok(())
/// }
/// ```
#[inline]
pub fn find_files_iter(
    pattern: &str,
    path: &str,
    hide_hidden: bool,
    case_sensitive: bool,
    thread_count: usize,
    keep_dirs: bool,
    keep_sys_paths: bool,
    max_depth: Option<usize>,
) -> Result<IntoIter<ArcStr>, ScanError> {
    let (tx, rx) = unbounded::<ArcStr>();
    let is_dot = pattern == ".";
    let re: Option<Regex> = if is_dot {
        None
    } else {
        Some(build_regex(pattern,case_sensitive)?)
    };
    WalkBuilder::new(path)
        .hidden(!hide_hidden)
        .filter_entry(move |entry| {
            keep_sys_paths || entry.depth() > DEPTH_CHECK || avoid_sys_paths(entry.path())
        })
        .git_global(false)
        .git_ignore(false)
        .git_exclude(false)
        .ignore(false)
        .max_depth(max_depth)
        .threads(thread_count)
        .build_parallel()
        .run(|| {
            let tx = tx.clone();
            let re = re.as_ref();
            Box::new(move |entry: Result<DirEntry, WalkError>| -> WalkState {
                entry.map_or(WalkState::Continue, |entry_path| {
                    path_to_str(&entry_path, keep_dirs).map_or(WalkState::Continue, |filename| {
                        process_file(filename, re, &tx, is_dot)
                    })
                })
            })
        });
    Ok(rx.into_iter())
}


/// Collects matching file paths into a vector
///
/// Convenience wrapper around `find_files_iter` that collects results into a Vec.
/// This function is useful when you need all results at once rather than processing
/// them iteratively.
///
/// # Arguments
/// * `pattern` - Regex pattern to match against file paths
/// * `path` - Root directory to start search from
/// * `hide_hidden` - Whether to skip hidden files/directories
/// * `case_sensitive` - Whether regex matching should be case sensitive
/// * `thread_count` - Number of parallel threads to use
/// * `keep_dirs` - Whether to include directory paths
/// * `keep_sys_paths` - Whether to include system paths
/// * `max_depth` - Maximum directory depth to traverse
///
/// # Errors
/// Returns `io::Error` if:
/// * The regex pattern is invalid
/// * Directory traversal fails
/// * File system access is denied
///
/// # Returns
/// * `Result<Vec<String>, io::Error>` - Vector of matched paths
///
/// # Examples
/// ```
/// use scanit::find_files;
/// use std::io;
///
/// fn main() -> Result<(), io::Error> {
///     let files = find_files(
///         r".*\.rs$",  // Find Rust files using proper regex
///         ".",        // Start from current directory
///         true,       // Hide hidden files
///         false,      // Case insensitive
///         4,          // Use 4 threads
///         false,      // Skip directories
///         false,      // Skip system paths
///         Some(5)     // Max depth of 5
///     )?;
///
///     for path in files {
///         println!("{}", path);
///     }
///     Ok(())
/// }
/// ```
#[inline]
pub fn find_files(
    pattern: &str,
    path: &str,
    hide_hidden: bool,
    case_sensitive: bool,
    thread_count: usize,
    keep_dirs: bool,
    keep_sys_paths: bool,
    max_depth: Option<usize>,
) -> Result<Vec<String>, ScanError> {
    Ok(find_files_iter(
        pattern,
        path,
        hide_hidden,
        case_sensitive,
        thread_count,
        keep_dirs,
        keep_sys_paths,
        max_depth,
    )?
    .map(|arcstr| arcstr.to_string())
    .collect())
}