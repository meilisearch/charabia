use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::{Language, Script, Token};

/// Normalize Hebrew characters by undiacritisizing (removing diacritics) them.
///
/// This Normalizer is inspired by [`niqqud`] to normalize the provided token.
///
/// [`niqqud`]: https://crates.io/crates/niqqud
pub struct HebrewNormalizer;

impl Normalizer for HebrewNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        if token.lemma().chars().any(is_diacritic) {
            let mut char_map = if options.create_char_map { Some(Vec::new()) } else { None };
            let mut lemma = String::new();
            for c in token.lemma().chars() {
                if is_diacritic(c) {
                    char_map.as_mut().map(|char_map| char_map.push((c.len_utf8() as u8, 0)));
                } else {
                    char_map
                        .as_mut()
                        .map(|char_map| char_map.push((c.len_utf8() as u8, c.len_utf8() as u8)));
                    lemma.push(c);
                }
            }

            token.lemma = Cow::Owned(lemma);
            token.char_map = char_map;
        }

        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, script: Script, _language: Option<Language>) -> bool {
        script == Script::Hebrew
    }
}

/// Returns true if the character is a diacritic
fn is_diacritic(c: char) -> bool {
    matches!(c, '\u{0590}'..='\u{05CF}')
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("כָּבוֹד".to_string()),
                char_end: "כָּבוֹד".chars().count(),
                byte_end: "כָּבוֹד".len(),
                script: Script::Hebrew,
                ..Default::default()
            },
            Token {
                lemma: Owned("לִקְפֹּץ".to_string()),
                char_end: "לִקְפֹּץ".chars().count(),
                byte_end: "לִקְפֹּץ".len(),
                script: Script::Hebrew,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("כבוד".to_string()),
                char_end: "כָּבוֹד".chars().count(),
                byte_end: "כָּבוֹד".len(),
                script: Script::Hebrew,
                char_map: Some(vec![(2, 2), (2, 0), (2, 0), (2, 2), (2, 2), (2, 0), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("לקפץ".to_string()),
                char_end: "לִקְפֹּץ".chars().count(),
                byte_end: "לִקְפֹּץ".len(),
                char_map: Some(vec![
                    (2, 2),
                    (2, 0),
                    (2, 2),
                    (2, 0),
                    (2, 2),
                    (2, 0),
                    (2, 0),
                    (2, 2),
                ]),
                script: Script::Hebrew,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("כבוד".to_string()),
                char_end: "כָּבוֹד".chars().count(),
                byte_end: "כָּבוֹד".len(),
                script: Script::Hebrew,
                char_map: Some(vec![(2, 2), (2, 0), (2, 0), (2, 2), (2, 2), (2, 0), (2, 2)]),
                ..Default::default()
            },
            Token {
                lemma: Owned("לקפץ".to_string()),
                char_end: "לִקְפֹּץ".chars().count(),
                byte_end: "לִקְפֹּץ".len(),
                char_map: Some(vec![
                    (2, 2),
                    (2, 0),
                    (2, 2),
                    (2, 0),
                    (2, 2),
                    (2, 0),
                    (2, 0),
                    (2, 2),
                ]),
                script: Script::Hebrew,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(HebrewNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
