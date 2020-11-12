extern crate journalctl_sys as sys;

#[macro_use]
mod utils;

mod journal;
pub use self::journal::Journal;
