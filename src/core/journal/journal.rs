use crate::sys;

pub struct Journal {
    journal: mut* sys::sd_journal;
};