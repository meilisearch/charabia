use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Unknown,
}

impl Default for TokenKind {
    fn default() -> Self {
        Self::Unknown
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
    /// number of bytes used in the normalized string
    ///  by each char in the original string
    pub char_map: Option<Vec<usize>>,
}

impl<'a> PartialEq for Token<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl<'a> Eq for Token<'a> {}

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

    /// Returns the number of chars in original token using number of bytes in normalized
    /// token.
    ///
    /// chars are counted in the pre-processed string (just before normalizing).
    /// For example, consider the string "Go💼od" which gets normalized to "gobriefcase od".
    /// `num_chars_from_bytes(11)` for this token will return `3` - the number of characters in
    /// the original string for 11 bytes in the normalized string.
    ///
    /// # Arguments
    ///
    /// * `num_bytes` - number of bytes in normalized token
    pub fn num_chars_from_bytes(&self, mut num_bytes: usize) -> usize {
        match &self.char_map {
            None => {
                // if we don't have a char_map, we look for the number of chars in the current
                //   (probably normalized) string
                self.word
                    .char_indices()
                    .take_while(|(byte_index, _)| *byte_index <= num_bytes)
                    .count()
                    - 1
            }
            Some(char_map) => char_map
                .iter()
                .cloned()
                .take_while(|bytes_in_char| {
                    let prev = num_bytes;
                    num_bytes = num_bytes.saturating_sub(*bytes_in_char);
                    prev > 0
                })
                .count(),
        }
    }
}
