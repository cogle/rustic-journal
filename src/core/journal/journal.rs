use crate::sys::journal as sys;
use crate::utils::c_error::CError;

pub struct Journal {
    journal_handle: *mut sys::sd_journal,
}

impl Journal {
    pub fn new() {
        let mut handle = std::ptr::null_mut() as *mut sys::sd_journal;

        let ret:Result<(), CError> = journal_try!(sys::sd_journal_open(
            &mut handle,
            sys::SD_JOURNAL_LOCAL_ONLY
        ));

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn journal_new() {
        let _j = Journal::new();
    }
}
