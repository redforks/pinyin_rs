use crate::db::parser::PinyinList;
use crate::Pinyin;

mod parser;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Polyphone(u16, u16, u16);

impl Polyphone {
    pub fn new(a: u16, b: u16, c: u16) -> Self {
        Polyphone(a, b, c)
    }

    pub fn iter(&self) -> impl Iterator<Item = Pinyin> {
        let iter = [self.0, self.1, self.2].into_iter();
        iter.filter(|x| *x != 0).map(|x| x.into())
    }
}

/// Convert first pinyin from polyphone
impl From<Polyphone> for Pinyin {
    fn from(value: Polyphone) -> Self {
        value.0.into()
    }
}

impl From<Vec<Pinyin>> for Polyphone {
    fn from(value: Vec<Pinyin>) -> Self {
        let mut iter = value.into_iter();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap_or_default();
        let third = iter.next().unwrap_or_default();
        Self(first.into(), second.into(), third.into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinyin::{py, Finals, Initials, Tones};

    #[test]
    fn to_pinyin() {
        let polyphone = Polyphone::new(0, 0, 0);
        let pinyin = Pinyin::from(polyphone);
        assert_eq!(pinyin, py(Initials::None, Finals::None, Tones::None));
    }

    #[test]
    fn polyphone_iter() {
        let polyphone = Polyphone::new(0, 0, 0);
        assert!(polyphone.iter().next().is_none());
    }

    #[test]
    fn vec_to_polyphone() {
        let py1 = py(Initials::None, Finals::A, Tones::None);
        let v = vec![py1];
        let p = Polyphone::from(v);
        assert_eq!((p.0, p.1, p.2), (py1.into(), 0, 0));
    }
}
