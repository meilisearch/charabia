use std::borrow::Cow;

use unicode_segmentation::UnicodeSegmentation;

use super::{TokenStream, Tokenizer};
use crate::processors::ProcessedText;
use crate::{Token, TokenKind};

pub struct UnicodeSegmenter;

impl Tokenizer for UnicodeSegmenter {
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a> {
        let stream = s.processed.as_ref().split_word_bound_indices().scan(
            0,
            |char_index, (byte_index, word)| {
                let index = *char_index;
                *char_index += word.chars().count();
                Some(Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(word),
                    byte_start: byte_index,
                    char_index: index,
                    byte_end: byte_index + word.len(),
                })
            },
        );

        TokenStream { inner: Box::new(stream) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_char_indices() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let positions = UnicodeSegmenter
            .tokenize(&processed)
            .map(|Token { char_index, .. }| char_index)
            .collect::<Vec<_>>();
        assert_eq!(
            positions,
            [
                0, 3, 4, 9, 10, 11, 12, 17, 18, 19, 20, 23, 24, 29, 30, 34, 35, 39, 40, 44, 45, 46,
                51, 52, 53, 56, 57, 58, 62, 63, 67, 68, 69
            ]
        );

        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let positions = UnicodeSegmenter
            .tokenize(&processed)
            .map(|Token { char_index, .. }| char_index)
            .collect::<Vec<_>>();
        assert_eq!(positions, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
    }

    #[test]
    fn test_simple() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = UnicodeSegmenter
            .tokenize(&processed)
            .map(|Token { word, .. }| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            tokens,
            [
                "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can\'t",
                " ", "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ", "Brr", ",",
                " ", "it\'s", " ", "29.3", "°", "F", "!"
            ]
        );

        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = UnicodeSegmenter
            .tokenize(&processed)
            .map(|Token { word, .. }| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            tokens,
            [
                "為", "一", "包", "含", "一", "千", "多", "萬", "目", "詞", "的", "帶", "標", "記",
                "平", "衡", "語", "料", "庫"
            ]
        );
    }

    #[test]
    fn test_byte_indices() {
        let tokenizer = UnicodeSegmenter;
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = tokenizer.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());

        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = tokenizer.tokenize(&processed).collect::<Vec<_>>();
        assert_eq!("為", tokens.first().unwrap().text());
        assert_eq!(
            orig,
            tokens.iter().map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>()
        );
    }
}
