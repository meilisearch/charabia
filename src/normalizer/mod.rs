mod identity;
mod lowercase;
mod deunicoder;
mod token_classifier;

use crate::Token;

pub use identity::IdentityNormalizer;
pub use lowercase::LowercaseNormalizer;
pub use deunicoder::DeunicodeNormalizer;
pub use token_classifier::TokenClassifier;

pub trait Normalizer: Sync + Send {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a>;
}

impl Normalizer for &[Box<dyn Normalizer>] {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a> {
        self.iter().fold(token, |token, normalizer| normalizer.normalize(token))
    }
}

impl Normalizer for Vec<Box<dyn Normalizer>> {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a> {
        (&self[..]).normalize(token)
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use super::*;
    use crate::TokenKind;
    use crate::detection::is_cjk;

    #[test]
    fn test_compose_normalizer() {
        let token = Token {
            word: Cow::Borrowed("Æneid"),
            char_index: 0,
            kind: TokenKind::Word,
            byte_start: 0,
            byte_end: 0,
        };

        let token_l = LowercaseNormalizer.normalize(token.clone());
        assert_eq!(token_l.word, "æneid");

        let token_d = DeunicodeNormalizer::default().normalize(token.clone());
        assert_eq!(token_d.word, "AEneid");

        let composed_normalizer: &[Box<dyn Normalizer>] = &[Box::new(LowercaseNormalizer), Box::new(DeunicodeNormalizer::default()), Box::new(LowercaseNormalizer)];
        let token_ld = composed_normalizer.normalize(token);
        assert_eq!(token_ld.word, "aeneid");

    }

    #[test]
    fn test_compose_normalizer_chinese() {
        let token = Token {
            word: Cow::Borrowed("生而自由"),
            char_index: 0,
            kind: TokenKind::Word,
            byte_start: 0,
            byte_end: 0,
        };

        let deunicoder = DeunicodeNormalizer::new(&|text: &str| text.chars().next().map_or(false, |c| is_cjk(c)));

        let token_l = LowercaseNormalizer.normalize(token.clone());
        assert_eq!(token_l.word, "生而自由");

        let token_d = deunicoder.normalize(token.clone());
        assert_eq!(token_d.word, "生而自由");

        let composed_normalizer: &[&dyn Normalizer] = &[&LowercaseNormalizer, &Box::new(deunicoder), &Arc::new(LowercaseNormalizer)];
        let token_ld = composed_normalizer.normalize(token);
        assert_eq!(token_ld.word, "生而自由");

    }
}
