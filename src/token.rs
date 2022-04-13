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
    /// index of the first and the last character of the original word
    pub char_start: usize,
    pub char_end: usize,
    /// index of the first and the last byte of the original word
    pub byte_start: usize,
    pub byte_end: usize,
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
    /// Returns a reference over the normalized text.
    pub fn text(&self) -> &str {
        self.word.as_ref()
    }

    /// Returns the lenght in bytes of the normalized text.
    pub fn byte_len(&self) -> usize {
        self.word.len()
    }

    /// Returns the lenght in bytes of the original text.
    pub fn original_byte_len(&self) -> usize {
        self.byte_end - self.byte_start
    }

    /// Returns the count of characters of the normalized text.
    pub fn char_count(&self) -> usize {
        self.word.chars().count()
    }

    /// Returns the count of characters of the original text.
    pub fn original_char_count(&self) -> usize {
        self.char_end - self.char_start
    }

    /// Returns the [`TokenKind`] of the current token.
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// Returns true if the current token is a word.
    ///
    /// A token is considered as a word if it's not a separator nor a stop word.
    pub fn is_word(&self) -> bool {
        self.kind == TokenKind::Word
    }

    /// Returns true if the current token is a stop word.
    pub fn is_stopword(&self) -> bool {
        self.kind == TokenKind::StopWord
    }

    /// Returns true if the current token is a separator.
    pub fn is_separator(&self) -> bool {
        self.separator_kind().map_or(false, |_| true)
    }

    /// Returns Some([`SeparatorKind`]) if the token is a separator and None if it's a word or a stop word.
    pub fn separator_kind(&self) -> Option<SeparatorKind> {
        if let TokenKind::Separator(s) = self.kind {
            Some(s)
        } else {
            None
        }
    }

    /// Returns the number of characters and bytes in original token using number of bytes in normalized
    /// token.
    ///
    /// chars are counted in the pre-processed string (just before normalizing).
    /// For example, consider the string "Go💼od" which gets normalized to "gobriefcase od".
    /// `num_chars_from_bytes(11)` for this token will return `(3, 6)` - the number of `(characters, bytes)` in
    /// the original string for 11 bytes in the normalized string.
    ///
    /// If the `char_map` hasn't been initialized (it is None), usually done
    /// by the de-unicoder, it counts the number of `(characters, bytes)` in self.word
    /// for the given number of bytes. A char is considered even if the number
    /// of bytes only covers a portion of it.
    ///
    /// # Arguments
    ///
    /// * `num_bytes` - number of bytes in normalized token
    pub fn original_lenghts(&self, num_bytes: usize) -> (usize, usize) {
        match &self.char_map {
            None => {
                // if we don't have a char_map, we look for the number of chars in the current
                //   (probably normalized) string
                self.word
                    .char_indices()
                    .take_while(|(byte_index, _)| *byte_index < num_bytes)
                    .enumerate()
                    .last()
                    .map_or((0, 0), |(char_index, (byte_index, _))| (char_index, byte_index))
            }
            Some(char_map) => {
                let mut byte_index = 0;
                let char_index = char_map
                    .iter()
                    .take_while(|bytes_in_char| {
                        if byte_index < num_bytes {
                            byte_index += *bytes_in_char;
                            true
                        } else {
                            false
                        }
                    })
                    .count();
                (char_index, byte_index)
            }
        }
    }
}
