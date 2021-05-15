use crate::sys::journal as journal_c;
use chrono::NaiveDateTime;
use libc::{c_void, size_t};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ffi::CStr;

#[derive(Debug)]
enum Timestamp {
    Mono(u64),
    Real(chrono::NaiveDateTime),
}

#[derive(Debug)]
enum TimestampType {
    Mono,
    Real,
}

#[derive(Debug)]
pub struct JournalData {
    journal_map: HashMap<String, String>,
    timestamp: Timestamp,
}

pub struct Journal {
    // NOTE: Function invoking sd_journal in non-const context are mut. This is because we are using a C FFI. In this C FFI the sd_journal pointer below may be mutated in the C function call. As such it's best practice, since rust can't track memory in FFI calls, to label this as mut and all function calls as mutable.
    journal_handle: *mut journal_c::sd_journal,
    timestamp_display: TimestampType,
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
            timestamp_display: TimestampType::Real,
        }
    }

    // TODO: Make this async so that when we reach the end we wait via sd_journal_wait()
    // https://man7.org/linux/man-pages/man3/sd_journal_wait.3.html
    pub fn read(&mut self) -> Option<JournalData> {
        self.advance()
    }

    fn advance(&mut self) -> Option<JournalData> {
        // https://www.man7.org/linux/man-pages/man3/sd_journal_next.3.html
        // According to the man pages if we have reached the end we will return 0 otherwise 1 will be returned.
        let inc = ffi_invoke_and_expect!(journal_c::sd_journal_next(self.journal_handle));

        if inc == 1 {
            return Some(self.obtain_journal_data());
        }

        None
    }

    fn get_journal_monotonic(&mut self) -> u64 {
        // https://man7.org/linux/man-pages/man3/sd_journal_get_monotonic_usec.3.html
        let mut usec: u64 = 0;
        let mut boot_id = journal_c::sd_id128_t::new();

        ffi_invoke_and_expect!(journal_c::sd_journal_get_monotonic_usec(
            self.journal_handle,
            &mut usec,
            &mut boot_id
        ));

        usec
    }

    fn get_journal_realtime(&mut self) -> NaiveDateTime {
        // https://man7.org/linux/man-pages/man3/sd_journal_get_realtime_usec.3.html
        let mut usec: u64 = 0;

        ffi_invoke_and_expect!(journal_c::sd_journal_get_realtime_usec(
            self.journal_handle,
            &mut usec,
        ));

        let usec_str = usec.to_string();

        let sec_str = usec_str[0..&usec_str.len() - 6].to_string();
        let milli_str = usec_str[&usec_str.len() - 6..].to_string();

        NaiveDateTime::from_timestamp(
            sec_str.parse::<i64>().unwrap(),
            milli_str.parse::<u32>().unwrap(),
        )
    }

    fn obtain_journal_data(&mut self) -> JournalData {
        let mut data_ptr = std::ptr::null_mut() as *mut c_void;
        let mut len: size_t = 0;

        let mut journal_entries: HashMap<String, String> = HashMap::new();

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
                    journal_entries.insert(key.to_string(), msg[1..].to_string());
                }
                _ => {}
            }

            if remaining == 0 {
                break;
            }
        }

        JournalData {
            journal_map: journal_entries,
            timestamp: self.obtain_journal_timestamp(),
        }
    }

    fn obtain_journal_timestamp(&mut self) -> Timestamp {
        match self.timestamp_display {
            TimestampType::Real => {
                return Timestamp::Real(self.get_journal_realtime());
            }
            TimestampType::Mono => {
                return Timestamp::Mono(self.get_journal_monotonic());
            }
        }
    }
}

#[test]
fn test_journal_new() {
    // Test should simply not panic
    let _j: Journal = Journal::new();
}
