use std::borrow::Cow;

use wana_kana::{ConvertJapanese, IsJapaneseStr, Options};

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

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
    // converting katakana to hiragana doesn't change the characters length,
    // so the `normalize` method is overloaded to skip the useless char_map computing.
    fn normalize<'o>(&self, mut token: Token<'o>, _options: &NormalizerOption) -> Token<'o> {
        // Convert Katakana to Hiragana
        let dst = token.lemma().to_hiragana_with_opt(Options {
            pass_romaji: true, // Otherwise 'ダメ駄目だめHi' would become 'だめ駄目だめひ'
            ..Default::default()
        });

        token.lemma = Cow::Owned(dst);
        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Cj
            && matches!(token.language, None | Some(Language::Jpn))
            && !token.lemma().is_hiragana()
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::token::TokenKind;

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
                lemma: Owned("た\u{3099}め".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 6), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Jpn),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("た\u{3099}め".to_string()),
                char_end: 2,
                byte_end: 6,
                char_map: Some(vec![(3, 6), (3, 3)]),
                script: Script::Cj,
                language: Some(Language::Jpn),
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("た\u{3099}め駄目た\u{3099}めHi".to_string()),
                char_end: 8,
                byte_end: 20,
                char_map: Some(vec![
                    (3, 6),
                    (3, 3),
                    (3, 3),
                    (3, 3),
                    (3, 6),
                    (3, 3),
                    (1, 1),
                    (1, 1),
                ]),
                script: Script::Cj,
                language: Some(Language::Jpn),
                kind: TokenKind::Word,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(JapaneseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
