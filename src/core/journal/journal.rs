use crate::sys::journal as sys;

use std::os::raw::c_int;

pub struct Journal {
    journal_handle: *mut sys::sd_journal,
}

impl Journal {
    pub fn new() -> c_int {
        let mut handle = std::ptr::null_mut() as *mut sys::sd_journal;

        let rc = unsafe { sys::sd_journal_open(&mut handle, sys::SD_JOURNAL_LOCAL_ONLY) };

        return rc;
    }
}
