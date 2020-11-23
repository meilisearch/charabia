use std::borrow::Cow;

use character_converter::CharacterConverter;
use once_cell::sync::Lazy;

use super::{PreProcessor, ProcessedText};

static CHARACTER_CONVERTER: Lazy<CharacterConverter> = Lazy::new(|| CharacterConverter::new());

pub struct ChineseTranslationPreProcessor;

impl PreProcessor for ChineseTranslationPreProcessor {
    fn process<'a>(&self, s: &'a str) -> ProcessedText<'a> {
        let processed = CHARACTER_CONVERTER.traditional_to_simplified(s);
        ProcessedText {
            processed: Cow::Owned(processed),
            original: s,
        }
    }
}
