use std::ffi::CStr;

use crate::sys::helpers as sys_helpers;
use nix::errno::errno;

#[derive(Debug)]
pub struct CError {
    pub message: String,
    pub error_code: i32,
}

pub fn check_c_error_code(result: i32) -> Result<i32, CError> {
    if result >= 0 {
        return Ok(result);
    }

    let error_code = errno();
    let error_message = unsafe {
        let c_pointer = sys_helpers::strerror(error_code);
        let c_str: &CStr = CStr::from_ptr(c_pointer);
        c_str.to_str().unwrap().to_owned()
    };

    return Err(CError {
        message: error_message,
        error_code: error_code,
    });
}
