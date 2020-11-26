use std::collections::HashSet;

use deunicode::deunicode_char;

use crate::{Token, TokenKind};
use crate::token::SeparatorKind;

#[derive(Debug,  Clone)]
pub struct TokenClassifier<'a> {
    stop_words: &'a HashSet<String>,
}

impl<'a> TokenClassifier<'a> {
    pub fn new(stop_words: &'a HashSet<String>) -> Self {
        Self { stop_words }
    }

    pub fn classify<'t>(&self, mut token: Token<'t>) -> Token<'t> {
        let word = token.word.as_ref();
        let mut is_hard_separator = false;
        if self.stop_words.contains(word) {
            token.kind = TokenKind::StopWord;
            token
        } else if word.chars().all(|c|
            match classify_separator(c) {
                Some(SeparatorKind::Hard) => {
                    is_hard_separator = true;
                    true
                }
                Some(SeparatorKind::Soft) => true,

                None => false,
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

fn classify_separator(c: char) -> Option<SeparatorKind> {
    match c {
        c if c.is_whitespace() => Some(SeparatorKind::Soft), // whitespaces
        c if deunicode_char(c) == Some("'") => Some(SeparatorKind::Soft), // quotes
        c if deunicode_char(c) == Some("\"") => Some(SeparatorKind::Soft), // double quotes
        '-' | '_' | '\'' | ':' | '/' | '\\' | '@' => Some(SeparatorKind::Soft),
        '.' | ';' | ',' | '!' | '?' | '(' | ')' => Some(SeparatorKind::Hard),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::*;
    use crate::Token;

    #[test]
    fn separators() {
        let stop_words = HashSet::new();
        let classifier = TokenClassifier::new(&stop_words);

        let token = classifier.classify(Token { word: Cow::Borrowed("   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = classifier.classify(Token { word: Cow::Borrowed("@   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = classifier.classify(Token { word: Cow::Borrowed("."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token = classifier.classify(Token { word: Cow::Borrowed("   ."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token = classifier.classify(Token { word: Cow::Borrowed("S.O.S"), ..Default::default() });
        assert!(token.is_word());
    }

    #[test]
    fn stop_words() {
        let stop_words = ["the"].iter().map(|s| s.to_string()).collect();
        let classifier = TokenClassifier::new(&stop_words);

        let token = classifier.classify(Token { word: Cow::Borrowed("the"), ..Default::default() });
        assert!(token.is_stopword());

        let token = classifier.classify(Token { word: Cow::Borrowed("The"), ..Default::default() });
        assert!(token.is_word());

        let token = classifier.classify(Token { word: Cow::Borrowed("foobar"), ..Default::default() });
        assert!(token.is_word());
    }
}
