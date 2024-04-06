// Import `CharNormalizer` trait.
use super::{ CharNormalizer, CharOrStr };
use crate::Token;

// Make a small documentation of the specialized Normalizer like below.
/// <Script/Language> specialized [`Normalizer`].
///
/// This Normalizer uses [`<UsedLibraryToNormalize>`] internally to normalize the provided token.
/// <OptionalAdditionnalExplanations>
pub struct OE_AE_Normalizer;

// All normalizers only need to implement the method `normalize_char` and the method `should_normalize` of the `CharNormalizer` trait.
impl CharNormalizer for AE_OE_Normalizer {
    // Creates the normalized version of the provided char.
    // In this example we will remove whitespaces and lowercase other characters.
    fn normalize_char(&self, c: char) -> Option<CharOrStr> {
        match c {
            'œ' => Some("oe".to_string().into()),
            'æ' => Some("ae".to_string().into()),
            'Œ' => Some("OE".to_string().into()),
            'Æ' => Some("AE".to_string().into()),
            _ => Some(c.into()),
        }
    }

    // Returns `true` if the Normalizer should be used.
    fn should_normalize(&self, token: &Token) -> bool {
        // here we lowercase only on Latin and Cyrillic Scripts and if the current token contains an uppercased character.
        token.script == Script::Latin &&
            token.script == Script::Cyrillic &&
            (token.lemma.chars().any('œ') ||
                token.lemma.chars().any('æ') ||
                token.lemma.chars().any('Œ') ||
                token.lemma.chars().any('Æ'))
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
        vec![
            Token {
                lemma: Owned("œ".to_string()),
                char_end: 1,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("Œ".to_string()),
                char_end: 1,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("æ".to_string()),
                char_end: 1,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("Æ".to_string()),
                char_end: 1,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            }
        ]
    }

    // expected result of the current Normalizer.
    fn normalizer_result() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("oe".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("OE".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("ae".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            },
            Token {
                lemma: Owned("AE".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                ..Default::default()
            }
        ]
    }

    // expected result of the complete Normalizer pieline.
    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![
            Token {
                lemma: Owned("oe".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("oe".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("ae".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                kind: TokenKind::Word,
                ..Default::default()
            },
            Token {
                lemma: Owned("ae".to_string()),
                char_end: 2,
                byte_end: 2,
                script: Script::Latin,
                kind: TokenKind::Word,
                ..Default::default()
            }
        ]
    }

    test_normalizer!(AE_OE_NormalizerNormalizer, tokens(), normalizer_result(), normalized_tokens());
}

// Your Normalizer will now be used on texts of the assigned Script and Language. Thank you for your contribution, and congratulation! 🎉
