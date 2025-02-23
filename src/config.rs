pub struct SearchConfig {
    pub pattern: String,
    pub root: String,
    pub hide_hidden: bool,
    pub case_sensitive: bool,
    pub thread_count: usize,
    pub keep_dirs: bool,
    pub keep_sys_paths: bool,
    pub max_depth: Option<usize>,
    pub use_glob: bool,
    pub full_path: bool,
}

impl SearchConfig {
    #[must_use]
    pub fn new(
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
    ) -> Self {
        Self {
            pattern: pattern.into(),
            root: root.into(),
            hide_hidden,
            case_sensitive,
            thread_count,
            keep_dirs,
            keep_sys_paths,
            max_depth,
            use_glob,
            full_path,
        }
    }
}
