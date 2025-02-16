

use std::io::Error as IoError;
use thiserror::Error;
use ignore::Error as WalkError;
use regex::Error as RegexError;
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("I/O error: {0}")]
    Io(#[from] IoError),
    #[error("Regex error, consider using -r to escape the Regex: {0:?}")]
    Regex(#[from] RegexError),
    #[error("Directory traversal error: {0}")]
    Walk(#[from] WalkError),
    #[error("Scanit error: {0}")]
    Other(String),
}