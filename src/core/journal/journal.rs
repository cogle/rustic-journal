use crate::sys::journal as sys;

pub struct Journal {
    journal_handle: *mut sys::sd_journal,
}

impl Drop for Journal {
    fn drop(&mut self) {
        unsafe {
            sys::sd_journal_close(self.journal_handle);
        }
    }
}

impl Journal {
    pub fn new() -> Journal {
        let mut handle = std::ptr::null_mut() as *mut sys::sd_journal;

        ffi_invoke_and_expect!(sys::sd_journal_open(
            &mut handle,
            sys::SD_JOURNAL_LOCAL_ONLY
        ));

        Journal {
            journal_handle: handle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_new() {
        // Test should simply not panic
        let _j: Journal = Journal::new();
    }
}
