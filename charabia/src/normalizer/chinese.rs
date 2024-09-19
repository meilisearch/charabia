#[cfg(feature = "chinese-normalization-pinyin")]
use pinyin::ToPinyin;

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
        #[cfg(feature = "chinese-normalization-pinyin")]
        let kvariant = match kvariant.to_pinyin().or_else(|| c.to_pinyin()) {
            Some(converted) => {
                let with_tone = converted.with_tone();

                with_tone.to_string()
            }
            None => kvariant.to_string(), // e.g. 杤
        };

        Some(kvariant.into())
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Cj
            && matches!(token.language, None | Some(Language::Cmn) | Some(Language::Zho))
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
            Token {
                lemma: Owned("澚䀾亚㮺刄杤".to_string()),
                char_end: 5,
                byte_end: 15,
                script: Script::Cj,
                language: Some(Language::Zho),
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    #[cfg(feature = "chinese-normalization-pinyin")]
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                // lowercased
                lemma: Owned("zūnyán".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 4), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                // lowercased
                lemma: Owned("shēngérzìyóu".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 6), (3, 3), (3, 3), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                // It would be "yudǔyàběnrèn" without the kvariant normalization.
                lemma: Owned("àoqìyàběnrènwàn".to_string()),
                char_end: 5,
                byte_end: 15,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 4), (3, 4), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Zho),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    #[cfg(feature = "chinese-normalization-pinyin")]
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("zūnyán".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 4), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("shēngérzìyóu".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 6), (3, 3), (3, 3), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("àoqìyàběnrènwàn".to_string()),
                char_end: 5,
                byte_end: 15,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 4), (3, 4), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Zho),
                kind: TokenKind::Word,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    #[cfg(not(feature = "chinese-normalization-pinyin"))]
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("尊嚴".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                lemma: Owned("生而自由".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                lemma: Owned("澳䁈亞本刃𣜜".to_string()),
                char_end: 5,
                byte_end: 15,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3), (3, 3), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Zho),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    #[cfg(not(feature = "chinese-normalization-pinyin"))]
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                kind: TokenKind::Word,
                lemma: Owned("尊嚴".to_string()),
                char_start: 0,
                char_end: 2,
                byte_start: 0,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
            },
            Token {
                kind: TokenKind::Word,
                lemma: Owned("生而自由".to_string()),
                char_start: 0,
                char_end: 4,
                byte_start: 0,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
            },
            Token {
                kind: TokenKind::Word,
                lemma: Owned("澳䁈亞本刃𣜜".to_string()),
                char_start: 0,
                char_end: 5,
                byte_start: 0,
                byte_end: 15,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3), (3, 3), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Zho),
            },
        ]
    }

    test_normalizer!(ChineseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
