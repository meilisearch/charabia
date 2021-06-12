mod jieba;
mod lindera;
mod unicode_segmenter;
mod legacy_meilisearch;

pub use jieba::Jieba;
// pub use lindera::Lindera;
pub use unicode_segmenter::UnicodeSegmenter;
pub use legacy_meilisearch::LegacyMeilisearch;

use crate::Token;
use crate::processors::ProcessedText;

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
pub trait Tokenizer: Sync + Send {
    /// create the tokenizer based on the given `text` and `char_index`
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a>;
}
