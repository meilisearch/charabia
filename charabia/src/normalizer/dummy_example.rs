// Import `CharNormalizer` trait.
use super::{CharNormalizer, CharOrStr};
use crate::Token;

// Make a small documentation of the specialized Normalizer like below.
/// <Script/Language> specialized [`Normalizer`].
///
/// This Normalizer uses [`<UsedLibraryToNormalize>`] internally to normalize the provided token.
/// <OptionalAdditionnalExplanations>
pub struct DummyNormalizer;

// All normalizers only need to implement the method `normalize_char` and the method `should_normalize` of the `CharNormalizer` trait.
impl CharNormalizer for DummyNormalizer {
    // Creates the normalized version of the provided char.
    // In this example we will remove whitespaces and lowercase other characters.
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        if c.is_whitespace() {
            // return None to remove the current character.
            None
        } else if c.is_lowercase() {
            // the current character is already lowercased.

            // return Some of the orginal version to keep the original character.
            // note that `into()` will convert any `char` or `String` into the expected type `CharOrStr`.
            Some(c.into())
        } else {
            // lowercase the provided char.
            let normalized: String = c.to_lowercase().collect();

            // return Some of the normalized version to apply this normalized version.
            // note that `into()` will convert any `char` or `String` into the expected type `CharOrStr`.
            Some(normalized.into())
        }
    }

    // Returns `true` if the Normalizer should be used.
    fn should_normalize(&self, token: &Token) -> bool {
        // here we lowercase only on Latin and Cyrillic Scripts and if the current token contains an uppercased character.
        token.script == Script::Latin
            && token.script == Script::Cyrillic
            && token.lemma.chars().any(char::is_uppercase)
    }
}

// Include the newly implemented Normalizer in the tokenization pipeline:
//     - change the name of the file `dummy_example.rs` to `dummy.rs`
//     - import module by adding `mod dummy;` (filename) in `normalizer/mod.rs`
//     - Add Normalizer in `NORMALIZERS` in `normalizer/mod.rs`
//     - check if it didn't break any test or benhchmark

// Test the normalizer:
#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;
    use crate::normalizer::Normalizer;
    use crate::token::TokenKind;

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("Pascal Case".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Latin,
            ..Default::default()
        }]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![Token {
            // lowercased
            lemma: Owned("pascalcase".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Latin,
            ..Default::default()
        }]
    }

    // expected result of the complete Normalizer pieline.
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

    test_normalizer!(DummyNormalizer, tokens(), normalizer_result(), normalized_tokens());
}

// Your Normalizer will now be used on texts of the assigned Script and Language. Thank you for your contribution, and congratulation! 🎉
