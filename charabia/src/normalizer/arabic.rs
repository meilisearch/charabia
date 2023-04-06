use super::{CharNormalizer, CharOrStr};
use crate::{Script, Token};

/// A global [`Normalizer`] for Arabic language.
/// Arabic alphabet:ا,ب,ت,ث,ج,ح,خ,د,ذ,ر,ز,س,ش,ص,ض,ط,ظ,ع,غ,ف,ق,ك,ل,م,ن,ه,و,ي,ء
/// Arabic text should be normalized by:
/// - removing the arabic Tatweel ('ـ') characters.
/// - normalizing the arabic Alef 'أ','إ','آ','ٱ' to 'ا'
/// - normalizing the arabic Yeh 'ى' to 'ي'
/// - Normalizing the arabic Taa Marbuta 'ة' to 'ه'
/// - removing the arabic diacritics: 'Fatḥah', 'Damma', 'Kasrah', 'Alif Khanjariyah', 'Maddah', 'Sukun', 'Tanwin', 'Shaddah'
/// - Arabic diacritics: 'َ', 'ُ', 'ِ', 'ٰ', 'ٓ', 'ْ', 'ۡ', 'ً', 'ٍ', 'ٌ', 'ّ',
/// https://en.wikipedia.org/wiki/Arabic_alphabet
/// https://en.wikipedia.org/wiki/Arabic_diacritics
/// https://en.wikipedia.org/wiki/Kashida

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
        'أ' | 'إ' | 'آ' | 'ٱ' => Some('ا'.into()),
        'ى' => Some('ي'.into()),
        'ة' => Some('ه'.into()),
        'َ' | 'ُ' | 'ِ' | 'ٰ' | 'ٓ' | 'ْ' | 'ۡ' | 'ً' | 'ٍ' | 'ٌ' | 'ّ' => None,
        _ => Some(c.into()),
    }
}

fn is_shoud_normalize(c: char) -> bool {
    match c {
        'ـ' | 'أ' | 'إ' | 'آ' | 'ٱ' | 'ى' | 'ة' | 'َ' | 'ُ' | 'ِ' | 'ٰ' | 'ٓ' | 'ْ' | 'ۡ' | 'ً' | 'ٍ' | 'ٌ'
        | 'ّ' => true,
        _ => false,
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
