use crate::sys;

#[derive(Debug)]
pub struct CJournalError {
    pub message: String,
    pub error_code: c_int
};

pub struct Journal {
    journal_handle: mut* sys::sd_journal;
};

impl Journal {
    pub fn new() -> Result<Journal, CJournalError> {
        CJournalError{
            message: "Test",
            error_code: -1
        };
    }
}
