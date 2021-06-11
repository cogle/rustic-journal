use crate::sys::journal as journal_c;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use libc::{c_void, size_t};
use std::cmp;
use std::collections::HashMap;
use std::ffi::CStr;

pub const DEFAULT_REALTIME_FORMAT: &str = "%d-%m-%Y %H:%M:%S%.6f%:z";

#[derive(Debug)]
pub enum Timestamp<'a> {
    Mono,
    Real(&'a str),
}

#[derive(Debug)]
pub struct JournalData {
    journal_map: HashMap<String, String>,
}

// FFI API wrapper around systemd journal commands.
pub struct Journal<'a> {
    // NOTE: We are using a C FFI, in this C FFI the sd_journal pointer below may be mutated in the C function
    // call. As such it's best practice, since rust can't track memory in FFI calls, to label this as mut and all
    // function calls that require journal_handle as mutable.
    journal_handle: *mut journal_c::sd_journal,
    timestamp_display: Timestamp<'a>,
}

impl<'a> Drop for Journal<'a> {
    fn drop(&mut self) {
        unsafe {
            journal_c::sd_journal_close(self.journal_handle);
        }
    }
}

fn split_usec_string(usec_string: &String) -> (&str, &str) {
    if usec_string.len() > 6 {
        let sec_str = &usec_string[0..std::cmp::max(0, &usec_string.len() - 6)];
        let milli_str = &usec_string[std::cmp::max(&usec_string.len() - 6, 0)..];

        return (sec_str, milli_str);
    } else {
        return ("0", &usec_string[..]);
    }
}

impl<'a> Journal<'a> {
    pub fn new(timestamp: Timestamp<'a>) -> Journal<'a> {
        let mut handle = std::ptr::null_mut() as *mut journal_c::sd_journal;

        ffi_invoke_and_expect!(journal_c::sd_journal_open(
            &mut handle,
            journal_c::SD_JOURNAL_LOCAL_ONLY
        ));

        Journal {
            journal_handle: handle,
            timestamp_display: timestamp,
        }
    }

    // TODO: Make this async so that when we reach the end we wait via sd_journal_wait()
    // https://man7.org/linux/man-pages/man3/sd_journal_wait.3.html
    pub fn read(&mut self) -> Option<JournalData> {
        self.advance()
    }

    fn advance(&mut self) -> Option<JournalData> {
        // https://www.man7.org/linux/man-pages/man3/sd_journal_next.3.html
        // According to the man pages if we have reached the end we will return 0 otherwise 1 will be
        // returned.
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

    fn get_journal_realtime(&mut self) -> u64 {
        // https://man7.org/linux/man-pages/man3/sd_journal_get_realtime_usec.3.html
        let mut usec: u64 = 0;

        ffi_invoke_and_expect!(journal_c::sd_journal_get_realtime_usec(self.journal_handle, &mut usec));

        usec
    }

    fn obtain_journal_data(&mut self) -> JournalData {
        // NOTE: Marked mut because this goes into the C func and its unknown as to what may go on there.
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
                // [..len] Ensures we only copy back the valid portion of the string as reported by the enumerate
                // function
                c_str.to_str().unwrap()[..len].to_string()
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

        self.obtain_journal_timestamp(&mut journal_entries);

        JournalData {
            journal_map: journal_entries,
        }
    }

    fn format_realtime(&self, timestamp_usec: u64, format_str: &str) -> String {
        let usec_string = timestamp_usec.to_string();
        let (sec_str, micro_str) = split_usec_string(&usec_string);

        // Multiply by 1000 to convert microseconds to nanoseconds
        let ndt = NaiveDateTime::from_timestamp(
            sec_str.parse::<i64>().unwrap(),
            micro_str.parse::<u32>().unwrap() * 1000,
        );

        // Convert here to local time. If wanted the client can convert to UTC; however, it is much harder
        // for the client or consumer at a later stage to convert to local time without all the additional
        // information as such the local time stamp conversion is done here.
        let ldt: DateTime<Local> = DateTime::from(DateTime::<Utc>::from_utc(ndt, Utc));

        ldt.format(format_str).to_string()
    }

    fn format_monotomic(&self, timestamp_usec: u64) -> String {
        let usec_string = timestamp_usec.to_string();
        let (sec_str, micro_str) = split_usec_string(&usec_string);

        format!("{:>5}.{:0>6}", sec_str, micro_str)
    }

    fn obtain_journal_timestamp(&mut self, journal_entries: &mut HashMap<String, String>) {
        match self.timestamp_display {
            Timestamp::Real(fmt_str) if !journal_entries.contains_key(journal_c::JOURNAL_REALTIME_TIMESTAMP_KEY) => {
                // This gets the naive datetime
                let naive_ts_usec = self.get_journal_realtime();
                journal_entries.insert(
                    journal_c::JOURNAL_REALTIME_TIMESTAMP_KEY.to_string(),
                    self.format_realtime(naive_ts_usec, fmt_str),
                );
            }
            Timestamp::Mono if !journal_entries.contains_key(journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY) => {
                // This get the journal time
                let usec_ts = self.get_journal_monotonic();
                journal_entries.insert(
                    journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY.to_string(),
                    self.format_monotomic(usec_ts),
                );
            }
            Timestamp::Real(_fmt_str) => {
                // TODO I really got no idea what todo here
            }
            Timestamp::Mono => {
                // In the case that the timestamp is already provided use and format that.
                let usec_ts: u64 = journal_entries
                    .get(journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY)
                    .unwrap()
                    .parse()
                    .unwrap();
                journal_entries.insert(
                    journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY.to_string(),
                    self.format_monotomic(usec_ts),
                );
            }
        }
    }
}

#[test]
fn test_journal_new() {
    // Test should simply not panic
    let timestamp = Timestamp::Real(DEFAULT_REALTIME_FORMAT);
    let _j: Journal = Journal::new(timestamp);
}
