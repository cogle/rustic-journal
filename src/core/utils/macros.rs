#[macro_export]
macro_rules! ffi_invoke_and_expect {
    ($func:expr) => {{
        unsafe { crate::utils::c_error::check_c_error_code($func).unwrap() }
    }};
}
