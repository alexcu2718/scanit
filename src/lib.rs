
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::single_char_lifetime_names)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::min_ident_chars)]
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

use std::sync::Arc;
pub use std::path::Path;
pub use ignore::{DirEntry, ParallelVisitor, ParallelVisitorBuilder, WalkBuilder, WalkState};
pub use regex::{Regex, RegexBuilder,Error as RegexError};
pub use std::io;
pub use std::process::exit as process_exit;
pub use std::sync::mpsc::{channel as unbounded,IntoIter,SendError,Sender};
pub use std::env::current_dir;

/// Constant pattern used for matching all files
/// also used to go into current dir.
pub const DOT_PATTERN: &str = ".";



///Default Start Prefix to use
#[cfg(unix)]
pub const START_PREFIX: &str = "/";
#[cfg(windows)]
pub const START_PREFIX: &str = r"C:/";

/// System paths to avoid during file scanning
#[cfg(unix)]
pub const AVOID: [&str; 6] = ["/proc", "/sys", "/tmp", "/run", "/dev", "/sbin"];
/// INTERNAL HEURISTIC USED FOR AVOIDING SYSPATHS
#[cfg(unix)]
const DEPTH_CHECK: usize = 1;

/// System paths to avoid during file scanning
#[cfg(windows)]
pub const AVOID: [&str; 4] = [r"C:\Windows\System32",r"C:\Windows\SysWOW64",r"C:\Windows\Temp",r"C:\$Recycle.Bin"];
/// INTERNAL HEURISTIC USED FOR AVOIDING SYSPATHS
#[cfg(windows)]
const DEPTH_CHECK:usize=3;

/// A type alias for thread-safe string references
pub type StaticStr = Arc<str>;

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
    !AVOID.iter().any(|x| filepath.starts_with(x) )
}
    


/// Converts a directory entry to a string path if it matches the directory filter
/// 
/// # Arguments
/// * `pathname` - The directory entry to convert
/// * `keep_dirs` - Whether to include directory paths
/// 
/// # Returns
/// * `Some(&str)` containing the path if it should be included
/// * `None` if the path should be excluded
#[allow(clippy::inline_always)]
#[inline(always)]
fn pathtostring(pathname: &DirEntry, keep_dirs: bool) -> Option<&str> {
    if !keep_dirs && pathname.file_type()?.is_dir() {
        return None;
    }
    pathname.path().to_str()
}



#[inline(never)]
#[cold]
pub fn handle_regex_error(pattern: &str, error: &RegexError)->!  {
    eprintln!("Error: Invalid regex pattern '{pattern}'\nDetails: {error}\nConsider Using -r to escape the Regex\n");
    process_exit(1)
}




/// Processes a file path against regex pattern and sends matches through channel
/// 
/// # Arguments
/// * `filename` - The path to process
/// * `re` - Optional regex pattern to match against
/// * `tx` - Channel sender for matched paths
/// * `is_dot` - Whether to match all files
/// 
/// # Returns
/// * `WalkState` indicating whether to continue or skip
#[allow(clippy::inline_always)]
#[inline(always)]
fn process_file(
    filename: &str,
    re: Option<&Regex>,
    tx: &Sender<StaticStr>,
    is_dot: bool,
) -> WalkState {
    
    if is_dot || re.is_some_and(|r| r.is_match(filename)) {
        match tx.send(Arc::from(filename)) {
            Ok(()) => WalkState::Continue,
            Err(_) => WalkState::Skip
        }
    } else {
        WalkState::Continue
    }
}


/// Builder for parallel file walker visitors
/// 
/// # Fields
/// * `tx` - Channel sender for matched paths
/// * `re` - Regex pattern for matching files
/// * `is_dot` - Flag for matching all files
/// * `keep_dirs` - Flag for including directories
struct FileWalkerBuilder<'a> {
    tx: Sender<StaticStr>,
    re: Option<&'a Regex>,
    is_dot: bool,
    keep_dirs: bool,
}


/// Visitor implementation for parallel file walking
/// 
/// # Fields
/// * `tx` - Channel sender for matched paths
/// * `re` - Regex pattern for matching files
/// * `is_dot` - Flag for matching all files
/// * `keep_dirs` - Flag for including directories
struct FileWalkerVisitor<'a> {
    tx: Sender<StaticStr>,
    re: Option<&'a Regex>,
    is_dot: bool,
    keep_dirs: bool,
}


impl ParallelVisitor for FileWalkerVisitor<'_> {

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn visit(&mut self, entry: Result<DirEntry, ignore::Error>) -> WalkState {
        if let Ok(entry_path) = entry {
            if let Some(filename) = pathtostring(&entry_path, self.keep_dirs) {
                return process_file(filename, self.re, &self.tx, self.is_dot);
            }
        }
        WalkState::Continue
    }
}

impl<'a> ParallelVisitorBuilder<'a> for FileWalkerBuilder<'a> {
    #[inline]
    fn build(&mut self) -> Box<dyn ParallelVisitor + 'a> {
        Box::new(FileWalkerVisitor {
            tx: self.tx.clone(),
            re: self.re,
            is_dot: self.is_dot,
            keep_dirs: self.keep_dirs,
        })
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
) ->  Result<IntoIter<StaticStr>, io::Error> {
    let (tx, rx) = unbounded::<StaticStr>();
    let is_dot = pattern == DOT_PATTERN;
    let re: Option<Regex> = if is_dot {
        None
    } else {
        let regex = RegexBuilder::new(pattern)
        .case_insensitive(case_sensitive)
        .build()
        .unwrap_or_else(|e| handle_regex_error(pattern, &e));
        Some(regex)
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
        .visit(&mut FileWalkerBuilder {
            tx,
            re:re.as_ref(),
            is_dot,
            keep_dirs,
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
) -> Result<Vec<String>, io::Error> {
    Ok(find_files_iter(
        pattern,
        path,
        hide_hidden,
        case_sensitive,
        thread_count,
        keep_dirs,
        keep_sys_paths,
        max_depth,
    )?.map(|x| x.to_string())
    .collect())
}
