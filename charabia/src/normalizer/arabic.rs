use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

/// A global [`Normalizer`] removing the arabic Tatweel ('ـ') characters.
/// https://www.compart.com/en/unicode/U+0640
/// https://en.wikipedia.org/wiki/Kashida
pub struct ArabicNormalizer;

impl CharNormalizer for ArabicNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        (!is_tatweel(c)).then(|| c.into())
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Arabic && token.lemma().chars().any(is_tatweel)
    }
}

fn is_tatweel(c: char) -> bool {
    c == 'ـ'
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
                lemma: Owned("الحمــــــد".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("رحــــــيم".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Arabic,
                char_map: Some(vec![
                    (2, 2),
                    (2, 2),
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

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("الحمد".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Arabic,
                char_map: Some(vec![
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 2),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("رحيم".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Arabic,
                char_map: Some(vec![
                    (2, 2),
                    (2, 2),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 2),
                    (2, 2),
                ]),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("الحمد".to_string()),
                char_end: 10,
                byte_end: 10,
                char_map: Some(vec![
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 2),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 2),
                ]),
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("رحيم".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Arabic,
                char_map: Some(vec![
                    (2, 2),
                    (2, 2),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 0),
                    (2, 2),
                    (2, 2),
                ]),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(ArabicNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
