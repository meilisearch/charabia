use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

/// This module contains the implementation of the `CharacterConverterNormalizer` struct, which is a character normalizer

pub struct CharacterConverterNormalizer;

// All normalizers only need to implement the method `normalize_char` and the method `should_normalize` of the `CharNormalizer` trait.
impl CharNormalizer for CharacterConverterNormalizer {
    // Creates the normalized version of the provided char.
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        let mut normalized = c.to_lowercase();

        // if the original character is converted in exactly one character,
        // then we return the character directly instead of creating a string for it.
        match (normalized.next(), normalized.next()) {
            (Some(c), None) => normalize_char(c),
            (Some(first), Some(second)) => {
                let first = normalize_char(first);
                let second = normalize_char(second);
                match (first, second) {
                    (Some(first), Some(second)) => Some(first.merge(&second)),
                    (Some(first), None) => Some(first),
                    (None, Some(second)) => Some(second),
                    (None, None) => None,
                }
            }
            (None, _) => None,
        }
    }

    // Returns `true` if the Normalizer should be used.
    fn should_normalize(&self, token: &Token) -> bool {
        true
    }
}

fn normalize_char(c: char) -> Option<CharOrStr> {
    match c {
        'œ' | 'Œ' => Some("oe".to_string().into()),
        'æ' | 'Æ' => Some("ae".to_string().into()),
        'ـ' => None,
        'ٱ' => Some('ا'.into()),
        'ى' => Some('ي'.into()),
        'ة' => Some('ه'.into()),
        '’' | '‘' | '‛' => Some('\''.into()),
        #[cfg(feature = "turkish")]
        'ı' => Some('i'.into()),
        #[cfg(feature = "vietnamese")]
        'Ð' | 'Đ' | 'đ' | 'ð' => Some("d".to_string().into()),
        _ => Some(c.into()),
    }
}

fn is_control(c: char) -> bool {
    c.is_control() && !c.is_whitespace()
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

    test_normalizer!(
        CharacterConverterNormalizer,
        tokens(),
        normalizer_result(),
        normalized_tokens()
    );
}
