/// script of a token (https://docs.rs/whatlang/0.10.0/whatlang/enum.Script.html)
pub type Script = whatlang::Script;

/// atomic item returned by `Tokenizer::next()`,
/// it determine the type of the token and contains a `WordSlice`
pub enum Token<'a> {
    /// the token is a word,
    /// meaning that it should be indexed as an important part of the document
    Word(WordSlice<'a>),
    /// the token is a stop word,
    /// meaning that it can be ignored to optimize size and performance or be indexed as a Word
    StopWord(WordSlice<'a>),
    /// the token is a separator,
    /// meaning that it shouln't be indexed but used to determine word proximity
    Separator(WordSlice<'a>)
}

/// The script, the char_index and the content of the token
pub struct WordSlice<'a> {
    /// content of the token
    pub word: &'a str,
    /// index of the first character of the token in the whole document
    pub char_index: usize,
    /// script of the token (https://docs.rs/whatlang/0.10.0/whatlang/enum.Script.html)
    pub script: Script,
}
