use std::borrow::Cow;

use deunicode::deunicode;
use unicode_segmentation::UnicodeSegmentation;

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
            let mut char_map = Vec::new();
            for grapheme in token.word.graphemes(true) {
                char_map.push(deunicode(grapheme).len());
            }
            let deunicoded = deunicode(token.word.as_ref());
            token.word = Cow::Owned(deunicoded);
            token.char_map = Some(char_map);
        }

        token
    }
}
