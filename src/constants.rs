pub const DOT_PATTERN: &str = ".";

#[cfg(unix)]
#[allow(unused)]
pub const START_PREFIX: &str = "/";

#[cfg(windows)]
#[allow(unused)]
pub const START_PREFIX: &str = r"C:/";

// System paths to avoid during file scanning
#[cfg(all(unix, not(target_os = "macos")))]
#[allow(unused)]
pub const AVOID: [&str; 6] = ["/proc", "/sys", "/tmp", "/run", "/dev", "/sbin"];

#[cfg(target_os = "macos")]
#[allow(unused)]
pub const AVOID: [&str; 6] = [
    "/System", "/Library", "/private", "/dev", "/tmp", "/Network",
];

#[cfg(windows)]
#[allow(unused)]
pub const AVOID: [&str; 4] = [
    r"C:\Windows\System32",
    r"C:\Windows\SysWOW64",
    r"C:\Windows\Temp",
    r"C:\$Recycle.Bin",
];

#[cfg(unix)]
#[allow(unused)]
pub const DEPTH_CHECK: usize = 1;

#[cfg(windows)]
#[allow(unused)]
pub const DEPTH_CHECK: usize = 3;
