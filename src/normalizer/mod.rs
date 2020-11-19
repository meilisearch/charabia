mod identity;
mod lowercase;
mod deunicoder;

use crate::Token;

pub use identity::IdentityNormalizer;
pub use lowercase::LowercaseNormalizer;
pub use deunicoder::DeunicodeNormalizer;

pub trait Normalizer: Sync + Send {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a>;
}

impl Normalizer for &[Box<dyn Normalizer>] {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a> {
        self.iter().fold(token, |token, normalizer| normalizer.normalize(token))
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;
    use super::*;
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

        let token_d = DeunicodeNormalizer.normalize(token.clone());
        assert_eq!(token_d.word, "AEneid");
        
        let composed_normalizer: &[Box<dyn Normalizer>] = &[Box::new(LowercaseNormalizer), Box::new(DeunicodeNormalizer)];
        let token_ld = composed_normalizer.normalize(token);
        assert_eq!(token_ld.word, "aeneid");

    }
}
