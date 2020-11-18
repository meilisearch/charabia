use std::borrow::Cow;
use crate::{Token, TokenKind};
use super::InternalTokenizer;
use jieba_rs::Jieba as JiebaTokenizer;
use jieba_rs::Token as JiebaToken;
// use deunicode::deunicode;

pub struct Jieba {
    jieba: JiebaTokenizer,
}

impl<'a> InternalTokenizer<'a> for Jieba {
    type Output = JiebaIterator<'a>;
    fn tokenize(&self, s: &'a str) -> Self::Output {
        let tokenized = self.jieba.tokenize(s, jieba_rs::TokenizeMode::Default, false);

        JiebaIterator(
            Box::new(tokenized.into_iter().scan(0, |state, jieba_token| {
                let old_state = *state;
                *state = *state + jieba_token.word.as_bytes().len();

                Some(Token {
                    kind: TokenKind::Word,
                    word: Cow::Borrowed(jieba_token.word),
                    byte_start: old_state,
                    byte_end: *state,
                })
            }))
        )
    }
}

impl Default for Jieba {
    fn default() -> Self { Jieba { jieba: JiebaTokenizer::new() } }
}

pub struct JiebaIterator<'a>(Box<dyn Iterator<Item = Token<'a>> + 'a>);

impl<'a> JiebaIterator<'a> {
    fn normalize(s: &'a str) -> Cow<str> {
        s.into()
    }
}

impl<'a> Iterator for JiebaIterator<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let tokenizer = Jieba::default();
        let orig = "The quick (\"brown\") fox can't jump 32.3 feet, right? Brr, it's 29.3°F!";
        let tokens = tokenizer.tokenize(orig);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
        
        let orig = "為一包含一千多萬目詞的帶標記平衡語料庫";
        let tokens = tokenizer.tokenize(orig);
        assert_eq!(orig, tokens.map(|t| &orig[t.byte_start..t.byte_end]).collect::<String>());
        let tokens = tokenizer.tokenize(orig);
        println!("{:#?}", tokens.collect::<Vec<_>>());
    }
}
