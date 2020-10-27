use token::{Token, TokenType};

struct Tokenizer<'a> {
    inner: &'a str,
}

impl<'a> Tokenizer<'a> {

}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {}
}