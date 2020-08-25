use crate::sys::journal as sys;

use std::os::raw::c_int;

#[derive(Debug)]
pub struct CJournalError {
    pub message: String,
    pub error_code: c_int
}

pub struct Journal {
    journal_handle: *mut sys::sd_journal
}

impl Journal {
    pub fn new() /*-> Result<Journal, CJournalError>*/ {
        /*
        let mut handle = std::ptr::null_mut() as *sys::sd_journal;
        
        let rc =  unsafe  {
            sys::sd_journal_open(&mut handle, sys::SD_JOURNAL_LOCAL_ONLY);
        }
        */

        /*
        CJournalError{
            message: "Test",
            error_code: -1
        };
        */
    }
}