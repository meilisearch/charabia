use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::{Script, Token};

/// Normalize Greek characters by:
/// 1. convert final sigma into ordinary sigma
///
pub struct GreekNormalizer;

impl Normalizer for GreekNormalizer {
    // converting  "ς" to "σ" doesn't change the characters length,
    // so the `normalize` method is overloaded to skip the useless char_map computing.
    fn normalize<'o>(&self, mut token: Token<'o>, _options: &NormalizerOption) -> Token<'o> {
        if let Some(prefix) = token.lemma.strip_suffix('ς') {
            token.lemma = Cow::Owned([prefix, "σ"].concat())
        }

        token
    }

    fn should_normalize(&self, token: &Token) -> bool {
        token.script == Script::Greek
    }
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
            lemma: Owned("Αγαπητός".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Greek,
            ..Default::default()
        }]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("Αγαπητόσ".to_string()),
            char_end: 10,
            byte_end: 10,
            char_map: None,
            script: Script::Greek,
            ..Default::default()
        }]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("αγαπητοσ".to_string()),
            char_end: 10,
            byte_end: 10,
            char_map: Some(vec![(2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2), (2, 2)]),
            script: Script::Greek,
            kind: TokenKind::Word,
            ..Default::default()
        }]
    }

    test_normalizer!(GreekNormalizer, tokens(), normalizer_result(), normalized_tokens());
}
