use std::borrow::Cow;

// Import `Normalizer` trait.
use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

// Make a small documentation of the specialized Normalizer like below.
/// <Script/Language> specialized [`Normalizer`].
///
/// This Normalizer uses [`<UsedLibraryToNormalize>`] internally to normalize the provided token.
/// <OptionalAdditionnalExplanations>
pub struct DummyNormalizer;

// All normalizers only need to implement the method `normalize` and the method `should_normalize` of the `Normalizer` trait.
impl Normalizer for DummyNormalizer {
    // Creates an iterator over the normalized version of the provided token.
    fn normalize<'o>(&self, mut token: Token<'o>, options: NormalizerOption) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        // lowercase the provided token lemma.
        token.lemma = Cow::Owned(token.lemma().to_lowercase());

        // Create an iterator over the normalized token.
        Box::new(Some(token).into_iter())
    }

    // Returns `true` if the Normalizer should be used.
    fn should_normalize(&self, script: Script, _language: Option<Language>) -> bool {
        // here we lowercase only on Latin and Cyrillic Scripts.
        script == Script::Latin && script == Script::Cyrillic
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

    // base tokens to normalize.
    fn tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("PascalCase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ПаскальКейс".to_string()),
                char_end: 11,
                byte_end: 22,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                // lowercased
                lemma: Owned("pascalcase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                // lowercased
                lemma: Owned("паскалькейс".to_string()),
                char_end: 11,
                byte_end: 22,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("pascalcase".to_string()),
                char_end: 10,
                byte_end: 10,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("paskal'keis".to_string()),
                char_end: 11,
                byte_end: 22,
                script: Script::Latin,
                ..Default::default()
            },
        ]
    }

    test_normalizer!(DummyNormalizer, tokens(), normalizer_result(), normalized_tokens());
}

// Your Normalizer will now be used on texts of the assigned Script and Language. Thank you for your contribution, and congratulation! 🎉
