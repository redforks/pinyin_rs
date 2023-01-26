use std::str::FromStr;
use core::str::Utf8Error;

/// Warp `param()` function not decode url encoded string.
/// Use this type to decode raw url encoded string to a `String`.
/// See: https://github.com/seanmonstar/warp/issues/242
#[derive(Debug)]
pub struct UrlEncodedString(String);

impl FromStr for UrlEncodedString {
    type Err = Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = percent_encoding::percent_decode(s.as_bytes())
            .decode_utf8()?;
        Ok(Self(s.to_string()))
    }
}

impl From<UrlEncodedString> for String {
    fn from(value: UrlEncodedString) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encoded_string() {
        let s = "hello%20world";
        let s: UrlEncodedString = s.parse().unwrap();
        assert_eq!(&String::from(s), "hello world");
    }
}
