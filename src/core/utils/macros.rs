#[macro_use]
pub mod macros {
    macro_rules! journal_try {
        ($func:expr) => {
            let rc = unsafe {
                $func
            };
        }
    }
}


 