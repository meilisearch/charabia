use std::borrow::Cow;

use pinyin::ToPinyin;

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};

/// Normalize Chinese characters by converting them into Pinyin characters.
///
/// This Normalizer uses [`pinyin`] internally to normalize the provided token.
pub struct ChineseNormalizer;

impl Normalizer for ChineseNormalizer {
    fn normalize_str<'o>(&self, src: &'o str) -> Cow<'o, str> {
        let mut dst = String::new();
        for c in src.chars() {
            match c.to_pinyin() {
                Some(converted) => {
                    let with_tone = converted.with_tone();

                    dst.push_str(with_tone);
                }
                None => {
                    dst.push(c);
                }
            }
        }

        Cow::Owned(dst)
    }

    fn should_normalize(&self, script: Script, language: Option<Language>) -> bool {
        script == Script::Cj && matches!(language, None | Some(Language::Cmn))
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;

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
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("zūnyán".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 4), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                lemma: Owned("shēngérzìyóu".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 6), (3, 3), (3, 3), (3, 4)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(ChineseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
