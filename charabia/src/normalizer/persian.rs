use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

/// A global [`Normalizer`] for the Persian language.
/// Persian alphabet: ا,ب,پ,ت,ث,ج,چ,ح,خ,د,ذ,ر,ز,ژ,س,ش,ص,ض,ط,ظ,ع,غ,ف,ق,ک,گ,ل,م,ن,و,ه,ی
/// Persian text should be normalized by:
/// - Normalizing the Persian Yeh 'ی' and 'ي' to 'ی'
/// - Normalizing the Persian Kaf 'ک' and 'ك' to 'ک'
/// - Normalizing the Persian numbers '۰'-'۹' to '0'-'9'
///   https://en.wikipedia.org/wiki/Persian_alphabet

pub struct PersianNormalizer;

// Implement the CharNormalizer trait for PersianNormalizer.
impl CharNormalizer for PersianNormalizer {
    // Creates the normalized version of the provided char.
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        normalize_persian_char(c)
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Arabic && token.lemma.chars().any(is_should_normalize)
    }
}

fn normalize_persian_char(c: char) -> Option<CharOrStr> {
    match c {
        'ـ' => None,
        'ي' => Some('ی'.into()),
        'ی' => Some('ی'.into()),
        'ك' => Some('ک'.into()),
        'ک' => Some('ک'.into()),
        '۰' => Some('0'.into()),
        '۱' => Some('1'.into()),
        '۲' => Some('2'.into()),
        '۳' => Some('3'.into()),
        '۴' => Some('4'.into()),
        '۵' => Some('5'.into()),
        '۶' => Some('6'.into()),
        '۷' => Some('7'.into()),
        '۸' => Some('8'.into()),
        '۹' => Some('9'.into()),
        _ => Some(c.into()),
    }}

// Check if a character should be normalized.
fn is_should_normalize(c: char) -> bool {
    matches!(c, 'ـ' | 'ی' | 'ک' | '۰' | '۱' | '۲' | '۳' | '۴' | '۵' | '۶' | '۷' | '۸' | '۹')
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;
    use super::PersianNormalizer;
    use crate::Script;

    // Base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            // Persian Yeh
            Token {
                lemma: Owned("سلاي".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                ..Default::default()
            },
            // Persian Kaf
            Token {
                lemma: Owned("كتاب".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                ..Default::default()
            },
            // Persian number
            Token {
                lemma: Owned("۱۲۳۴۵".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("پژوهش".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                ..Default::default()
            },
        ]
    }

    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("سلام".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("کتاب".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("12345".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                ..Default::default()
            },
        ]
    }

    // Expected result of the complete Normalizer pipeline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("سلام".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("کتاب".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("12345".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("پژوهش".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(PersianNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
