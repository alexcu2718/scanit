pub struct SearchConfig<'a> {
    pub pattern: &'a str,
    pub root: &'a str,
    pub hide_hidden: bool,
    pub case_sensitive: bool,
    pub thread_count: usize,
    pub keep_dirs: bool,
    pub keep_sys_paths: bool,
    pub max_depth: Option<usize>,
    pub use_glob: bool,
    pub full_path: bool,
}