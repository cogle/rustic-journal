//use std::collections::HashMap;

//const FILTER_PID_KEY: &str = "PID";
//const FILTER_SYSTEMD_UNIT_KEY: &str = "_SYSTEMD_UNIT";
//const FILTER_MESSAGE_KEY: &str = "MESSAGE";
//const FILTER_SYSLOG_ID_KEY: &str =  "SYSLOG_IDENTIFIER";

/*
const FILTER_FIELD_KEYS: &[&str] = &[
    FILTER_PID_KEY,
    "_COMM",
    FILTER_MESSAGE_KEY,
    "PRIORITY",
    "_TRANSPORT",
    "_HOSTNAME",
    "SYSLOG_PID",
    "SYSLOG_IDENTIFIER",
    "_SOURCE_REALTIME_TIMESTAMP",
    "_SOURCE_MONOTONIC_TIMESTAMP",
    "CONFIG_FILE",
    FILTER_SYSTEMD_UNIT_KEY,
    "_SYSTEMD_USER_UNIT",
    "DOCUMENTATION",
];


// Look up the values associated with some keys.
let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
for &book in &to_find {
    match book_reviews.get(book) {
        Some(review) => println!("{}: {}", book, review),
        None => println!("{} is unreviewed.", book)
    }
}

*/

// Source Reference Code
// https://github.com/systemd/systemd/blob/master/src/shared/logs-show.c
// TODO focus on library then formatting can come last

/*
pub fn default_fmt(data: &HashMap<String, String>) -> Option<String> {
    //println!("{:?}", data);

    Some(format!(
        "[{}]: {}",
        data.get(FILTER_SYSLOG_ID_KEY).unwrap(),
        data.get(FILTER_MESSAGE_KEY).unwrap()
    ))
}
*/
