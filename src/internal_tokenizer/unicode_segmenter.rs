use std::borrow::Cow;
use crate::{Token, TokenKind};
use super::InternalTokenizer;
use unicode_segmentation::{UWordBoundIndices, UnicodeSegmentation};
use super::TokenStream;

pub struct UnicodeSegmenter;
pub struct UnicodeSegmenterIterator<'a>(UWordBoundIndices<'a>);

impl<'a> Iterator for UnicodeSegmenterIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(index, word)| {
                Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(word),
                    byte_start: index,
                    byte_end: index + word.as_bytes().len(),
                }
            })
    }
}

impl InternalTokenizer for UnicodeSegmenter {
    fn tokenize<'a>(&self, s: &'a str) -> TokenStream<'a> {
        TokenStream {
            inner: Box::new(UnicodeSegmenterIterator(s.split_word_bound_indices()))
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
        let tokens = tokenizer.tokenize(orig);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
        
        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let tokens = tokenizer.tokenize(orig);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
    }
}
