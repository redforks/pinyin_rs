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