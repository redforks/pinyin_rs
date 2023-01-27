use modular_bitfield::prelude::{bitfield, BitfieldSpecifier};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    BitfieldSpecifier,
    strum_macros::Display,
    strum_macros::AsRefStr,
    strum_macros::EnumIter,
)]
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

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    BitfieldSpecifier,
    strum_macros::Display,
    strum_macros::EnumIter,
    strum_macros::AsRefStr,
    strum_macros::IntoStaticStr,
)]
#[bits = 8]
#[repr(u8)]
#[strum(serialize_all = "snake_case")]
pub enum Finals {
    None,
    A,
    O,
    E,
    U,
    I,
    AI,
    AO,
    EI,
    OU,
    Ong,
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
    UI,
    UA,
    UN,
    Uan,
    Uang,
    UO,
    UE,
    #[strum(serialize = "ü")]
    V,
    #[strum(serialize = "üe")]
    VE,
    #[strum(serialize = "üan")]
    Van,
}

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    BitfieldSpecifier,
    strum_macros::Display,
    strum_macros::EnumIter,
)]
#[bits = 3]
pub enum Tones {
    #[strum(serialize = "")]
    None,
    #[strum(serialize = "1")]
    One,
    #[strum(serialize = "2")]
    Two,
    #[strum(serialize = "3")]
    Three,
    #[strum(serialize = "4")]
    Four,
}

#[bitfield]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Pinyin {
    tones: Tones,
    initials: Initials,
    finals: Finals,
}

impl From<Pinyin> for u16 {
    fn from(p: Pinyin) -> Self {
        let bytes = p.into_bytes();
        u16::from_be_bytes(bytes)
    }
}

impl From<u16> for Pinyin {
    fn from(value: u16) -> Self {
        let bytes = value.to_be_bytes();
        Pinyin::from_bytes(bytes)
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FinalWithTones(pub(crate) Finals, pub(crate) Tones);

#[derive(Debug)]
pub(crate) struct FinalWithTonesFromStrError(String);

impl Display for FinalWithTonesFromStrError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid final with tones: '{}'", self.0)
    }
}

impl Error for FinalWithTonesFromStrError {}

