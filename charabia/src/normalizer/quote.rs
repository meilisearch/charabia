use super::{CharNormalizer, CharOrStr};
use crate::detection::Script;
use crate::Token;

/// Latin specialized [`Normalizer`].
///
/// This Normalizer replaces unicode high quotation marks by a single quote.
pub struct QuoteNormalizer;

impl CharNormalizer for QuoteNormalizer {
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        if is_unicode_high_quotation_mark(c) {
            Some('\''.into())
        } else {
            Some(c.into())
        }
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Latin && token.lemma.chars().any(is_unicode_high_quotation_mark)
    }
}

fn is_unicode_high_quotation_mark(c: char) -> bool {
    matches!(c, '’' | '‘' | '‛')
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::{Normalizer, NormalizerOption};
    use crate::token::TokenKind;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("l'l’l‘l‛".to_string()),
            char_end: 8,
            byte_end: 14,
            script: Script::Latin,
            ..Default::default()
        }]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("l'l'l'l'".to_string()),
            char_end: 8,
            byte_end: 14,
            script: Script::Latin,
            char_map: Some(vec![(1, 1), (1, 1), (1, 1), (3, 1), (1, 1), (3, 1), (1, 1), (3, 1)]),
            ..Default::default()
        }]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("l'l'l'l'".to_string()),
            char_end: 8,
            byte_end: 14,
            script: Script::Latin,
            char_map: Some(vec![(1, 1), (1, 1), (1, 1), (3, 1), (1, 1), (3, 1), (1, 1), (3, 1)]),
            kind: TokenKind::Word,
            ..Default::default()
        }]
    }

    test_normalizer!(QuoteNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
