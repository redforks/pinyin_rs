mod pinyin;
pub use pinyin::{ToneRepresentation, Pinyin};


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
