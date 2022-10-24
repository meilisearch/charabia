use std::borrow::Cow;

use super::{Normalizer, NormalizerOption};
use crate::detection::{Language, Script};
use crate::Token;

/// A global [`Normalizer`] lowercasing characters.
///
pub struct LowercaseNormalizer;

impl Normalizer for LowercaseNormalizer {
    // lowercasing characters doesn't change the characters length,
    // so the `normalize` method is overloaded to skip the useless char_map computing.
    fn normalize<'o>(&self, mut token: Token<'o>, _options: NormalizerOption) -> Token<'o> {
        if let Cow::Owned(lemma) = self.normalize_str(token.lemma()) {
            token.lemma = Cow::Owned(lemma);
        }

        token
    }

    fn normalize_str<'o>(&self, src: &'o str) -> Cow<'o, str> {
        if src.chars().any(char::is_uppercase) {
            Cow::Owned(src.to_lowercase())
        } else {
            Cow::Borrowed(src)
        }
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
