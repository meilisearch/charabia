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
        let original_byte_len = s.original.len();
        let mut original = s.original.char_indices()
            // map only byte indices
            .map(|(byte_index, _)| byte_index)
            // add ending byte index
            .chain(std::iter::once(original_byte_len));

        TokenStream {
            inner: Box::new(tokenized.into_iter().scan(0, move |byte_index, lindera_token| {
                println!("lindera token text: {:?}", lindera_token.text);
                
                let char_start = 0;
                let char_end = lindera_token.detail.len();
                let byte_start = *byte_index;
                let byte_end = match *byte_index {
                    0 => original.nth(char_end),
                    _ => original.nth(char_end - char_start - 1),
                };

                #[cfg(test)]
                let byte_end = byte_end.unwrap();

                #[cfg(not(test))]
                let byte_end = match byte_end {
                    Some(byte_end) => byte_end,
                    None => return None
                };

                *byte_index = byte_end;

                Some(Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(lindera_token.text),
                    char_index: char_start,
                    byte_start,
                    byte_end,
                })
            }))
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ko_ja_simple() {
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let tokens = Lindera.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        println!("{:?}", tokens);

        let orig = "関西国際空港限定トートバッグ";
        let processed = ProcessedText {
            original: orig,
            processed: Cow::Borrowed(orig),
        };
        let tokens = Lindera.tokenize(&processed).map(|Token { word, .. }| word.to_owned()).collect::<Vec<_>>();
        println!("{:?}", tokens);
    }

}
