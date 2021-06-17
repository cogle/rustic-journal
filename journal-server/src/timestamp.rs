use crate::sys::journal as journal_c;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use journal::Journal;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Timestamp {
    Mono,
    Real(String),
}

pub fn obtain_journal_timestamp(
    timestamp_display: &Timestamp,
    journal: &mut Journal,
    journal_entries: &mut HashMap<String, String>,
) {
    match timestamp_display {
        Timestamp::Real(fmt_str) if !journal_entries.contains_key(journal_c::JOURNAL_REALTIME_TIMESTAMP_KEY) => {
            // This gets the naive datetime
            let naive_ts_usec = journal.get_journal_realtime();
            journal_entries.insert(
                journal_c::JOURNAL_REALTIME_TIMESTAMP_KEY.to_string(),
                format_realtime(naive_ts_usec, &fmt_str),
            );
        }
        Timestamp::Mono if !journal_entries.contains_key(journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY) => {
            // This get the journal time
            let usec_ts = journal.get_journal_monotonic();
            journal_entries.insert(
                journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY.to_string(),
                format_monotomic(usec_ts),
            );
        }
        Timestamp::Real(_fmt_str) => {
            // So for this its a little complex, there appears to be in the source code a lot of
            // cases to handle a situation like this and it will have to be copied over and
            // rustified.
        }
        Timestamp::Mono => {
            // In the case that the timestamp is already provided use and format that.
            let usec_ts: u64 = journal_entries
                .get(journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY)
                .unwrap()
                .parse()
                .unwrap();
            journal_entries.insert(
                journal_c::JOURNAL_MONOTONIC_TIMESTAMP_KEY.to_string(),
                format_monotomic(usec_ts),
            );
        }
    }
}

fn split_usec_string(usec_string: &String) -> (&str, &str) {
    if usec_string.len() > 6 {
        let sec_str = &usec_string[0..std::cmp::max(0, &usec_string.len() - 6)];
        let milli_str = &usec_string[std::cmp::max(&usec_string.len() - 6, 0)..];

        return (sec_str, milli_str);
    } else {
        return ("0", &usec_string[..]);
    }
}

fn format_realtime(timestamp_usec: u64, format_str: &str) -> String {
    let usec_string = timestamp_usec.to_string();
    let (sec_str, micro_str) = split_usec_string(&usec_string);

    // Multiply by 1000 to convert microseconds to nanoseconds
    let ndt = NaiveDateTime::from_timestamp(
        sec_str.parse::<i64>().unwrap(),
        micro_str.parse::<u32>().unwrap() * 1000,
    );

    // Convert here to local time. If wanted the client can convert to UTC; however, it is much harder
    // for the client or consumer at a later stage to convert to local time without all the additional
    // information as such the local time stamp conversion is done here.
    let ldt: DateTime<Local> = DateTime::from(DateTime::<Utc>::from_utc(ndt, Utc));

    ldt.format(format_str).to_string()
}

fn format_monotomic(timestamp_usec: u64) -> String {
    let usec_string = timestamp_usec.to_string();
    let (sec_str, micro_str) = split_usec_string(&usec_string);

    format!("{:>5}.{:0>6}", sec_str, micro_str)
}

mod testing {
    use crate::timestamp;
    use chrono::{DateTime, Local, NaiveDateTime, Utc};

    #[test]
    fn test_split_usec_string_below_threshold() {
        {
            let test_string = "0".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "0");
        }
        {
            let test_string = "1".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "1");
        }
        {
            let test_string = "00".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "00");
        }
        {
            let test_string = "10".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "10");
        }
        {
            let test_string = "123".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "123");
        }
        {
            let test_string = "1234".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "1234");
        }
        {
            let test_string = "12345".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "12345");
        }
        {
            let test_string = "123456".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "0");
            assert_eq!(bot, "123456");
        }
    }

    #[test]
    fn test_split_usec_string_above_threshold() {
        {
            let test_string = "1234567".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "1");
            assert_eq!(bot, "234567");
        }
        {
            let test_string = "7654321".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "7");
            assert_eq!(bot, "654321");
        }
        {
            let test_string = "12345673913148412741".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "12345673913148");
            assert_eq!(bot, "412741");
        }
        {
            let test_string = "4859248723472492123456733312913148412301843183201730741".to_string();
            let (top, bot) = timestamp::split_usec_string(&test_string);

            assert_eq!(top, "4859248723472492123456733312913148412301843183201");
            assert_eq!(bot, "730741");
        }
    }
}
