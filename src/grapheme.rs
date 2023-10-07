//! # Grapheme
//!
//! `grapheme` manages the characters and their width at the display.
//!
//! Note that to manage the width of character is
//! in order to consider how many the positions of cursor should be moved
//! when e.g. emojis and the special characters are displayed on the terminal.
use std::{
    fmt,
    iter::FromIterator,
    ops::{Deref, DerefMut},
};

use radix_trie::TrieKey;
use unicode_width::UnicodeWidthChar;

/// A character and its width.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Grapheme {
    pub ch: char,
    pub width: usize,
}

impl From<char> for Grapheme {
    fn from(c: char) -> Self {
        Self {
            ch: c,
            width: UnicodeWidthChar::width(c).unwrap_or(0),
        }
    }
}

/// Characters and their width.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Graphemes(pub Vec<Grapheme>);

impl Deref for Graphemes {
    type Target = Vec<Grapheme>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graphemes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<S: AsRef<str>> From<S> for Graphemes {
    fn from(string: S) -> Self {
        string.as_ref().chars().map(Grapheme::from).collect()
    }
}

impl TrieKey for Graphemes {
    fn encode_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}

impl FromIterator<Grapheme> for Graphemes {
    fn from_iter<I: IntoIterator<Item = Grapheme>>(iter: I) -> Self {
        let mut g = Graphemes::default();
        for i in iter {
            g.push(i);
        }
        g
    }
}

impl fmt::Display for Graphemes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.iter()
                .fold(String::new(), |s, g| format!("{}{}", s, g.ch))
        )
    }
}

pub fn matrixify(width: usize, g: Graphemes) -> Vec<Graphemes> {
    let mut ret = vec![];
    let mut row = Graphemes::default();
    for ch in g.iter() {
        let width_with_next_char = row.iter().fold(0, |mut layout, g| {
            layout += g.width;
            layout
        }) + ch.width;
        if !row.is_empty() && (width as usize) < width_with_next_char {
            ret.push(row);
            row = Graphemes::default();
        }
        if (width as usize) >= ch.width {
            row.push(ch.clone());
        }
    }
    ret.push(row);
    ret
}

#[cfg(test)]
mod test {
    mod matrixify {
        use super::super::*;

        #[test]
        fn test() {
            let expect = vec![
                Graphemes::from(">>"),
                Graphemes::from(" a"),
                Graphemes::from("aa"),
                Graphemes::from(" "),
            ];
            assert_eq!(expect, matrixify(2, Graphemes::from(">> aaa ")),);
        }

        #[test]
        fn test_with_emoji() {
            let expect = vec![
                Graphemes::from(">>"),
                Graphemes::from(" "),
                Graphemes::from("😎"),
                Graphemes::from("😎"),
                Graphemes::from(" "),
            ];
            assert_eq!(expect, matrixify(2, Graphemes::from(">> 😎😎 ")),);
        }

        #[test]
        fn test_with_emoji_at_narrow_terminal() {
            let expect = vec![
                Graphemes::from(">"),
                Graphemes::from(">"),
                Graphemes::from(" "),
                Graphemes::from(" "),
            ];
            assert_eq!(expect, matrixify(1, Graphemes::from(">> 😎😎 ")),);
        }
    }
}
