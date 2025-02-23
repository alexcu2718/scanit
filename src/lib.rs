#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::single_char_lifetime_names)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::fn_params_excessive_bools)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::multiple_crate_versions)]

//!CLI TOOL FOR SEARCHING FILE PATHS
//! A CLI tool for efficiently searching file paths in parallel
//!
//! This crate provides functionality to:
//! - Search files using regex patterns
//! - Filter system paths
//! - Handle both Unix and Windows paths
//! - Process directories in parallel

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL_ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(target_env = "msvc")]
#[global_allocator]
static GLOBAL_ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

use fnmatch_regex2::glob_to_regex;
use ignore::{DirEntry, WalkBuilder, WalkState};
use regex::{bytes::Regex, bytes::RegexBuilder};
pub use std::ffi::OsString;
use std::path::PathBuf;
use std::process::exit as process_exit;
pub use std::sync::mpsc::{channel as unbounded, Receiver,Sender};
pub type BoxBytes = Box<[u8]>;
use std::collections::HashSet;
use std::sync::OnceLock;
mod process_entries;
use process_entries::{process_entry_fullpath, process_entry_shortpath};
pub use process_entries::{FileNameBytes,AsBytes};
mod config;
mod constants;
pub use config::SearchConfig;
mod error;
use constants::{AVOID, START_PREFIX};
pub(crate) use constants::{DEPTH_CHECK, DOT_PATTERN};
pub use error::ScanError;

static AVOID_PATHS: OnceLock<HashSet<PathBuf>> = OnceLock::new();

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
fn avoid_sys_paths(path_entry: &DirEntry) -> bool {
    if path_entry.depth() > DEPTH_CHECK {
        return true;
    }
    let paths = AVOID_PATHS.get_or_init(|| AVOID.iter().map(PathBuf::from).collect::<HashSet<_>>());
    !paths.contains(path_entry.path())
}

#[allow(clippy::missing_errors_doc)]
#[must_use = "builds regex but modifies errors to map to custom error type"]
fn build_regex(pattern: &str, case_sensitive: bool) -> Result<Regex, ScanError> {
    RegexBuilder::new(pattern)
        .case_insensitive(case_sensitive)
        .build()
        .map_err(ScanError::Regex)
}

#[must_use]
fn process_glob_regex(glob_pattern: &str) -> String {
    glob_to_regex(glob_pattern).map_or_else(
        |_| {
            eprintln!("This can't be processed as a glob pattern");
            process_exit(1)
        },
        |good_pattern| good_pattern.as_str().into(),
    )
}

