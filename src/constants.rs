


/// Constant pattern used for matching all files
/// also used to go into current dir.
pub const DOT_PATTERN: &str = ".";



///Default Start Prefix to use
pub const START_PREFIX: &str =if cfg!(unix){"/"}else{r"C:/"};


/// System paths to avoid during file scanning
#[cfg(unix)]
pub const AVOID: [&str; 6] = ["/proc", "/sys", "/tmp", "/run", "/dev", "/sbin"];

/// System paths to avoid during file scanning
#[cfg(windows)]
pub const AVOID: [&str; 4] = [r"C:\Windows\System32",r"C:\Windows\SysWOW64",r"C:\Windows\Temp",r"C:\$Recycle.Bin"];


/// INTERNAL HEURISTIC USED FOR AVOIDING SYSPATHS
pub const DEPTH_CHECK: usize = if cfg!(unix){1}else{3};

pub const BUFFER_SIZE: usize = 2 * 1024 * 1024;
pub const FLUSH_THRESHOLD: usize = BUFFER_SIZE - (BUFFER_SIZE / 20);
pub const NEWLINE:&[u8; 1]=b"\n";

/// Regex characters to avoid
pub const ESCAPE_REGEX:[char;14]=['[', ']', '(', ')', '{', '}', '.', '*', '+', '?', '^', '$', '\\', '|'];