use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::detection::Script;
use crate::Token;

/// A global [`Normalizer`] lowercasing characters.
///
pub struct LowercaseNormalizer;

impl Normalizer for LowercaseNormalizer {
    // lowercasing characters doesn't change the characters length,
    // so the `normalize` method is overloaded to skip the useless char_map computing.
    fn normalize<'o>(&self, mut token: Token<'o>, _options: &NormalizerOption) -> Token<'o> {
        token.lemma = Cow::Owned(token.lemma().to_lowercase());

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
