use crate::sys::journal as sys;

pub struct Journal {
    journal_handle: *mut sys::sd_journal,
}

impl Journal {
    pub fn new() {
        let mut handle = std::ptr::null_mut() as *mut sys::sd_journal;

        let ret:i32 = journal_try!(sys::sd_journal_open(
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
        let _j =  Journal::new();
    }
}

