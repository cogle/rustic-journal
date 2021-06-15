mod timestamp;
use crate::timestamp::{obtain_journal_timestamp, Timestamp, DEFAULT_REALTIME_FORMAT};

mod config;
use crate::config::Config;

mod journal_server;
use crate::journal_server::JournalServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {}
}
