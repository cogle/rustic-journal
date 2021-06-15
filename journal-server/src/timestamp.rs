use chrono::{DateTime, Local, NaiveDateTime, Utc};
use journal::Journal;
use journalctl_sys::journal as journal_c;
use std::collections::HashMap;

pub const DEFAULT_REALTIME_FORMAT: &str = "%d-%m-%Y %H:%M:%S%.6f%:z";

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
            // TODO I really got no idea what todo here
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
