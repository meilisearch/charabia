use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

/// A global [`Normalizer`] for Arabic language.
/// Arabic alphabet:ا,ب,ت,ث,ج,ح,خ,د,ذ,ر,ز,س,ش,ص,ض,ط,ظ,ع,غ,ف,ق,ك,ل,م,ن,ه,و,ي,ء
/// Arabic text should be normalized by:
/// - removing the arabic Tatweel ('ـ') characters.
/// - normalizing the arabic Alef 'أ','إ','آ','ٱ' to 'ا'
/// - normalizing the arabic Yeh 'ى' to 'ي'
/// - Normalizing the arabic Taa Marbuta 'ة' to 'ه'
///   https://en.wikipedia.org/wiki/Arabic_alphabet
///   https://en.wikipedia.org/wiki/Kashida
pub struct ArabicNormalizer;

// All normalizers only need to implement the method `normalize_char` and the method `should_normalize` of the `CharNormalizer` trait.
impl CharNormalizer for ArabicNormalizer {
    // Creates the normalized version of the provided char.
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        normalize_arabic_char(c)
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Arabic && token.lemma.chars().any(is_shoud_normalize)
    }
}

fn normalize_arabic_char(c: char) -> Option<CharOrStr> {
    match c {
        'ـ' => None,
        'ٱ' => Some('ا'.into()),
        'ى' => Some('ي'.into()),
        'ة' => Some('ه'.into()),
        _ => Some(c.into()),
    }
}

fn is_shoud_normalize(c: char) -> bool {
    matches!(c, 'ـ' | 'ٱ' | 'ى' | 'ة')
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
            // Tatweel
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
            // Alef wasla
            Token {
                lemma: Owned("ٱلحمد".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                ..Default::default()
            },
            // Yeh
            Token {
                lemma: Owned("يومى".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                ..Default::default()
            },
            // Taa Marbuta
            Token {
                lemma: Owned("النهاردة".to_string()),
                char_end: 8,
                byte_end: 16,
                script: Script::Arabic,
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
            Token {
                lemma: Owned("الحمد".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("يومي".to_string()),
                char_end: 4,
                byte_end: 8,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2)]),
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("النهارده".to_string()),
                char_end: 8,
                byte_end: 16,
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
                script: Script::Arabic,
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
                kind: TokenKind::Word,
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
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("الحمد".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2), (2, 2)]),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("يومي".to_string()),
                char_end: 4,
                byte_end: 8,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2)]),
                script: Script::Arabic,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("النهارده".to_string()),
                char_end: 8,
                byte_end: 16,
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
                script: Script::Arabic,
                kind: TokenKind::Word,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(ArabicNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
