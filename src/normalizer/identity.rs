use crate::Token;
use super::Normalizer;

pub struct IdentityNormalizer;

impl Normalizer for IdentityNormalizer {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a> {
        token
    }
}
