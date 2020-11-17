mod unicode_segmenter;

use crate::Token;

pub struct TokenStream<'a> {
    inner: Box<dyn Iterator<Item = Token<'a>> + 'a>
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// trait defining an internal tokenizer,
/// an internal tokenizer should be a script specialized tokenizer,
/// this should be implemented as an `Iterator` with `Token` as `Item`,
pub trait InternalTokenizer {
    /// create the tokenizer based on the given `text` and `char_index`
    fn tokenize<'a>(&self, s: &'a str) -> TokenStream<'a>;
}
