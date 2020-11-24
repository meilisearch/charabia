use cow_utils::CowUtils;

use super::{ProcessedText, PreProcessor};

pub struct Eraser(char, String);

impl Eraser {
    pub fn new(c: char) -> Self {
        Self(c, std::iter::repeat(' ').take(c.len_utf8()).collect())
    }
}

impl PreProcessor for Eraser {
    fn process<'a>(&self, s: &'a str) -> ProcessedText<'a> {
        ProcessedText {
            original: s,
            processed: s.cow_replace(|c| c == self.0, &self.1)
        }
    }
}
