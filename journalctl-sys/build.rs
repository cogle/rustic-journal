use pkg_config;

static LIBRARY_NAME: &str = "systemd";

fn main() {
    pkg_config::probe_library(LIBRARY_NAME).unwrap();
    println!("cargo:rustc-link-lib={}", LIBRARY_NAME)
}