use std::str::FromStr;
use core::str::Utf8Error;
use serde::Deserialize;
use utoipa::ToSchema;

/// How to represent the tone of a pinyin syllable.
#[derive(Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum ToneRepresentation {
    None,
    Numbered,
    Unicode,
}

impl Default for ToneRepresentation {
    fn default() -> Self {
        Self::Unicode
    }
}


/// Return pinyin of a Chinese characters separated by space.
pub fn pinyin(_s: &str, _tone_repr: ToneRepresentation) -> String {
    todo!()
    // let mut result = String::new();
    // for c in s.chars() {
    //     if let Some(pinyin) = pinyin::pinyin(c) {
    //         let pinyin = match tone {
    //             ToneRepresentation::None => pinyin.without_tone(),
    //             ToneRepresentation::Numbered => pinyin.with_tone(),
    //             ToneRepresentation::Unicode => pinyin.with_tone_mark(),
    //         };
    //         result.push_str(&pinyin);
    //     } else {
    //         result.push(c);
    //     }
    //     result.push(' ');
    // }
    // result
}

/// Replace Chinese characters with their first letter. Ignore non-printable characters.
/// Non Chinese characters are kept as is. If a character has multiple pinyin,
/// all combinations are returned separated by space.
pub fn first_letters(_s: &str) -> String {
    todo!()
    // let mut result = String::new();
    // for c in s.chars() {
    //     if let Some(pinyin) = pinyin::pinyin(c) {
    //         result.push_str(&pinyin.first_letter());
    //     } else {
    //         result.push(c);
    //     }
    //     result.push(' ');
    // }
    // result
}

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
