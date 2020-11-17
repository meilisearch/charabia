mod identity;

use std::borrow::Cow;

pub use identity::IdentityPreProcessor;

pub trait PreProcessor: Sync + Send {
    fn process<'a>(&self, s: &'a str) -> Cow<'a, str>;
}
