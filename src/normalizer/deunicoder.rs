use std::borrow::Cow;

use deunicode::deunicode;

use super::Normalizer;
use crate::Token;

pub struct DeunicodeNormalizer;

impl Normalizer for DeunicodeNormalizer {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Token<'a> {
        token.word = Cow::Owned(deunicode(token.word.as_ref()));
        token
    }
}
