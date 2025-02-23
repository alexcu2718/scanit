use crate::{BoxBytes, Regex,Sender};
use ignore::{DirEntry, WalkState};
//use os_str_bytes::OsStrBytes;
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;


pub trait AsBytes {
    fn as_true_bytes(&self) -> &[u8];
}

pub trait FileNameBytes {
    fn filename_bytes(&self) -> &[u8];
}

impl AsBytes for OsStr {
    #[cfg(unix)]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn as_true_bytes(&self) -> &[u8] {
        self.as_bytes()
    }

   
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[cfg(windows)]
    fn as_true_bytes(&self) -> &[u8] {
        self.as_encoded_bytes()
    }
}

impl AsBytes for DirEntry {
    #[cfg(unix)]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn as_true_bytes(&self) -> &[u8] {
        self.path().as_os_str().as_bytes()
    }
    #[allow(clippy::inline_always)]
    #[inline(always)]
    #[cfg(windows)]
    fn as_true_bytes(&self) -> &[u8] {
        self.path().as_os_str().as_encoded_bytes()
    }
  


}

impl FileNameBytes for DirEntry {
    #[cfg(unix)]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn filename_bytes(&self) -> &[u8] {
        self.file_name().as_bytes()
    }
    #[cfg(windows)]
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn filename_bytes(&self) -> &[u8] {
        self.file_name().as_encoded_bytes()
    }
}

#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn process_entry_fullpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
) -> WalkState {
    let filename = entry_path.as_true_bytes();
    if re.map_or(true, |search| search.is_match(filename)) {
        tx.send(filename.into())
            .map_or(WalkState::Skip, |()| WalkState::Continue)
    } else {
        WalkState::Continue
    }
}

#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn process_entry_shortpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
) -> WalkState {
    if re.map_or(true, |search| search.is_match(entry_path.filename_bytes())) {
        tx.send(entry_path.as_true_bytes().into())
            .map_or(WalkState::Skip, |()| WalkState::Continue)
    } else {
        WalkState::Continue
    }
}

/*


use crate::{BoxBytes, Regex};
use ignore::{DirEntry, WalkState};
use os_str_bytes::OsStrBytes;
use std::sync::mpsc::Sender;


#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn process_entry_fullpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
) -> WalkState {



    let filename = entry_path.path().to_raw_bytes();
    if re.map_or(true,|search| search.is_match(&filename)) {



        tx.send(filename.into()).map_or(WalkState::Skip,|()|WalkState::Continue)}else {
            WalkState::Continue
        }
    }




#[doc(hidden)]
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn process_entry_shortpath(
    entry_path: &DirEntry,
    re: Option<&Regex>,
    tx: &Sender<BoxBytes>,
) -> WalkState {
    if re.map_or(true,|search| search.is_match(&entry_path.file_name().to_raw_bytes())) {
        tx.send(entry_path.path().to_raw_bytes().into()).map_or(WalkState::Skip,|()|WalkState::Continue)}else {
            WalkState::Continue
        }
    }
*/
