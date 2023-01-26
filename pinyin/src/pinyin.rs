use modular_bitfield::prelude::*;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

/// How to represent the tone of a pinyin syllable.
#[derive(Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
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

#[derive(Clone, Copy, Eq, PartialEq, BitfieldSpecifier, strum_macros::Display)]
#[bits = 5]
#[strum(serialize_all = "snake_case")]
pub enum Initials {
    #[strum(serialize = "")]
    None,
    B,
    P,
    M,
    F,
    D,
    T,
    N,
    L,
    G,
    K,
    H,
    J,
    Q,
    X,
    ZH,
    CH,
    SH,
    R,
    Z,
    C,
    S,
    Y,
    W,
}

#[derive(Clone, Copy, PartialEq, Eq, BitfieldSpecifier, strum_macros::Display)]
#[bits = 8]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum Finals {
    A,
    O,
    E,
    U,
    I,
    AI,
    EI,
    OU,
    AN,
    EN,
    Ang,
    Eng,
    ER,
    IA,
    IE,
    Iao,
    Ian,
    Ing,
    Iang,
    IN,
    Iou,
    Iong,
    IU,
    UA,
    UO,
    Uai,
    Uan,
    Uang,
    Ueng,
    #[strum(serialize = "ü")]
    V,
    #[strum(serialize = "üe")]
    VE,
    #[strum(serialize = "üan")]
    Van,
}

#[derive(Copy, Clone, PartialEq, Eq, BitfieldSpecifier)]
#[bits = 3]
pub enum Tones {
    None,
    One,
    Two,
    Three,
    Four,
}

#[derive(Copy, Clone)]
#[bitfield]
pub struct Pinyin {
    tones: Tones,
    initials: Initials,
    finals: Finals,
}

/// create a new pinyin syllable
pub fn py(initials: Initials, finals: Finals, tones: Tones) -> Pinyin {
    let mut r = Pinyin::new();
    r.set_initials(initials);
    r.set_finals(finals);
    r.set_tones(tones);
    r
}

impl Display for Pinyin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        PinyinDisplay::UnicodeTone(*self).fmt(f)
    }
}

pub enum PinyinDisplay {
    UnicodeTone(Pinyin),
    NumberedTone(Pinyin),
    NoTones(Pinyin),
    FirstLetter(Pinyin),
}

