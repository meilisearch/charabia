use std::borrow::Cow;

use super::{PreProcessor, ProcessedText};

pub struct IdentityPreProcessor;

impl PreProcessor for IdentityPreProcessor {
    fn process<'a>(&self, s: &'a str) -> ProcessedText<'a> {
        ProcessedText { processed: Cow::Borrowed(s), original: s }
    }
}
