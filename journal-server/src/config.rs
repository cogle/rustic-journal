use crate::timestamp::Timestamp;
use std::fs::File;

// Default values for the config struct to use
mod defaults {
    pub const DEFAULT_REALTIME_FORMAT: &str = "%d-%m-%Y %H:%M:%S%.6f%:z";
}

#[derive(Debug)]
pub struct Config {
    timestamp: Timestamp,
}

impl Config {
    // This function returns the default configureation file
    pub fn new() -> Self {
        Config {
            timestamp: Timestamp::Real(defaults::DEFAULT_REALTIME_FORMAT.to_string()),
        }
    }

    // This function takes in the path to the provided configuration file and then parses from it and
    // extracts out the relevant parts. This is done by using the default configuration and then
    // overwriting the default values with the ones specified in the configuration file. This is done so
    // that the user does not need to specifiy a complete configuration in order to operate the program.
    pub fn parse(file: &mut std::fs::File) -> Self {
        let mut config = Config::new();
        config.timestamp = Timestamp::Mono;

        config
    }
}

#[test]
fn test_config_new() {}
