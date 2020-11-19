use std::borrow::Cow;

use jieba_rs::Jieba as JiebaTokenizer;

use crate::{Token, TokenKind};
use crate::processors::ProcessedText;
use super::{InternalTokenizer, TokenStream};

pub struct Jieba {
    jieba: JiebaTokenizer,
}

impl InternalTokenizer for Jieba {
    fn tokenize<'a>(&self, s: &'a ProcessedText<'a>) -> TokenStream<'a> {
        let tokenized = self.jieba.tokenize(&s.processed, jieba_rs::TokenizeMode::Default, false);

        let original_byte_len = s.original.len();
        let mut original = s.original.char_indices()
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
                    None => return None
                };

                *byte_index = byte_end;

                Some(Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(jieba_token.word),
                    char_index: char_start,
                    byte_start,
                    byte_end,
                })
            }))
        }
    }
}

impl Default for Jieba {
    fn default() -> Self { Jieba { jieba: JiebaTokenizer::new() } }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let tokenizer = Jieba::default();
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
        let tokens = tokenizer.tokenize(&processed);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
    }
}
