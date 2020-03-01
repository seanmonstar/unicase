#![cfg(feature = "serde")]

//! Implementations of [`Serialize`] and [`Deserialize`] for [`UniCase`] and [`Ascii`].
//!
//! Opt-in with the feature `serde`. (Requires the rust standard library)
//!
//! ## Serialization
//!
//! Serialization for any `UniCase<S>` and `Ascii<S>` where `S: AsRef<str>` is implemented.
//!
//! ## Deserialization
//!
//! Deserialization for `UniCase<S>` and `Ascii<S>` where `S` is either a `String`,
//! `&'de str` or `Cow<'de, str>` is implemented.
//!
//! ## Example
//!
//!```rust
//! #[macro_use]
//! extern crate serde;
//! extern crate unicase;
//!
//! use std::borrow::Cow;
//! use unicase::{UniCase, Ascii};
//!
//! #[derive(Serialize, Deserialize)]
//! struct UniCaseExample<'a> {
//!     owned: UniCase<String>,
//!     #[serde(borrow)]
//!     borrowed_str: UniCase<&'a str>,
//!     #[serde(borrow)]
//!     cow_str: UniCase<Cow<'a, str>>,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct AsciiExample<'a> {
//!     owned: Ascii<String>,
//!     #[serde(borrow)]
//!     borrowed_str: Ascii<&'a str>,
//!     #[serde(borrow)]
//!     cow_str: Ascii<Cow<'a, str>>,
//! }
//!
//! fn main() {}
//! ```
//!
//! [`Serialize`]: ../serde/trait.Serialize.html
//! [`Deserialize`]: ../serde/trait.Deserialize.html
//! [`UniCase`]: ../unicase/struct.UniCase.html
//! [`Ascii`]: ../unicase/struct.Ascii.html

extern crate serde;

use {UniCase, Ascii};

use core::marker::PhantomData;
use core::str::FromStr;

use alloc::borrow::Cow;
use alloc::str;
use alloc::string::String;
use alloc::string::ToString;
use self::serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use self::serde::de::Unexpected;
use self::serde::export::{fmt, Vec};

macro_rules! serialize_impl {
    ($for:ident) => (
        impl<S: AsRef<str>> Serialize for $for<S> {
            #[inline]
            fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
                self.as_ref().serialize(serializer)
            }
        }
    );
}

serialize_impl!(UniCase);
serialize_impl!(Ascii);

/// Used when ownership of the data is needed.
///
/// Conversion is done using the `FromStr::from_str` function.
///
/// ## Example
///
/// As generic type bound.
///
///```rust
/// #[macro_use]
/// extern crate serde;
/// extern crate unicase;
///
/// use std::str::FromStr;
/// use unicase::{UniCase, Ascii};
/// use unicase::serde::owned;
///
/// #[derive(Serialize, Deserialize)]
/// struct Example<T: FromStr + AsRef<str>> {
///     #[serde(deserialize_with = "owned::unicase_deserialize")]
///     unicase: UniCase<T>,
///     #[serde(deserialize_with = "owned::ascii_deserialize")]
///     ascii: Ascii<T>,
/// }
///
/// fn main() {}
/// ```
pub mod owned {
    use super::{Ascii, UniCase, de, fmt, Deserialize, Deserializer, FromStr, ToString, String,
                Unexpected, str, PhantomData};

    macro_rules! deserialize_impl {
        ($for:ident, $func:ident) => (
            pub fn $func<'de, S, D>(deserializer: D) -> Result<$for<S>, D::Error>
            where
                S: FromStr + AsRef<str>,
                D: Deserializer<'de>,
            {
                struct Visitor<S>(PhantomData<S>);

                impl<'de, S: FromStr + AsRef<str>> de::Visitor<'de> for Visitor<S> {
                    type Value = $for<S>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a string")
                    }

                    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                        v.to_string()
                            .parse()
                            .map(|s| $for::new(s))
                            .map_err(|_| E::custom("FromStr conversion failed"))
                    }

                    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                        $for::from_str(s).map_err(|_| E::custom("FromStr conversion failed"))
                    }

                    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                        match str::from_utf8(v) {
                            Ok(s) => self.visit_str(s),
                            Err(e) => Err(
                                de::Error::invalid_value(
                                    Unexpected::Other(&e.to_string()),
                                    &"valid utf-8")
                            ),
                        }
                    }
                }

                deserializer.deserialize_string(Visitor(PhantomData))
            }

            impl<'de> Deserialize<'de> for $for<String> {
                #[inline]
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    $func(deserializer)
                }
            }
        );
    }

    deserialize_impl!(UniCase, unicase_deserialize);
    deserialize_impl!(Ascii, ascii_deserialize);
}

/// Used when no ownership of the data is needed, this allows zero-copy deserialization as
/// data is only borrowed.
///
/// Conversion is done using the `Into::into` function.
pub mod borrowed {
    use super::{Ascii, UniCase, de, fmt, Deserialize, Deserializer, str, ToString, Unexpected,
                PhantomData};

