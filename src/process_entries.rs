

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





