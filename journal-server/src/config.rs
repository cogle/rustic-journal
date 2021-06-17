use crate::timestamp::Timestamp;

mod defaults {
    pub const DEFAULT_REALTIME_FORMAT: &str = "%d-%m-%Y %H:%M:%S%.6f%:z";
}

#[derive(Debug)]
pub struct Config {
    timestamp: Timestamp,
}

impl Config {
    pub fn new() -> Config {
        Config {
            timestamp: Timestamp::Real(defaults::DEFAULT_REALTIME_FORMAT.to_string()),
        }
    }

    pub fn parse() -> Config {
        Config {
            timestamp: Timestamp::Mono,
        }
    }
}

#[test]
fn test_config_new() {}
