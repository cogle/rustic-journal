use crate::journal::{Journal};
use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub const DEFAULT_REALTIME_FORMAT: &str = "%d-%m-%Y %H:%M:%S%.6f%:z";

#[derive(Debug)]
pub enum Timestamp<'a> {
    Mono,
    Real(&'a str),
}

fn obtain_journal_timestamp(&mut Journal journal, journal_entries: &mut HashMap<String, String>) {
    match self.timestamp_display {
        Timestamp::Real(fmt_str) if !journal_entries.contains_key(journal_c::JOURNAL_REALTIME_TIMESTAMP_KEY) => {
            // This gets the naive datetime
            let naive_ts_usec = journal.get_journal_realtime();
            journal_entries.insert(
                journal_c::JOURNAL_REALTIME_TIMESTAMP_KEY.to_string(),
                format_realtime(naive_ts_usec, fmt_str),
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

fn format_realtime(&self, timestamp_usec: u64, format_str: &str) -> String {
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

fn format_monotomic(&self, timestamp_usec: u64) -> String {
    let usec_string = timestamp_usec.to_string();
    let (sec_str, micro_str) = split_usec_string(&usec_string);

    format!("{:>5}.{:0>6}", sec_str, micro_str)
}
