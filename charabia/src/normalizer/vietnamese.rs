use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

pub struct VietnameseNormalizer;

impl CharNormalizer for VietnameseNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        match c {
            'Ð' | 'Đ' | 'đ' | 'ð' => Some("d".to_string().into()), // not only Vietnamese, but also many European countries use these letters
            _ => Some(c.into()),
        }
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Latin && token.lemma.chars().any(is_should_normalize)
    }
}

fn is_should_normalize(c: char) -> bool {
    matches!(c, 'Ð' | 'Đ' | 'đ' | 'ð')
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
            Token {
                lemma: Owned("Ðại Việt".to_string()),
                char_end: 8,
                byte_end: 13,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("Đại Việt".to_string()),
                char_end: 8,
                byte_end: 13,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("đại Việt".to_string()),
                char_end: 8,
                byte_end: 13,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("dại Việt".to_string()),
                char_end: 8,
                byte_end: 13,
                char_map: Some(vec![
                    (2, 1),
                    (3, 3),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                    (1, 1),
                ]),
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("dại Việt".to_string()),
                char_end: 8,
                byte_end: 13,
                char_map: Some(vec![
                    (2, 1),
                    (3, 3),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                    (1, 1),
                ]),
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("dại Việt".to_string()),
                char_end: 8,
                byte_end: 13,
                char_map: Some(vec![
                    (2, 1),
                    (3, 3),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                    (1, 1),
                ]),
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                kind: TokenKind::Word,
                lemma: Owned("dai viet".to_string()),
                char_end: 8,
                byte_end: 13,
                char_map: Some(vec![
                    (2, 1),
                    (3, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 1),
                    (1, 1),
                ]),
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                kind: TokenKind::Word,
                lemma: Owned("dai viet".to_string()),
                char_end: 8,
                byte_end: 13,
                char_map: Some(vec![
                    (2, 1),
                    (3, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 1),
                    (1, 1),
                ]),
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                kind: TokenKind::Word,
                lemma: Owned("dai viet".to_string()),
                char_end: 8,
                byte_end: 13,
                char_map: Some(vec![
                    (2, 1),
                    (3, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 1),
                    (1, 1),
                ]),
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(VietnameseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
