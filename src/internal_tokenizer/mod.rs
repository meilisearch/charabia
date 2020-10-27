use crate::token::{Token, WordSlice, Script};

/// trait defining an internal tokenizer,
/// an internal tokenizer should be a script specialized tokenizer,
/// this should be implemented as an `Iterator` with `Token` as `Item`,
pub(crate) trait InternalTokenizer<'a>: Iterator<Item = Token<'a>> {
    /// create the tokenizer based on the given `text` and `char_index`
    /// [ERR]: this trait cannot be made into an object because associated function `new` has no `self` parameter the trait `internal_tokenizer::InternalTokenizer` cannot be made into an object
    fn new(text: &'a str, char_index: usize) -> Self;
    /// return the `char_index` of the next potential token,
    /// it should be used when switching internal tokenizer
    /// to retrieve the tokenization state
    fn char_index(&self) -> usize;
}

/// return a box of the specialized internal tokenizer for the given `Script`,
/// calling the method new of the chosen internal tokenizer
pub(crate) fn from_script<'a>(script: Script, inner: &'a str, char_index: usize) -> Option<Box<dyn InternalTokenizer<'a>>> { unimplemented!() }