impl FromStr for FinalWithTones {
    type Err = FinalWithTonesFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "a" => FinalWithTones(Finals::A, Tones::None),
            "o" => FinalWithTones(Finals::O, Tones::None),
            "e" => FinalWithTones(Finals::E, Tones::None),
            "u" => FinalWithTones(Finals::U, Tones::None),
            "i" => FinalWithTones(Finals::I, Tones::None),
            "ai" => FinalWithTones(Finals::AI, Tones::None),
            "ei" => FinalWithTones(Finals::EI, Tones::None),
            "ou" => FinalWithTones(Finals::OU, Tones::None),
            "an" => FinalWithTones(Finals::AN, Tones::None),
            "en" => FinalWithTones(Finals::EN, Tones::None),
            "ang" => FinalWithTones(Finals::Ang, Tones::None),
            "eng" => FinalWithTones(Finals::Eng, Tones::None),
            "er" => FinalWithTones(Finals::ER, Tones::None),
            "ia" => FinalWithTones(Finals::IA, Tones::None),
            "iā" => FinalWithTones(Finals::IA, Tones::One),
            "iá" => FinalWithTones(Finals::IA, Tones::Two),
            "iǎ" => FinalWithTones(Finals::IA, Tones::Three),
            "ià" => FinalWithTones(Finals::IA, Tones::Four),
            "ie" => FinalWithTones(Finals::IE, Tones::None),
            "iē" => FinalWithTones(Finals::IE, Tones::One),
            "ié" => FinalWithTones(Finals::IE, Tones::Two),
            "iě" => FinalWithTones(Finals::IE, Tones::Three),
            "iè" => FinalWithTones(Finals::IE, Tones::Four),
            "iao" => FinalWithTones(Finals::Iao, Tones::None),
            "iāo" => FinalWithTones(Finals::Iao, Tones::One),
            "iáo" => FinalWithTones(Finals::Iao, Tones::Two),
            "iǎo" => FinalWithTones(Finals::Iao, Tones::Three),
            "iào" => FinalWithTones(Finals::Iao, Tones::Four),
            "ao" => FinalWithTones(Finals::AO, Tones::None),
            "āo" => FinalWithTones(Finals::AO, Tones::One),
            "áo" => FinalWithTones(Finals::AO, Tones::Two),
            "ǎo" => FinalWithTones(Finals::AO, Tones::Three),
            "ào" => FinalWithTones(Finals::AO, Tones::Four),
            "ian" => FinalWithTones(Finals::Ian, Tones::None),
            "iān" => FinalWithTones(Finals::Ian, Tones::One),
            "ián" => FinalWithTones(Finals::Ian, Tones::Two),
            "iǎn" => FinalWithTones(Finals::Ian, Tones::Three),
            "iàn" => FinalWithTones(Finals::Ian, Tones::Four),
            "ing" => FinalWithTones(Finals::Ing, Tones::None),
            "īng" => FinalWithTones(Finals::Ing, Tones::One),
            "íng" => FinalWithTones(Finals::Ing, Tones::Two),
            "ǐng" => FinalWithTones(Finals::Ing, Tones::Three),
            "ìng" => FinalWithTones(Finals::Ing, Tones::Four),
            "iang" => FinalWithTones(Finals::Iang, Tones::None),
            "iāng" => FinalWithTones(Finals::Iang, Tones::One),
            "iáng" => FinalWithTones(Finals::Iang, Tones::Two),
            "iǎng" => FinalWithTones(Finals::Iang, Tones::Three),
            "iàng" => FinalWithTones(Finals::Iang, Tones::Four),
            "in" => FinalWithTones(Finals::IN, Tones::None),
            "īn" => FinalWithTones(Finals::IN, Tones::One),
            "ín" => FinalWithTones(Finals::IN, Tones::Two),
            "ǐn" => FinalWithTones(Finals::IN, Tones::Three),
            "ìn" => FinalWithTones(Finals::IN, Tones::Four),
            "ui" => FinalWithTones(Finals::UI, Tones::None),
            "uī" => FinalWithTones(Finals::UI, Tones::One),
            "uí" => FinalWithTones(Finals::UI, Tones::Two),
            "uǐ" => FinalWithTones(Finals::UI, Tones::Three),
            "uì" => FinalWithTones(Finals::UI, Tones::Four),
            "iou" => FinalWithTones(Finals::Iou, Tones::None),
            "iōu" => FinalWithTones(Finals::Iou, Tones::One),
            "ióu" => FinalWithTones(Finals::Iou, Tones::Two),
            "iǒu" => FinalWithTones(Finals::Iou, Tones::Three),
            "iòu" => FinalWithTones(Finals::Iou, Tones::Four),
            "iong" => FinalWithTones(Finals::Iong, Tones::None),
            "iōng" => FinalWithTones(Finals::Iong, Tones::One),
            "ióng" => FinalWithTones(Finals::Iong, Tones::Two),
            "iǒng" => FinalWithTones(Finals::Iong, Tones::Three),
            "iòng" => FinalWithTones(Finals::Iong, Tones::Four),
            "iu" => FinalWithTones(Finals::IU, Tones::None),
            "iū" => FinalWithTones(Finals::IU, Tones::One),
            "iú" => FinalWithTones(Finals::IU, Tones::Two),
            "iǔ" => FinalWithTones(Finals::IU, Tones::Three),
            "iù" => FinalWithTones(Finals::IU, Tones::Four),
            "ua" => FinalWithTones(Finals::UA, Tones::None),
            "uā" => FinalWithTones(Finals::UA, Tones::One),
            "uá" => FinalWithTones(Finals::UA, Tones::Two),
            "uǎ" => FinalWithTones(Finals::UA, Tones::Three),
            "uà" => FinalWithTones(Finals::UA, Tones::Four),
            "uan" => FinalWithTones(Finals::Uan, Tones::None),
            "uān" => FinalWithTones(Finals::Uan, Tones::One),
            "uán" => FinalWithTones(Finals::Uan, Tones::Two),
            "uǎn" => FinalWithTones(Finals::Uan, Tones::Three),
            "uàn" => FinalWithTones(Finals::Uan, Tones::Four),
            "un" => FinalWithTones(Finals::UN, Tones::None),
            "ūn" => FinalWithTones(Finals::UN, Tones::One),
            "ún" => FinalWithTones(Finals::UN, Tones::Two),
            "ǔn" => FinalWithTones(Finals::UN, Tones::Three),
            "ùn" => FinalWithTones(Finals::UN, Tones::Four),
            "uang" => FinalWithTones(Finals::Uang, Tones::None),
            "uāng" => FinalWithTones(Finals::Uang, Tones::One),
            "uáng" => FinalWithTones(Finals::Uang, Tones::Two),
            "uǎng" => FinalWithTones(Finals::Uang, Tones::Three),
            "uàng" => FinalWithTones(Finals::Uang, Tones::Four),
            "uo" => FinalWithTones(Finals::UO, Tones::None),
            "uō" => FinalWithTones(Finals::UO, Tones::One),
            "uó" => FinalWithTones(Finals::UO, Tones::Two),
            "uǒ" => FinalWithTones(Finals::UO, Tones::Three),
            "uò" => FinalWithTones(Finals::UO, Tones::Four),
            "ü" => FinalWithTones(Finals::V, Tones::None),
            "ǖ" => FinalWithTones(Finals::V, Tones::One),
            "ǘ" => FinalWithTones(Finals::V, Tones::Two),
            "ǚ" => FinalWithTones(Finals::V, Tones::Three),
            "ǜ" => FinalWithTones(Finals::V, Tones::Four),
            "üe" => FinalWithTones(Finals::VE, Tones::None),
            "üē" => FinalWithTones(Finals::VE, Tones::One),
            "üé" => FinalWithTones(Finals::VE, Tones::Two),
            "üě" => FinalWithTones(Finals::VE, Tones::Three),
            "üè" => FinalWithTones(Finals::VE, Tones::Four),
            "ue" => FinalWithTones(Finals::UE, Tones::None),
            "uē" => FinalWithTones(Finals::UE, Tones::One),
            "ué" => FinalWithTones(Finals::UE, Tones::Two),
            "uě" => FinalWithTones(Finals::UE, Tones::Three),
            "uè" => FinalWithTones(Finals::UE, Tones::Four),
            "üan" => FinalWithTones(Finals::Van, Tones::None),
            "ǖan" => FinalWithTones(Finals::Van, Tones::One),
            "ǘan" => FinalWithTones(Finals::Van, Tones::Two),
            "ǚan" => FinalWithTones(Finals::Van, Tones::Three),
            "ǜan" => FinalWithTones(Finals::Van, Tones::Four),
            "ā" => FinalWithTones(Finals::A, Tones::One),
            "á" => FinalWithTones(Finals::A, Tones::Two),
            "ǎ" => FinalWithTones(Finals::A, Tones::Three),
            "à" => FinalWithTones(Finals::A, Tones::Four),
            "ō" => FinalWithTones(Finals::O, Tones::One),
            "ó" => FinalWithTones(Finals::O, Tones::Two),
            "ǒ" => FinalWithTones(Finals::O, Tones::Three),
            "ò" => FinalWithTones(Finals::O, Tones::Four),
            "ē" => FinalWithTones(Finals::E, Tones::One),
            "é" => FinalWithTones(Finals::E, Tones::Two),
            "ě" => FinalWithTones(Finals::E, Tones::Three),
            "è" => FinalWithTones(Finals::E, Tones::Four),
            "ū" => FinalWithTones(Finals::U, Tones::One),
            "ú" => FinalWithTones(Finals::U, Tones::Two),
            "ǔ" => FinalWithTones(Finals::U, Tones::Three),
            "ù" => FinalWithTones(Finals::U, Tones::Four),
            "ī" => FinalWithTones(Finals::I, Tones::One),
            "í" => FinalWithTones(Finals::I, Tones::Two),
            "ǐ" => FinalWithTones(Finals::I, Tones::Three),
            "ì" => FinalWithTones(Finals::I, Tones::Four),
            "āi" => FinalWithTones(Finals::AI, Tones::One),
            "ái" => FinalWithTones(Finals::AI, Tones::Two),
            "ǎi" => FinalWithTones(Finals::AI, Tones::Three),
            "ài" => FinalWithTones(Finals::AI, Tones::Four),
            "ēi" => FinalWithTones(Finals::EI, Tones::One),
            "éi" => FinalWithTones(Finals::EI, Tones::Two),
            "ěi" => FinalWithTones(Finals::EI, Tones::Three),
            "èi" => FinalWithTones(Finals::EI, Tones::Four),
            "ōu" => FinalWithTones(Finals::OU, Tones::One),
            "óu" => FinalWithTones(Finals::OU, Tones::Two),
            "ǒu" => FinalWithTones(Finals::OU, Tones::Three),
            "òu" => FinalWithTones(Finals::OU, Tones::Four),
            "ong" => FinalWithTones(Finals::Ong, Tones::None),
            "ōng" => FinalWithTones(Finals::Ong, Tones::One),
            "óng" => FinalWithTones(Finals::Ong, Tones::Two),
            "ǒng" => FinalWithTones(Finals::Ong, Tones::Three),
            "òng" => FinalWithTones(Finals::Ong, Tones::Four),
            "ān" => FinalWithTones(Finals::AN, Tones::One),
            "án" => FinalWithTones(Finals::AN, Tones::Two),
            "ǎn" => FinalWithTones(Finals::AN, Tones::Three),
            "àn" => FinalWithTones(Finals::AN, Tones::Four),
            "ēn" => FinalWithTones(Finals::EN, Tones::One),
            "én" => FinalWithTones(Finals::EN, Tones::Two),
            "ěn" => FinalWithTones(Finals::EN, Tones::Three),
            "èn" => FinalWithTones(Finals::EN, Tones::Four),
            "āng" => FinalWithTones(Finals::Ang, Tones::One),
            "áng" => FinalWithTones(Finals::Ang, Tones::Two),
            "ǎng" => FinalWithTones(Finals::Ang, Tones::Three),
            "àng" => FinalWithTones(Finals::Ang, Tones::Four),
            "ēng" => FinalWithTones(Finals::Eng, Tones::One),
            "éng" => FinalWithTones(Finals::Eng, Tones::Two),
            "ěng" => FinalWithTones(Finals::Eng, Tones::Three),
            "èng" => FinalWithTones(Finals::Eng, Tones::Four),
            "ēr" => FinalWithTones(Finals::ER, Tones::One),
            "ér" => FinalWithTones(Finals::ER, Tones::Two),
            "ěr" => FinalWithTones(Finals::ER, Tones::Three),
            "èr" => FinalWithTones(Finals::ER, Tones::Four),
            _ => FinalWithTones(Finals::None, Tones::None),
        };
        if let FinalWithTones(Finals::None, Tones::None) = r {
            Err(FinalWithTonesFromStrError(s.to_string()))
        } else {
            Ok(r)
        }
    }
}

