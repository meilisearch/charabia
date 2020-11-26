use deunicode::deunicode_char;
use fst::Set;

use crate::{Token, TokenKind};
use crate::token::SeparatorKind;

#[derive(Debug,  Clone)]
pub struct TokenClassifier<'a, A>
where
    A: AsRef<[u8]>
{
    stop_words: &'a Set<A>,
}

impl<'a, A> TokenClassifier<'a, A>
where
    A: AsRef<[u8]>
{
    pub fn new(stop_words: &'a Set<A>) -> Self {
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
    match deunicode_char(c)?.chars().next()? {
        c if c.is_whitespace() => Some(SeparatorKind::Soft), // whitespaces
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
        let stop_words = Set::default();
        let classifier = TokenClassifier::new(&stop_words);

        let token = classifier.classify(Token { word: Cow::Borrowed("   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = classifier.classify(Token { word: Cow::Borrowed("@   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = classifier.classify(Token { word: Cow::Borrowed("."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token = classifier.classify(Token { word: Cow::Borrowed("   ."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token = classifier.classify(Token { word: Cow::Borrowed("  。"), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token = classifier.classify(Token { word: Cow::Borrowed("S.O.S"), ..Default::default() });
        assert!(token.is_word());
    }

    #[test]
    fn stop_words() {
        let stop_words = Set::from_iter(["the"].iter()).unwrap();
        let classifier = TokenClassifier::new(&stop_words);

        let token = classifier.classify(Token { word: Cow::Borrowed("the"), ..Default::default() });
        assert!(token.is_stopword());

        let token = classifier.classify(Token { word: Cow::Borrowed("The"), ..Default::default() });
        assert!(token.is_word());

        let token = classifier.classify(Token { word: Cow::Borrowed("foobar"), ..Default::default() });
        assert!(token.is_word());
    }
}
