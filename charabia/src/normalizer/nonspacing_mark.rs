use std::collections::HashSet;

use once_cell::sync::Lazy;

use super::CharNormalizer;
use crate::detection::Script;
use crate::normalizer::CharOrStr;
use crate::Token;

static NONSPACING_MARKS: Lazy<HashSet<u32>> = Lazy::new(|| {
    let bytes = include_bytes!("../../dictionaries/bin/nonspacing_mark/marks.bin");

    HashSet::from_iter(
        bytes.chunks_exact(4).map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap())),
    )
});

/// A global [`Normalizer`] removing nonspacing marks.
///
/// This normalizer uses built-in `HashSet` internally to check over the marks set
pub struct NonspacingMarkNormalizer;

impl CharNormalizer for NonspacingMarkNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        (!is_nonspacing_mark(c)).then(|| c.into())
    }

    fn should_normalize(&self, token: &Token) -> bool {
        matches!(
            token.script,
            Script::Hebrew | Script::Thai | Script::Arabic | Script::Latin | Script::Greek
        ) && token.lemma().chars().any(is_nonspacing_mark)
    }
}

/// Returns true if the character is a nonspacing mark
fn is_nonspacing_mark(c: char) -> bool {
    NONSPACING_MARKS.contains(&(c as u32))
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("ง่าย".to_string()),
                char_end: "ง่าย".chars().count(),
                byte_end: "ง่าย".len(),
                script: Script::Thai,
                ..Default::default()
            },
            Token {
                lemma: Owned("أَب".to_string()),
                char_end: "أَب".chars().count(),
                byte_end: "أَب".len(),
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("כָּבוֹד".to_string()),
                char_end: "כָּבוֹד".chars().count(),
                byte_end: "כָּבוֹד".len(),
                script: Script::Hebrew,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("งาย".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 0), (3, 3), (3, 3)]),
                script: Script::Thai,
                ..Default::default()
            },
            Token {
                lemma: Owned("أب".to_string()),
                char_end: "أَب".chars().count(),
                byte_end: "أَب".len(),
                char_map: Some(vec![(2, 2), (2, 0), (2, 2)]),
                script: Script::Arabic,
                ..Default::default()
            },
            Token {
                lemma: Owned("כבוד".to_string()),
                char_end: "כָּבוֹד".chars().count(),
                byte_end: "כָּבוֹד".len(),
                script: Script::Hebrew,
                char_map: Some(vec![(2, 2), (2, 0), (2, 0), (2, 2), (2, 2), (2, 0), (2, 2)]),
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pipeline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("งาย".to_string()),
                char_end: 4,
                byte_end: 12,
                char_map: Some(vec![(3, 3), (3, 0), (3, 3), (3, 3)]),
                script: Script::Thai,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("اب".to_string()),
                char_end: "أَب".chars().count(),
                byte_end: "أَب".len(),
                char_map: Some(vec![(2, 2), (2, 0), (2, 2)]),
                script: Script::Arabic,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("כבוד".to_string()),
                char_end: "כָּבוֹד".chars().count(),
                byte_end: "כָּבוֹד".len(),
                script: Script::Hebrew,
                char_map: Some(vec![(2, 2), (2, 0), (2, 0), (2, 2), (2, 2), (2, 0), (2, 2)]),
                kind: TokenKind::Word,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(NonspacingMarkNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
