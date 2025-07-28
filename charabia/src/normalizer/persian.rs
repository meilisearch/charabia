use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token, detection::Language};

/// A global [`Normalizer`] for the Persian language.
/// Persian alphabet: ا,ب,پ,ت,ث,ج,چ,ح,خ,د,ذ,ر,ز,ژ,س,ش,ص,ض,ط,ظ,ع,غ,ف,ق,ک,گ,ل,م,ن,و,ه,ی
/// Persian text should be normalized by:
/// - Normalizing the Persian Yeh 'ی', 'ي', 'ى', 'ۀ' to 'ی'
/// - Normalizing the Persian Kaf 'ک' and 'ك' to 'ک'
/// - Normalizing the Persian numbers '۰'-'۹' to '0'-'9'
/// - Normalizing Alef variants 'أ', 'إ', 'آ', 'ٱ' to 'ا'
/// - Normalizing Taa Marbuta 'ة' to 'ه'
/// - Removing Arabic Tatweel 'ـ'
/// - Removing diacritics '◌َ' to '◌ْ' (Fatha to Sukun)
/// - Normalizing Rial sign '﷼' to 'RIAL'
/// - Removing ZWNJ '‌'
/// - Normalizing punctuation (e.g., '،' to ',', '٫' to '.') to ASCII
///   https://en.wikipedia.org/wiki/Persian_alphabet

pub struct PersianNormalizer;

impl CharNormalizer for PersianNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        normalize_persian_char(c)
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Arabic &&
            token.language.map_or(true, |lang| lang == Language::Pes) &&
            token.lemma.chars().any(|c| is_should_normalize(c))
    }
}

fn normalize_persian_char(c: char) -> Option<CharOrStr> {
    match c {
        // Arabic Yeh, Persian Yeh, Yeh without dots, Yeh with Hamza to Persian Yeh
        'ي' | 'ی' | 'ى' | 'ۀ' => Some('ی'.into()),
        // Arabic Kaf and Persian Kaf to Persian Kaf
        'ك' | 'ک' => Some('ک'.into()),
        // Persian digits to ASCII digits
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
        // Alef variants to standard Alef
        'أ' | 'إ' | 'آ' | 'ٱ' => Some('ا'.into()),
        // Taa Marbuta to Heh
        'ة' => Some('ه'.into()),
        // Remove Tatweel
        'ـ' => None,
        // Remove diacritics
        '\u{064B}'..='\u{0652}' => None,
        // Normalize Rial sign to "RIAL"
        '\u{FDFC}' => Some(CharOrStr::Str("RIAL".into())),
        // Remove ZWNJ
        '\u{200C}' => None,
        // Normalize punctuation to ASCII
        '\u{060C}' => Some(','.into()),
        '\u{061B}' => Some(';'.into()),
        '\u{061F}' => Some('?'.into()),
        '\u{066B}' => Some('.'.into()),
        '\u{066C}' => Some(','.into()),
        '\u{00AB}' => Some('"'.into()),
        '\u{00BB}' => Some('"'.into()),
        // Preserve all other characters
        _ => Some(c.into()),
    }
}

