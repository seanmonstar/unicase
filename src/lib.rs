#![deny(missing_docs)]
#![cfg_attr(test, deny(warnings))]
#![feature(core, hash, std_misc)]

//! # Case
//!
//! Case provices a way of specifying strings that are case-insensitive.
//!
//! ## Example
//!
//! ```rust
//! use unicase::UniCase;
//!
//! let a = UniCase("foobar");
//! let b = UniCase("FoObAr");
//!
//! assert_eq!(a, b);
//! ```

use std::ascii::AsciiExt;
use std::fmt;
use std::hash;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Case Insensitive wrapper of strings.
#[derive(Clone, Debug)]
pub struct UniCase<S>(pub S);

impl<S> Deref for UniCase<S> {
    type Target = S;
    #[inline]
    fn deref<'a>(&'a self) -> &'a S {
        &self.0
    }
}

impl<S> DerefMut for UniCase<S> {
    #[inline]
    fn deref_mut<'a>(&'a mut self) -> &'a mut S {
        &mut self.0
    }
}

impl<S: Deref<Target=str>> Str for UniCase<S> {
    #[inline]
    fn as_slice(&self) -> &str {
        self.0.as_slice()
    }

}

impl<S: fmt::Display> fmt::Display for UniCase<S> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl<S: Deref<Target=str>> PartialEq for UniCase<S> {
    #[inline]
    fn eq(&self, other: &UniCase<S>) -> bool {
        self.eq_ignore_ascii_case(&***other)
    }
}


impl<S: Deref<Target=str>> PartialEq<S> for UniCase<S> {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        self.eq_ignore_ascii_case(&**other)
    }
}

impl<S: Deref<Target=str>> Eq for UniCase<S> {}

impl<S: FromStr> FromStr for UniCase<S> {
    type Err = <S as FromStr>::Err;
    fn from_str(s: &str) -> Result<UniCase<S>, <S as FromStr>::Err> {
        s.parse().map(UniCase)
    }
}

impl<H: hash::Writer + hash::Hasher, S: Deref<Target=str>> hash::Hash<H> for UniCase<S> {
    #[inline]
    fn hash(&self, hasher: &mut H) {
        for byte in self.as_slice().bytes().map(|b| b.to_ascii_lowercase()) {
            hasher.write(&[byte]);
        }
    }
}

#[test]
fn test_case_insensitive() {
    use std::hash::{hash, SipHasher};

    let a = UniCase("foobar");
    let b = UniCase("FOOBAR");

    assert_eq!(a, b);
    assert_eq!(hash::<_, SipHasher>(&a), hash::<_, SipHasher>(&b));
}
