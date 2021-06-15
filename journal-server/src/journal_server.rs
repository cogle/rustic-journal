use journal::Journal;

pub struct JournalServer {
    journal: Journal,
}

impl JournalServer {
    pub fn new() -> JournalServer {
        JournalServer {
            journal: Journal::new(),
        }
    }

    pub fn run(&self) {}

    fn read_log(&mut self) {}
}

#[test]
fn test_journal_server_new() {
    let _server = JournalServer::new();
}
