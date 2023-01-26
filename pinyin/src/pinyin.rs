use serde::Deserialize;
use modular_bitfield::prelude::*;

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

#[derive(BitfieldSpecifier, strum_macros::Display)]
#[bits = 5]
#[strum(serialize_all = "snake_case")]
enum Initials {
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

#[derive(BitfieldSpecifier)]
#[bits = 8]
#[repr(u8)]
enum Finals {
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
    IU,
    UA,
    UO,
    Uai,
    Uan,
    Uang,
    Ueng,
    Ung,
    V,
    VE,
    Van,
    Vang,
    Veng,
    NG,
}

#[derive(BitfieldSpecifier)]
#[bits = 3]
enum Tones {
    None,
    One,
    Two,
    Three,
    Four,
}

#[bitfield]
pub struct Pinyin {
    tones: Tones,
    initials: Initials,
    finals: Finals,
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
}