fn is_should_normalize(c: char) -> bool {
    matches!(c,
        'ي' | 'ی' | 'ى' | 'ۀ' | // Yeh variants
        'ك' | 'ک' | // Kaf variants
        '۰'..='۹' | // Persian digits
        'أ' | 'إ' | 'آ' | 'ٱ' | // Alef variants
        'ة' | 'ـ' | // Taa Marbuta, Tatweel
        '\u{064B}'..='\u{0652}' | // Diacritics
        '\u{FDFC}' | // Rial sign
        '\u{200C}' | // ZWNJ
        '\u{060C}' | '\u{061B}' | '\u{061F}' | '\u{066B}' | '\u{066C}' | '\u{00AB}' | '\u{00BB}' // Punctuation
    )
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;

    fn tokens() -> Vec<Token<'static>> {
        vec![
            // Arabic Yeh
            Token {
                lemma: Owned("علي".to_string()),
                char_end: 3,
                byte_end: 6,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            // Arabic Kaf
            Token {
                lemma: Owned("كتاب".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            // Persian digits
            Token {
                lemma: Owned("۱۲۳۴۵۶۷۸۹۰".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            // Mixed Arabic/Persian forms
            Token {
                lemma: Owned("كیك ۱۲۳ یک کتاب".to_string()),
                char_end: 13,
                byte_end: 24,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            // Non-Persian Arabic script
            Token {
                lemma: Owned("سلام".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            // Alef variants
            Token {
                lemma: Owned("آرام".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            // Yeh with Hamza (Dari)
            Token {
                lemma: Owned("خانه".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            // Taa Marbuta
            Token {
                lemma: Owned("جامعة".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
        ]
    }

    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("علی".to_string()),
                char_end: 3,
                byte_end: 6, // 3 chars * 2 bytes
                script: Script::Arabic,
                language: Some(Language::Pes),
                char_map: Some(vec![(2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("کتاب".to_string()),
                char_end: 4,
                byte_end: 8, // 4 chars * 2 bytes
                script: Script::Arabic,
                language: Some(Language::Pes),
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("1234567890".to_string()),
                char_end: 10,
                byte_end: 10, // Corrected from 20 to 10 (10 ASCII digits * 1 byte)
                script: Script::Arabic,
                language: Some(Language::Pes),
                char_map: Some(vec![(2, 1); 10]),
                ..Default::default()
            },
            Token {
                lemma: Owned("کیک 123 یک کتاب".to_string()),
                char_end: 13,
                byte_end: 24,
                script: Script::Arabic,
                language: Some(Language::Pes),
                char_map: Some(vec![
                    (2, 2), (2, 2), (2, 2), // کیک
                    (1, 1), // space
                    (2, 1), (2, 1), (2, 1), // ۱۲۳ (Persian digits, normalized to ASCII)
                    (1, 1), // space
                    (2, 2), (2, 2), // یک
                    (1, 1), // space
                    (2, 2), (2, 2), (2, 2), (2, 2), // کتاب
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("سلام".to_string()),
                char_end: 4,
                byte_end: 8, // 4 chars * 2 bytes
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            Token {
                lemma: Owned("ارام".to_string()),
                char_end: 4,
                byte_end: 8, // 4 chars * 2 bytes
                script: Script::Arabic,
                language: Some(Language::Pes),
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("خانه".to_string()),
                char_end: 4,
                byte_end: 8, // 4 chars * 2 bytes
                script: Script::Arabic,
                language: Some(Language::Pes),
                ..Default::default()
            },
            Token {
                lemma: Owned("جامعه".to_string()),
                char_end: 5,
                byte_end: 10, // 5 chars * 2 bytes
                script: Script::Arabic,
                language: Some(Language::Pes),
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
        ]
    }

    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("علی".to_string()),
                char_end: 3,
                byte_end: 6,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("کتاب".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("1234567890".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 1); 10]),
                ..Default::default()
            },
            Token {
                lemma: Owned("کیک 123 یک کتاب".to_string()),
                char_end: 13,
                byte_end: 24,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                char_map: Some(vec![
                    (2, 2), (2, 2), (2, 2), // کیک
                    (1, 1), // space
                    (2, 1), (2, 1), (2, 1), // ۱۲۳ (Persian digits, normalized to ASCII)
                    (1, 1), // space
                    (2, 2), (2, 2), // یک
                    (1, 1), // space
                    (2, 2), (2, 2), (2, 2), (2, 2), // کتاب
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("سلام".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("ارام".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("خانه".to_string()),
                char_end: 4,
                byte_end: 8,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("جامعه".to_string()),
                char_end: 5,
                byte_end: 10,
                script: Script::Arabic,
                language: Some(Language::Pes),
                kind: TokenKind::Word,
                char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2), (2, 2)]),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(
        PersianNormalizer,
        tokens(),
        normalizer_result(),
        normalized_tokens()
    );
}