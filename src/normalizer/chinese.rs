use std::borrow::Cow;

use pinyin::ToPinyin;

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

/// Normalize Chinese characters by converting them into Pinyin characters.
///
/// This Normalizer uses [`pinyin`] internally to normalize the provided token.
pub struct ChineseNormalizer;

impl Normalizer for ChineseNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        let mut lemma = String::new();

        // Need to create char_map before converting into Pinyin
        if options.create_char_map {
            let mut char_map = Vec::new();

            for c in token.lemma().chars() {
                let char_len = c.len_utf8() as u8;

                char_map.push((char_len, char_len));
            }

            token.char_map = Some(char_map);
        }

        for c in token.lemma().chars() {
            match c.to_pinyin() {
                Some(converted) => lemma.push_str(converted.plain()),
                None => lemma.push(c),
            }
        }

        token.lemma = Cow::Owned(lemma);
        token.char_end = token.lemma.chars().count();

        Box::new(Some(token).into_iter())
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
                lemma: Owned("zunyan".to_string()),
                char_end: 6,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                // lowercased
                lemma: Owned("shengerziyou".to_string()),
                char_end: 12,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3)]),
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
                lemma: Owned("zunyan".to_string()),
                char_end: 6,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
            Token {
                lemma: Owned("shengerziyou".to_string()),
                char_end: 12,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 3), (3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Cmn),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(ChineseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
