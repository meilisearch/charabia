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
pub struct JapaneseNormalizer;

impl Normalizer for JapaneseNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        if is_hiragana(token.lemma()) {
            // No need to convert

            if options.create_char_map && token.char_map.is_none() {
                let mut char_map = Vec::new();
                for c in token.lemma().chars() {
                    char_map.push((c.len_utf8() as u8, c.len_utf8() as u8));
                }
                token.char_map = Some(char_map);
            }
        } else {
            // Convert Katakana to Hiragana
            let new_lemma = to_hiragana_with_opt(
                token.lemma(),
                Options {
                    pass_romaji: true, // Otherwise 'ダメ駄目だめHi' would become 'だめ駄目だめひ'
                    ..Default::default()
                },
            );

            if options.create_char_map && token.char_map.is_none() {
                debug_assert!(
                    token.lemma().len() == new_lemma.len(),
                    concat!(
                        r#"`to_hiragana` changed the lemma len from {} to {} but the current `char_map` computation "#,
                        r#"expected them to be equal. If `to_hiragana` does change len of char somehow, consider "#,
                        r#"calling `to_hiragana(char)` char by char instead of only calling `to_hiragana(lemma)` once."#
                    ),
                    token.lemma().len(),
                    new_lemma.len()
                );
                let old_new_chars = token.lemma().chars().zip(new_lemma.chars());

                let mut char_map = Vec::new();
                for (_i, (old_char, new_char)) in old_new_chars.enumerate() {
                    char_map.push((old_char.len_utf8() as u8, new_char.len_utf8() as u8))
                }
                token.char_map = Some(char_map);
            }

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
                char_map: Some(vec![(3, 3), (3, 3)]),
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
                char_map: Some(vec![
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (1, 1),
                    (1, 1),
                ]),
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
                char_map: Some(vec![(3, 3), (3, 3)]),
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
                char_map: Some(vec![
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (1, 1),
                    (1, 1),
                ]),
                script: Script::Cj,
                language: Some(Language::Jpn),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(JapaneseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
