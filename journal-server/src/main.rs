use clap::{App, Arg};
use journalctl_sys as sys;
use std::fs::File;

mod config;
use config::Config;

mod journal_server;
mod timestamp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("Rustic-Journal Server")
        .version("0.1")
        .author("cogle")
        .about("A server that connects into systemd via FFI")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .about("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let mut config: Option<Config> = None;
    if let Some(file_url) = app.value_of("config") {
        match File::open(file_url) {
            Ok(mut file) => {
                config = Some(Config::parse(&mut file));
            }
            Err(err) => {
                panic!("Accessing configuration file {} failed with error: {}", file_url, err);
            }
        }
    }

    // If no config argument has been provided use the default and pass into the server.
    config.unwrap_or(Config::new());

    Ok(())
}
