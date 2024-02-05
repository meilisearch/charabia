use character_converter::traditional_to_simplified;

use super::CharNormalizer;
use crate::detection::{Language, Script};
use crate::normalizer::CharOrStr;
use crate::Token;

/// Normalize Chinese characters by:
/// 1. convert Z, Simplified, Semantic, Old, and Wrong variants
/// 2. converting them into Pinyin characters
///
/// This Normalizer uses [`pinyin`] internally to normalize the provided token.
pub struct ChineseNormalizer;

impl CharNormalizer for ChineseNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        // Normalize Z, Simplified, Semantic, Old, and Wrong variants
        let kvariant = match irg_kvariants::KVARIANTS.get(&c) {
            Some(kvariant) => kvariant.destination_ideograph,
            None => c,
        };

        // Normalize to Pinyin
        // If we don't manage to convert the kvariant, we try to convert the original character.
        // If none of them are converted, we return the kvariant.
        Some(traditional_to_simplified(kvariant.to_string().as_str()).to_string().into())
        // match kvariant.to_pinyin().or_else(|| c.to_pinyin()) {
        //     Some(converted) => {
        //         let with_tone = converted.with_tone();

        //         Some(with_tone.to_string().into())
        //     }
        //     None => Some(kvariant.into()), // e.g. 杤
        // }
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Cj && matches!(token.language, None | Some(Language::Cmn))
    }
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
                lemma: Owned("尊嚴".to_string()),
                char_end: 2,
                byte_end: 6,
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                lemma: Owned("生而自由".to_string()),
                char_end: 4,
                byte_end: 12,
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                // lowercased
                lemma: Owned("尊严".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                // lowercased
                lemma: Owned("生而自由".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            }
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("尊严".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("生而自由".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                kind: TokenKind::Word,
                ..Default::default()
            }
        ]
    }

    test_normalizer!(ChineseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
