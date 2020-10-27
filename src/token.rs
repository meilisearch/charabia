use whatlang::Script;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TokenType {
    Word { stop_word: bool },
    Separator,
}

impl TokenType {
    fn is_stop_word(&self) -> bool { 
        if self == Self::Word { stop_word: true } {
            true
        } else {
            false
        }
    }

    fn is_separator(&self) -> bool {
        if self == Self::Separator {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub word: &'a str,
    /// index of the token in the token sequence
    pub index: usize,
    pub word_index: usize,
    pub char_index: usize,
    token_type: TokenType,
    script: Script,
}

impl Token {
    pub fn is_stop_word(&self) -> bool { 
        self.token_type.is_stop_word()
    }

    pub fn is_separator(&self) -> bool {
        self.token_type.is_separator()
    }
}
