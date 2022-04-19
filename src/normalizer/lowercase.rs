use std::borrow::Cow;

use cow_utils::CowUtils;

use super::Normalizer;
use crate::detection::{Language, Script};
use crate::Token;

pub struct LowercaseNormalizer;

impl Normalizer for LowercaseNormalizer {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Box<dyn Iterator<Item = Token<'a>> + 'a> {
        // Cow::Borrowed holds a reference to token, which makes it impossible to directly replace
        // word with the `cow_to_lowercase` result
        if let Cow::Owned(s) = token.word.cow_to_lowercase() {
            token.word = Cow::Owned(s);
        }

        Box::new(Some(token).into_iter())
    }

    fn should_normalize(&self, _script: Script, _language: Option<Language>) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow::Owned;

    use crate::normalizer::test::test_normalizer;

    fn tokens() -> Vec<Token<'static>> {
        vec![Token {
            word: Owned("PascalCase".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Latin,
            ..Default::default()
        }]
    }

    fn normalized_tokens() -> Vec<Token<'static>> {
        vec![Token {
            word: Owned("pascalcase".to_string()),
            char_end: 10,
            byte_end: 10,
            script: Script::Latin,
            ..Default::default()
        }]
    }

    test_normalizer!(LowercaseNormalizer, tokens(), normalized_tokens(), normalized_tokens());
}
