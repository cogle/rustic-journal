use journalctl_sys as sys;

#[macro_use]
mod utils;

mod log_format;
mod journal;
pub use self::journal::Journal;
