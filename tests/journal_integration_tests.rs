use journal::Journal;

#[test]
fn test_journal_read() {
    // Eventually this test can be, I submit a message to the daemon and attempt to read it back.
    let mut j: Journal = Journal::new();
    for _ in 0..1 {
        match j.read() {
            Some(map) => println!("{:#?}", map),
            _ => {}
        }
    }
}
