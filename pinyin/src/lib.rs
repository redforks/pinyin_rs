mod db;
mod pinyin;
pub use crate::pinyin::Pinyin;
use std::fmt::Write;

lazy_static::lazy_static! {
    static ref DB: db::DB = {
        db::DB::load(include_str!("pinyin.txt")).unwrap()
    };
}

/// How to represent the tone of a pinyin syllable.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
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
pub fn pinyin(s: &str, tone_repr: ToneRepresentation) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if let Some(pinyin) = DB.get(c) {
            let repr = match tone_repr {
                ToneRepresentation::None => pinyin::PinyinDisplay::NoTones(pinyin.into()),
                ToneRepresentation::Numbered => pinyin::PinyinDisplay::NumberedTone(pinyin.into()),
                ToneRepresentation::Unicode => pinyin::PinyinDisplay::UnicodeTone(pinyin.into()),
            };
            write!(&mut result, "{}", repr).unwrap();
        } else {
            result.push(c);
        }
        result.push(' ');
    }
    result
}

/// Replace Chinese characters with their first letter. Ignore non-printable characters.
/// Non Chinese characters are kept as is.
pub fn first_letters(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        if let Some(pinyin) = DB.get(c) {
            write!(
                &mut result,
                "{}",
                pinyin::PinyinDisplay::FirstLetter(pinyin.into())
            )
            .unwrap();
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinyin() {
        assert_eq!(pinyin("你好", ToneRepresentation::None), "ni hao ");
        assert_eq!(pinyin("你好", ToneRepresentation::Numbered), "ni3 hao3 ");
        assert_eq!(pinyin("你好", ToneRepresentation::Unicode), "nǐ hǎo ");
    }

    #[test]
    fn test_first_letters() {
        assert_eq!(first_letters("你l好"), "nlh");
    }
}
