use crate::sys::journal as sys;

pub struct Journal {
    journal_handle: *mut sys::sd_journal,
}

impl Journal {
    pub fn new() {
        let mut handle = std::ptr::null_mut() as *mut sys::sd_journal;

        journal_try!(sys::sd_journal_open(
            &mut handle,
            sys::SD_JOURNAL_LOCAL_ONLY
        ));
    }
}
