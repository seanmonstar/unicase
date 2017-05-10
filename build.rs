extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    if version().unwrap() >= Version::parse("1.5.0").unwrap() {
        println!("cargo:rustc-cfg=__unicase__iter_cmp");
    }
    if version().unwrap() >= Version::parse("1.13.0").unwrap() {
        println!("cargo:rustc-cfg=__unicase__defauler_hasher");
    }
}
