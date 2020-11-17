mod unicode_segmenter;

use crate::Token;

/// trait defining an internal tokenizer,
/// an internal tokenizer should be a script specialized tokenizer,
/// this should be implemented as an `Iterator` with `Token` as `Item`,
pub(crate) trait InternalTokenizer<'a> {
    type Output: Iterator<Item = Token<'a>>;
    /// create the tokenizer based on the given `text` and `char_index`
    /// [ERR]: this trait cannot be made into an object because associated function `new` has no `self` parameter the trait `internal_tokenizer::InternalTokenizer` cannot be made into an object
    fn tokenize(&self, s: &'a str) -> Self::Output;
}
