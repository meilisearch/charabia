use std::collections::HashSet;

use crate::{Token, TokenKind};
use crate::token::SeparatorKind;
use super::Normalizer;

#[derive(Debug, Default, Clone)]
pub struct TokenClassifier {
    hard_separators: HashSet<char>,
    soft_separators: HashSet<char>,
    stop_words: HashSet<String>,
}

impl TokenClassifier {
    pub fn new(stop_words: HashSet<String>, soft_separators: HashSet<char>, hard_separators: HashSet<char>) -> Self {
        Self { soft_separators, stop_words, hard_separators }
    }
}

impl Normalizer for TokenClassifier {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Token<'a> {
        let word = token.word.as_ref();
        let mut is_hard_separator = false;
        if self.stop_words.contains(word) {
            token.kind = TokenKind::StopWord;
            token
        } else if word.chars().all(|c| {
            is_hard_separator = self.hard_separators.contains(&c);
            self.soft_separators.contains(&c) || is_hard_separator
        }) {
            if is_hard_separator {
                token.kind = TokenKind::Separator(SeparatorKind::Hard);
            } else {
                token.kind = TokenKind::Separator(SeparatorKind::Soft);
            }
            token
        } else {
            token.kind = TokenKind::Word;
            token
        }
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::*;
    use crate::Token;

    #[test]
    fn separators() {
        let normalizer = TokenClassifier {
            stop_words: ["the"].iter().map(|s| s.to_string()).collect(),
            soft_separators: [' ', '\t', ','].iter().cloned().collect(),
            hard_separators: ['.'].iter().cloned().collect(),
        };

        let token = normalizer.normalize(Token { word: Cow::Borrowed("   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = normalizer.normalize(Token { word: Cow::Borrowed(",   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = normalizer.normalize(Token { word: Cow::Borrowed("."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token = normalizer.normalize(Token { word: Cow::Borrowed("   ."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token = normalizer.normalize(Token { word: Cow::Borrowed("S.O.S"), ..Default::default() });
        assert!(token.is_word());
    }

    #[test]
    fn stop_words() {
        let normalizer = TokenClassifier {
            stop_words: ["the"].iter().map(|s| s.to_string()).collect(),
            ..Default::default()
        };

        let token = normalizer.normalize(Token { word: Cow::Borrowed("the"), ..Default::default() });
        assert!(token.is_stopword());

        let token = normalizer.normalize(Token { word: Cow::Borrowed("The"), ..Default::default() });
        assert!(token.is_word());

        let token = normalizer.normalize(Token { word: Cow::Borrowed("foobar"), ..Default::default() });
        assert!(token.is_word());
    }
}
