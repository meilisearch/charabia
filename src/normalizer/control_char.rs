use std::borrow::Cow;

use super::Normalizer;
use crate::detection::{Language, Script};
use crate::Token;

/// A [`Normalizer`] removing control characters.
///
pub struct ControlCharNormalizer;

impl Normalizer for ControlCharNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        if token.lemma().chars().any(is_control) {
            let mut lemma = String::new();
            let char_map = match token.char_map.take() {
                // modify the current char map
                Some(mut char_map) => {
                    let mut byte_index = 0;
                    for l in char_map.iter_mut() {
                        let subset: String = token.lemma
                            [byte_index as usize..(byte_index + *l) as usize]
                            .chars()
                            .filter(|c| !is_control(*c))
                            .collect();
                        byte_index += *l;
                        *l = subset.len() as u8;
                        lemma.push_str(&subset);
                    }

                    char_map
                }
                // create and compute a new char map
                None => {
                    let mut char_map = Vec::new();
                    for c in token.lemma().chars() {
                        if is_control(c) {
                            // skip character
                            char_map.push(0);
                        } else {
                            char_map.push(c.len_utf8() as u8);
                            lemma.push(c);
                        }
                    }

                    char_map
                }
            };

            token.lemma = Cow::Owned(lemma);
            token.char_map = Some(char_map);
        }

        // Create an iterator over the normalized token.
        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, _script: Script, _language: Option<Language>) -> bool {
        true
    }
}

fn is_control(c: char) -> bool {
    c.is_control() && !c.is_whitespace()
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("\0生而自由\u{2}oo\0".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Mandarin,
                ..Default::default()
            },
            Token {
                lemma: Owned("\0生而自由\u{2}oo\0".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Mandarin,
                char_map: Some(vec![1, 3, 3, 3, 3, 1, 1, 1, 1]),
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Mandarin,
                char_map: Some(vec![0, 3, 3, 3, 3, 0, 1, 1, 0]),
                ..Default::default()
            },
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Mandarin,
                char_map: Some(vec![0, 3, 3, 3, 3, 0, 1, 1, 0]),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Mandarin,
                char_map: Some(vec![0, 3, 3, 3, 3, 0, 1, 1, 0]),
                ..Default::default()
            },
            Token {
                lemma: Owned("生而自由oo".to_string()),
                char_end: 9,
                byte_end: 17,
                script: Script::Mandarin,
                char_map: Some(vec![0, 3, 3, 3, 3, 0, 1, 1, 0]),
                ..Default::default()
            },
        ]
    }

    test_normalizer!(ControlCharNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
