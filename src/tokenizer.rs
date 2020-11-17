use crate::token::Token;

pub struct TokenizerConfig;

struct Tokenizer<'a> {
    /// script specialized tokenizer, this can be switched during
    /// document tokenization if the document contains several scripts
    current_tokenizer: Option<Box<dyn Iterator<Item = Token<'a>>>>,
    /// current character index in the document
    current_char_index: u64,
    /// reference on the document content
    inner: &'a str,
}

impl<'a> Tokenizer<'a> {
    /// create a new tokenizer detecting script
    /// and chose the specialized internal tokenizer
    pub fn new(_inner: &'a str) -> Self { unimplemented!() }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    /// return the next token calling the `next` method of the internal tokenizer,
    /// if the internal tokeizer return None, the function try to redetect script and chose a new tokenizer,
    /// if no iternal tokenizer is chosen, the method return None
    fn next(&mut self) -> Option<Self::Item> { unimplemented!() }
}
