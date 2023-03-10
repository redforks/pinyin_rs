use crate::db::{Polyphone, DB};
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

fn comment(i: &str) -> IResult<&str, Option<(char, Polyphone)>> {
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
            '??' | '??' | '??' | '??' |
            '??' | '??' | '??' | '??' |
            '??' | '??' | '??' | '??' |
            '??' | '??' | '??' | '??' |
            '??' | '??' | '??' | '??' |
            '??' | '??' | '??' | '??' |
            '??' |
            '??' | '??' | '??'
        )
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
            nom::error::ErrorKind::Tag,
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

fn empty_line(i: &str) -> IResult<&str, Option<(char, Polyphone)>> {
    value(None, pair(space0, newline))(i)
}

fn parse_line(i: &str) -> IResult<&str, Option<(char, Polyphone)>> {
    let pn_list = separated_list1(char(','), pinyin);
    let char_and_pinyin = separated_pair(code_point, tag(": "), pn_list);
    let mut line = terminated(char_and_pinyin, alt((comment, empty_line)));
    let (remains, (ch, pinyin_list)) = line(i)?;
    Ok((remains, Some((ch, pinyin_list.into()))))
}

fn parse_lines(
    i: &str,
) -> Result<impl Iterator<Item = (char, Polyphone)>, nom::Err<nom::error::Error<&str>>> {
    let (remains, lines) = many0(alt((empty_line, comment, parse_line)))(i)?;
    if !remains.is_empty() {
        panic!(
            "remains: {}",
            remains.chars().into_iter().take(10).collect::<String>()
        );
    }
    Ok(lines.into_iter().flatten())
}

pub fn parse_db(i: &str) -> Result<DB, nom::error::Error<&str>> {
    match parse_lines(i) {
        Ok(iter) => {
            let mut db = DB::new();
            for (ch, polyphone) in iter {
                db.insert(ch, polyphone);
            }
            db.shrink_to_fit();
            Ok(db)
        }
        Err(nom::Err::Incomplete(_)) => unreachable!(),
        Err(nom::Err::Error(e)) => Err(e),
        Err(nom::Err::Failure(e)) => Err(e),
    }
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
        assert_eq!(code_point("U+4E2D"), Ok(("", '???')));
        assert_eq!(code_point("U+2104C"), Ok(("", '????')));
    }

    #[test]
    fn parse_initials() {
        assert_eq!(initials("b"), Ok(("", Initials::B)));
        assert_eq!(initials("ch"), Ok(("", Initials::CH)));
        assert_eq!(initials("c"), Ok(("", Initials::C)));
        assert_eq!(initials("y"), Ok(("", Initials::Y)));
        assert_eq!(initials("zh"), Ok(("", Initials::ZH)));
    }

    #[test]
    fn test_is_py_chr() {
        assert!(is_pinyin_char('a'));
        assert!(is_pinyin_char('??'));
        assert!(is_pinyin_char('??'));
        assert!(is_pinyin_char('??'));
        assert!(is_pinyin_char('??'));
        assert!(is_pinyin_char('??'));
    }

    #[rstest]
    #[case("a", Finals::A, Tones::None)]
    #[case("??", Finals::A, Tones::Two)]
    #[case("ang", Finals::Ang, Tones::None)]
    #[case("??n", Finals::AN, Tones::Three)]
    #[case("??n", Finals::UN, Tones::Two)]
    fn parse_final_and_tones(#[case] s: &str, #[case] finals: Finals, #[case] tones: Tones) {
        assert_eq!(final_and_tones(s), Ok(("", FinalWithTones(finals, tones))));
    }

    #[rstest]
    #[case("a", Initials::None, Finals::A, Tones::None)]
    #[case("b??", Initials::B, Finals::A, Tones::Two)]
    #[case("h??n", Initials::H, Finals::UN, Tones::Two)]
    fn parse_pinyin(
        #[case] s: &str,
        #[case] initials: Initials,
        #[case] finals: Finals,
        #[case] tones: Tones,
    ) {
        assert_eq!(pinyin(s), Ok(("", py(initials, finals, tones))));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("U+4E2D: zh??ng\n"),
            Ok((
                "",
                Some((
                    '???',
                    Polyphone::from(vec![py(Initials::ZH, Finals::Ong, Tones::One)])
                ))
            ))
        );
        assert_eq!(
            parse_line("U+4E2D: zh??ng,zh??ng\n"),
            Ok((
                "",
                Some((
                    '???',
                    Polyphone::from(vec![
                        py(Initials::ZH, Finals::Ong, Tones::One),
                        py(Initials::ZH, Finals::Ong, Tones::Four)
                    ])
                ))
            ))
        );
        assert_eq!(
            parse_line("U+4E2D: zh??ng,zh??ng # comment\n"),
            Ok((
                "",
                Some((
                    '???',
                    Polyphone::from(vec![
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
        let parse = |s: &str| parse_lines(s).unwrap().collect::<Vec<_>>();

        // parse empty
        assert_eq!(parse(""), vec![]);
        assert_eq!(parse("\n"), vec![]);

        assert_eq!(
            parse(
                r#" # comment
U+4E2D: zh??ng

U+3007: l??ng,yu??n,x??ng  # ???
"#
            ),
            vec![
                (
                    '???',
                    Polyphone(py(Initials::ZH, Finals::Ong, Tones::One).into(), 0, 0)
                ),
                (
                    '???',
                    Polyphone(
                        py(Initials::L, Finals::Ing, Tones::Two).into(),
                        py(Initials::Y, Finals::Uan, Tones::Two).into(),
                        py(Initials::X, Finals::Ing, Tones::One).into()
                    )
                ),
            ]
        );
    }

    #[test]
    fn test_parse_db() {
        let count = parse_lines(include_str!("../pinyin.txt")).unwrap().count();
        assert!(count > 20902, "count = {}", count);
        let db = parse_db(include_str!("../pinyin.txt")).unwrap();
        let po: Pinyin = db.get('????').unwrap().into();
        assert_eq!(po, py(Initials::B, Finals::Iang, Tones::Two));
    }
}
