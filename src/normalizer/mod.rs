mod identity;
mod lowercase;

use crate::Token;

pub use identity::IdentityNormalizer;
pub use lowercase::LowercaseNormalizer;

pub trait Normalizer: Sync + Send {
    fn normalize<'a>(&self, token: Token<'a>) -> Token<'a>;
}
