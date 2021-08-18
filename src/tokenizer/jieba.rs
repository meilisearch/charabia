use std::borrow::Cow;

use jieba_rs::Jieba as JiebaTokenizer;
use once_cell::sync::Lazy;

use super::{TokenStream, Tokenizer};
use crate::processors::ProcessedText;
use crate::{Token, TokenKind};

static JIEBA: Lazy<JiebaTokenizer> = Lazy::new(|| JiebaTokenizer::new());

#[derive(Debug, Default)]
pub struct Jieba;

impl Tokenizer for Jieba {
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a> {
        let tokenized = JIEBA.tokenize(&s.processed, jieba_rs::TokenizeMode::Default, true);

        let original_byte_len = s.original.len();
        let mut original = s
            .original
            .char_indices()
            // map only byte indices
            .map(|(byte_index, _)| byte_index)
            // add ending byte index
            .chain(std::iter::once(original_byte_len));

        TokenStream {
            inner: Box::new(tokenized.into_iter().scan(0, move |byte_index, jieba_token| {
                let char_start = jieba_token.start;
                let char_end = jieba_token.end;
                let byte_start = *byte_index;

                // iter.nth(0) == iter.next(), so nth is computed as `char_end - char_start - 1`
                // but not for the first iteration where nth is computed as `char_end`
                let byte_end = match *byte_index {
                    0 => original.nth(char_end),
                    _ => original.nth(char_end - char_start - 1),
                };

                #[cfg(test)]
                let byte_end = byte_end.unwrap();

                #[cfg(not(test))]
                let byte_end = match byte_end {
                    Some(byte_end) => byte_end,
                    None => return None,
                };

                *byte_index = byte_end;

                Some(Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(jieba_token.word),
                    char_index: char_start,
                    byte_start,
                    byte_end,
                })
            })),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Jieba
            .tokenize(&processed)
            .map(|Token { word, .. }| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            tokens,
            [
                "The", " ", "quick", " ", "(", "\"", "brown", "\"", ")", " ", "fox", " ", "can",
                "\'", "t", " ", "jump", " ", "32.3", " ", "feet", ",", " ", "right", "?", " ",
                "Brr", ",", " ", "it", "\'", "s", " ", "29.3", "°", "F", "!"
            ]
        );

        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Jieba
            .tokenize(&processed)
            .map(|Token { word, .. }| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            tokens,
            ["為", "一", "包含", "一千多", "萬目", "詞", "的", "帶", "標記", "平衡", "語料", "庫"]
        );
    }

    #[test]
    fn test_byte_positions() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Jieba.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());

        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Jieba.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
    }

    #[test]
    fn test_char_indices() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let positions = Jieba
            .tokenize(&processed)
            .map(|Token { char_index, .. }| char_index)
            .collect::<Vec<_>>();
        assert_eq!(
            positions,
            [
                0, 3, 4, 9, 10, 11, 12, 17, 18, 19, 20, 23, 24, 27, 28, 29, 30, 34, 35, 39, 40, 44,
                45, 46, 51, 52, 53, 56, 57, 58, 60, 61, 62, 63, 67, 68, 69
            ]
        );

        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let positions = Jieba
            .tokenize(&processed)
            .map(|Token { char_index, .. }| char_index)
            .collect::<Vec<_>>();
        assert_eq!(positions, [0, 1, 2, 4, 7, 9, 10, 11, 12, 14, 16, 18]);
    }
}