impl From<FinalWithTones> for &'static str {
    fn from(value: FinalWithTones) -> Self {
        match (value.0, value.1) {
            (_, Tones::None) => value.0.into(),
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
            (Finals::AO, Tones::One) => "āo",
            (Finals::AO, Tones::Two) => "áo",
            (Finals::AO, Tones::Three) => "ǎo",
            (Finals::AO, Tones::Four) => "ào",
            (Finals::EI, Tones::One) => "ēi",
            (Finals::EI, Tones::Two) => "éi",
            (Finals::EI, Tones::Three) => "ěi",
            (Finals::EI, Tones::Four) => "èi",
            (Finals::OU, Tones::One) => "ōu",
            (Finals::OU, Tones::Two) => "óu",
            (Finals::OU, Tones::Three) => "ǒu",
            (Finals::OU, Tones::Four) => "òu",
            (Finals::Ong, Tones::One) => "ōng",
            (Finals::Ong, Tones::Two) => "óng",
            (Finals::Ong, Tones::Three) => "ǒng",
            (Finals::Ong, Tones::Four) => "òng",
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
            (Finals::UI, Tones::One) => "uī",
            (Finals::UI, Tones::Two) => "uí",
            (Finals::UI, Tones::Three) => "uǐ",
            (Finals::UI, Tones::Four) => "uì",
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
            (Finals::UA, Tones::One) => "uā",
            (Finals::UA, Tones::Two) => "uá",
            (Finals::UA, Tones::Three) => "uǎ",
            (Finals::UA, Tones::Four) => "uà",
            (Finals::UN, Tones::One) => "ūn",
            (Finals::UN, Tones::Two) => "ún",
            (Finals::UN, Tones::Three) => "ǔn",
            (Finals::UN, Tones::Four) => "ùn",
            (Finals::Uan, Tones::One) => "uān",
            (Finals::Uan, Tones::Two) => "uán",
            (Finals::Uan, Tones::Three) => "uǎn",
            (Finals::Uan, Tones::Four) => "uàn",
            (Finals::Uang, Tones::One) => "uāng",
            (Finals::Uang, Tones::Two) => "uáng",
            (Finals::Uang, Tones::Three) => "uǎng",
            (Finals::Uang, Tones::Four) => "uàng",
            (Finals::UO, Tones::One) => "uō",
            (Finals::UO, Tones::Two) => "uó",
            (Finals::UO, Tones::Three) => "uǒ",
            (Finals::UO, Tones::Four) => "uò",
            (Finals::V, Tones::One) => "ǖ",
            (Finals::V, Tones::Two) => "ǘ",
            (Finals::V, Tones::Three) => "ǚ",
            (Finals::V, Tones::Four) => "ǜ",
            (Finals::VE, Tones::One) => "üē",
            (Finals::VE, Tones::Two) => "üé",
            (Finals::VE, Tones::Three) => "üě",
            (Finals::VE, Tones::Four) => "üè",
            (Finals::UE, Tones::One) => "uē",
            (Finals::UE, Tones::Two) => "ué",
            (Finals::UE, Tones::Three) => "uě",
            (Finals::UE, Tones::Four) => "uè",
            (Finals::Van, Tones::One) => "ǖan",
            (Finals::Van, Tones::Two) => "ǘan",
            (Finals::Van, Tones::Three) => "ǚan",
            (Finals::Van, Tones::Four) => "ǜan",
            (Finals::None, Tones::One) => unreachable!(),
            (Finals::None, Tones::Two) => unreachable!(),
            (Finals::None, Tones::Three) => unreachable!(),
            (Finals::None, Tones::Four) => unreachable!(),
        }
    }
}

