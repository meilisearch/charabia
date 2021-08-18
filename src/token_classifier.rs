use fst::Set;

use crate::detection::classify_separator;
use crate::token::SeparatorKind;
use crate::{Token, TokenKind};

#[derive(Clone)]
pub struct TokenClassifier<'a, A = Vec<u8>> {
    stop_words: Option<&'a Set<A>>,
}

impl Default for TokenClassifier<'_> {
    fn default() -> Self {
        Self { stop_words: None }
    }
}

impl<'a, A> TokenClassifier<'a, A> {
    pub fn new(stop_words: Option<&'a Set<A>>) -> Self {
        Self { stop_words }
    }
}

impl<'a, A> TokenClassifier<'a, A>
where
    A: AsRef<[u8]>,
{
    pub fn classify<'t>(&self, mut token: Token<'t>) -> Token<'t> {
        let word = token.word.as_ref();
        let mut is_hard_separator = false;
        if self.stop_words.map(|stop_words| stop_words.contains(word)).unwrap_or(false) {
            token.kind = TokenKind::StopWord;
            token
        } else if word.chars().all(|c| match classify_separator(c) {
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

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use super::*;
    use crate::Token;

    #[test]
    fn separators() {
        let classifier = TokenClassifier::default();

        let token = classifier.classify(Token { word: Cow::Borrowed("   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = classifier.classify(Token { word: Cow::Borrowed("\" "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token =
            classifier.classify(Token { word: Cow::Borrowed("@   "), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Soft));

        let token = classifier.classify(Token { word: Cow::Borrowed("."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token =
            classifier.classify(Token { word: Cow::Borrowed("   ."), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token =
            classifier.classify(Token { word: Cow::Borrowed("  。"), ..Default::default() });
        assert_eq!(token.is_separator(), Some(SeparatorKind::Hard));

        let token =
            classifier.classify(Token { word: Cow::Borrowed("S.O.S"), ..Default::default() });
        assert!(token.is_word());

        let token = classifier.classify(Token { word: Cow::Borrowed("ь"), ..Default::default() });
        assert!(token.is_word());

        // non-breaking space
        let token =
            classifier.classify(Token { word: Cow::Borrowed("\u{00a0}"), ..Default::default() });
        assert!(token.is_word());
    }

    #[test]
    fn stop_words() {
        let stop_words = Set::from_iter(["the"].iter()).unwrap();
        let classifier = TokenClassifier::new(Some(&stop_words));

        let token = classifier.classify(Token { word: Cow::Borrowed("the"), ..Default::default() });
        assert!(token.is_stopword());

        let token = classifier.classify(Token { word: Cow::Borrowed("The"), ..Default::default() });
        assert!(token.is_word());

        let token =
            classifier.classify(Token { word: Cow::Borrowed("foobar"), ..Default::default() });
        assert!(token.is_word());
    }
}
