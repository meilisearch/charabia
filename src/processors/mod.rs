mod identity;

use std::borrow::Cow;

pub use identity::IdentityPreProcessor;

#[allow(dead_code)]
pub(crate) struct ProcessedText<'a> {
    pub(crate) processed: Cow<'a, str>,
    pub(crate) original: &'a str,
}

pub trait PreProcessor: Sync + Send {
    fn process<'a>(&self, s: &'a str) -> ProcessedText<'a>;
}
