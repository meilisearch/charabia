mod control_character_remover;
mod deunicoder;
mod identity;
mod lowercase;

pub use control_character_remover::ControlCharacterRemover;
pub use deunicoder::DeunicodeNormalizer;
pub use identity::IdentityNormalizer;
pub use lowercase::LowercaseNormalizer;

use crate::Token;

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
    use crate::detection::is_cjk;
    use crate::TokenKind;

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

        let composed_normalizer: &[Box<dyn Normalizer>] = &[
            Box::new(LowercaseNormalizer),
            Box::new(DeunicodeNormalizer::default()),
            Box::new(LowercaseNormalizer),
        ];
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

        let deunicoder = DeunicodeNormalizer::new(&|text: &str| {
            text.chars().next().map_or(false, |c| is_cjk(c))
        });

        let token_l = LowercaseNormalizer.normalize(token.clone());
        assert_eq!(token_l.word, "生而自由");

        let token_d = deunicoder.normalize(token.clone());
        assert_eq!(token_d.word, "生而自由");

        let composed_normalizer: &[Box<dyn Normalizer>] =
            &[Box::new(LowercaseNormalizer), Box::new(deunicoder), Box::new(LowercaseNormalizer)];
        let token_ld = composed_normalizer.normalize(token);
        assert_eq!(token_ld.word, "生而自由");
    }
}
