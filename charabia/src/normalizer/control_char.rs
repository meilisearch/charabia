use super::CharNormalizer;
use crate::normalizer::CharOrStr;
use crate::Token;

/// A global [`Normalizer`] removing control characters.
///
pub struct ControlCharNormalizer;

impl CharNormalizer for ControlCharNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        (!is_control(c)).then(|| c.into())
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.lemma().chars().any(is_control)
    }
}

fn is_control(c: char) -> bool {
    c.is_control() && !c.is_whitespace()
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;
    use crate::Script;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("\0生而自由\u{2}oo\0".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Cj,
                ..Default::default()
            },
            Token {
                lemma: Owned("\0生而自由\u{2}oo\0".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Cj,
                char_map: Some(vec![
                    (1, 1),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                ]),
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Cj,
                char_map: Some(vec![
                    (1, 0),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (1, 0),
                    (1, 1),
                    (1, 1),
                    (1, 0),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Cj,
                char_map: Some(vec![
                    (1, 0),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (1, 0),
                    (1, 1),
                    (1, 1),
                    (1, 0),
                ]),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Cj,
                char_map: Some(vec![
                    (1, 0),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (1, 0),
                    (1, 1),
                    (1, 1),
                    (1, 0),
                ]),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Cj,
                char_map: Some(vec![
                    (1, 0),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (1, 0),
                    (1, 1),
                    (1, 1),
                    (1, 0),
                ]),
                kind: TokenKind::Word,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(ControlCharNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
