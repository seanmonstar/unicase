//! serde Serialization and Deserialization
//!
//! Support for serialization and deserialization using
//! [serde](https://serde.rs/) is enabled with the `serde` feature flag, which
//! is enabled by default. You can usually derive or implement `Serialize`
//! and `Deserialize` directly without any issues.
//!
//! ## Serialization
//! Serialization for any `UniCase<S>` and `Ascii<S>` where  `S: AsRef<str>` is
//! implemented. Examples for `S` include `&str`, `Cow<'a, str>`, and `String`.
//!
//! ## Deserialization
//!
//! You can deserialize strings into `UniCase<S>` and `Ascii<S>` where
//!
//! - `S: FromStr + AsRef<str>`
//! - `S: From<&'de str> + AsRef<str> + 'de`
//!
//! ### `S: FromStr + AsRef<str>`
//! The first case is implemented directly as a trait, and you do not have to do anything special
//! other than to derive or implement `Deserialize` to use it. Conversion is done using the
//! `FromStr::from_str` function.
//!
//! Typically, you will use the direct implementation
//! for any Rust built in type that owns the data and does not borrow anything.
//! Example include `String`.
//!
//! You will know when you need to use the second case
//! when you get trait bound errors such
//! "as the trait bound `&'a str: std::convert::From<&str>` is not satisfied".
//!
//! ### `S: From<&'de str> + AsRef<str> + 'de`
//!
//! The second case is meant for usage with Rust built in types that borrow data. Conversion is done using the
//! `Into::into` function.
//!
//! Due to the lack of specialisation at the time of writing, usage of the second case is more
//! cumbersome. You will have to use the `deserialize_with` field attribute from serde
//! (along with the `borrow` attribute). See serde
//! [documentation](https://serde.rs/field-attrs.html) for more details.
//!
//! If you use the second case with any type that owns data, you will get an error at runtime.
//!
//! ## Example Serialization and Deserialization
//! ```rust
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate unicase;
//!
//! use std::borrow::Cow;
//! use unicase::{UniCase, Ascii};
//! use unicase::serde::{deserialize_borrowed_unicase, deserialize_borrowed_ascii};
//!
//! #[derive(Serialize, Deserialize)]
//! struct TestUniCase<'a> {
//!     owned: UniCase<String>,
//!     #[serde(borrow, deserialize_with="deserialize_borrowed_unicase")]
//!     borrowed_str: UniCase<&'a str>,
//!     #[serde(borrow, deserialize_with="deserialize_borrowed_unicase")]
//!     cow_str: UniCase<Cow<'a, str>>,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct TestAscii<'a> {
//!     owned: Ascii<String>,
//!     #[serde(borrow, deserialize_with="deserialize_borrowed_ascii")]
//!     borrowed_str: Ascii<&'a str>,
//!     #[serde(borrow, deserialize_with="deserialize_borrowed_ascii")]
//!     cow_str: Ascii<Cow<'a, str>>,
//! }
//!
//! # fn main() {
//! # }
//! ```
//!
//! ## Example with Custom "string" types
//! This example will demonstrate how you can use a "custom" string type and
//! still use serialization and deserialization when wrapped inside a `UniCase` or `Ascii`. This
//! is particularly useful for types like `UniCase<Cow<'a, String>>` or `UniCase<&'a String>`
//! because `&String` does not implement `From::<&str>`.
//!
//! As you can see from the example below, you can use the direct `Deserialize` implementation
//! to deserialize borrowed data. This usually does not work with Rust built in types due to
//! missing trait implementation. However, because the conversion is done using the
//! `FromStr::from_str` function, and the function signature indicates that the `&str` passed in
//! might have an ephemeral lifetime, implementors will have to convert the `&str` into an owned
//! version. So you are better off using the borrowed deserializer.
//!
//! ```rust
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate unicase;
//!
//! use std::borrow::Cow;
//! use std::str::FromStr;
//! use unicase::UniCase;
//! use unicase::serde::deserialize_borrowed_unicase;
//!
//! #[derive(Eq, PartialEq, Debug)]
//! struct CustomStr<'a>(Cow<'a, str>);
//!
//! impl<'a> AsRef<str> for CustomStr<'a> {
//!     fn as_ref(&self) -> &str {
//!         self.0.as_ref()
//!     }
//! }
//!
//! impl<'a> FromStr for CustomStr<'a> {
//!     type Err = ();
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         Ok(CustomStr(Cow::from(s.to_string())))
//!     }
//! }
//!
//! impl<'a> From<&'a str> for CustomStr<'a> {
//!     fn from(s: &'a str) -> Self {
//!         CustomStr(Cow::from(s))
//!     }
//! }
//!
//! #[derive(Eq, PartialEq, Debug)]
//! struct CustomString<'a>(Cow<'a, String>);
//!
//! impl<'a> AsRef<str> for CustomString<'a> {
//!     fn as_ref(&self) -> &str {
//!         self.0.as_ref()
//!     }
//! }
//!
//! impl<'a> FromStr for CustomString<'a> {
//!     type Err = ();
//!     fn from_str(s: &str) -> Result<Self, Self::Err> {
//!         Ok(CustomString(Cow::Owned(s.to_string())))
//!     }
//! }
//!
//! impl<'a> From<&'a str> for CustomString<'a> {
//!     fn from(s: &'a str) -> Self {
//!         CustomString(Cow::Owned(s.to_string()))
//!     }
//! }
//!
//! #[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
//! struct TestCustomStruct<'a> {
//!     #[serde(borrow)]
//!     test_str: UniCase<CustomStr<'a>>,
//!     #[serde(borrow)]
//!     test_string: UniCase<CustomString<'a>>,
//!     #[serde(borrow, deserialize_with="deserialize_borrowed_unicase")]
//!     test_str_borrowed: UniCase<CustomStr<'a>>,
//!     #[serde(borrow, deserialize_with="deserialize_borrowed_unicase")]
//!     test_string_borrowed: UniCase<CustomString<'a>>,
//! }
//!
//! # fn main() {
//! # }
//! ```
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;

