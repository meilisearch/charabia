mod unicode_segmenter;

pub use unicode_segmenter::UnicodeSegmenter;

use crate::Token;

pub struct TokenStream<'a> {
    pub(crate) inner: Box<dyn Iterator<Item = Token<'a>> + 'a>
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
pub trait InternalTokenizer: Sync + Send {
    /// create the tokenizer based on the given `text` and `char_index`
    fn tokenize<'a>(&self, s: &'a str) -> TokenStream<'a>;
}
