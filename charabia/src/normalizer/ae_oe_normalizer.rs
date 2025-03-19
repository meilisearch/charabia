use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

/// This module contains the implementation of the `AeOeNormalizer` struct, which is a character normalizer
/// that replaces the characters 'œ', 'æ', 'Œ', and 'Æ' with their respective replacements 'oe', 'ae', 'OE', and 'AE'.
/// It also provides a test suite to validate the normalizer's functionality.
pub struct AeOeNormalizer;

// All normalizers only need to implement the method `normalize_char` and the method `should_normalize` of the `CharNormalizer` trait.
impl CharNormalizer for AeOeNormalizer {
    // Creates the normalized version of the provided char.
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        match c {
            'œ' | 'Œ' => Some("oe".to_string().into()),
            'æ' | 'Æ' => Some("ae".to_string().into()),
            _ => Some(c.into()),
        }
    }

    // Returns `true` if the Normalizer should be used.
    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Latin && token.lemma.chars().any(is_should_normalize)
    }
}
fn is_should_normalize(c: char) -> bool {
    matches!(c, 'œ' | 'æ' | 'Œ' | 'Æ')
}

// Test the normalizer:
#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("œ".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("Œ".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("æ".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("Æ".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("oe".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("oe".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("ae".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("ae".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("oe".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("oe".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("ae".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("ae".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(2, 2)]),
                kind: TokenKind::Word,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(AeOeNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
