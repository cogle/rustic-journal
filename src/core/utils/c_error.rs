use std::os::raw::c_int;

#[derive(Debug)]
pub struct CError {
    pub message: String,
    pub error_code: c_int,
}
