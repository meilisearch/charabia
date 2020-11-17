use std::borrow::Cow;

use super::PreProcessor;

pub struct IdentityPreProcessor;

impl PreProcessor for IdentityPreProcessor {
    fn process<'a>(&self, s: &'a str) -> Cow<'a, str> {
        Cow::Borrowed(s)
    }
}
