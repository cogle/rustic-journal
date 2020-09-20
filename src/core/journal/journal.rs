use crate::sys::journal as sys;

pub struct Journal {
    journal_handle: *mut sys::sd_journal,
}

impl Drop for Journal {
    fn drop(&mut self) {
        unsafe {
            sys::sd_journal_close(self.journal_handle);
        }
    }
}

impl Journal {
    pub fn new() -> Journal {
        let mut handle = std::ptr::null_mut() as *mut sys::sd_journal;

        ffi_invoke_and_expect!(sys::sd_journal_open(
            &mut handle,
            sys::SD_JOURNAL_LOCAL_ONLY
        ));

        Journal {
            journal_handle: handle,
        }
    }

    // TODO: Prolly want to return some sort of data struct here.
    // TODO: Make this async so that when we reach the end we wait via sd_journal_wait()
    // https://man7.org/linux/man-pages/man3/sd_journal_wait.3.html
    pub fn read(&mut self) {
        self.advance();
    }

    // TODO: Prolly want to return some sort of data struct here. Variant None or Some
    fn advance(&mut self) {
        // https://www.man7.org/linux/man-pages/man3/sd_journal_next.3.html
        // According to the man pages if we have reached the end we will return 0 otherwise 1 will be returned.
        let inc = ffi_invoke_and_expect!(sys::sd_journal_next(self.journal_handle));

        if inc == 1 {
            self.read_next()
        }
    }

    fn read_next(&mut self) {}
}

#[test]
fn test_journal_new() {
    // Test should simply not panic
    let _j: Journal = Journal::new();
}

#[test]
fn test_journal_advance() {
    // Eventually this test can be, I submit a message to the
    // daemon and attempt to read it back.
    let mut j: Journal = Journal::new();
    j.advance();
}

#[test]
fn test_journal_read() {
    // Eventually this test can be, I submit a message to the
    // daemon and attempt to read it back.
    let mut j: Journal = Journal::new();
    j.read();
}
