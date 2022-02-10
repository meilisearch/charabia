use cow_utils::CowUtils;

use super::{PreProcessor, ProcessedText};

pub struct Eraser(char, String);

impl Eraser {
    /// replaces the occurences of `c`, if any, with the equivalent amount of spaces in bytes.
    /// e.g: "l’espagne" becomes "l   espagne" because '’' is 3 bytes long. This allows the string
    /// to remains the same size as the original, an preserve the offsets of the tokens
    pub fn new(c: char) -> Self {
        Self(c, std::iter::repeat(' ').take(c.len_utf8()).collect())
    }
}

impl PreProcessor for Eraser {
    fn process<'a>(&self, s: &'a str) -> ProcessedText<'a> {
        ProcessedText { original: s, processed: s.cow_replace(|c| c == self.0, &self.1) }
    }
}
