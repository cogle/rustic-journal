use crate::journal::{Journal, Timestamp, DEFAULT_REALTIME_FORMAT};

pub struct JournalServer<'a> {
    journal: Journal<'a>,
}

impl<'a> JournalServer<'a> {
    pub fn new() -> JournalServer<'a> {
        let timestamp = journal::Timestamp::Real(journal::DEFAULT_REALTIME_FORMAT);
        JournalServer {
            journal: Journal::new(timestamp),
        }
    }

    pub fn run(&self) {}

    fn read_log(&mut self) {}
}

#[test]
fn test_journal_server_new() {
    let _server = JournalServer::new();
}
