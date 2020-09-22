#![allow(non_camel_case_types)]

use std::os::raw::{c_char, c_int};

extern "C" {
    pub fn strerror(errnum: c_int) -> *const c_char;
}
