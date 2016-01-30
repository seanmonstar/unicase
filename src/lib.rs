#![cfg_attr(test, deny(missing_docs))]
#![cfg_attr(test, deny(warnings))]
#![cfg_attr(feature = "heap_size", feature(custom_derive, plugin))]
#![cfg_attr(feature = "heap_size", plugin(heapsize_plugin))]

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

#[cfg(feature = "heap_size")]
extern crate heapsize;

use std::ascii::AsciiExt;
#[cfg(iter_cmp)]
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// Case Insensitive wrapper of strings.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "heap_size", derive(HeapSizeOf))]
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

#[cfg(iter_cmp)]
impl<T: AsRef<str>> PartialOrd for UniCase<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(iter_cmp)]
impl<T: AsRef<str>> Ord for UniCase<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chars = self.as_ref().chars().map(|c| c.to_ascii_lowercase());
        let other_chars = other.as_ref().chars().map(|c| c.to_ascii_lowercase());
        self_chars.cmp(other_chars)
    }
}

impl<S: AsRef<str>> AsRef<str> for UniCase<S> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }

}

impl<S: fmt::Display> fmt::Display for UniCase<S> {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, fmt)
    }
}

impl<S: AsRef<str>> PartialEq for UniCase<S> {
    #[inline]
    fn eq(&self, other: &UniCase<S>) -> bool {
        self.as_ref().eq_ignore_ascii_case(other.as_ref())
    }
}


impl<S: AsRef<str>> PartialEq<S> for UniCase<S> {
    #[inline]
    fn eq(&self, other: &S) -> bool {
        self.as_ref().eq_ignore_ascii_case(other.as_ref())
    }
}

impl<S: AsRef<str>> Eq for UniCase<S> {}

impl<S: FromStr> FromStr for UniCase<S> {
    type Err = <S as FromStr>::Err;
    fn from_str(s: &str) -> Result<UniCase<S>, <S as FromStr>::Err> {
        s.parse().map(UniCase)
    }
}

impl<S: AsRef<str>> Hash for UniCase<S> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for byte in self.as_ref().bytes().map(|b| b.to_ascii_lowercase()) {
            hasher.write(&[byte]);
        }
    }
}

#[cfg(test)]
mod test {
    use super::UniCase;
    use std::hash::{Hash, Hasher, SipHasher};

    fn hash<T: Hash>(t: &T) -> u64 {
        let mut s = SipHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    #[test]
    fn test_case_insensitive() {
        let a = UniCase("foobar");
        let b = UniCase("FOOBAR");

        assert_eq!(a, b);
        assert_eq!(hash(&a), hash(&b));
    }

    #[cfg(iter_cmp)]
    #[test]
    fn test_case_cmp() {
        assert!(UniCase("foobar") == UniCase("FOOBAR"));
        assert!(UniCase("a") < UniCase("B"));

        assert!(UniCase("A") < UniCase("b"));
        assert!(UniCase("aa") > UniCase("a"));

        assert!(UniCase("a") < UniCase("aa"));
        assert!(UniCase("a") < UniCase("AA"));
    }
}
