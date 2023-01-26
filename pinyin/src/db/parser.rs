use crate::pinyin::Initials;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::char,
    character::complete::hex_digit1,
    combinator::{map_res, value},
    sequence::{pair, preceded},
    IResult,
};
use std::num::ParseIntError;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
