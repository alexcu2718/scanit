

use crate::{BoxBytes, Regex};
use std::sync::mpsc::Sender;
use ignore::{WalkState,DirEntry};



#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;




#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(unix)]
pub fn process_entry_fullpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
    is_dot: bool,
    include_dirs: bool
) -> WalkState {
   
if !include_dirs && entry_path.file_type().is_some_and(|filetype| filetype.is_dir()) {
    return WalkState::Continue;
}



let filename=entry_path.path().as_os_str();

if is_dot || re.is_some_and(|search| search.is_match(filename.as_bytes())) {
    match tx.send(filename.as_bytes().into()) {
        Ok(()) => WalkState::Continue,
        Err(_) => WalkState::Skip,
    }
} else {
    WalkState::Continue
}
}




#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(not(unix))]
pub fn process_entry_fullpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
    is_dot: bool,
    include_dirs: bool
) -> WalkState {
   
if !include_dirs && entry_path.file_type().is_some_and(|filetype| filetype.is_dir()) {
    return WalkState::Continue;
}

let filename=entry_path.path().as_os_str();
if is_dot || re.is_some_and(|search| search.is_match(filename.as_encoded_bytes())) {
    match tx.send(filename.as_encoded_bytes().into()) {
        Ok(()) => WalkState::Continue,
        Err(_) => WalkState::Skip,
    }
} else {
    WalkState::Continue
}
}



#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
#[cfg(unix)]
pub fn process_entry_shortpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
    is_dot: bool,
    include_dirs: bool
) -> WalkState {
   

   
if !include_dirs && entry_path.file_type().is_some_and(|filetype| filetype.is_dir()) {
    return WalkState::Continue;
}


if is_dot || re.is_some_and(|search| search.is_match(entry_path.file_name().as_bytes())) {
     match tx.send(entry_path.path().as_os_str().as_bytes().into()){
        Ok(()) => WalkState::Continue,
        Err(_) => WalkState::Skip,
    }
} else {
    WalkState::Continue
}
}



#[doc(hidden)]
#[cfg(not(unix))]
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn process_entry_shortpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
    is_dot: bool,
    include_dirs: bool
) -> WalkState {
   
if !include_dirs && entry_path.file_type().is_some_and(|filetype| filetype.is_dir()) {
    return WalkState::Continue;
}




if is_dot || re.is_some_and(|search| search.is_match(entry_path.file_name().as_encoded_bytes())) {
    match tx.send(entry_path.path().as_os_str().as_encoded_bytes().into()) {
        Ok(()) => WalkState::Continue,
        Err(_) => WalkState::Skip,
    }
} else {
    WalkState::Continue
}
}