impl Display for PinyinDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PinyinDisplay::UnicodeTone(p) => {
                f.write_fmt(format_args!("{}", p.initials()))?;
                if p.tones() == Tones::None {
                    f.write_fmt(format_args!("{}", p.finals()))
                } else {
                    f.write_str(FinalWithTones(p.finals(), p.tones()).into())
                }
            }
            PinyinDisplay::NumberedTone(p) => {
                f.write_fmt(format_args!("{}", p.initials()))?;
                f.write_fmt(format_args!("{}", p.finals()))?;
                f.write_fmt(format_args!("{}", p.tones()))
            }
            PinyinDisplay::NoTones(p) => {
                f.write_fmt(format_args!("{}", p.initials()))?;
                f.write_fmt(format_args!("{}", p.finals()))
            }
            PinyinDisplay::FirstLetter(p) => match p.initials() {
                Initials::None => f.write_str(&p.finals().as_ref()[0..1]),
                _ => f.write_str(&p.initials().as_ref()[0..1]),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use strum::IntoEnumIterator;

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

    #[rstest]
    #[case("a", py(Initials::None, Finals::A, Tones::None))]
    #[case("beng", py(Initials::B, Finals::Eng, Tones::None))]
    #[case("zhü1", py(Initials::ZH, Finals::V, Tones::One))]
    #[case("a1", py(Initials::None, Finals::A, Tones::One))]
    #[case("a2", py(Initials::None, Finals::A, Tones::Two))]
    #[case("a3", py(Initials::None, Finals::A, Tones::Three))]
    #[case("a4", py(Initials::None, Finals::A, Tones::Four))]
    fn pinyin_numbered_format(#[case] exp: &str, #[case] val: Pinyin) {
        assert_eq!(exp, PinyinDisplay::NumberedTone(val).to_string());
    }

    #[rstest]
    #[case("a", py(Initials::None, Finals::A, Tones::None))]
    #[case("beng", py(Initials::B, Finals::Eng, Tones::None))]
    #[case("zhü", py(Initials::ZH, Finals::V, Tones::One))]
    #[case("a", py(Initials::None, Finals::A, Tones::One))]
    fn no_tones_format(#[case] exp: &str, #[case] val: Pinyin) {
        assert_eq!(exp, PinyinDisplay::NoTones(val).to_string());
    }

    #[rstest]
    #[case("a", py(Initials::None, Finals::A, Tones::Two))]
    #[case("b", py(Initials::B, Finals::Eng, Tones::None))]
    #[case("z", py(Initials::ZH, Finals::V, Tones::One))]
    #[case("a", py(Initials::None, Finals::A, Tones::One))]
    #[case("e", py(Initials::None, Finals::ER, Tones::One))]
    #[case("s", py(Initials::SH, Finals::I, Tones::One))]
    fn first_letter_format(#[case] exp: &str, #[case] val: Pinyin) {
        assert_eq!(exp, PinyinDisplay::FirstLetter(val).to_string());
    }

    #[test]
    fn final_and_tones_to_from_str() {
        itertools::iproduct!(Finals::iter(), Tones::iter()).for_each(|(f, t)| {
            if f != Finals::None {
                let s: &str = FinalWithTones(f, t).into();
                let fwt = s.parse::<FinalWithTones>().unwrap();
                assert_eq!(fwt, FinalWithTones(f, t));
            }
        });
    }

    #[test]
    fn pinyin_into_u16() {
        itertools::iproduct!(Initials::iter(), Finals::iter(), Tones::iter()).for_each(
            |(i, f, t)| {
                let p = py(i, f, t);
                let n: u16 = p.into();
                assert_eq!(p, n.into());
            },
        );
    }
}
