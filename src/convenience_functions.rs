
use scanit::{DOT_PATTERN,START_PREFIX,current_dir};

const ESCAPE_REGEX:[char;14]=['[', ']', '(', ')', '{', '}', '.', '*', '+', '?', '^', '$', '\\', '|'];

#[allow(clippy::must_use_candidate)]
#[inline(never)]
#[cold]
///This is to avoid using the default . pattern, it doesnt show the full path, which considering this is written by a lazy
/// person like me, i dont like it.
pub fn resolve_directory(args_cd: bool, args_directory: Option<String>) -> String {
    if args_cd {
        current_dir().map_or_else(
            |_| DOT_PATTERN.into(),
            |path_res| path_res.to_str().map_or_else(||DOT_PATTERN.into(),Into::into)
        )
    } else {
        args_directory.unwrap_or_else(|| START_PREFIX.into())

    }
}






#[inline(never)]
#[cold]
pub fn get_threads() -> usize {
    env!("CPU_COUNT").parse::<usize>().unwrap_or(1)
}



pub fn escape_regex_string(input: &str,avoid_regex:bool) -> String {

    if !avoid_regex{return input.into()}
 
    let mut result = String::with_capacity(input.len() * 2);
    
    for c in input.chars() {
        if ESCAPE_REGEX.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

