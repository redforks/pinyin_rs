use crate::Pinyin;
use std::collections::HashMap;

mod parser;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
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

/// Pinyin database for each chinese character.
/// Indexed by unicode code point.
pub struct DB {
    pages: HashMap<u16, [Polyphone; 256]>,
}

impl DB {
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    pub fn get(&self, c: char) -> Option<Polyphone> {
        let code_point = c as u32;
        let page = (code_point >> 8) as u16;
        let offset = code_point as u8;
        self.pages.get(&page).and_then(|page| {
            let r = page[offset as usize];
            if r == Polyphone::default() {
                None
            } else {
                Some(r)
            }
        })
    }

    pub fn insert(&mut self, c: char, polyphone: Polyphone) {
        let code_point = c as u32;
        debug_assert!(code_point <= 0xffffff);
        let page = (code_point >> 8) as u16;
        let offset = code_point as u8;
        let page = self
            .pages
            .entry(page)
            .or_insert_with(|| [Polyphone::default(); 256]);
        debug_assert_eq!(page[offset as usize], Polyphone::default());
        page[offset as usize] = polyphone;
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

    #[test]
    fn db_put_get() {
        let mut db = DB::new();
        let polyphone = Polyphone::new(1, 0, 0);
        db.insert('a', polyphone);
        assert_eq!(db.get('a'), Some(polyphone));

        db.insert('汉', polyphone);
        assert_eq!(db.get('汉'), Some(polyphone));
        assert_eq!(db.get('b'), None);
    }
}
