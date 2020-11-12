use crate::sys::journal as journal_c;
use libc::{c_void, size_t};
use std::collections::HashMap;
use std::ffi::CStr;

pub struct Journal {
    journal_handle: *mut journal_c::sd_journal,
}

impl Drop for Journal {
    fn drop(&mut self) {
        unsafe {
            journal_c::sd_journal_close(self.journal_handle);
        }
    }
}

impl Journal {
    pub fn new() -> Journal {
        let mut handle = std::ptr::null_mut() as *mut journal_c::sd_journal;

        ffi_invoke_and_expect!(journal_c::sd_journal_open(
            &mut handle,
            journal_c::SD_JOURNAL_LOCAL_ONLY
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
        let inc = ffi_invoke_and_expect!(journal_c::sd_journal_next(self.journal_handle));

        if inc == 1 {
            self.obtain_journal_data()
        }
    }

    fn obtain_journal_data(&mut self) {
        let mut data_ptr = std::ptr::null_mut() as *mut c_void;
        let mut len: size_t = 0;

        unsafe {
            // https://man7.org/linux/man-pages/man3/sd_journal_restart_data.3.html
            // This restarts the data, in the sense that it allows you to grab the next line.
            journal_c::sd_journal_restart_data(self.journal_handle);
        }

        loop {
            // https://man7.org/linux/man-pages/man3/sd_journal_enumerate_data.3.html
            // https://www.freedesktop.org/software/systemd/man/sd_journal_get_data.html
            let remaining = ffi_invoke_and_expect!(journal_c::sd_journal_enumerate_data(
                self.journal_handle,
                &data_ptr,
                &mut len
            ));

            let message = unsafe {
                let c_str: &CStr = CStr::from_ptr(data_ptr as *const _);
                c_str.to_str().unwrap().to_owned()
            };

            println!(
                "Got message of len: {}\tMessage: {}\t Remaining: {}",
                len, message, remaining
            );

            if remaining == 0 {
                break;
            }
        }
    }
}

#[test]
fn test_journal_new() {
    // Test should simply not panic
    let _j: Journal = Journal::new();
}
