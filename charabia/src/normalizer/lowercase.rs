use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::detection::Script;
use crate::Token;

/// A global [`Normalizer`] lowercasing characters.
///
pub struct LowercaseNormalizer;

impl Normalizer for LowercaseNormalizer {
    // lowercasing characters cna change the characters length, so we need
    // to make sure that the char mapping is correct and remap it if necessary.
    // <https://github.com/meilisearch/charabia/pull/234>
    fn normalize<'o>(&self, mut token: Token<'o>, _options: &NormalizerOption) -> Token<'o> {
        match token.char_map.take() {
            Some(char_map) => {
                let mut new_lemma = String::with_capacity(token.lemma.len());
                let mut new_char_map = Vec::with_capacity(char_map.len());
                let mut s = token.lemma.as_ref();
                for (orig_len, new_len) in char_map {
                    let (chunk, tail) = s.split_at(new_len as usize);
                    s = tail;
                    let lowercased_chunk = chunk.to_lowercase();
                    new_char_map.push((orig_len, lowercased_chunk.len() as u8));
                    new_lemma.push_str(&lowercased_chunk);
                }
                token.lemma = Cow::Owned(new_lemma);
                token.char_map = Some(new_char_map);
            }
            None => token.lemma = Cow::Owned(token.lemma().to_lowercase()),
        }

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        // https://en.wikipedia.org/wiki/Letter_case#Capitalisation
        matches!(token.script, Script::Latin | Script::Cyrillic | Script::Greek | Script::Georgian)
            && token.lemma.chars().any(char::is_uppercase)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::token::TokenKind;

    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("PascalCase".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Latin,
            ..Default::default()
        }]
    }

    fn normalizer_result() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("pascalcase".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Latin,
            ..Default::default()
        }]
    }

    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("pascalcase".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Latin,
            kind: TokenKind::Word,
            ..Default::default()
        }]
    }

    test_normalizer!(LowercaseNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
