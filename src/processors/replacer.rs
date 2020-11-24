use cow_utils::CowUtils;

use super::{ProcessedText, PreProcessor};

pub struct Replacer<F: 'static>(&'static F , String);

impl<F> Replacer<F> 
where
    F: Fn(char) -> bool + 'static + Send + Sync
{
    pub fn new(replace_fn: &'static F, with: char) -> Self {
        Self(replace_fn, with.to_string())
    }
}

impl<F> PreProcessor for Replacer<F>
where
    F: Fn(char) -> bool + 'static + Send + Sync
{
    fn process<'a>(&self, s: &'a str) -> ProcessedText<'a> {
        ProcessedText {
            original: s,
            processed: s.cow_replace(&self.0, &self.1)
        }
    }
}
