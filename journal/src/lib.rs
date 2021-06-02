use journalctl_sys as sys;

#[macro_use]
mod utils;

mod journal;
pub use self::journal::Journal;
pub use self::journal::Timestamp;
pub use self::journal::DEFAULT_REAL_TIME_FORMAT;
