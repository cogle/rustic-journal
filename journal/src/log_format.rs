use std::collections::HashMap;

const DEFAULT_FILTER_FIELD_KEYS: &[&str] = &[
    "_PID",
    "_COMM",
    "MESSAGE",
    "PRIORITY",
    "_TRANSPORT",
    "_HOSTNAME",
    "SYSLOG_PID",
    "SYSLOG_IDENTIFIER",
    "_SOURCE_REALTIME_TIMESTAMP",
    "_SOURCE_MONOTONIC_TIMESTAMP",
    "CONFIG_FILE",
    "_SYSTEMD_UNIT",
    "_SYSTEMD_USER_UNIT",
    "DOCUMENTATION",
];

// Source Reference Code
// https://github.com/systemd/systemd/blob/master/src/shared/logs-show.c

pub fn default_fmt(data: &HashMap<String, String>) -> String {
    String::from(":)")
}
