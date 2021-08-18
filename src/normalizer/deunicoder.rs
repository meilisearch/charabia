use std::borrow::Cow;

use deunicode::deunicode;

use super::Normalizer;
use crate::Token;

type SkipNormalizationFn = &'static (dyn Fn(&str) -> bool + Sync + Send + 'static);

pub struct DeunicodeNormalizer {
    skip_normalization: SkipNormalizationFn,
}

impl DeunicodeNormalizer {
    pub fn new(skip_normalization: SkipNormalizationFn) -> Self {
        Self { skip_normalization }
    }
}

impl Default for DeunicodeNormalizer {
    fn default() -> Self {
        Self::new(&|_| false)
    }
}

impl Normalizer for DeunicodeNormalizer {
    fn normalize<'a>(&self, mut token: Token<'a>) -> Token<'a> {
        if !(self.skip_normalization)(&token.word) {
            token.word = Cow::Owned(deunicode(token.word.as_ref()));
        }

        token
    }
}
