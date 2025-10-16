use std::borrow::Cow;

use pinyin::ToPinyin;
use wana_kana::{ConvertJapanese, IsJapaneseChar, IsJapaneseStr, Options};

use super::{Normalizer, NormalizerOption};
use crate::{Language, Script, Token};

pub struct CjkNormalizer;

impl Normalizer for CjkNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>, _options: &NormalizerOption) -> Token<'o> {
        let mut chars = token.lemma.chars();
        let mut new_lemma = String::new();
        while let Some(c) = chars.next() {
            let (normalized, skip_char) = normalize_cjk(c);
            if !skip_char {
                new_lemma.push_str(&normalized);
            }
        }
        if !new_lemma.is_empty() {
            token.lemma = Cow::Owned(new_lemma);
        }

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        (token.script == Script::Cj
            && matches!(token.language, None | Some(Language::Jpn))
            && !token.lemma().is_hiragana())
            || (token.script == Script::Cj
                && matches!(token.language, None | Some(Language::Cmn) | Some(Language::Zho)))
    }
}

fn normalize_cjk(c: char) -> (String, bool) {
    if c.is_japanese() {
        // japanese.
        if c.is_katakana() {
            let mut buffer = [0; 4];
            let src = c.encode_utf8(&mut buffer);
            // Convert Katakana to Hiragana
            let dst = src.to_hiragana_with_opt(Options {
                pass_romaji: true, // Otherwise 'ダメ駄目だめHi' would become 'だめ駄目だめひ'
                ..Default::default()
            });

            // we converted the char to hiragana, so only one char is returned
            (dst, false)
        } else {
            // we don't convert japanese characters that are not katakana
            (c.to_string(), false)
        }
    } else if c == '〷' {
        // IDEOGRAPHIC TELEGRAPH LINE FEED SEPARATOR SYMBOL
        // this must be removed
        (String::new(), true)
    } else {
        // chinese
        // Normalize Z, Simplified, Semantic, Old, and Wrong variants
        let kvariant = irg_kvariants::KVARIANTS.get(&c).unwrap().destination_ideograph;

        // Normalize to Pinyin
        // If we don't manage to convert the kvariant, we try to convert the original character.
        // If none of them are converted, we return the kvariant.
        let kvariant = match kvariant.to_pinyin().or_else(|| c.to_pinyin()) {
            Some(converted) => {
                let with_tone = converted.with_tone();

                with_tone.to_string()
            }
            None => kvariant.to_string(), // e.g. 杤
        };

        (kvariant, false)
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
        ]
    }

    test_normalizer!(CjkNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
