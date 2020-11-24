use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SeparatorKind {
    Hard,
    Soft,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Word,
    /// the token is a stop word,
    /// meaning that it can be ignored to optimize size and performance or be indexed as a Word
    StopWord,
    /// the token is a separator,
    /// meaning that it shouldn't be indexed but used to determine word proximity
    Separator(SeparatorKind),
    Any,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::Any
    }
}

/// script of a token (https://docs.rs/whatlang/0.10.0/whatlang/enum.Script.html)
#[derive(Debug, Clone, Default)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub word: Cow<'a, str>,
    /// index of the first character of the word
    pub char_index: usize,
    /// indexes of start and end of the byte slice
    pub byte_start: usize,
    pub byte_end: usize,
}

impl<'a> Token<'a> {
    pub fn text(&self) -> &str {
        self.word.as_ref()
    }

    pub fn byte_len(&self) -> usize {
        self.byte_end - self.byte_start
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }
    pub fn is_word(&self) -> bool {
        self.kind == TokenKind::Word
    }
    pub fn is_separator(&self) -> Option<SeparatorKind> {
        if let TokenKind::Separator(s) = self.kind {
            Some(s)
        } else {
            None
        }
    }
    pub fn is_stopword(&self) -> bool {
        self.kind == TokenKind::StopWord
    }
}
