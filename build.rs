extern crate autocfg;

fn main() {
    autocfg::rerun_path(file!());

    let ac = autocfg::new();

    if ac.probe_rustc_version(1, 5) {
        autocfg::emit("__unicase__iter_cmp");
    }

    if ac.probe_rustc_version(1, 13) {
        autocfg::emit("__unicase__default_hasher");
    }

    if ac.probe_rustc_version(1, 31) {
        autocfg::emit("__unicase__const_fns");
    }

    if ac.probe_rustc_version(1, 36) {
        autocfg::emit("__unicase__core_and_alloc");
    }
}
