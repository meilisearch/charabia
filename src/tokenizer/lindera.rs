use std::borrow::Cow;

use crate::{Token, TokenKind};
use crate::processors::ProcessedText;
use super::{Tokenizer, TokenStream};

use lindera::tokenizer::Tokenizer as LinderaTokenizer;


pub struct Lindera {
    pub tokenizer: LinderaTokenizer
}

impl Tokenizer for Lindera {
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a> {
        let tokenized = self.tokenizer.clone().tokenize(&s.processed);

        TokenStream {
            inner: Box::new(tokenized.into_iter().scan((0, 0), move |(char_index, byte_index), lindera_token| {
                let char_count = lindera_token.text.chars().count();
                let char_start = *char_index;

                let byte_count = lindera_token.text.len();
                let byte_start = *byte_index;

                *char_index += char_count;
                *byte_index += byte_count;

                Some(Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(lindera_token.text),
                    char_index: char_start,
                    byte_start,
                    byte_end: *byte_index,
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

        
        let en_tokens = Lindera { normal_mode: true, dict: "" }.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        
        assert_eq!(
            en_tokens,
            ["The", " ", "quick", " ", "(\"", "brown", "\")", " ", "fox", " ", "can", "\'", "t", " ", "jump", " ", "32", ".", "3", " ", "feet", ",", " ", "right", "?", " ", "Brr", ",", " ", "it", "\'", "s", " ", "29", ".", "3", "°", "F", "!"]
        );

        let orig = "関西国際空港限定トートバッグ";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let ja_tokens = Lindera { normal_mode: true, dict: "" }.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        
        assert_eq!(
            ja_tokens,
            ["関西国際空港", "限定", "トートバッグ"]
        );
    }

    #[test]
    fn test_ko_simple() {
        // TODO: need to build dictionary
        let orig = "한글형태소분석기 테스트 중 입니다.";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let ko_tokens = Lindera { normal_mode: true, dict: "" }.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        println!("{:?}", ko_tokens);
    }

}
