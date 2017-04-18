extern crate rustc_version as rustc;

fn main() {
    if let Ok(version) = rustc::version() {
        if version.major == 1 && version.minor >= 5 {
            println!("cargo:rustc-cfg=__unicase__iter_cmp");
        }
        if version.major == 1 && version.minor >= 13 {
            println!("cargo:rustc-cfg=__unicase__defauler_hasher");
        }
    }
}
