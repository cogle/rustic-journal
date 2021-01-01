use journal::Journal;

#[test]
fn test_journal_new() {
    // Test should simply not panic
    let _j: Journal = Journal::new();
}

//#[test]
//fn test_journal_advance() {
//    // Eventually this test can be, I submit a message to the
//    // daemon and attempt to read it back.
//    let mut j: Journal = Journal::new();
//    for i in 0..100 {
//        println!("Idx {}", i);
//        j.advance();
//    }
//}
//

#[test]
fn test_journal_read() {
    // Eventually this test can be, I submit a message to the
    // daemon and attempt to read it back.
    let mut j: Journal = Journal::new();
    for i in 0..1000 {
        j.read();
    }
}

