use std::borrow::Cow;

use deunicode::{deunicode, deunicode_char};

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

/// Latin specialized [`Normalizer`] converting unicode chars into Ascii.
///
/// This Normalizer uses [`deunicode`] internally to normalize the provided token.
pub struct LatinNormalizer;

impl Normalizer for LatinNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        if !token.lemma().is_ascii() {
            let mut lemma = String::new();
            if options.create_char_map {
                let mut char_map = Vec::new();
                for c in token.lemma().chars() {
                    // if a char can't be deunicoded, skip it.
                    let deunicoded = deunicode_char(c).unwrap_or("").trim();
                    char_map.push((c.len_utf8() as u8, deunicoded.len() as u8));
                    lemma.push_str(&deunicoded);
                }
                token.char_map = Some(char_map);
            } else {
                lemma.push_str(&deunicode(token.lemma()));
            }
            token.lemma = Cow::Owned(lemma);
        }

        // Create an iterator over the normalized token.
        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, script: Script, _language: Option<Language>) -> bool {
        script == Script::Latin
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
                lemma: Owned("Léopard…".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("Leopard...".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("leopard...".to_string()),
                char_end: 8,
                byte_end: 11,
                script: Script::Latin,
                char_map: Some(vec![
                    (1, 1),
                    (2, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (1, 1),
                    (3, 3),
                ]),
                ..Default::default()
            },
            Token {
                lemma: Owned("lion".to_string()),
                char_end: 4,
                byte_end: 4,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(LatinNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