use serdelib::{Serialize, Serializer, Deserialize, Deserializer};
use serdelib::de;

/// Straightforward Serialization for UniCase
impl<S: AsRef<str>> Serialize for ::UniCase<S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

/// Straightforward Serialization for Ascii
impl<S: AsRef<str>> Serialize for ::Ascii<S> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error>
    where
        Ser: Serializer,
    {
        serializer.serialize_str(self.as_ref())
    }
}

/// Deserialization for `UniCase<S>` where `S: FromStr + AsRef<str>`
impl<'de, S: FromStr + AsRef<str>> Deserialize<'de> for ::UniCase<S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct UniCaseVisitor<S>(PhantomData<S>);

        impl<'de, S: FromStr + AsRef<str>> de::Visitor<'de> for UniCaseVisitor<S> {
            type Value = ::UniCase<S>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FromStr::from_str(s)
                    .map_err(|_| E::custom("FromStr conversion failed"))
            }
        }

        deserializer.deserialize_str(UniCaseVisitor(PhantomData))
    }
}

/// Deserialization for `Ascii<S>` where `S: FromStr + AsRef<str>`
impl<'de, S: FromStr + AsRef<str>> Deserialize<'de> for ::Ascii<S> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        struct AsciiVisitor<S>(PhantomData<S>);

        impl<'de, S: FromStr + AsRef<str>> de::Visitor<'de> for AsciiVisitor<S> {
            type Value = ::Ascii<S>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FromStr::from_str(s)
                    .map_err(|_| E::custom("FromStr conversion failed"))
            }
        }

        deserializer.deserialize_str(AsciiVisitor(PhantomData))
    }
}

