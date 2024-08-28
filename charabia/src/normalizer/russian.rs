use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::{Script, Token};
use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;

pub struct RussianNormalizer;

static MATCHING_STR: Lazy<AhoCorasick> =
    Lazy::new(|| AhoCorasick::new(["Е\u{308}", "е\u{308}"]).unwrap());

impl Normalizer for RussianNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>, options: &NormalizerOption) -> Token<'o> {
        match token.char_map.take() {
            Some(mut char_map) => {
                // if a char_map already exists,iterate over it to reconstruct sub-strings.
                let mut lemma = String::new();
                let mut tail = token.lemma.as_ref();
                let mut normalized = String::new();
                for (_, normalized_len) in char_map.iter_mut() {
                    let (head, t) = tail.split_at(*normalized_len as usize);
                    tail = t;
                    normalized.clear();
                    // then normalize each sub-strings recomputing the size in the char_map.
                    let mut peekable = head.chars().peekable();
                    while let Some(c) = peekable.next() {
                        let (c, peek_consumed) = normalize_russian(c, peekable.peek());

                        if peek_consumed {
                            peekable.next();
                        }

                        normalized.push(c);
                    }

                    *normalized_len = normalized.len() as u8;
                    lemma.push_str(normalized.as_ref());
                }

                token.lemma = Cow::Owned(lemma);
                token.char_map = Some(char_map);
            }
            None => {
                // if no char_map exists, iterate over the lemma recomposing characters.
                let mut char_map = Vec::new();
                let mut lemma = String::new();
                let mut peekable = token.lemma.chars().peekable();
                while let Some(c) = peekable.next() {
                    let (normalized, peek_consumed) = normalize_russian(c, peekable.peek());

                    if peek_consumed {
                        peekable.next();
                    }

                    if options.create_char_map {
                        char_map.push((c.len_utf8() as u8, normalized.len_utf8() as u8));
                    }
                    lemma.push(normalized);
                }
                token.lemma = Cow::Owned(lemma);
                if options.create_char_map {
                    token.char_map = Some(char_map);
                }
            }
        }

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Cyrillic && MATCHING_STR.is_match(token.lemma())
    }
}

// https://en.wikipedia.org/wiki/Russian_alphabet
// Only decomposed forms are considered, as compatibility decomposition already takes care of 1-codepoint forms.
fn normalize_russian(current: char, next: Option<&char>) -> (char, bool) {
    match (current, next) {
        // ё -> е, grammatically permissible, common in writing
        ('Е', Some('\u{308}')) => ('Е', true),
        ('е', Some('\u{308}')) => ('е', true),

        (c, _) => (c, false),
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::Normalizer;
    use crate::token::TokenKind;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("Ёё".to_string()),
            char_end: 2,
            byte_end: 2,
            script: Script::Cyrillic,
            ..Default::default()
        }]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("Ёё".to_string()),
            char_end: 2,
            byte_end: 2,
            script: Script::Cyrillic,
            char_map: None,
            ..Default::default()
        }]
    }

    // expected result of the complete Normalizer pipeline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("ее".to_string()),
            char_end: 2,
            byte_end: 2,
            script: Script::Cyrillic,
            char_map: Some(vec![(2, 2), (2, 2)]),
            kind: TokenKind::Word,
            ..Default::default()
        }]
    }

    test_normalizer!(RussianNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
