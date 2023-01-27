use crate::pinyin::{py, FinalWithTones, Initials};
use crate::Pinyin;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, hex_digit1},
    combinator::{map_res, opt, value},
    sequence::{pair, preceded},
    IResult, InputTakeAtPosition,
};
use std::num::ParseIntError;
use std::str::FromStr;

fn comment(i: &str) -> IResult<&str, ()> {
    value(
        (), // Output is thrown away.
        pair(char('#'), is_not("\n\r")),
    )(i)
}

fn code_point(i: &str) -> IResult<&str, char> {
    // parse a char in the form of U+XXXX
    preceded(
        tag("+U"),
        map_res(hex_digit1, |s: &str| -> Result<char, ParseIntError> {
            let code_point = u32::from_str_radix(s, 16)?;
            Ok(char::from_u32(code_point).unwrap())
        }),
    )(i)
}

fn initials(i: &str) -> IResult<&str, Initials> {
    alt((
        value(Initials::B, char('b')),
        value(Initials::CH, tag("ch")),
        value(Initials::C, char('c')),
        value(Initials::D, char('d')),
        value(Initials::F, char('f')),
        value(Initials::G, char('g')),
        value(Initials::H, char('h')),
        value(Initials::J, char('j')),
        value(Initials::K, char('k')),
        value(Initials::L, char('l')),
        value(Initials::M, char('m')),
        value(Initials::N, char('n')),
        value(Initials::P, char('p')),
        value(Initials::Q, char('q')),
        value(Initials::R, char('r')),
        value(Initials::SH, tag("sh")),
        value(Initials::S, char('s')),
        value(Initials::T, char('t')),
        value(Initials::W, char('w')),
        value(Initials::X, char('x')),
        alt((
            value(Initials::Y, char('y')),
            value(Initials::ZH, tag("zh")),
            value(Initials::Z, char('z')),
        )),
    ))(i)
}

#[rustfmt::skip]
fn is_pinyin_char(c: char) -> bool {
    if c.is_ascii_alphabetic() {
        true
    } else {
        matches!(c,
            'ā' | 'á' | 'ǎ' | 'à' |
            'ē' | 'é' | 'ě' | 'è' |
            'ī' | 'í' | 'ǐ' | 'ì' |
            'ō' | 'ó' | 'ǒ' | 'ò' |
            'ū' | 'ú' | 'ǔ' | 'ù' |
            'ǖ' | 'ǘ' | 'ǚ' | 'ǜ' |
            'ü')
    }
}

fn final_and_tones(i: &str) -> IResult<&str, FinalWithTones> {
    let (i, s) =
        i.split_at_position1_complete(|x| !is_pinyin_char(x), nom::error::ErrorKind::Alpha)?;
    let r = FinalWithTones::from_str(s);
    match r {
        Ok(r) => Ok((i, r)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            i,
            nom::error::ErrorKind::AlphaNumeric,
        ))),
    }
}

fn pinyin(i: &str) -> IResult<&str, Pinyin> {
    let (i, (initials, final_with_tones)) = pair(opt(initials), final_and_tones)(i)?;
    Ok((
        i,
        py(
            initials.unwrap_or(Initials::None),
            final_with_tones.0,
            final_with_tones.1,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinyin::{Finals, Tones};
    use rstest::rstest;

    #[test]
    fn parse_comment() {
        assert_eq!(comment("# Hello world"), Ok(("", ())));
        assert_eq!(comment("# Hello world\n"), Ok(("\n", ())));
        assert_eq!(comment("# Hello world\r\n"), Ok(("\r\n", ())));
    }

    #[test]
    fn parse_code_point() {
        assert_eq!(code_point("+U4E2D"), Ok(("", '中')));
    }

    #[test]
    fn parse_initials() {
        assert_eq!(initials("b"), Ok(("", Initials::B)));
        assert_eq!(initials("ch"), Ok(("", Initials::CH)));
        assert_eq!(initials("c"), Ok(("", Initials::C)));
        assert_eq!(initials("y"), Ok(("", Initials::Y)));
        assert_eq!(initials("zh"), Ok(("", Initials::ZH)));
    }

    #[rstest]
    #[case("a", Finals::A, Tones::None)]
    #[case("á", Finals::A, Tones::Two)]
    #[case("ang", Finals::Ang, Tones::None)]
    fn parse_final_and_tones(#[case] s: &str, #[case] finals: Finals, #[case] tones: Tones) {
        assert_eq!(final_and_tones(s), Ok(("", FinalWithTones(finals, tones))));
    }

    #[rstest]
    #[case("a", Initials::None, Finals::A, Tones::None)]
    #[case("bá", Initials::B, Finals::A, Tones::Two)]
    fn parse_pinyin(
        #[case] s: &str,
        #[case] initials: Initials,
        #[case] finals: Finals,
        #[case] tones: Tones,
    ) {
        assert_eq!(pinyin(s), Ok(("", py(initials, finals, tones))));
    }
}
