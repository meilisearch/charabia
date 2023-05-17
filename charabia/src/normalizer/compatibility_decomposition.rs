use std::iter::once;

use unicode_normalization::{is_nfkd_quick, UnicodeNormalization};

use super::{CharNormalizer, CharOrStr};
use crate::Token;

/// A global [`Normalizer`] normalizing to the Unicode Normalization Form KD.
///
/// This Normalizer uses [`unicode-normalization::nfkd`] internally to normalize the provided token.
///
/// The Unicode Normalization Form KD (NFKD) is the Compatibility Decomposition normalization, see
/// <https://www.unicode.org/reports/tr15/#Norm_Forms> for more information.
pub struct CompatibilityDecompositionNormalizer;

impl CharNormalizer for CompatibilityDecompositionNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        let mut normalized = c.nfkd();

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
        !(token.lemma().is_ascii()
            || matches!(
                is_nfkd_quick(token.lemma().chars()),
                unicode_normalization::IsNormalized::Yes
            ))
    }
}

// Test the normalizer:
#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;
    use crate::{Language, Script};

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                // Decompose 1E69 to 0073 0323 0307
                lemma: Owned("ṩ ṩ".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ｶﾞｷﾞｸﾞｹﾞｺﾞ".to_string()),
                char_end: "ｶﾞｷﾞｸﾞｹﾞｺﾞ".chars().count(),
                byte_end: "ｶﾞｷﾞｸﾞｹﾞｺﾞ".len(),
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("s\u{0323}\u{0307} s\u{0323}\u{0307}".to_string()),
                char_end: 2,
                byte_end: 2,
                char_map: Some(vec![(3, 5), (1, 1), (3, 5)]),
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("カ\u{3099}キ\u{3099}ク\u{3099}ケ\u{3099}コ\u{3099}".to_string()),
                char_end: "ｶﾞｷﾞｸﾞｹﾞｺﾞ".chars().count(),
                byte_end: "ｶﾞｷﾞｸﾞｹﾞｺﾞ".len(),
                script: Script::Cj,
                char_map: Some(vec![
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                ]),
                language: Some(Language::Jpn),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("s s".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                char_map: Some(vec![(3, 1), (1, 1), (3, 1)]),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                #[cfg(feature = "japanese-transliteration")]
                lemma: Owned("か\u{3099}き\u{3099}く\u{3099}け\u{3099}こ\u{3099}".to_string()),
                #[cfg(not(feature = "japanese-transliteration"))]
                lemma: Owned("カ\u{3099}キ\u{3099}ク\u{3099}ケ\u{3099}コ\u{3099}".to_string()),
                char_end: "ｶﾞｷﾞｸﾞｹﾞｺﾞ".chars().count(),
                byte_end: "ｶﾞｷﾞｸﾞｹﾞｺﾞ".len(),
                script: Script::Cj,
                language: Some(Language::Jpn),
                char_map: Some(vec![
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                ]),
                kind: TokenKind::Word,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(
        CompatibilityDecompositionNormalizer,
        tokens(),
        normalizer_result(),
        normalized_tokens()
    );
}
