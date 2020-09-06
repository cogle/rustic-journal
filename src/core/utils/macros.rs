#[macro_export]
macro_rules! journal_try {
    ($func:expr) => {{
        unsafe { crate::utils::c_error::check_c_error_code($func) }
    }};
}