/// Borrowed Deserializer for `UniCase`.
///
/// Typically, you will use this for any types that borrow data. Example include `&str`. If you
/// use this with any type that owns data, you will get an error at runtime.
///
/// Due to the lack of specialisation at the time of writing, usage of the second case is more
/// cumbersome. You will have to use the `deserialize_with` field attribute from serde
/// (along with the `borrow` attribute). See serde
/// [documentation](https://serde.rs/field-attrs.html) for more details.
pub fn deserialize_borrowed_unicase<'de, S, D>(deserializer: D) -> Result<::UniCase<S>, D::Error>
where
    S: From<&'de str> + AsRef<str> + 'de,
    D: Deserializer<'de>,
{
    struct UniCaseVisitor<S>(PhantomData<S>);

    impl<'de, S: From<&'de str> + AsRef<str>> de::Visitor<'de> for UniCaseVisitor<S> {
        type Value = ::UniCase<S>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_borrowed_str<E>(self, s: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(::UniCase::unicode(s.into()))
        }
    }

    deserializer.deserialize_str(UniCaseVisitor(PhantomData))
}

/// Borrowed Deserializer for `Ascii`.
///
/// Typically, you will use this for any types that borrow data. Example include `&str`. If you
/// use this with any type that owns data, you will get an error at runtime.
///
/// Due to the lack of specialisation at the time of writing, usage of the second case is more
/// cumbersome. You will have to use the `deserialize_with` field attribute from serde
/// (along with the `borrow` attribute). See serde
/// [documentation](https://serde.rs/field-attrs.html) for more details.
pub fn deserialize_borrowed_ascii<'de, S, D>(deserializer: D) -> Result<::Ascii<S>, D::Error>
where
    S: From<&'de str> + AsRef<str> + 'de,
    D: Deserializer<'de>,
{
    struct AsciiVisitor<S>(PhantomData<S>);

    impl<'de, S: From<&'de str> + AsRef<str>> de::Visitor<'de> for AsciiVisitor<S> {
        type Value = ::Ascii<S>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_borrowed_str<E>(self, s: &'de str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(::Ascii::new(s.into()))
        }
    }

    deserializer.deserialize_str(AsciiVisitor(PhantomData))
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use serde_test::{Token, assert_tokens};
    use ::{UniCase, Ascii};
    use super::{deserialize_borrowed_unicase, deserialize_borrowed_ascii};

    #[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
    struct TestUniCase<'a> {
        owned: UniCase<String>,
        #[serde(borrow, deserialize_with="deserialize_borrowed_unicase")]
        borrowed_str: UniCase<&'a str>,
        #[serde(borrow, deserialize_with="deserialize_borrowed_unicase")]
        cow_str: UniCase<Cow<'a, str>>,
    }

    #[derive(Eq, PartialEq, Debug, Serialize, Deserialize)]
    struct TestAscii<'a> {
        owned: Ascii<String>,
        #[serde(borrow, deserialize_with="deserialize_borrowed_ascii")]
        borrowed_str: Ascii<&'a str>,
        #[serde(borrow, deserialize_with="deserialize_borrowed_ascii")]
        cow_str: Ascii<Cow<'a, str>>,
    }

    #[test]
    fn test_unicase_serde() {
        let test = TestUniCase {
            owned: UniCase::unicode("owned string".to_string()),
            borrowed_str: UniCase::unicode("borrowed str"),
            cow_str: UniCase::unicode(Cow::from("Cow str")),
        };

        assert_tokens(&test, &[
            Token::Struct { name: "TestUniCase", len: 3 },
            Token::Str("owned"),
            Token::String("owned string"),
            Token::Str("borrowed_str"),
            Token::BorrowedStr("borrowed str"),
            Token::Str("cow_str"),
            Token::BorrowedStr("Cow str"),
            Token::StructEnd,
        ]);
    }

    #[test]
    fn test_ascii_serde() {
        let test = TestAscii {
            owned: Ascii::new("owned string".to_string()),
            borrowed_str: Ascii::new("borrowed str"),
            cow_str: Ascii::new(Cow::from("Cow str")),
        };

        assert_tokens(&test, &[
            Token::Struct { name: "TestAscii", len: 3 },
            Token::Str("owned"),
            Token::String("owned string"),
            Token::Str("borrowed_str"),
            Token::BorrowedStr("borrowed str"),
            Token::Str("cow_str"),
            Token::BorrowedStr("Cow str"),
            Token::StructEnd,
        ]);
    }
}
