use std::borrow::Cow;

use super::Normalizer;
use crate::Token;

pub struct ZeroesRemover;

impl Normalizer for ZeroesRemover {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Token<'a> {
        if token.word.chars().any(|c| c == '\0') {
            token.word = Cow::Owned(token.word.chars().filter(|c| *c != '\0').collect());
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_zeroes() {
        let s = "\0loloo\0";
        let token = Token {
            word: Cow::Borrowed(s),
            ..Token::default()
        };
        let token = ZeroesRemover.normalize(token);
        assert!(!token.word.chars().any(|c| c == '\0'));
    }
}
