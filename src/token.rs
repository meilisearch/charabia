use std::borrow::Cow;

use crate::detection::{Language, Script};

/// Define the kind of a [`TokenKind::Separator`].
///
/// A separator has two kinds:
/// - `Hard`: Separate two tokens that are not in the same context (different phrases).
/// - `Soft`: Separate two tokens that are in the same context (same phrase).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorKind {
    Hard,
    Soft,
}

/// Define the kind of a [`Token`].
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

#[derive(Debug, Clone, Default)]
pub struct Token<'a> {
    /// kind of the Token assigned by the classifier
    pub kind: TokenKind,
    pub word: Cow<'a, str>,
    /// index of the first character of the word
    pub char_index: usize,
    /// index of the first byte of the byte slice
    pub byte_index: usize,
    /// number of bytes used in the normalized string
    /// by each char in the original string
    pub char_map: Option<Vec<usize>>,
    /// script of the Token
    pub script: Script,
    /// language of the Token
    pub language: Option<Language>,
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
        self.word.len()
    }

    pub fn byte_end(&self) -> usize {
        self.byte_index + self.byte_len()
    }

    pub fn char_count(&self) -> usize {
        self.word.chars().count()
    }

    pub fn char_end(&self) -> usize {
        self.char_index + self.char_count()
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
    /// If the `char_map` hasn't been initialized (it is None), usually done
    /// by the de-unicoder, it counts the number of characters in self.word
    /// for the given number of bytes. A char is considered even if the number
    /// of bytes only covers a portion of it.
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
                    .take_while(|(char_index, _)| *char_index < num_bytes)
                    .count()
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
