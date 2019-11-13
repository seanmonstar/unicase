extern crate autocfg;

fn main() {
    autocfg::rerun_path(file!());

    let ac = autocfg::new();

    ac.emit_has_path("core::iter::Iterator::cmp");
    ac.emit_has_type("std::collections::hash_map::DefaultHasher");

    // For `const fn` support
    ac.emit_rustc_version(1, 31);

    ac.emit_sysroot_crate("alloc");
}