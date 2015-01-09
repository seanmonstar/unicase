#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]

//! # Case
//!
//! Case provices a way of specifying strings that are case-insensitive.
//!
//! ## Example
//!
//! ```rust
//! use caseless::Caseless;
//!
//! let a = Caseless("foobar");
//! let b = Caseless("FoObAr");
//!
//! assert_eq!(a, b);
//! ```

use std::ascii::AsciiExt;
use std::fmt;
use std::hash;
use std::ops::{Deref, DerefMut};

/// Case Insensitive wrapper of strings.
#[derive(Clone, Show)]
pub struct Caseless<S>(pub S);

impl<S> Deref for Caseless<S> {
    type Target = S;
    #[inline]
    fn deref<'a>(&'a self) -> &'a S {
        &self.0
    }
}

impl<S> DerefMut for Caseless<S> {
    #[inline]
    fn deref_mut<'a>(&'a mut self) -> &'a mut S {
        &mut self.0
    }
}

#[allow(unstable)]
impl<S: Deref<Target=str>> Str for Caseless<S> {
    #[inline]
    fn as_slice(&self) -> &str {
        self.0.as_slice()
    }

}

#[allow(unstable)]
impl<S: fmt::String> fmt::String for Caseless<S> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::String::fmt(&self.0, fmt)
    }
}

impl<S: Deref<Target=str>> PartialEq for Caseless<S> {
    #[inline]
    #[allow(unstable)]
    fn eq(&self, other: &Caseless<S>) -> bool {
        self.eq_ignore_ascii_case(&***other)
    }
}


impl<S: Deref<Target=str>> PartialEq<S> for Caseless<S> {
    #[inline]
    #[allow(unstable)]
    fn eq(&self, other: &S) -> bool {
        self.eq_ignore_ascii_case(&**other)
    }
}

impl<S: Deref<Target=str>> Eq for Caseless<S> {}

#[allow(unstable)]
impl<H: hash::Writer + hash::Hasher, S: Deref<Target=str>> hash::Hash<H> for Caseless<S> {
    #[inline]
    fn hash(&self, hasher: &mut H) {
        for byte in self.as_slice().bytes().map(|b| b.to_ascii_lowercase()) {
            hasher.write(&[byte]);
        }
    }
}

#[test]
#[allow(unstable)]
fn test_case_insensitive() {
    use std::hash::{hash, SipHasher};

    let a = Caseless("foobar");
    let b = Caseless("FOOBAR");

    assert_eq!(a, b);
    assert_eq!(hash::<_, SipHasher>(&a), hash::<_, SipHasher>(&b));
}
