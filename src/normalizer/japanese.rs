use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;
use wana_kana::is_hiragana::*;
use wana_kana::to_hiragana::to_hiragana_with_opt;
use wana_kana::Options;

/// Japanese specialized [`Normalizer`].
///
/// This Normalizer uses [`wana_kana`] internally to convert Katakana and Kanji to Hiragana
///
/// The input and output of `is_hiragana` will have identical char len according to manual testing [1],
/// therefore the `options.create_char_map` should be ignored for now [2].
///
/// [wana_kana]: https://docs.rs/wana_kana/latest/wana_kana/
/// [1]: https://github.com/meilisearch/charabia/pull/149#issuecomment-1273540805
/// [2]: https://github.com/meilisearch/charabia/pull/149#discussion_r991337772
pub struct JapaneseNormalizer;

impl Normalizer for JapaneseNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        _options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        if !is_hiragana(token.lemma()) {
            // Convert Katakana to Hiragana
            let new_lemma = to_hiragana_with_opt(
                token.lemma(),
                Options {
                    pass_romaji: true, // Otherwise 'ダメ駄目だめHi' would become 'だめ駄目だめひ'
                    ..Default::default()
                },
            );

            token.lemma = Cow::Owned(new_lemma);
        }

        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, script: Script, language: Option<Language>) -> bool {
        script == Script::Cj && matches!(language, None | Some(Language::Jpn))
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
                lemma: Owned("だめ".to_string()),
                char_end: 2,
                byte_end: 6,
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
            Token {
                lemma: Owned("だめ".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
            Token {
                lemma: Owned("ダメ駄目だめHi".to_string()),
                char_end: 8,
                byte_end: 20,
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
                lemma: Owned("だめ".to_string()),
                char_end: 2,
                byte_end: 6,
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
            Token {
                lemma: Owned("だめ".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
            Token {
                lemma: Owned("だめ駄目だめHi".to_string()),
                char_end: 8,
                byte_end: 20,
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("だめ".to_string()),
                char_end: 2,
                byte_end: 6,
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
            Token {
                lemma: Owned("だめ".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 3), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
            Token {
                lemma: Owned("だめ駄目だめHi".to_string()),
                char_end: 8,
                byte_end: 20,
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(JapaneseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
