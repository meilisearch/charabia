use super::Normalizer;
use crate::Token;

pub struct IdentityNormalizer;

impl Normalizer for IdentityNormalizer {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a> {
        token
    }
}
