#![feature(default_type_params)]
#![deny(missing_doc)]
#![deny(warnings)]
#![unstable]

//! # Case
//!
//! Case provices a way of specifying strings that are case-insensitive.
//!
//! ## Example
//!
//! ```rust
//! use case::CaseInsensitive;
//!
//! let a = CaseInsensitive("foobar");
//! let b = CaseInsensitive("FoObAr");
//!
//! assert_eq!(a, b);
//! ```
use std::ascii::{AsciiExt, ASCII_LOWER_MAP};
use std::fmt;
use std::hash;
use std::str::Str;

/// Case Insensitive wrapper of `Str`s.
#[deriving(Clone)]
pub struct CaseInsensitive<S: Str>(pub S);

impl<S: Str> Str for CaseInsensitive<S> {
    fn as_slice(&self) -> &str {
        let CaseInsensitive(ref s) = *self;
        s.as_slice()
    }

}

impl<S: Str> fmt::Show for CaseInsensitive<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(fmt)
    }
}

impl<S: Str> PartialEq for CaseInsensitive<S> {
    fn eq(&self, other: &CaseInsensitive<S>) -> bool {
        self.as_slice().eq_ignore_ascii_case(other.as_slice())
    }
}

impl<S: Str> Eq for CaseInsensitive<S> {}

impl<H: hash::Writer, S: Str> hash::Hash<H> for CaseInsensitive<S> {
    #[inline]
    fn hash(&self, hasher: &mut H) {
        for byte in self.as_slice().bytes() {
            hasher.write([ASCII_LOWER_MAP[byte as uint]].as_slice());
        }
    }
}

#[test]
fn test_case_insensitive() {
    use std::hash::sip::hash;

    let a = CaseInsensitive("foobar");
    let b = CaseInsensitive("FOOBAR");

    assert_eq!(a, b);
    assert_eq!(hash(&a), hash(&b));
}
