use std::borrow::Cow;

use character_converter::CharacterConverter;
use once_cell::sync::Lazy;

use super::{PreProcessor, ProcessedText};

static CHARACTER_CONVERTER: Lazy<CharacterConverter> = Lazy::new(CharacterConverter::new);

pub struct ChineseTranslationPreProcessor;

impl PreProcessor for ChineseTranslationPreProcessor {
    fn process<'a>(&self, s: &'a str) -> ProcessedText<'a> {
        let processed = CHARACTER_CONVERTER.traditional_to_simplified(s);
        ProcessedText { processed: Cow::Owned(processed), original: s }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_traditional_to_simplified() {
        let preprocessor = ChineseTranslationPreProcessor;

        let traditional = "人人生而自由﹐在尊嚴和權利上一律平等。他們賦有理性和良心﹐並應以兄弟關係的精神互相對待。";
        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";

        let analyzed = preprocessor.process(&traditional);

        assert_eq!(simplified, &analyzed.processed);
    }

    #[test]
    fn test_simplified_to_simplified() {
        let preprocessor = ChineseTranslationPreProcessor;

        let simplified = "人人生而自由﹐在尊严和权利上一律平等。他们赋有理性和良心﹐并应以兄弟关系的精神互相对待。";

        let analyzed = preprocessor.process(&simplified);

        assert_eq!(simplified, &analyzed.processed);
    }
}
