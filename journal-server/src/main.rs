use journalctl_sys as sys;

mod config;
mod journal_server;
mod timestamp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {}
}
