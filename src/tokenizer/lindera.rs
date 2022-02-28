use std::borrow::Cow;

use lindera::tokenizer::Tokenizer as LinderaTokenizer;
use once_cell::sync::Lazy;

use super::{TokenStream, Tokenizer};
use crate::processors::ProcessedText;
use crate::{Token, TokenKind};

static LINDERA: Lazy<LinderaTokenizer> = Lazy::new(|| LinderaTokenizer::new().unwrap());

#[derive(Debug, Default)]
pub struct Lindera;

impl Tokenizer for Lindera {
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a> {
        let tokens = LINDERA.tokenize(&s.processed).unwrap();

        TokenStream {
            inner: Box::new(tokens.into_iter().scan(
                (0, 0),
                move |(char_index, byte_index), lindera_token| {
                    let char_count = lindera_token.text.chars().count();
                    let char_start = *char_index;
                    *char_index += char_count;

                    let byte_count = lindera_token.text.len();
                    let byte_start = *byte_index;
                    *byte_index += byte_count;

                    Some(Token {
                        kind: TokenKind::Word,
                        word: Cow::Borrowed(lindera_token.text),
                        char_index: char_start,
                        byte_start,
                        byte_end: *byte_index,
                        char_map: None,
                    })
                },
            )),
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
        let tokens = Lindera
            .tokenize(&processed)
            .map(|Token { word, .. }| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            tokens,
            [
                "The", " ", "quick", " ", "(\"", "brown", "\")", " ", "fox", " ", "can", "\'", "t",
                " ", "jump", " ", "32", ".", "3", " ", "feet", ",", " ", "right", "?", " ", "Brr",
                ",", " ", "it", "\'", "s", " ", "29", ".", "3", "°", "F", "!"
            ]
        );

        let orig = "関西国際空港限定トートバッグ";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Lindera
            .tokenize(&processed)
            .map(|Token { word, .. }| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(tokens, ["関西国際空港", "限定", "トートバッグ"]);

        let orig = "すもももももももものうち";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Lindera
            .tokenize(&processed)
            .map(|Token { word, .. }| word.to_owned())
            .collect::<Vec<_>>();
        assert_eq!(tokens, ["すもも", "も", "もも", "も", "もも", "の", "うち"]);
    }
    #[test]
    fn test_byte_positions() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Lindera.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());

        let orig = "関西国際空港限定トートバッグ";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Lindera.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());

        let orig = "すもももももももものうち";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let tokens = Lindera.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
    }

    #[test]
    fn test_char_indices() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let positions = Lindera
            .tokenize(&processed)
            .map(|Token { char_index, .. }| char_index)
            .collect::<Vec<_>>();
        assert_eq!(
            positions,
            [
                0, 3, 4, 9, 10, 12, 17, 19, 20, 23, 24, 27, 28, 29, 30, 34, 35, 37, 38, 39, 40, 44,
                45, 46, 51, 52, 53, 56, 57, 58, 60, 61, 62, 63, 65, 66, 67, 68, 69
            ]
        );

        let orig = "関西国際空港限定トートバッグ";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let positions = Lindera
            .tokenize(&processed)
            .map(|Token { char_index, .. }| char_index)
            .collect::<Vec<_>>();
        assert_eq!(positions, [0, 6, 8]);

        let orig = "すもももももももものうち";
        let processed = ProcessedText { original: orig, processed: Cow::Borrowed(orig) };
        let positions = Lindera
            .tokenize(&processed)
            .map(|Token { char_index, .. }| char_index)
            .collect::<Vec<_>>();
        assert_eq!(positions, [0, 3, 4, 6, 7, 9, 10]);
    }
}
