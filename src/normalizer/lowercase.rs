use std::borrow::Cow;

use cow_utils::CowUtils;

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

/// A global [`Normalizer`] lowercasing characters.
///
pub struct LowercaseNormalizer;

impl Normalizer for LowercaseNormalizer {
    fn normalize<'o>(
        &self,
        mut token: Token<'o>,
        _options: NormalizerOption,
    ) -> Box<dyn Iterator<Item = Token<'o>> + 'o> {
        // Cow::Borrowed holds a reference to token, which makes it impossible to directly replace
        // word with the `cow_to_lowercase` result
        if let Cow::Owned(s) = token.lemma.cow_to_lowercase() {
            token.lemma = Cow::Owned(s);
        }

        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, script: Script, _language: Option<Language>) -> bool {
        // https://en.wikipedia.org/wiki/Letter_case#Capitalisation
        matches!(script, Script::Latin | Script::Cyrillic | Script::Greek | Script::Georgian)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;

    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            lemma: Owned("PascalCase".to_string()),
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
            ..Default::default()
        }]
    }

    test_normalizer!(LowercaseNormalizer, tokens(), normalized_tokens(), normalized_tokens());
}
