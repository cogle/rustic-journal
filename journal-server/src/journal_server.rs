use crate::journal::Journal;

pub struct JournalServer {
    journal: Journal,
}

impl JournalServer {
    pub fn new() -> JournalServer {
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
