use std::borrow::Cow;

use lindera::tokenizer::Tokenizer as LinderaTokenizer;
use lindera_core::core::viterbi::Mode;
use once_cell::sync::Lazy;

use crate::{Token, TokenKind};
use crate::processors::ProcessedText;
use super::{Tokenizer, TokenStream};


#[derive(Debug, Default)]
pub struct Lindera;

impl Tokenizer for Lindera {
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a> {
        let mut tokenizer = LinderaTokenizer::new(Mode::Normal, "");
        let tokenized = tokenizer.tokenize(&s.processed);
        TokenStream {
            inner: Box::new(tokenized.into_iter().scan(0, move |_, lindera_token| {
                let char_start = 0;
                let char_end = lindera_token.text.len();
                Some(Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(lindera_token.text),
                    char_index: 0,
                    byte_start: char_start,
                    byte_end: char_end,
                })
            }))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ja_simple() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let en_tokens = Lindera.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        println!("{:?}", en_tokens);
        assert_eq!(
            en_tokens,
            ["The", " ", "quick", " ", "(\"", "brown", "\")", " ", "fox", " ", "can", "\'", "t", " ", "jump", " ", "32", ".", "3", " ", "feet", ",", " ", "right", "?", " ", "Brr", ",", " ", "it", "\'", "s", " ", "29", ".", "3", "°", "F", "!"]
        );

        let orig = "関西国際空港限定トートバッグ";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let ja_tokens = Lindera.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        println!("{:?}", ja_tokens);
        assert_eq!(
            ja_tokens,
            ["関西国際空港", "限定", "トートバッグ"]
        );
    }

    #[test]
    fn test_ko_simple() {
        // TODO: need to build dictionary
        let orig = "한글 형태소분석기 테스트 중 입니다.";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let ko_tokens = Lindera.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        println!("{:?}", ko_tokens);
    }

}
