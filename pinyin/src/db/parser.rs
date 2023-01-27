use crate::pinyin::{py, FinalWithTones, Initials};
use crate::Pinyin;
use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::streaming::is_not,
    character::complete::{char, hex_digit1, newline, space0},
    combinator::{map_res, opt, value},
    multi::{many0, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult, InputTakeAtPosition,
};
use std::num::ParseIntError;
use std::str::FromStr;

fn comment(i: &str) -> IResult<&str, Option<(char, PinyinList)>> {
    value(None, tuple((space0, char('#'), opt(is_not("\n")), newline)))(i)
}

fn code_point(i: &str) -> IResult<&str, char> {
    // parse a char in the form of U+XXXX
    preceded(
        tag("U+"),
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PinyinList(pub (Pinyin, Option<Pinyin>, Option<Pinyin>));

impl From<Vec<Pinyin>> for PinyinList {
    fn from(value: Vec<Pinyin>) -> Self {
        let mut iter = value.into_iter();
        let first = iter.next().unwrap();
        let second = iter.next();
        let third = iter.next();
        PinyinList((first, second, third))
    }
}

fn empty_line(i: &str) -> IResult<&str, Option<(char, PinyinList)>> {
    value(None, pair(space0, newline))(i)
}

fn parse_line(i: &str) -> IResult<&str, Option<(char, PinyinList)>> {
    let pn_list = separated_list1(char(','), pinyin);
    let char_and_pinyin = separated_pair(code_point, tag(": "), pn_list);
    let mut line = terminated(char_and_pinyin, alt((comment, empty_line)));
    let (remains, (ch, pinyin_list)) = line(i)?;
    Ok((remains, Some((ch, pinyin_list.into()))))
}

pub fn parse_lines(i: &str) -> IResult<&str, Vec<(char, PinyinList)>> {
    let (remains, lines) = many0(alt((empty_line, comment, parse_line)))(i)?;
    let lines = lines.into_iter().flatten().collect();
    Ok((remains, lines))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinyin::{Finals, Tones};
    use rstest::rstest;

    #[test]
    fn parse_comment() {
        assert_eq!(comment("#\n"), Ok(("", None)));
        assert_eq!(comment("# Hello world\n"), Ok(("", None)));
        assert_eq!(comment("  # Hello world\n"), Ok(("", None)));
        assert_eq!(comment("# Hello world\r\n"), Ok(("", None)));
    }

    #[test]
    fn parse_code_point() {
        assert_eq!(code_point("U+4E2D"), Ok(("", '中')));
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

    #[test]
    fn vec_to_pinyin_list() {
        let py1 = py(Initials::None, Finals::A, Tones::None);
        let v = vec![py1];
        let p = PinyinList::from(v);
        assert_eq!(p.0, (py1, None, None));
    }

    #[test]
    fn test_parse_line() {
        // assert_eq!(parse_line("# Hello world"), Ok(("", None)));
        assert_eq!(
            parse_line("U+4E2D: zhōng\n"),
            Ok((
                "",
                Some((
                    '中',
                    PinyinList::from(vec![py(Initials::ZH, Finals::Ong, Tones::One)])
                ))
            ))
        );
        assert_eq!(
            parse_line("U+4E2D: zhōng,zhòng\n"),
            Ok((
                "",
                Some((
                    '中',
                    PinyinList::from(vec![
                        py(Initials::ZH, Finals::Ong, Tones::One),
                        py(Initials::ZH, Finals::Ong, Tones::Four)
                    ])
                ))
            ))
        );
        assert_eq!(
            parse_line("U+4E2D: zhōng,zhòng # comment\n"),
            Ok((
                "",
                Some((
                    '中',
                    PinyinList::from(vec![
                        py(Initials::ZH, Finals::Ong, Tones::One),
                        py(Initials::ZH, Finals::Ong, Tones::Four),
                    ])
                ))
            ))
        );
    }

    #[test]
    fn parse_empty_line() {
        assert_eq!(empty_line("\n"), Ok(("", None)));
        assert_eq!(empty_line(" \t\n"), Ok(("", None)));
        assert!(matches!(empty_line("foo"), Err(_)));
    }

    #[test]
    fn test_parse_lines() {
        // parse empty
        assert_eq!(parse_lines(""), Ok(("", vec![])));
        assert_eq!(parse_lines("\n"), Ok(("", vec![])));

        assert_eq!(
            parse_lines(
                r#" # comment
+U4E2D: zhōng

+U3007: líng,yuán,xīng  # 〇
"#
            ),
            Ok((
                "",
                vec![
                    (
                        '中',
                        PinyinList((py(Initials::ZH, Finals::Ong, Tones::One), None, None))
                    ),
                    (
                        '〇',
                        PinyinList((
                            py(Initials::L, Finals::Ing, Tones::Two),
                            Some(py(Initials::Y, Finals::Uan, Tones::Two)),
                            Some(py(Initials::X, Finals::Ing, Tones::One))
                        ))
                    ),
                ]
            ))
        );
    }
}
