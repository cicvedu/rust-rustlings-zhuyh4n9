
use std::fs;
use std::path::Path;

fn main() {
    if let Ok(test) = std::env::var("CARGO_PKG_NAME") {
        if test == "drive3" {
            let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            println!("cargo:rustc-env=TEST_FOO={}", timestamp);
        } else if test == "drive4" {
            println!("cargo:rustc-cfg=feature=\"pass\"");
        }
    }
}