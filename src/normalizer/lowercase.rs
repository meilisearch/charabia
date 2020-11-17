use std::borrow::Cow;

use super::Normalizer;
use crate::Token;

pub struct LowercaseNormalizer;

impl Normalizer for LowercaseNormalizer {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Token<'a> {
        // TODO: use cow_lowercase for better performance
        token.word = Cow::Owned(token.word.to_lowercase());
        token
    }
}