/// Creates an iterator over files matching the given search configuration.
///
/// # Arguments
///
/// The search configuration (`SearchConfig`) contains:
/// * `pattern` - A regex pattern (or a glob pattern if `use_glob` is true) to match against file paths.
/// * `root` - The root directory from which to start the search.
/// * `hide_hidden` - Whether to skip hidden files and directories.
/// * `case_sensitive` - Whether regex matching should be case sensitive.
/// * `thread_count` - Number of parallel threads to use during traversal.
/// * `keep_dirs` - Whether to include directory paths in the output.
/// * `keep_sys_paths` - Whether system paths should be included, overriding default filtering.
/// * `max_depth` - Maximum directory depth to traverse.
/// * `use_glob` - If true, the input pattern is treated as a glob pattern.
/// * `full_path` - If true, matching is performed against the full file path instead of just the filename.
///
/// # Errors
///
/// Returns a `ScanError` if:
/// * The regex (or glob-to-regex conversion) fails to compile.
/// * Directory traversal fails.
/// * File system access is denied.
///
/// # Returns
///
/// * `Result<Receiver<Box<[u8]>>, ScanError>` - An iterator over matched file paths represented as boxed bytes.
///
/// # Examples
/// ```rust
/// use scanit::{find_files_iter, SearchConfig, ScanError};
///
/// fn main() -> Result<(), ScanError> {
///     let search_config = SearchConfig {
///         pattern: r".*\.rs$".into(),
///         root: ".".into(),
///         hide_hidden: true,
///         case_sensitive: false,
///         thread_count: 4,
///         keep_dirs: false,
///         keep_sys_paths: false,
///         max_depth: Some(5),
///         use_glob: false,
///         full_path: false,
///     };
///     
///     
///     
///     for path in  find_files_iter(&search_config)?.iter() {
///         println!("{:?}", &*path);
///     }
///     
///     Ok(())
/// }
/// ```
#[inline]
pub fn find_files_iter(search_config: &SearchConfig) -> Result<Receiver<BoxBytes>, ScanError> {
    let (tx, rx) = unbounded::<BoxBytes>();

    let pattern_to_use = if search_config.use_glob {
        process_glob_regex(&search_config.pattern)
    } else {
        search_config.pattern.clone()
    };

    let re: Option<Regex> = if search_config.pattern == DOT_PATTERN {
        None
    } else {
        Some(build_regex(&pattern_to_use, search_config.case_sensitive)?)
    };

    //This just avoids unnecessary boolean checks(trivial but good to do)
    let conditional_check: bool =
        search_config.root != START_PREFIX || search_config.keep_sys_paths;

    //implementing this switch here improves performance.
    let process_entry = if search_config.use_glob || search_config.full_path {
        process_entry_fullpath
    } else {
        process_entry_shortpath
    };

    WalkBuilder::new(&search_config.root)
        .hidden(!search_config.hide_hidden)
        .filter_entry(move |entry| conditional_check || avoid_sys_paths(entry))
        .git_global(false)
        .git_ignore(false)
        .git_exclude(false)
        .ignore(false)
        .max_depth(search_config.max_depth)
        .threads(search_config.thread_count)
        .build_parallel()
        .run(|| {
            Box::new(|entry| {
                entry.map_or(WalkState::Continue, |entry_path| {
                    if !search_config.keep_dirs
                        && entry_path
                            .file_type()
                            .is_some_and(|filetype| filetype.is_dir())
                    {
                        return WalkState::Continue;
                    }

                    process_entry(&entry_path, re.as_ref(), &tx)
                })
            })
        });
    Ok(rx)
}

/// # Examples
/// ```
/// use scanit::{find_files, ScanError};
///
/// fn main() -> Result<(), ScanError> {
///     // Find all Rust source files in current directory
///     let files = find_files(
///         r"\.rs$",        // Match files ending in .rs
///         ".",             // Search in current directory
///         true,            // Skip hidden files
///         false,           // Case-insensitive matching
///         4,               // Use 4 parallel threads
///         false,           // Don't include directory paths
///         false,           // Skip system paths
///         Some(5),         // Search up to 5 directories deep
///         false,           // Use regex (not glob) pattern
///         false,           // Match against filename only
///     )?;
///
///     // Example output: ["main.rs", "lib.rs", "tests/common.rs"]
///     for path in files {
///         println!("{:?}", path);
///     }
///     Ok(())
/// }
/// ```
///
/// # Errors
/// Returns `ScanError` if:
/// * The regex pattern is invalid (`ScanError::Regex`)
/// * Directory traversal fails (`ScanError::Walk`)
/// * File system access is denied (`ScanError::Io`)
/// * Memory allocation fails during path collection
#[inline]
pub fn find_files(
    pattern: &str,
    root: &str,
    hide_hidden: bool,
    case_sensitive: bool,
    thread_count: usize,
    keep_dirs: bool,
    keep_sys_paths: bool,
    max_depth: Option<usize>,
    use_glob: bool,
    full_path: bool,
) -> Result<Vec<OsString>, ScanError> {
    let search_config = SearchConfig {
        pattern: pattern.to_string(),
        root: root.into(),
        hide_hidden,
        case_sensitive,
        thread_count,
        keep_dirs,
        keep_sys_paths,
        max_depth,
        use_glob,
        full_path,
    };

    Ok(find_files_iter(&search_config)?
        .iter()
        .map(|arc_str| unsafe { OsString::from_encoded_bytes_unchecked(arc_str.into()) })
        // SAFETY: The bytes in arc_str are guaranteed to be valid OsString data
        // because they were originally created from OsStrings in process_entry_*.rs
        // and have only been transmitted as raw bytes for performance reasons.
        // This conversion is safe because:
        // 1. The original data came from valid OsStrings
        // 2. The bytes have not been modified during transmission
        // 3. The platform encoding has not changed during the operation
        .collect::<Vec<OsString>>())
}
