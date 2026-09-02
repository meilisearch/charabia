use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::{Script, Token};

/// Normalize Greek characters by:
/// 1. convert final sigma (`ς`) into ordinary sigma (`σ`)
///
pub struct GreekNormalizer;

impl Normalizer for GreekNormalizer {
    // converting  "ς" to "σ" doesn't change the characters length,
    // so the `normalize` method is overloaded to skip the useless char_map computing.
    fn normalize<'o>(&self, mut token: Token<'o>, _options: &NormalizerOption) -> Token<'o> {
        if token.lemma.contains('ς') {
            match token.lemma {
                Cow::Borrowed(lemma) => token.lemma = Cow::Owned(lemma.replace('ς', "σ")),
                Cow::Owned(mut lemma) => {
                    let mut start = 0;
                    while let Some(index) = lemma[start..].find('ς') {
                        start += index;
                        lemma.replace_range(start..start + 'ς'.len_utf8(), "σ");
                        start += 'σ'.len_utf8();
                    }
                    token.lemma = Cow::Owned(lemma);
                }
            }
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

    #[test]
    fn normalize_sigma_in_every_position() {
        let token = Token {
            lemma: Owned("ςοφόςς".to_string()),
            char_end: 6,
            byte_end: 12,
            script: Script::Greek,
            ..Default::default()
        };

        assert_eq!(GreekNormalizer.normalize(token, &TEST_NORMALIZER_OPTIONS).lemma(), "σοφόσσ");
    }

    #[test]
    fn normalize_final_sigma_followed_by_nonspacing_mark() {
        let marked = Token {
            lemma: Owned("κοσμος\u{0301}".to_string()),
            char_end: 7,
            byte_end: 14,
            script: Script::Greek,
            ..Default::default()
        };
        let unmarked = Token {
            lemma: Owned("κοσμος".to_string()),
            char_end: 6,
            byte_end: 12,
            script: Script::Greek,
            ..Default::default()
        };

        assert_eq!(
            marked.normalize(&TEST_NORMALIZER_OPTIONS).lemma(),
            unmarked.normalize(&TEST_NORMALIZER_OPTIONS).lemma()
        );
    }
}
