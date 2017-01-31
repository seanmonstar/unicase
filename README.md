# unicase

[![Build Status](https://travis-ci.org/seanmonstar/unicase.svg?branch=master)](https://travis-ci.org/seanmonstar/unicase)

[Documentation](https://docs.rs/unicase)

Compare strings when case is not important.

```rust
if UniCase::new(method) == UniCase::new('GET') {
    // GET request
}
```

## License

[MIT](./LICENSE)
