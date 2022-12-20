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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Token<'o> {
    /// kind of the Token assigned by the classifier
    pub kind: TokenKind,
    pub lemma: Cow<'o, str>,
    /// index of the first and the last character of the original lemma
    pub char_start: usize,
    pub char_end: usize,
    /// index of the first and the last byte of the original lemma
    pub byte_start: usize,
    pub byte_end: usize,
    /// number of bytes used in the original string mapped to the number of bytes used in the normalized string by each char in the original string.
    /// The char_map must be the same length as the number of chars in the original lemma.
    pub char_map: Option<Vec<(u8, u8)>>,
    /// script of the Token
    pub script: Script,
    /// language of the Token
    pub language: Option<Language>,
}

impl Token<'_> {
    /// Returns a reference over the normalized lemma.
    pub fn lemma(&self) -> &str {
        self.lemma.as_ref()
    }

    /// Returns the length in bytes of the normalized lemma.
    pub fn byte_len(&self) -> usize {
        self.lemma.len()
    }

    /// Returns the length in bytes of the original lemma.
    pub fn original_byte_len(&self) -> usize {
        self.byte_end - self.byte_start
    }

    /// Returns the count of characters of the normalized lemma.
    pub fn char_count(&self) -> usize {
        self.lemma.chars().count()
    }

    /// Returns the count of characters of the original lemma.
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
    /// For example, consider the string "léopard" which gets normalized to "leopard".
    /// `original_lengths(3)` for this token will return `(3, 4)` - the number of `(characters, bytes)` in
    /// the original string for 3 bytes in the normalized string.
    ///
    /// If the `char_map` hasn't been initialized (it is None), usually done
    /// by the de-unicoder, it counts the number of `(characters, bytes)` in self.lemma
    /// for the given number of bytes. A char is considered even if the number
    /// of bytes only covers a portion of it.
    ///
    /// # Arguments
    ///
    /// * `num_bytes` - number of bytes in normalized token
    pub fn original_lengths(&self, num_bytes: usize) -> (usize, usize) {
        match &self.char_map {
            None => {
                // if we don't have a char_map, we look for the number of chars in the current
                //   (probably normalized) string
                self.lemma
                    .char_indices()
                    .take_while(|(byte_index, _)| *byte_index < num_bytes)
                    .enumerate()
                    .last()
                    .map_or((0, 0), |(char_index, (byte_index, c))| {
                        let char_count = char_index + 1;
                        let byte_len = byte_index + c.len_utf8();
                        (char_count, byte_len)
                    })
            }
            Some(char_map) => {
                let mut normalized_byte_len = 0;
                let mut original_byte_len = 0;
                let char_count = char_map
                    .iter()
                    .take_while(|(original_bytes_in_char, normalized_bytes_in_char)| {
                        if normalized_byte_len < num_bytes {
                            original_byte_len += *original_bytes_in_char as usize;
                            normalized_byte_len += *normalized_bytes_in_char as usize;
                            true
                        } else {
                            false
                        }
                    })
                    .count();
                (char_count, original_byte_len)
            }
        }
    }
}