fn finals_with_tones_to_string(finals: Finals, tones: Tones) -> &'static str {
    match (finals, tones) {
        (_, Tones::None) => unreachable!(),
        (Finals::A, Tones::One) => "ā",
        (Finals::A, Tones::Two) => "á",
        (Finals::A, Tones::Three) => "ǎ",
        (Finals::A, Tones::Four) => "à",
        (Finals::O, Tones::One) => "ō",
        (Finals::O, Tones::Two) => "ó",
        (Finals::O, Tones::Three) => "ǒ",
        (Finals::O, Tones::Four) => "ò",
        (Finals::E, Tones::One) => "ē",
        (Finals::E, Tones::Two) => "é",
        (Finals::E, Tones::Three) => "ě",
        (Finals::E, Tones::Four) => "è",
        (Finals::U, Tones::One) => "ū",
        (Finals::U, Tones::Two) => "ú",
        (Finals::U, Tones::Three) => "ǔ",
        (Finals::U, Tones::Four) => "ù",
        (Finals::I, Tones::One) => "ī",
        (Finals::I, Tones::Two) => "í",
        (Finals::I, Tones::Three) => "ǐ",
        (Finals::I, Tones::Four) => "ì",
        (Finals::AI, Tones::One) => "āi",
        (Finals::AI, Tones::Two) => "ái",
        (Finals::AI, Tones::Three) => "ǎi",
        (Finals::AI, Tones::Four) => "ài",
        (Finals::EI, Tones::One) => "ēi",
        (Finals::EI, Tones::Two) => "éi",
        (Finals::EI, Tones::Three) => "ěi",
        (Finals::EI, Tones::Four) => "èi",
        (Finals::OU, Tones::One) => "ōu",
        (Finals::OU, Tones::Two) => "óu",
        (Finals::OU, Tones::Three) => "ǒu",
        (Finals::OU, Tones::Four) => "òu",
        (Finals::AN, Tones::One) => "ān",
        (Finals::AN, Tones::Two) => "án",
        (Finals::AN, Tones::Three) => "ǎn",
        (Finals::AN, Tones::Four) => "àn",
        (Finals::EN, Tones::One) => "ēn",
        (Finals::EN, Tones::Two) => "én",
        (Finals::EN, Tones::Three) => "ěn",
        (Finals::EN, Tones::Four) => "èn",
        (Finals::Ang, Tones::One) => "āng",
        (Finals::Ang, Tones::Two) => "áng",
        (Finals::Ang, Tones::Three) => "ǎng",
        (Finals::Ang, Tones::Four) => "àng",
        (Finals::Eng, Tones::One) => "ēng",
        (Finals::Eng, Tones::Two) => "éng",
        (Finals::Eng, Tones::Three) => "ěng",
        (Finals::Eng, Tones::Four) => "èng",
        (Finals::ER, Tones::One) => "ēr",
        (Finals::ER, Tones::Two) => "ér",
        (Finals::ER, Tones::Three) => "ěr",
        (Finals::ER, Tones::Four) => "èr",
        (Finals::IA, Tones::One) => "iā",
        (Finals::IA, Tones::Two) => "iá",
        (Finals::IA, Tones::Three) => "iǎ",
        (Finals::IA, Tones::Four) => "ià",
        (Finals::IE, Tones::One) => "iē",
        (Finals::IE, Tones::Two) => "ié",
        (Finals::IE, Tones::Three) => "iě",
        (Finals::IE, Tones::Four) => "iè",
        (Finals::Iao, Tones::One) => "iāo",
        (Finals::Iao, Tones::Two) => "iáo",
        (Finals::Iao, Tones::Three) => "iǎo",
        (Finals::Iao, Tones::Four) => "iào",
        (Finals::Iou, Tones::One) => "iōu",
        (Finals::Iou, Tones::Two) => "ióu",
        (Finals::Iou, Tones::Three) => "iǒu",
        (Finals::Iou, Tones::Four) => "iòu",
        (Finals::Ian, Tones::One) => "iān",
        (Finals::Ian, Tones::Two) => "ián",
        (Finals::Ian, Tones::Three) => "iǎn",
        (Finals::Ian, Tones::Four) => "iàn",
        (Finals::IN, Tones::One) => "īn",
        (Finals::IN, Tones::Two) => "ín",
        (Finals::IN, Tones::Three) => "ǐn",
        (Finals::IN, Tones::Four) => "ìn",
        (Finals::Iang, Tones::One) => "iāng",
        (Finals::Iang, Tones::Two) => "iáng",
        (Finals::Iang, Tones::Three) => "iǎng",
        (Finals::Iang, Tones::Four) => "iàng",
        (Finals::Ing, Tones::One) => "īng",
        (Finals::Ing, Tones::Two) => "íng",
        (Finals::Ing, Tones::Three) => "ǐng",
        (Finals::Ing, Tones::Four) => "ìng",
        (Finals::Iong, Tones::One) => "iōng",
        (Finals::Iong, Tones::Two) => "ióng",
        (Finals::Iong, Tones::Three) => "iǒng",
        (Finals::Iong, Tones::Four) => "iòng",
        (Finals::IU, Tones::One) => "iū",
        (Finals::IU, Tones::Two) => "iú",
        (Finals::IU, Tones::Three) => "iǔ",
        (Finals::IU, Tones::Four) => "iù",
        (Finals::Uan, Tones::One) => "uān",
        (Finals::Uan, Tones::Two) => "uán",
        (Finals::Uan, Tones::Three) => "uǎn",
        (Finals::Uan, Tones::Four) => "uàn",
        (Finals::Uang, Tones::One) => "uāng",
        (Finals::Uang, Tones::Two) => "uáng",
        (Finals::Uang, Tones::Three) => "uǎng",
        (Finals::Uang, Tones::Four) => "uàng",
        (Finals::Uai, Tones::One) => "uāi",
        (Finals::Uai, Tones::Two) => "uái",
        (Finals::Uai, Tones::Three) => "uǎi",
        (Finals::Uai, Tones::Four) => "uài",
        (Finals::UA, Tones::One) => "uā",
        (Finals::UA, Tones::Two) => "uá",
        (Finals::UA, Tones::Three) => "uǎ",
        (Finals::UA, Tones::Four) => "uà",
        (Finals::UO, Tones::One) => "uō",
        (Finals::UO, Tones::Two) => "uó",
        (Finals::UO, Tones::Three) => "uǒ",
        (Finals::UO, Tones::Four) => "uò",
        (Finals::Ueng, Tones::One) => "uēng",
        (Finals::Ueng, Tones::Two) => "uéng",
        (Finals::Ueng, Tones::Three) => "uěng",
        (Finals::Ueng, Tones::Four) => "uèng",
        (Finals::V, Tones::One) => "ǖ",
        (Finals::V, Tones::Two) => "ǘ",
        (Finals::V, Tones::Three) => "ǚ",
        (Finals::V, Tones::Four) => "ǜ",
        (Finals::VE, Tones::One) => "ǖe",
        (Finals::VE, Tones::Two) => "ǘe",
        (Finals::VE, Tones::Three) => "ǚe",
        (Finals::VE, Tones::Four) => "ǜe",
        (Finals::Van, Tones::One) => "ǖan",
        (Finals::Van, Tones::Two) => "ǘan",
        (Finals::Van, Tones::Three) => "ǚan",
        (Finals::Van, Tones::Four) => "ǜan",
    }
}

impl Display for PinyinDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (initials, finals, tones) = match self {
            PinyinDisplay::UnicodeTone(p) => (p.initials(), p.finals(), p.tones()),
            PinyinDisplay::NumberedTone(p) => (p.initials(), p.finals(), p.tones()),
            PinyinDisplay::NoTones(p) => (p.initials(), p.finals(), p.tones()),
            PinyinDisplay::FirstLetter(p) => (p.initials(), p.finals(), p.tones()),
        };

        f.write_fmt(format_args!("{}", initials))?;
        if tones == Tones::None {
            f.write_fmt(format_args!("{}", finals))
        } else {
            f.write_str(finals_with_tones_to_string(finals, tones))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", Initials::None)]
    #[case("b", Initials::B)]
    #[case("zh", Initials::ZH)]
    fn initials_display(#[case] exp: &str, #[case] val: Initials) {
        assert_eq!(exp, val.to_string());
    }

    #[rstest]
    #[case("a", Finals::A)]
    #[case("eng", Finals::Eng)]
    #[case("ü", Finals::V)]
    #[case("üan", Finals::Van)]
    fn finals_display(#[case] exp: &str, #[case] val: Finals) {
        assert_eq!(exp, val.to_string());
    }

    #[rstest]
    #[case("a", py(Initials::None, Finals::A, Tones::None))]
    #[case("beng", py(Initials::B, Finals::Eng, Tones::None))]
    #[case("zhǖ", py(Initials::ZH, Finals::V, Tones::One))]
    #[case("ēr", py(Initials::None, Finals::ER, Tones::One))]
    fn pinyin_unicode_format(#[case] exp: &str, #[case] val: Pinyin) {
        assert_eq!(exp, val.to_string());
    }
}
