use std::iter::once;

use super::{CharNormalizer, CharOrStr};
use crate::detection::Script;
use crate::Token;

/// A global [`Normalizer`] lowercasing characters.
///
pub struct LowercaseNormalizer;

impl CharNormalizer for LowercaseNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        let mut normalized = c.to_lowercase();

        // if the original character is converted in exactly one character,
        // then we return the character directly instead of creating a string for it.
        match (normalized.next(), normalized.next()) {
            (Some(c), None) => Some(c.into()),
            (Some(first), Some(second)) => {
                let normalized: String =
                    once(first).chain(once(second)).chain(normalized).collect();
                Some(normalized.into())
            }
            (None, _) => None,
        }
    }

    fn should_normalize(&self, token: &Token) -> bool {
        // https://en.wikipedia.org/wiki/Letter_case#Capitalisation
        matches!(
            token.script,
            Script::Latin | Script::Cyrillic | Script::Greek | Script::Georgian | Script::Armenian
        ) && token.lemma.chars().any(char::is_uppercase)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;

    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("PascalCase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ՀայասՏան".to_string()),
                char_end: 8,
                byte_end: 16,
                script: Script::Armenian,
                ..Default::default()
            },
        ]
    }

    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("pascalcase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("հայաստան".to_string()),
                char_end: 8,
                byte_end: 16,
                script: Script::Armenian,
                char_map: Some(vec![
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                ]),
                ..Default::default()
            },
        ]
    }

    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("pascalcase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("հայաստան".to_string()),
                char_end: 8,
                byte_end: 16,
                script: Script::Armenian,
                kind: TokenKind::Word,
                char_map: Some(vec![
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                ]),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(LowercaseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
