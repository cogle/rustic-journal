#![allow(non_camel_case_types)]
extern crate libc;

use libc::{c_int, c_void, size_t};

pub const SD_JOURNAL_LOCAL_ONLY: c_int = 1 << 0;
pub const SD_JOURNAL_RUNTIME_ONLY: c_int = 1 << 1;
pub const SD_JOURNAL_SYSTEM: c_int = 1 << 2;
pub const SD_JOURNAL_CURRENT_USER: c_int = 1 << 3;
pub const SD_JOURNAL_OS_ROOT: c_int = 1 << 4;
pub const SD_JOURNAL_ALL_NAMESPACES: c_int = 1 << 5;
pub const SD_JOURNAL_INCLUDE_DEFAULT_NAMESPACE: c_int = 1 << 6;

pub const JOURNAL_REALTIME_TIMESTAMP_KEY: &'static str = "_SOURCE_REALTIME_TIMESTAMP";
pub const JOURNAL_MONOTOMIC_TIMESTAMP_KEY: &'static str = "_SOURCE_MONOTOMIC_TIMESTAMP";

// Opaque Struct Documentation reference here
// https://doc.rust-lang.org/1.30.0/book/first-edition/ffi.html#representing-opaque-structs
#[repr(C)]
pub struct sd_journal {
    private: [u8; 0],
}

// union sd_id128 {
//         uint8_t bytes[16];
//         uint64_t qwords[2];
// };
// The above is the struct in C; since its max size is 16 bytes making it of size 16 bytes should work
#[repr(C)]
pub struct sd_id128_t {
    padding: [u8; 16],
}

// Simply for convience
impl sd_id128_t {
    pub fn new() -> sd_id128_t {
        sd_id128_t { padding: [0; 16] }
    }
}

extern "C" {
    pub fn sd_journal_close(j: *mut sd_journal);
    pub fn sd_journal_enumerate_data(
        j: *mut sd_journal,
        data: *const *mut c_void,
        len: *mut size_t,
    ) -> c_int;
    pub fn sd_journal_next(j: *mut sd_journal) -> c_int;
    pub fn sd_journal_open(ret: *mut *mut sd_journal, flags: c_int) -> c_int;
    pub fn sd_journal_restart_data(j: *mut sd_journal);
    pub fn sd_journal_get_realtime_usec(j: *mut sd_journal, usec: *mut u64) -> c_int;
    pub fn sd_journal_get_monotonic_usec(
        j: *mut sd_journal,
        usec: *mut u64,
        boot_id: *mut sd_id128_t,
    ) -> c_int;
}
