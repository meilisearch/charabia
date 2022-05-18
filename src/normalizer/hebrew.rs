use std::borrow::Cow;

use niqqud;

use super::Normalizer;
use crate::{Language, Script, Token};

/// Normalize Hebrew characters by undiacritisizing (removing diacritics) them.
///
/// This Normalizer uses [`niqqud`] internally to normalize the provided token.
pub struct HebrewNormalizer;

impl Normalizer for HebrewNormalizer {
    fn normalize<'o>(&self, mut token: Token<'o>) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        token.lemma = Cow::Owned(niqqud::remove(token.lemma()).into());

        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, script: Script, _language: Option<Language>) -> bool {
        script == Script::Hebrew
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
                char_end: "כבוד".chars().count(),
                byte_end: "כבוד".len(),
                script: Script::Hebrew,
                ..Default::default()
            },
            Token {
                lemma: Owned("לקפץ".to_string()),
                char_end: "לקפץ".chars().count(),
                byte_end: "לקפץ".len(),
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
                char_end: "כבוד".chars().count(),
                byte_end: "כבוד".len(),
                script: Script::Hebrew,
                ..Default::default()
            },
            Token {
                lemma: Owned("לקפץ".to_string()),
                char_end: "לקפץ".chars().count(),
                byte_end: "לקפץ".len(),
                script: Script::Hebrew,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(HebrewNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
