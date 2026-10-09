# unicase

[![crates.io](https://img.shields.io/crates/v/unicase.svg)](https://crates.io/crates/unicase)
[![Released API docs](https://docs.rs/unicase/badge.svg)](https://docs.rs/unicase)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![CI](https://github.com/seanmonstar/unicase/actions/workflows/CI.yml/badge.svg)](https://github.com/seanmonstar/unicase/actions?query=workflow%3ACI)

Compare strings when case is not important (using Unicode Case-folding).

```rust
// ignore ASCII case
let a = UniCase::new("foobar");
let b = UniCase::new("FOOBAR");

assert_eq!(a, b);

// using unicode case-folding
let c = UniCase::new("Maße")
let d = UniCase::new("MASSE");
assert_eq!(c, d);
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/license/mit)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
