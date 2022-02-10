use std::borrow::Cow;

use super::Normalizer;
use crate::Token;

/// Remove the control characters but keeps whitespaces.
pub struct ControlCharacterRemover;

impl Normalizer for ControlCharacterRemover {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Token<'a> {
        if token.word.chars().any(is_non_control) {
            let word = token.word.chars().filter(|c| !is_non_control(*c)).collect();
            token.word = Cow::Owned(word);
        }
        token
    }
}

fn is_non_control(c: char) -> bool {
    c.is_control() && !c.is_whitespace()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_non_control() {
        let s = "\0lol\u{2}oo\0";
        let token = Token { word: Cow::Borrowed(s), ..Token::default() };
        let token = ControlCharacterRemover.normalize(token);
        assert!(!token.word.chars().any(is_non_control));
    }
}
