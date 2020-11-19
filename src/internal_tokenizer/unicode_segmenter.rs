use std::borrow::Cow;

use unicode_segmentation::{UWordBoundIndices, UnicodeSegmentation};

use crate::{Token, TokenKind};
use crate::processors::ProcessedText;
use super::InternalTokenizer;
use super::TokenStream;

pub struct UnicodeSegmenter;
pub struct UnicodeSegmenterIterator<'a>(UWordBoundIndices<'a>, usize);

impl<'a> Iterator for UnicodeSegmenterIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(index, word)| {
                let char_index = self.1;
                self.1 += word.chars().count();
                Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(word),
                    byte_start: index,
                    char_index,
                    byte_end: index + word.as_bytes().len(),
                }
            })
    }
}

impl InternalTokenizer for UnicodeSegmenter {
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a> {
        TokenStream {
            inner: Box::new(UnicodeSegmenterIterator(s.processed.as_ref().split_word_bound_indices(), 0))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let tokenizer = UnicodeSegmenter;
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let tokens = tokenizer.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
        
        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let tokens = tokenizer.tokenize(&processed).collect::<Vec<_>>();
        assert_eq!("為", tokens.first().unwrap().text());
        assert_eq!(orig, tokens.iter().map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
    }
}
