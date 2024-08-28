use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

/// Turkish specialized [`Normalizer`].
///
/// Turkish text should be normalized by:
/// - Normalizing the Turkish alphabet: 'ı' to 'i'
///
/// There are other peculiar characters in the Turkish alphabet[1],
/// but this `Normalizer` only supports 'ı', as normalization
/// is already achieved by the existing `Normalizer` used
/// in the case of `Script::Latin`.
///
/// [1]: https://en.wikipedia.org/wiki/Turkish_alphabet
pub struct TurkishNormalizer;

impl CharNormalizer for TurkishNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        match c {
            'ı' => Some("i".to_string().into()),
            _ => Some(c.into()),
        }
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Latin && token.lemma.chars().any(is_should_normalize)
    }
}

fn is_should_normalize(c: char) -> bool {
    c == 'ı'
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            // Turkish alphabet
            Token {
                lemma: Owned("ABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZ".to_string()),
                char_end: 29,
                byte_end: 35,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("abcçdefgğhıijklmnoöprsştuüvyz".to_string()),
                char_end: 29,
                byte_end: 35,
                script: Script::Latin,
                ..Default::default()
            },
            // Turkish texts containing 'ı'
            Token {
                lemma: Owned("çalışma".to_string()),
                char_end: 7,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("şarkı".to_string()),
                char_end: 5,
                byte_end: 7,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ışık".to_string()),
                char_end: 4,
                byte_end: 7,
                script: Script::Latin,
                ..Default::default()
            },
            // Turkish texts without 'ı'
            // - verify that the complete pipeline normalizes turkish text as expected
            Token {
                lemma: Owned("günlük".to_string()),
                char_end: 6,
                byte_end: 8,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("İstanbul".to_string()),
                char_end: 8,
                byte_end: 9,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("İstasyon".to_string()),
                char_end: 8,
                byte_end: 9,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ömür".to_string()),
                char_end: 4,
                byte_end: 6,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ütü".to_string()),
                char_end: 3,
                byte_end: 5,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            // Turkish alphabet
            Token {
                lemma: Owned("ABCÇDEFGĞHIİJKLMNOÖPRSŞTUÜVYZ".to_string()),
                char_end: 29,
                byte_end: 35,
                script: Script::Latin,
                char_map: None,
                ..Default::default()
            },
            Token {
                lemma: Owned("abcçdefgğhiijklmnoöprsştuüvyz".to_string()),
                char_end: 29,
                byte_end: 35,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 2),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 2),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 2),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 2),
                    (1, 1),
                    (1, 1),
                    (2, 2),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                ]),
                ..Default::default()
            },
            // Turkish texts containing 'ı'
            Token {
                lemma: Owned("çalişma".to_string()),
                char_end: 7,
                byte_end: 10,
                script: Script::Latin,
                char_map: Some(vec![(2, 2), (1, 1), (1, 1), (2, 1), (2, 2), (1, 1), (1, 1)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("şarki".to_string()),
                char_end: 5,
                byte_end: 7,
                script: Script::Latin,
                char_map: Some(vec![(2, 2), (1, 1), (1, 1), (1, 1), (2, 1)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("işik".to_string()),
                char_end: 4,
                byte_end: 7,
                script: Script::Latin,
                char_map: Some(vec![(2, 1), (2, 2), (2, 1), (1, 1)]),
                ..Default::default()
            },
            // Turkish texts without 'ı'
            // - verify that the complete pipeline normalizes turkish text as expected
            Token {
                lemma: Owned("günlük".to_string()),
                char_end: 6,
                byte_end: 8,
                script: Script::Latin,
                char_map: None,
                ..Default::default()
            },
            Token {
                lemma: Owned("İstanbul".to_string()),
                char_end: 8,
                byte_end: 9,
                script: Script::Latin,
                char_map: None,
                ..Default::default()
            },
            Token {
                lemma: Owned("İstasyon".to_string()),
                char_end: 8,
                byte_end: 9,
                script: Script::Latin,
                char_map: None,
                ..Default::default()
            },
            Token {
                lemma: Owned("ömür".to_string()),
                char_end: 4,
                byte_end: 6,
                script: Script::Latin,
                char_map: None,
                ..Default::default()
            },
            Token {
                lemma: Owned("ütü".to_string()),
                char_end: 3,
                byte_end: 5,
                script: Script::Latin,
                char_map: None,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pipeline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            // Turkish alphabet
            Token {
                lemma: Owned("abccdefgghiijklmnooprsstuuvyz".to_string()),
                char_end: 29,
                byte_end: 35,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("abccdefgghiijklmnooprsstuuvyz".to_string()),
                char_end: 29,
                byte_end: 35,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                ]),
                ..Default::default()
            },
            // Turkish texts containing 'ı'
            Token {
                lemma: Owned("calisma".to_string()),
                char_end: 7,
                byte_end: 10,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 1), (1, 1), (1, 1), (2, 1), (2, 1), (1, 1), (1, 1)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("sarki".to_string()),
                char_end: 5,
                byte_end: 7,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 1), (1, 1), (1, 1), (1, 1), (2, 1)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("isik".to_string()),
                char_end: 4,
                byte_end: 7,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 1), (2, 1), (2, 1), (1, 1)]),
                ..Default::default()
            },
            // Turkish texts without 'ı'
            // - verify that the complete pipeline normalizes turkish text as expected
            Token {
                lemma: Owned("gunluk".to_string()),
                char_end: 6,
                byte_end: 8,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![(1, 1), (2, 1), (1, 1), (1, 1), (2, 1), (1, 1)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("istanbul".to_string()),
                char_end: 8,
                byte_end: 9,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![
                    (2, 1),
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
                lemma: Owned("istasyon".to_string()),
                char_end: 8,
                byte_end: 9,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![
                    (2, 1),
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
                lemma: Owned("omur".to_string()),
                char_end: 4,
                byte_end: 6,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 1), (1, 1), (2, 1), (1, 1)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("utu".to_string()),
                char_end: 3,
                byte_end: 5,
                script: Script::Latin,
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 1), (1, 1), (2, 1)]),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(TurkishNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
