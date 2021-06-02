use journalctl_sys as sys;

#[macro_use]
mod utils;

mod journal;
pub use self::journal::{Journal, Timestamp, DEFAULT_REAL_TIME_FORMAT};
