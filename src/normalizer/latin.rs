use deunicode::deunicode_char;

use crate::detection::Script;
use crate::normalizer::{CharNormalizer, CharOrStr};
use crate::Token;

/// Latin specialized [`Normalizer`] converting unicode chars into Ascii.
///
/// This Normalizer uses [`deunicode`] internally to normalize the provided token.
pub struct LatinNormalizer;

impl CharNormalizer for LatinNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        // if deunicode don't manage to decode the character, we remove it.
        let normalized = deunicode_char(c)?;
        let mut chars = normalized.chars();

        // if the original character is converted in exactly one character,
        // then we return the character directly instead of creating a string for it.
        match (chars.next(), chars.next()) {
            (Some(c), None) => Some(c.into()),
            _otherwise => Some(normalized.to_string().into()),
        }
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Latin && !token.lemma().is_ascii()
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("Léopard…".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("Leopard...".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("leopard...".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(LatinNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
