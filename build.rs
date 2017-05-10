extern crate rustc_version as rustc;

fn main() {
    if rustc::version().unwrap() >= rustc::Version::parse("1.5.0").unwrap() {
        println!("cargo:rustc-cfg=__unicase__iter_cmp");
    }
    if rustc::version().unwrap() >= rustc::Version::parse("1.13.0").unwrap() {
        println!("cargo:rustc-cfg=__unicase__defauler_hasher");
    }
}
