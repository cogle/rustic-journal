use crate::log_format;
use crate::sys::journal as journal_c;
use libc::{c_void, size_t};
use std::collections::HashMap;
use std::ffi::CStr;

const JOURNAL_TIME_KEY: &'static str = "JOURNAL_ENTRY_TIMESTAMP";

// 1 TODO how to format data, journalctl offers some sort of fmt

// https://github.com/systemd/systemd/blob/master/src/journal/journalctl.c

// What here needs to be mutable and what doesn't

pub struct Journal {
    journal_handle: *mut journal_c::sd_journal,
    journal_entries: HashMap<String, String>,
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
            journal_entries: HashMap::new(),
        }
    }

    // TODO: Make this async so that when we reach the end we wait via sd_journal_wait()
    // https://man7.org/linux/man-pages/man3/sd_journal_wait.3.html
    pub fn read(&mut self) -> Option<String> {
        self.journal_entries.clear();
        match self.advance() {
            Some(_) => return Some(log_format::default_fmt(&self.journal_entries)),
            _ => return None,
        }
    }

    fn advance(&mut self) -> Option<()> {
        // https://www.man7.org/linux/man-pages/man3/sd_journal_next.3.html
        // According to the man pages if we have reached the end we will return 0 otherwise 1 will be returned.
        let inc = ffi_invoke_and_expect!(journal_c::sd_journal_next(self.journal_handle));

        if inc == 1 {
            self.obtain_journal_data();
            return Some(());
        }

        None
    }

    fn get_journal_monotonic(&mut self) {
        // TODO currently this is a null pointer for the last arg but should be of type sd_id128_t
        // https://man7.org/linux/man-pages/man3/sd_journal_get_monotonic_usec.3.html
        //int sd_journal_get_monotonic_usec(sd_journal *j, uint64_t *usec, sd_id128_t *boot_id);
    }

    fn get_journal_realtime(&mut self) {
        // https://man7.org/linux/man-pages/man3/sd_journal_get_realtime_usec.3.html
        let mut usec: u64 = 0;
        ffi_invoke_and_expect!(journal_c::sd_journal_get_realtime_usec(
            self.journal_handle,
            &mut usec,
        ));
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

            let journal_message = unsafe {
                let c_str: &CStr = CStr::from_ptr(data_ptr as *const _);
                c_str.to_str().unwrap()
            };

            match journal_message.find('=') {
                Some(idx) => {
                    let (key, msg) = journal_message.split_at(idx);
                    self.journal_entries
                        .insert(key.to_string(), msg.to_string());
                }
                _ => {}
            }

            if remaining == 0 {
                break;
            }
        }
    }

    fn obtain_journal_timestamp(&mut self) {}
}

#[test]
fn test_journal_new() {
    // Test should simply not panic
    let _j: Journal = Journal::new();
}
