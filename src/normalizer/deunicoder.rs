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
            // required to convert char to &str
            // ref: https://stackoverflow.com/a/47634755/11199009
            let mut tmp = [0; 4];

            // find length (bytes) of deunicoded str for each char
            let char_map = token
                .word
                .chars()
                .map(|char| deunicode(char.encode_utf8(&mut tmp)).len())
                .collect();

            let deunicoded = deunicode(token.word.as_ref());
            token.word = Cow::Owned(deunicoded);
            token.char_map = Some(char_map);
        }

        token
    }
}
