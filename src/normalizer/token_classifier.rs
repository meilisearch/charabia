use std::collections::HashSet;

use deunicode::deunicode_char;

use crate::{Token, TokenKind};
use crate::token::SeparatorKind;
use super::Normalizer;

#[derive(Debug, Default, Clone)]
pub struct TokenClassifier {
    stop_words: HashSet<String>,
}

impl TokenClassifier {
    pub fn new(stop_words: HashSet<String>) -> Self {
        Self { stop_words }
    }
}

impl Normalizer for TokenClassifier {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Token<'a> {
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
        let normalizer = TokenClassifier {
            stop_words: ["the"].iter().map(|s| s.to_string()).collect(),
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
