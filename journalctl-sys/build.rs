extern crate pkg_config;

fn main() {
    pkg_config::probe_library("systemd").unwrap();
    // TODO update below to actually get it to compile with tests
    // println!("cargo:rustc-link-lib=systemd")
}