    macro_rules! deserialize_impl {
        ($for:ident, $func:ident) => (
            pub fn $func<'de: 'a, 'a, S, D>(deserializer: D) -> Result<$for<S>, D::Error>
            where
                S: From<&'a str> + AsRef<str> + 'a,
                D: Deserializer<'de>,
            {
                struct Visitor<S>(PhantomData<S>);

                impl<'de: 'a, 'a, S: From<&'a str> + AsRef<str> + 'a> de::Visitor<'de> for Visitor<S> {
                    type Value = $for<S>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a borrowed str")
                    }

                    fn visit_borrowed_str<E: de::Error>(self, s: &'de str) -> Result<Self::Value, E> {
                        Ok($for::new(s.into()))
                    }

                    fn visit_borrowed_bytes<E: de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                        match str::from_utf8(v) {
                            Ok(s) => self.visit_borrowed_str(s),
                            Err(e) => Err(
                                de::Error::invalid_value(
                                    Unexpected::Other(&e.to_string()),
                                    &"valid utf-8")
                            ),
                        }
                    }
                }

                deserializer.deserialize_str(Visitor(PhantomData))
            }

            impl<'de: 'a, 'a> Deserialize<'de> for $for<&'a str> {
                #[inline]
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    $func(deserializer)
                }
            }
        );
    }

    deserialize_impl!(UniCase, unicase_deserialize);
    deserialize_impl!(Ascii, ascii_deserialize);
}

macro_rules! deserialize_cow_impl {
    ($for:ident, $bool:path) => (
        impl<'de: 'a, 'a> Deserialize<'de> for $for<Cow<'a, str>> {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct Visitor;

                impl<'de> de::Visitor<'de> for Visitor {
                    type Value = $for<Cow<'de, str>>;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a borrowed str or string")
                    }

                    fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                        Ok($bool(Cow::Owned(v.to_string())))
                    }

                    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                        Ok($for::new(Cow::Owned(v.to_string())))
                    }

                    fn visit_borrowed_str<E: de::Error>(self, v: &'de str) -> Result<Self::Value, E> {
                        Ok($for::new(Cow::Borrowed(v)))
                    }

                    fn visit_string<E: de::Error>(self, v: String) -> Result<Self::Value, E> {
                        Ok($for::new(Cow::Owned(v)))
                    }

                    fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                        str::from_utf8(v)
                            .map(|s|$for::new(Cow::Owned(s.to_string())))
                            .map_err(|e|de::Error::invalid_value(
                                Unexpected::Other(&e.to_string()), &"valid utf-8")
                            )
                    }

                    fn visit_borrowed_bytes<E: de::Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                        str::from_utf8(v)
                            .map(|s|$for::new(Cow::Borrowed(s)))
                            .map_err(|e|de::Error::invalid_value(
                                Unexpected::Other(&e.to_string()), &"valid utf-8")
                            )
                    }

                    fn visit_byte_buf<E: de::Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                        String::from_utf8(v)
                            .map(|s|$for::new(Cow::Owned(s)))
                            .map_err(|e|de::Error::invalid_value(
                                Unexpected::Other(&e.to_string()), &"valid utf-8")
                            )
                    }
                }

                deserializer.deserialize_str(Visitor)
            }
        }
    );
}

deserialize_cow_impl!(UniCase, UniCase::ascii);
deserialize_cow_impl!(Ascii, Ascii::new);

#[cfg(test)]
mod tests {
    extern crate serde_test;

    use super::{UniCase, Ascii, Cow};
    use self::serde_test::{assert_de_tokens, assert_tokens, Token};

    macro_rules! tests_impl {
        ($for:ident, $str_test:ident, $string_test:ident, $cow_test:ident) => (
            #[test]
            fn $str_test() {
                let foo = $for::new("foo");
                assert_tokens(&foo, &[Token::BorrowedStr("foo")]);
                assert_de_tokens(&foo, &[Token::BorrowedBytes(b"foo")]);
            }

            #[test]
            fn $string_test() {
                let foo = $for::new("foo".to_string());
                assert_tokens(&foo, &[Token::Str("foo")]);
                assert_tokens(&foo, &[Token::BorrowedStr("foo")]);
                assert_tokens(&foo, &[Token::String("foo")]);
                assert_de_tokens(&foo, &[Token::Bytes(b"foo")]);
                assert_de_tokens(&foo, &[Token::BorrowedBytes(b"foo")]);
                assert_de_tokens(&foo, &[Token::ByteBuf(b"foo")]);

                assert_de_tokens(&$for::new("true".to_string()), &[Token::Bool(true)]);
                assert_de_tokens(&$for::new("false".to_string()), &[Token::Bool(false)]);

                assert_de_tokens(&$for::new("a".to_string()), &[Token::Char('a')]);
            }

            #[test]
            fn $cow_test() {
                let foo = $for::new(Cow::Borrowed("foo"));
                assert_tokens(&foo, &[Token::Str("foo")]);
                assert_tokens(&foo, &[Token::BorrowedStr("foo")]);
                assert_tokens(&foo, &[Token::String("foo")]);
                assert_de_tokens(&foo, &[Token::Bytes(b"foo")]);
                assert_de_tokens(&foo, &[Token::BorrowedBytes(b"foo")]);
                assert_de_tokens(&foo, &[Token::ByteBuf(b"foo")]);

                assert_de_tokens(&$for::new(Cow::Borrowed("true")), &[Token::Bool(true)]);
                assert_de_tokens(&$for::new(Cow::Borrowed("false")), &[Token::Bool(false)]);

                assert_de_tokens(&$for::new(Cow::Borrowed("a")), &[Token::Char('a')]);
            }
        );
    }

    tests_impl!(UniCase, unicase_str, unicase_string, unicase_cow);
    tests_impl!(Ascii, ascii_str, ascii_string, ascii_cow);
}
